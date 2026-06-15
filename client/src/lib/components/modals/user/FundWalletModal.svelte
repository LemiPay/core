<script lang="ts">
	import {
		getBalance,
		getGasPrice,
		getPublicClient,
		readContract,
		waitForTransactionReceipt
	} from '@wagmi/core';
	import { estimateContractGas } from 'viem/actions';
	import { formatUnits, parseUnits } from 'viem';
	import { sepolia } from '@reown/appkit/networks';
	import { env } from '$env/dynamic/public';
	import { formatAmount } from '$lib/utils/format_utils';
	import { shortenAddress } from '$lib/utils/address_utils';
	import {
		calculateFundBreakdown,
		calculateFundGasEstimate,
		DEFAULT_FUND_FEE_BPS,
		formatEthGasCost,
		formatTokenBalance,
		hasEnoughEthForGas,
		parseBlockchainTxError,
		parseGasEstimateWarning,
		walletAddressToBytes32
	} from '$lib/utils/blockchain_utils';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import CurrencySelectField from '$lib/components/input_fields/CurrencySelectField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import {
		authActions,
		walletAuthState,
		wagmiAdapter
	} from '../../../../routes/wallet_auth.svelte';

	const vaultAbi = [
		{
			type: 'function',
			name: 'feeBps',
			inputs: [],
			outputs: [{ type: 'uint256' }],
			stateMutability: 'view'
		},
		{
			type: 'function',
			name: 'fund',
			inputs: [
				{ type: 'bytes32', name: 'walletAddress', internalType: 'bytes32' },
				{ type: 'address', name: 'token', internalType: 'address' },
				{ type: 'uint256', name: 'amount', internalType: 'uint256' }
			],
			outputs: [],
			stateMutability: 'nonpayable'
		}
	] as const;

	const erc20Abi = [
		{
			type: 'function',
			name: 'balanceOf',
			inputs: [{ type: 'address', name: 'account' }],
			outputs: [{ type: 'uint256' }],
			stateMutability: 'view'
		},
		{
			type: 'function',
			name: 'allowance',
			inputs: [
				{ type: 'address', name: 'owner' },
				{ type: 'address', name: 'spender' }
			],
			outputs: [{ type: 'uint256' }],
			stateMutability: 'view'
		},
		{
			type: 'function',
			name: 'approve',
			inputs: [
				{ type: 'address', name: 'spender' },
				{ type: 'uint256', name: 'value' }
			],
			outputs: [{ type: 'bool' }],
			stateMutability: 'nonpayable'
		}
	] as const;

	const TOKEN_CONFIG: Record<string, { address: string; decimals: number }> = {
		USDC: { address: '0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238', decimals: 6 }
	};

	interface Props {
		open: boolean;
		wallet_id: string;
		wallet_address: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, wallet_id, wallet_address, onclose, onsuccess }: Props = $props();

	const form = new ModalState();
	let selectedTicker = $state('');
	let amount = $state('');

	const tokenConfig = $derived(TOKEN_CONFIG[selectedTicker]);

	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const formValid = $derived(selectedTicker !== '' && amountValid && !!tokenConfig);

	let txPhase = $state<'idle' | 'approving' | 'funding'>('idle');
	let connectedAddress = $state<string | null>(null);
	let tokenBalanceRaw = $state<bigint | null>(null);
	let ethBalanceRaw = $state<bigint | null>(null);
	let feeBps = $state<bigint>(DEFAULT_FUND_FEE_BPS);
	let needsApproval = $state(false);
	let approveGasWei = $state<bigint | null>(null);
	let fundGasWei = $state<bigint | null>(null);
	let totalGasWei = $state<bigint | null>(null);
	let gasEstimateApproximate = $state(false);
	let loadingQuote = $state(false);
	let quoteWarning = $state('');

	const grossAmountStr = $derived(String(amount).replace(',', '.'));
	const breakdown = $derived.by(() => {
		if (!amountValid || !tokenConfig) return null;
		try {
			const grossUnits = parseUnits(grossAmountStr, tokenConfig.decimals);
			return calculateFundBreakdown(grossUnits, feeBps);
		} catch {
			return null;
		}
	});

	const feePercentLabel = $derived(
		breakdown?.feePercentLabel ?? `${Number(DEFAULT_FUND_FEE_BPS) / 100}%`
	);

	const approveButtonLabel = $derived(
		txPhase === 'approving' ? 'Aprobando...' : `Aprobar ${selectedTicker || 'token'}`
	);
	const fundButtonLabel = $derived(txPhase === 'funding' ? 'Fondeando...' : 'Fondear');
	const canApprove = $derived(
		formValid && needsApproval && !form.loading && !loadingQuote && txPhase === 'idle'
	);
	const canFund = $derived(
		formValid && !needsApproval && !form.loading && !loadingQuote && txPhase === 'idle'
	);

	const QUICK_AMOUNTS = [10, 25, 50, 100];

	function setQuickAmount(value: number) {
		amount = String(value);
	}

	function setMaxAmount() {
		if (tokenBalanceRaw === null || !tokenConfig) return;
		amount = formatUnits(tokenBalanceRaw, tokenConfig.decimals);
	}

	function resetQuote() {
		connectedAddress = null;
		tokenBalanceRaw = null;
		ethBalanceRaw = null;
		feeBps = DEFAULT_FUND_FEE_BPS;
		needsApproval = false;
		approveGasWei = null;
		fundGasWei = null;
		totalGasWei = null;
		gasEstimateApproximate = false;
		quoteWarning = '';
	}

	async function loadFundQuote() {
		const config = tokenConfig;
		const vaultAddress = env.PUBLIC_VAULT_CONTRACT_ADDRESS as `0x${string}` | undefined;
		if (!open || !selectedTicker || !config || !vaultAddress) {
			resetQuote();
			return;
		}

		const address = await authActions.ensureWalletReadyForTx();
		if (!address || !open) {
			resetQuote();
			return;
		}

		loadingQuote = true;
		quoteWarning = '';
		const chainId = sepolia.id;
		const account = address as `0x${string}`;
		const tokenAddress = config.address as `0x${string}`;

		try {
			const [tokenBalance, ethBalance, onChainFeeBps, currentAllowance] = await Promise.all([
				readContract(wagmiAdapter.wagmiConfig, {
					address: tokenAddress,
					abi: erc20Abi,
					functionName: 'balanceOf',
					args: [account],
					chainId
				}),
				getBalance(wagmiAdapter.wagmiConfig, { address: account, chainId }),
				readContract(wagmiAdapter.wagmiConfig, {
					address: vaultAddress,
					abi: vaultAbi,
					functionName: 'feeBps',
					chainId
				}).catch(() => DEFAULT_FUND_FEE_BPS),
				readContract(wagmiAdapter.wagmiConfig, {
					address: tokenAddress,
					abi: erc20Abi,
					functionName: 'allowance',
					args: [account, vaultAddress],
					chainId
				})
			]);

			connectedAddress = address;
			tokenBalanceRaw = tokenBalance;
			ethBalanceRaw = ethBalance.value;
			feeBps = onChainFeeBps;
			quoteWarning = '';

			if (!amountValid) {
				needsApproval = false;
				approveGasWei = null;
				fundGasWei = null;
				totalGasWei = null;
				return;
			}

			const grossUnits = parseUnits(grossAmountStr, config.decimals);
			const requiresApproval = currentAllowance < grossUnits;
			needsApproval = requiresApproval;

			const gasPrice = await getGasPrice(wagmiAdapter.wagmiConfig, { chainId });
			const walletAddressBytes32 = walletAddressToBytes32(wallet_address);

			let approveGasUnits: bigint | undefined;
			let fundGasUnits: bigint | undefined;
			let gasIsApproximate = false;

			const client = getPublicClient(wagmiAdapter.wagmiConfig, { chainId });
			let gasEstimateError: unknown = null;

			if (client) {
				if (requiresApproval) {
					try {
						approveGasUnits = await estimateContractGas(client, {
							account,
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'approve',
							args: [vaultAddress, grossUnits]
						});
					} catch (err) {
						gasIsApproximate = true;
						gasEstimateError = err;
					}
				}

				if (!requiresApproval) {
					try {
						fundGasUnits = await estimateContractGas(client, {
							account,
							address: vaultAddress,
							abi: vaultAbi,
							functionName: 'fund',
							args: [walletAddressBytes32, tokenAddress, grossUnits]
						});
					} catch (err) {
						gasIsApproximate = true;
						gasEstimateError = err;
					}
				} else {
					gasIsApproximate = true;
				}
			} else {
				gasIsApproximate = true;
			}

			if (tokenBalance < grossUnits) {
				quoteWarning = `Saldo insuficiente de ${selectedTicker} en tu wallet. El gas mostrado es aproximado.`;
			} else if (requiresApproval && !gasEstimateError) {
				quoteWarning = parseGasEstimateWarning(null, {
					ticker: selectedTicker,
					needsApproval: true
				});
			} else if (gasIsApproximate) {
				quoteWarning = parseGasEstimateWarning(gasEstimateError, { ticker: selectedTicker });
			}

			const gasEstimate = calculateFundGasEstimate(gasPrice, requiresApproval, {
				approveGasUnits,
				fundGasUnits,
				isApproximate: gasIsApproximate
			});
			approveGasWei = gasEstimate.approveGasWei;
			fundGasWei = gasEstimate.fundGasWei;
			totalGasWei = gasEstimate.totalGasWei;
			gasEstimateApproximate = gasEstimate.isApproximate;

			if (tokenBalance < grossUnits && !quoteWarning) {
				quoteWarning = `Saldo insuficiente de ${selectedTicker}. Tenés ${formatTokenBalance(tokenBalance, config.decimals)} ${selectedTicker} y querés fondear ${grossAmountStr} ${selectedTicker}.`;
			} else if (
				!hasEnoughEthForGas(ethBalance.value, totalGasWei) &&
				!quoteWarning.includes('Saldo insuficiente')
			) {
				quoteWarning = `Puede que no tengas ETH suficiente para el gas (tenés ${formatTokenBalance(ethBalance.value, 18)} ETH, estimado ~${formatEthGasCost(totalGasWei)} ETH).`;
			}
		} catch {
			resetQuote();
			quoteWarning = 'No se pudo cargar el resumen. Verificá que tu wallet esté conectada.';
		} finally {
			loadingQuote = false;
		}
	}

	$effect(() => {
		if (!open || !selectedTicker) return;

		const amountKey = amountValid ? grossAmountStr : '';
		void amountKey;
		void loadFundQuote();
	});

	function handleClose() {
		amount = '';
		selectedTicker = '';
		txPhase = 'idle';
		resetQuote();
		form.reset();
		onclose();
	}

	async function ensureConnectedAddress(): Promise<string | null> {
		let userAddress = await authActions.ensureWalletReadyForTx();
		if (userAddress) return userAddress;

		await authActions.openLogin();
		userAddress = await authActions.ensureWalletReadyForTx();
		if (!userAddress) {
			form.error = 'Necesitás conectar tu wallet para continuar.';
		}
		return userAddress;
	}

	type TxContext =
		| {
				ok: true;
				vaultAddress: `0x${string}`;
				tokenAddress: `0x${string}`;
				account: `0x${string}`;
				parsedUnits: bigint;
				parsedAmount: string;
				config: { address: string; decimals: number };
		  }
		| { ok: false; message: string; status: number };

	async function buildTxContext(userAddress: string, parsedAmount: string): Promise<TxContext> {
		const config = tokenConfig;
		if (!config) {
			return {
				ok: false,
				status: 400,
				message: `Token ${selectedTicker} no soportado para fondeo on-chain.`
			};
		}

		const vaultAddress = env.PUBLIC_VAULT_CONTRACT_ADDRESS as `0x${string}` | undefined;
		if (!vaultAddress) {
			return { ok: false, status: 500, message: 'Contrato del vault no configurado.' };
		}

		return {
			ok: true,
			vaultAddress,
			tokenAddress: config.address as `0x${string}`,
			account: userAddress as `0x${string}`,
			parsedUnits: parseUnits(parsedAmount, config.decimals),
			parsedAmount,
			config
		};
	}

	function insufficientFundsMessage(
		tokenBalance: bigint,
		ethBalanceWei: bigint,
		parsedUnits: bigint,
		ticker: string,
		parsedAmount: string,
		decimals: number,
		requiredGasWei?: bigint | null
	): string | null {
		if (!hasEnoughEthForGas(ethBalanceWei, requiredGasWei ?? undefined)) {
			const eth = formatTokenBalance(ethBalanceWei, 18);
			const required = requiredGasWei
				? formatEthGasCost(requiredGasWei)
				: formatEthGasCost(100_000_000_000_000n);
			return `No tenés ETH en Sepolia para pagar el gas (tenés ${eth} ETH, estimado ~${required} ETH). Conseguí ETH de prueba en un faucet de Sepolia.`;
		}

		if (tokenBalance < parsedUnits) {
			const available = formatUnits(tokenBalance, decimals);
			return `Saldo insuficiente de ${ticker}. Tenés ${available} ${ticker} en tu wallet y querés fondear ${parsedAmount} ${ticker}.`;
		}

		return null;
	}

	async function handleApprove() {
		form.setAttempted();
		if (!canApprove) return;

		const userAddress = await ensureConnectedAddress();
		if (!userAddress) return;

		const parsedAmount = String(amount).replace(',', '.');
		txPhase = 'approving';

		await form.submit(
			async () => {
				const txCtx = await authActions.getWagmiTxContext();
				if (!txCtx) {
					return {
						ok: false as const,
						status: 400,
						message:
							'No se pudo conectar la wallet para firmar. Si usás Google/Reown, cerrá sesión y volvé a iniciar.',
						body: null
					};
				}

				const ctx = await buildTxContext(txCtx.address, parsedAmount);
				if (!ctx.ok) {
					return { ok: false as const, status: ctx.status, message: ctx.message, body: null };
				}

				const chainId = sepolia.id;
				const { vaultAddress, tokenAddress, account, parsedUnits, config } = ctx;

				try {
					const [tokenBalance, ethBalance, currentAllowance] = await Promise.all([
						readContract(wagmiAdapter.wagmiConfig, {
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'balanceOf',
							args: [account],
							chainId
						}),
						getBalance(wagmiAdapter.wagmiConfig, { address: account, chainId }),
						readContract(wagmiAdapter.wagmiConfig, {
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'allowance',
							args: [account, vaultAddress],
							chainId
						})
					]);

					if (currentAllowance >= parsedUnits) {
						return {
							ok: false as const,
							status: 400,
							message: `El gasto de ${selectedTicker} ya está aprobado. Podés fondear directamente.`,
							body: null
						};
					}

					const fundsError = insufficientFundsMessage(
						tokenBalance,
						ethBalance.value,
						parsedUnits,
						selectedTicker,
						parsedAmount,
						config.decimals,
						approveGasWei ?? totalGasWei
					);
					if (fundsError) {
						return { ok: false as const, status: 400, message: fundsError, body: null };
					}

					const approveHash = await authActions.writeSepoliaContract({
						address: tokenAddress,
						abi: erc20Abi,
						functionName: 'approve',
						args: [vaultAddress, parsedUnits]
					});
					await waitForTransactionReceipt(wagmiAdapter.wagmiConfig, { hash: approveHash, chainId });
				} catch (err: unknown) {
					return {
						ok: false as const,
						status: 400,
						message: parseBlockchainTxError(err, {
							ticker: selectedTicker,
							functionName: 'approve'
						}),
						body: null
					};
				}

				return { ok: true as const, status: 200, body: null, message: '' };
			},
			{
				successMsg: `Aprobación de ${selectedTicker} exitosa. Ahora ejecutá el fondeo.`,
				onSuccess: () => loadFundQuote()
			}
		);

		txPhase = 'idle';
	}

	async function handleFund() {
		form.setAttempted();
		if (!canFund) return;

		const userAddress = await ensureConnectedAddress();
		if (!userAddress) return;

		const parsedAmount = String(amount).replace(',', '.');
		txPhase = 'funding';

		await form.submit(
			async () => {
				const txCtx = await authActions.getWagmiTxContext();
				if (!txCtx) {
					return {
						ok: false as const,
						status: 400,
						message:
							'No se pudo conectar la wallet para firmar. Si usás Google/Reown, cerrá sesión y volvé a iniciar.',
						body: null
					};
				}

				const ctx = await buildTxContext(txCtx.address, parsedAmount);
				if (!ctx.ok) {
					return { ok: false as const, status: ctx.status, message: ctx.message, body: null };
				}

				const chainId = sepolia.id;
				const { vaultAddress, tokenAddress, account, parsedUnits, config } = ctx;

				try {
					const [tokenBalance, ethBalance, currentAllowance] = await Promise.all([
						readContract(wagmiAdapter.wagmiConfig, {
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'balanceOf',
							args: [account],
							chainId
						}),
						getBalance(wagmiAdapter.wagmiConfig, { address: account, chainId }),
						readContract(wagmiAdapter.wagmiConfig, {
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'allowance',
							args: [account, vaultAddress],
							chainId
						})
					]);

					if (currentAllowance < parsedUnits) {
						return {
							ok: false as const,
							status: 400,
							message: `Primero tenés que aprobar el gasto de ${selectedTicker} antes de fondear.`,
							body: null
						};
					}

					const fundsError = insufficientFundsMessage(
						tokenBalance,
						ethBalance.value,
						parsedUnits,
						selectedTicker,
						parsedAmount,
						config.decimals,
						fundGasWei ?? totalGasWei
					);
					if (fundsError) {
						return { ok: false as const, status: 400, message: fundsError, body: null };
					}

					const walletAddressBytes32 = walletAddressToBytes32(wallet_address);
					const fundHash = await authActions.writeSepoliaContract({
						address: vaultAddress,
						abi: vaultAbi,
						functionName: 'fund',
						args: [walletAddressBytes32, tokenAddress, parsedUnits]
					});
					await waitForTransactionReceipt(wagmiAdapter.wagmiConfig, { hash: fundHash, chainId });
				} catch (err: unknown) {
					return {
						ok: false as const,
						status: 400,
						message: parseBlockchainTxError(err, {
							ticker: selectedTicker,
							functionName: 'fund'
						}),
						body: null
					};
				}

				return { ok: true as const, status: 200, body: null, message: '' };
			},
			{
				successMsg: '¡Fondeo realizado exitosamente!',
				onSuccess: () => {
					onsuccess();
					handleClose();
				}
			}
		);

		txPhase = 'idle';
	}
</script>

<Modal
	description="Primero aprobás el gasto de tokens y luego se ejecuta el fondeo."
	error={form.error}
	loading={form.loading}
	onclose={handleClose}
	{open}
	panelClass="w-[80vw] max-w-[70vw]"
	success={form.success}
	title="Fondear wallet"
>
	{#snippet children()}
		<div class="max-h-[min(70vh,40rem)] overflow-y-auto pr-1 md:max-h-none md:overflow-visible">
			<div class="grid gap-5 md:grid-cols-[1fr_2fr] md:items-stretch md:gap-8 lg:gap-10">
				<div
					class="flex h-full min-h-[18rem] flex-col gap-4 rounded-2xl border border-border/60 bg-muted/20 p-4 md:min-h-0 md:p-5"
				>
					{#if connectedAddress}
						<div class="flex flex-wrap items-center gap-2">
							<span
								class="inline-flex items-center gap-1.5 rounded-full border border-border/70 bg-muted/50 px-2.5 py-1 text-xs text-muted-foreground"
							>
								<span class="size-1.5 rounded-full bg-emerald-500"></span>
								{shortenAddress(connectedAddress)}
							</span>
							<span
								class="rounded-full border border-violet-200 bg-violet-50 px-2.5 py-1 text-xs font-medium text-violet-700 dark:border-violet-400/20 dark:bg-violet-400/10 dark:text-violet-300"
							>
								Sepolia
							</span>
							{#if walletAuthState.accountType}
								<span
									class="rounded-full border border-border/70 bg-muted/50 px-2.5 py-1 text-xs font-medium text-muted-foreground"
								>
									{walletAuthState.accountType === 'smartAccount' ? 'Smart Account' : 'EOA'}
								</span>
							{/if}
						</div>
						{#if walletAuthState.accountType === 'smartAccount'}
							<p class="text-[11px] leading-relaxed text-amber-700 dark:text-amber-400">
								Fondeá USDC y ETH en esta dirección exacta. Si usaste otra wallet, el saldo no
								aplica acá.
							</p>
						{/if}
					{/if}

					{#if wallet_address}
						<div class="rounded-xl border border-border/60 bg-muted/30 px-3 py-2.5">
							<p class="text-[11px] font-medium text-muted-foreground">Wallet destino en Lemipay</p>
							<p class="mt-0.5 font-mono text-sm font-medium text-foreground">
								{shortenAddress(wallet_address)}
							</p>
						</div>
					{/if}

					<CurrencySelectField
						id="fund-currency"
						label="Moneda"
						bind:value={selectedTicker}
						attempted={form.attempted}
					/>

					<NumberField
						id="fund-amount"
						label="Monto"
						min={0.0001}
						placeholder="Ej. 10.00"
						bind:value={amount}
						attempted={form.attempted}
					/>

					{#if selectedTicker && tokenBalanceRaw !== null}
						<div class="flex flex-wrap gap-2">
							{#each QUICK_AMOUNTS as quickAmount}
								<button
									type="button"
									onclick={() => setQuickAmount(quickAmount)}
									class="rounded-lg border border-border/70 bg-background px-3 py-1.5 text-xs font-medium text-foreground transition hover:border-lime-300 hover:bg-lime-50 dark:hover:border-lime-400/30 dark:hover:bg-lime-400/10"
								>
									{quickAmount}
								</button>
							{/each}
							<button
								type="button"
								onclick={setMaxAmount}
								class="rounded-lg border border-lime-200 bg-lime-50 px-3 py-1.5 text-xs font-semibold text-lime-800 transition hover:bg-lime-100 dark:border-lime-400/25 dark:bg-lime-400/10 dark:text-lime-300 dark:hover:bg-lime-400/15"
							>
								Máx
							</button>
						</div>
					{/if}

					{#if amountValid && (needsApproval || selectedTicker)}
						<div
							class="rounded-xl border border-amber-200/80 bg-amber-50/80 px-3 py-2.5 text-xs leading-relaxed text-amber-900 dark:border-amber-400/20 dark:bg-amber-400/10 dark:text-amber-200"
						>
							<p class="font-semibold">2 pasos en tu wallet</p>
							<div class="mt-2 space-y-1.5">
								<p class="flex items-center gap-2 text-amber-800/90 dark:text-amber-200/90">
									<span
										class="inline-flex size-5 shrink-0 items-center justify-center rounded-full text-[10px] font-bold {needsApproval
											? 'bg-amber-200 text-amber-900 dark:bg-amber-400/25 dark:text-amber-100'
											: 'bg-emerald-200 text-emerald-900 dark:bg-emerald-400/25 dark:text-emerald-100'}"
									>
										{needsApproval ? '1' : '✓'}
									</span>
									Aprobá el gasto de {selectedTicker}
								</p>
								<p class="flex items-center gap-2 text-amber-800/90 dark:text-amber-200/90">
									<span
										class="inline-flex size-5 shrink-0 items-center justify-center rounded-full text-[10px] font-bold {needsApproval
											? 'bg-muted text-muted-foreground'
											: 'bg-amber-200 text-amber-900 dark:bg-amber-400/25 dark:text-amber-100'}"
									>
										2
									</span>
									Ejecutá el fondeo en Lemipay
								</p>
							</div>
						</div>
					{/if}

					<div class="hidden min-h-0 flex-1 md:block">
						<p class="text-xs leading-relaxed text-muted-foreground">
							Elegí la moneda y el monto. A la derecha vas a ver cuánto gastás en tokens, cuánto en
							gas y cuánto se acredita en Lemipay.
						</p>
					</div>

					<div class="mt-auto flex shrink-0 flex-wrap gap-2 border-t border-border/50 pt-4">
						<Button label="Cancelar" variant="secondary" onclick={handleClose} />

						{#if needsApproval}
							<Button
								label={approveButtonLabel}
								onclick={handleApprove}
								disabled={!canApprove}
								loading={form.loading && txPhase === 'approving'}
							/>
							<Button label={fundButtonLabel} variant="secondary" disabled={true} />
						{:else if formValid}
							<Button
								label={fundButtonLabel}
								onclick={handleFund}
								disabled={!canFund}
								loading={form.loading && txPhase === 'funding'}
							/>
						{/if}
					</div>
				</div>

				<div
					class="flex h-full min-h-[18rem] flex-col overflow-hidden rounded-2xl border border-lime-200/70 bg-linear-to-br from-lime-50/90 via-background to-emerald-50/50 shadow-sm md:min-h-0 dark:border-lime-400/20 dark:from-lime-400/5 dark:to-emerald-400/5"
				>
					<div
						class="shrink-0 border-b border-lime-200/50 bg-lime-500/8 px-4 py-2 dark:border-lime-400/15 dark:bg-lime-400/10"
					>
						<p
							class="text-xs font-semibold tracking-wide text-lime-800 uppercase dark:text-lime-300"
						>
							Resumen de la operación
						</p>
					</div>

					<div class="flex flex-1 flex-col gap-3 px-4 py-3">
						{#if !selectedTicker}
							<div class="flex flex-1 items-center justify-center py-8 text-center">
								<p class="text-sm text-muted-foreground">
									Elegí una moneda para ver el resumen del fondeo.
								</p>
							</div>
						{:else if loadingQuote}
							<div
								class="flex flex-1 items-center justify-center gap-2 text-sm text-muted-foreground"
							>
								<div
									class="size-4 animate-spin rounded-full border-2 border-lime-300 border-t-lime-600"
								></div>
								Calculando saldos y costos...
							</div>
						{:else if tokenBalanceRaw !== null && ethBalanceRaw !== null}
							<div class="flex flex-1 flex-col gap-3">
								<div class="grid grid-cols-2 gap-2">
									<div class="rounded-xl border border-border/60 bg-background/80 px-3 py-2">
										<p class="text-[11px] font-medium text-muted-foreground">
											Tu {selectedTicker}
										</p>
										<p class="mt-0.5 text-sm font-semibold text-foreground tabular-nums">
											{formatTokenBalance(tokenBalanceRaw, tokenConfig?.decimals ?? 6)}
											<span class="text-xs font-medium text-muted-foreground">{selectedTicker}</span
											>
										</p>
									</div>
									<div class="rounded-xl border border-border/60 bg-background/80 px-3 py-2">
										<p class="text-[11px] font-medium text-muted-foreground">Tu ETH (Sepolia)</p>
										<p class="mt-0.5 text-sm font-semibold text-foreground tabular-nums">
											{formatTokenBalance(ethBalanceRaw, 18)}
											<span class="text-xs font-medium text-muted-foreground">ETH</span>
										</p>
									</div>
								</div>

								{#if amountValid && breakdown}
									<div class="space-y-2 border-t border-lime-200/40 pt-3 dark:border-lime-400/15">
										<div class="flex items-center justify-between gap-3 text-sm">
											<span class="text-muted-foreground">Vas a gastar en {selectedTicker}</span>
											<span class="shrink-0 font-semibold tabular-nums"
												>{formatAmount(grossAmountStr)} {selectedTicker}</span
											>
										</div>

										<div class="flex items-center justify-between gap-3 text-sm">
											<span class="text-muted-foreground">Comisión ({feePercentLabel})</span>
											<span
												class="shrink-0 font-medium text-amber-700 tabular-nums dark:text-amber-400"
												>-{formatAmount(
													formatUnits(breakdown.feeUnits, tokenConfig?.decimals ?? 6)
												)}
												{selectedTicker}</span
											>
										</div>

										<div
											class="flex items-center justify-between gap-3 rounded-xl border border-emerald-200/70 bg-emerald-50/80 px-3 py-2 dark:border-emerald-400/20 dark:bg-emerald-400/10"
										>
											<span class="text-sm font-medium text-emerald-800 dark:text-emerald-300"
												>Vas a recibir en Lemipay</span
											>
											<span
												class="shrink-0 text-sm font-bold text-emerald-700 tabular-nums dark:text-emerald-300"
												>{formatAmount(formatUnits(breakdown.netUnits, tokenConfig?.decimals ?? 6))}
												{selectedTicker}</span
											>
										</div>

										<div class="space-y-1 pt-0.5">
											<div class="flex items-center justify-between gap-2">
												<p
													class="text-[11px] font-semibold tracking-wide text-muted-foreground uppercase"
												>
													Vas a gastar en ETH (gas)
												</p>
												{#if gasEstimateApproximate}
													<span
														class="rounded-full bg-amber-100 px-2 py-0.5 text-[10px] font-medium text-amber-800 dark:bg-amber-400/15 dark:text-amber-300"
														>aprox.</span
													>
												{/if}
											</div>
											{#if totalGasWei !== null}
												<div class="flex items-center justify-between gap-3 text-sm">
													<span class="text-muted-foreground">
														{needsApproval ? 'Aprobación + fondeo' : 'Solo fondeo'}
													</span>
													<span class="shrink-0 font-semibold tabular-nums"
														>~{formatEthGasCost(totalGasWei)} ETH</span
													>
												</div>
												{#if needsApproval && approveGasWei !== null && fundGasWei !== null}
													<div class="flex justify-between pl-2 text-xs text-muted-foreground">
														<span>Aprobación</span>
														<span class="tabular-nums">~{formatEthGasCost(approveGasWei)} ETH</span>
													</div>
													<div class="flex justify-between pl-2 text-xs text-muted-foreground">
														<span>Fondeo</span>
														<span class="tabular-nums">~{formatEthGasCost(fundGasWei)} ETH</span>
													</div>
												{/if}
											{/if}
										</div>
									</div>

									<div
										class="mt-auto space-y-3 border-t border-lime-200/40 pt-3 dark:border-lime-400/15"
									>
										{#if quoteWarning}
											<p class="text-xs leading-relaxed text-amber-700 dark:text-amber-400">
												{quoteWarning}
											</p>
										{/if}

										<div
											class="rounded-xl border border-lime-300/60 bg-lime-50/80 px-3 py-2.5 shadow-sm dark:border-lime-400/25 dark:bg-lime-400/10"
										>
											<p
												class="text-[11px] font-medium text-lime-800/80 uppercase dark:text-lime-300/80"
											>
												Resumen total
											</p>
											<p class="mt-1 text-sm font-semibold text-foreground tabular-nums">
												{formatAmount(grossAmountStr)}
												{selectedTicker}
												{#if totalGasWei !== null}
													<span class="font-normal text-muted-foreground">
														+ ~{formatEthGasCost(totalGasWei)} ETH
													</span>
												{/if}
											</p>
											<p class="mt-0.5 text-xs text-muted-foreground">
												Crédito Lemipay:
												<span class="font-semibold text-emerald-700 dark:text-emerald-300">
													{formatAmount(
														formatUnits(breakdown.netUnits, tokenConfig?.decimals ?? 6)
													)}
													{selectedTicker}
												</span>
											</p>
										</div>
									</div>
								{:else if !connectedAddress}
									<div class="flex flex-1 items-center justify-center py-6">
										<p class="text-center text-sm text-muted-foreground">
											Conectá tu wallet para ver saldos y el desglose del fondeo.
										</p>
									</div>
								{:else}
									<div class="flex flex-1 items-center justify-center py-6">
										<p class="text-center text-sm text-muted-foreground">
											Ingresá un monto para ver cuánto gastás y cuánto recibís en Lemipay.
										</p>
									</div>
								{/if}
							</div>
						{:else}
							<div class="flex flex-1 items-center justify-center py-6">
								<p class="text-center text-sm text-muted-foreground">
									Conectá tu wallet para ver saldos y el desglose del fondeo.
								</p>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/snippet}
</Modal>
