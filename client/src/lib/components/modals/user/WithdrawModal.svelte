<script lang="ts">
	import { signMessage } from '@wagmi/core';
	import { getAddress } from 'viem';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';
	import { ModalState } from '$lib/utils/modal_state.svelte';
	import { formatAmount, truncateToDecimals, DISPLAY_DECIMALS } from '$lib/utils/format_utils';
	import { shortenAddress } from '$lib/utils/address_utils';
	import {
		authActions,
		walletAuthState,
		wagmiAdapter
	} from '../../../../routes/wallet_auth.svelte';
	import { isSuccess } from '$lib/types/client.types';
	import { requestWithdrawChallenge, withdrawFromWallet } from '$lib/api/endpoints/user_wallet';
	import { getWithdrawAppOrigin } from '$lib/utils/withdraw_utils';

	interface Props {
		open: boolean;
		wallet_id: string;
		wallet_address: string;
		ticker: string;
		balance: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, wallet_id, wallet_address, ticker, balance, onclose, onsuccess }: Props = $props();

	const form = new ModalState();
	let amount = $state('');
	let connectedAddress = $state<string | null>(null);
	let loadingWallet = $state(false);

	const grossAmountStr = $derived(String(amount).replace(',', '.'));
	const parsedAmount = $derived(Number(grossAmountStr));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const lemiPayBalanceNum = $derived(Number(String(balance).replace(',', '.')));
	const lemiPayBalanceValid = $derived(
		Number.isFinite(lemiPayBalanceNum) && lemiPayBalanceNum >= 0
	);
	const formValid = $derived(
		amountValid && lemiPayBalanceValid && parsedAmount <= lemiPayBalanceNum
	);

	const remainingBalance = $derived.by(() => {
		if (!amountValid || !lemiPayBalanceValid) return null;
		const next = truncateToDecimals(lemiPayBalanceNum - parsedAmount, DISPLAY_DECIMALS);
		return next < 0 ? 0 : next;
	});

	const quoteWarning = $derived.by(() => {
		if (!amountValid) return '';
		if (!lemiPayBalanceValid) return 'No se pudo leer el saldo disponible en Lemipay.';
		if (parsedAmount > lemiPayBalanceNum) {
			return `Saldo insuficiente en Lemipay. Tenés ${formatAmount(balance)} ${ticker} y querés retirar ${grossAmountStr} ${ticker}.`;
		}
		if (!connectedAddress) {
			return 'Conectá tu wallet con Reown para recibir los tokens en Sepolia.';
		}
		return '';
	});

	const QUICK_FRACTIONS = [0.25, 0.5, 0.75] as const;

	function setQuickFraction(fraction: number) {
		if (!lemiPayBalanceValid || lemiPayBalanceNum <= 0) return;
		const value = truncateToDecimals(lemiPayBalanceNum * fraction, DISPLAY_DECIMALS);
		amount = value > 0 ? String(value) : '';
	}

	function setMaxAmount() {
		if (!lemiPayBalanceValid) return;
		amount = String(truncateToDecimals(lemiPayBalanceNum, DISPLAY_DECIMALS));
	}

	function resetQuote() {
		connectedAddress = null;
	}

	async function loadWithdrawQuote() {
		if (!open) {
			resetQuote();
			return;
		}

		loadingWallet = true;
		try {
			const address = await authActions.ensureWalletReadyForTx();
			connectedAddress = address;
		} catch {
			connectedAddress = walletAuthState.address ?? null;
		} finally {
			loadingWallet = false;
		}
	}

	$effect(() => {
		if (!open) return;
		void loadWithdrawQuote();
	});

	function handleClose() {
		amount = '';
		resetQuote();
		form.reset();
		onclose();
	}

	async function handleWithdraw() {
		form.setAttempted();
		if (!formValid) return;

		if (!walletAuthState.isConnected) {
			await authActions.openLogin();
			await loadWithdrawQuote();
			if (!walletAuthState.isConnected) {
				form.error = 'Necesitás conectar tu wallet para firmar.';
				return;
			}
		}

		await form.submit(
			async () => {
				const parsedAmount = grossAmountStr;
				const txCtx = await authActions.getWagmiTxContext();
				const rawAddress = txCtx?.address ?? walletAuthState.address;
				if (!rawAddress) {
					return {
						ok: false as const,
						status: 400,
						message: 'Wallet no conectada.',
						body: null
					};
				}

				let userAddress: `0x${string}`;
				try {
					userAddress = getAddress(rawAddress);
				} catch {
					return {
						ok: false as const,
						status: 400,
						message: 'Dirección de wallet inválida.',
						body: null
					};
				}

				await authActions.ensureWalletReadyForTx();

				const appOrigin = getWithdrawAppOrigin();
				const challenge = await requestWithdrawChallenge(
					parsedAmount,
					wallet_id,
					userAddress,
					appOrigin
				);
				if (!isSuccess(challenge)) {
					return {
						ok: false as const,
						status: challenge.status,
						message: challenge.message || 'No se pudo preparar la firma del retiro.',
						body: null
					};
				}

				const message = challenge.body.message;

				let signature: string;
				try {
					signature = await signMessage(wagmiAdapter.wagmiConfig, { message });
				} catch (err: unknown) {
					const msg = err instanceof Error ? err.message : '';
					return {
						ok: false as const,
						status: 400,
						message:
							msg.includes('reject') || msg.includes('denied')
								? 'Firma cancelada.'
								: msg || 'Firma rechazada por el usuario.',
						body: null
					};
				}

				return await withdrawFromWallet(
					parsedAmount,
					wallet_id,
					signature,
					userAddress,
					appOrigin,
					message
				);
			},
			{
				successMsg: '¡Retiro realizado exitosamente!',
				onSuccess: () => {
					onsuccess();
					handleClose();
				}
			}
		);
	}
</script>

<Modal
	{open}
	title="Retirar fondos"
	description="Firmá un mensaje con tu wallet. Lemipay envía los tokens a tu dirección en Sepolia."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
	panelClass="w-[80vw] max-w-[70vw]"
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
					{:else if loadingWallet}
						<p class="text-xs text-muted-foreground">Conectando wallet...</p>
					{/if}

					{#if wallet_address}
						<div class="rounded-xl border border-border/60 bg-muted/30 px-3 py-2.5">
							<p class="text-[11px] font-medium text-muted-foreground">Wallet origen en Lemipay</p>
							<p class="mt-0.5 font-mono text-sm font-medium text-foreground">
								{shortenAddress(wallet_address)}
							</p>
						</div>
					{/if}

					<div>
						<label for="withdraw-ticker" class="mb-1.5 block text-sm font-medium text-foreground"
							>Moneda</label
						>
						<input
							id="withdraw-ticker"
							type="text"
							value={ticker}
							disabled
							class="w-full rounded-md border border-input bg-muted px-3 py-2 text-sm text-muted-foreground"
						/>
					</div>

					<NumberField
						id="withdraw-amount"
						label="Monto"
						min={0.0001}
						placeholder="Ej. 10.00"
						bind:value={amount}
						attempted={form.attempted}
					/>

					{#if lemiPayBalanceValid && lemiPayBalanceNum > 0}
						<div class="flex flex-wrap gap-2">
							{#each QUICK_FRACTIONS as fraction}
								<button
									type="button"
									onclick={() => setQuickFraction(fraction)}
									class="rounded-lg border border-border/70 bg-background px-3 py-1.5 text-xs font-medium text-foreground transition hover:border-rose-300 hover:bg-rose-50 dark:hover:border-rose-400/30 dark:hover:bg-rose-400/10"
								>
									{Math.round(fraction * 100)}%
								</button>
							{/each}
							<button
								type="button"
								onclick={setMaxAmount}
								class="rounded-lg border border-rose-200 bg-rose-50 px-3 py-1.5 text-xs font-semibold text-rose-800 transition hover:bg-rose-100 dark:border-rose-400/25 dark:bg-rose-400/10 dark:text-rose-300 dark:hover:bg-rose-400/15"
							>
								Máx
							</button>
						</div>
					{/if}

					{#if amountValid}
						<div
							class="rounded-xl border border-amber-200/80 bg-amber-50/80 px-3 py-2.5 text-xs leading-relaxed text-amber-900 dark:border-amber-400/20 dark:bg-amber-400/10 dark:text-amber-200"
						>
							<p class="font-semibold">1 paso en tu wallet</p>
							<p class="mt-2 flex items-center gap-2 text-amber-800/90 dark:text-amber-200/90">
								<span
									class="inline-flex size-5 shrink-0 items-center justify-center rounded-full bg-amber-200 text-[10px] font-bold text-amber-900 dark:bg-amber-400/25 dark:text-amber-100"
								>
									1
								</span>
								Firmá el mensaje para autorizar el retiro
							</p>
						</div>
					{/if}

					<div class="hidden min-h-0 flex-1 md:block">
						<p class="text-xs leading-relaxed text-muted-foreground">
							El retiro se debita de tu saldo en Lemipay. Los tokens llegan a tu wallet conectada en
							Sepolia.
						</p>
					</div>

					<div class="mt-auto flex shrink-0 flex-wrap gap-2 border-t border-border/50 pt-4">
						<Button label="Cancelar" variant="secondary" onclick={handleClose} />
						<Button
							label={form.loading ? 'Retirando...' : 'Retirar'}
							onclick={handleWithdraw}
							disabled={!formValid || form.loading}
							loading={form.loading}
						/>
					</div>
				</div>

				<div
					class="flex h-full min-h-[18rem] flex-col overflow-hidden rounded-2xl border border-rose-200/70 bg-linear-to-br from-rose-50/90 via-background to-orange-50/50 shadow-sm md:min-h-0 dark:border-rose-400/20 dark:from-rose-400/5 dark:to-orange-400/5"
				>
					<div
						class="shrink-0 border-b border-rose-200/50 bg-rose-500/8 px-4 py-2 dark:border-rose-400/15 dark:bg-rose-400/10"
					>
						<p
							class="text-xs font-semibold tracking-wide text-rose-800 uppercase dark:text-rose-300"
						>
							Resumen de la operación
						</p>
					</div>

					<div class="flex flex-1 flex-col gap-3 px-4 py-3">
						{#if loadingWallet && !connectedAddress}
							<div
								class="flex flex-1 items-center justify-center gap-2 text-sm text-muted-foreground"
							>
								<div
									class="size-4 animate-spin rounded-full border-2 border-rose-300 border-t-rose-600"
								></div>
								Preparando resumen...
							</div>
						{:else}
							<div class="flex flex-1 flex-col gap-3">
								<div class="grid grid-cols-2 gap-2">
									<div class="rounded-xl border border-border/60 bg-background/80 px-3 py-2">
										<p class="text-[11px] font-medium text-muted-foreground">
											Disponible en Lemipay
										</p>
										<p class="mt-0.5 text-sm font-semibold text-foreground tabular-nums">
											{formatAmount(balance)}
											<span class="text-xs font-medium text-muted-foreground">{ticker}</span>
										</p>
									</div>
									<div class="rounded-xl border border-border/60 bg-background/80 px-3 py-2">
										<p class="text-[11px] font-medium text-muted-foreground">Destino (Sepolia)</p>
										<p class="mt-0.5 font-mono text-sm font-semibold text-foreground">
											{connectedAddress ? shortenAddress(connectedAddress) : '—'}
										</p>
									</div>
								</div>

								{#if amountValid}
									<div class="space-y-2 border-t border-rose-200/40 pt-3 dark:border-rose-400/15">
										<div class="flex items-center justify-between gap-3 text-sm">
											<span class="text-muted-foreground">Se debita de Lemipay</span>
											<span
												class="shrink-0 font-semibold text-rose-800 tabular-nums dark:text-rose-300"
												>-{formatAmount(grossAmountStr)} {ticker}</span
											>
										</div>

										<div
											class="flex items-center justify-between gap-3 rounded-xl border border-emerald-200/70 bg-emerald-50/80 px-3 py-2 dark:border-emerald-400/20 dark:bg-emerald-400/10"
										>
											<span class="text-sm font-medium text-emerald-800 dark:text-emerald-300">
												Recibís en tu wallet
											</span>
											<span
												class="shrink-0 text-sm font-bold text-emerald-700 tabular-nums dark:text-emerald-300"
											>
												{formatAmount(grossAmountStr)}
												{ticker}
											</span>
										</div>

										{#if remainingBalance !== null}
											<div class="flex items-center justify-between gap-3 text-sm">
												<span class="text-muted-foreground">Queda en Lemipay</span>
												<span class="shrink-0 font-medium tabular-nums">
													{formatAmount(remainingBalance)}
													{ticker}
												</span>
											</div>
										{/if}

										<div class="space-y-1 pt-0.5">
											<p
												class="text-[11px] font-semibold tracking-wide text-muted-foreground uppercase"
											>
												Costo para vos
											</p>
											<div class="flex items-center justify-between gap-3 text-sm">
												<span class="text-muted-foreground">Gas on-chain (Sepolia)</span>
												<span class="shrink-0 font-medium text-emerald-700 dark:text-emerald-400"
													>Sin costo</span
												>
											</div>
											<div class="flex items-center justify-between gap-3 text-sm">
												<span class="text-muted-foreground">Comisión Lemipay</span>
												<span class="shrink-0 font-medium text-emerald-700 dark:text-emerald-400"
													>0 {ticker}</span
												>
											</div>
											<p class="text-[11px] leading-relaxed text-muted-foreground">
												Solo firmás un mensaje. Lemipay ejecuta el retiro on-chain.
											</p>
										</div>
									</div>

									<div
										class="mt-auto space-y-3 border-t border-rose-200/40 pt-3 dark:border-rose-400/15"
									>
										{#if quoteWarning}
											<p class="text-xs leading-relaxed text-amber-700 dark:text-amber-400">
												{quoteWarning}
											</p>
										{/if}

										<div
											class="rounded-xl border border-rose-300/60 bg-rose-50/80 px-3 py-2.5 shadow-sm dark:border-rose-400/25 dark:bg-rose-400/10"
										>
											<p
												class="text-[11px] font-medium text-rose-800/80 uppercase dark:text-rose-300/80"
											>
												Resumen total
											</p>
											<p class="mt-1 text-sm font-semibold text-foreground tabular-nums">
												-{formatAmount(grossAmountStr)}
												{ticker}
												<span class="font-normal text-muted-foreground"> en Lemipay</span>
											</p>
											<p class="mt-0.5 text-xs text-muted-foreground">
												Recibís:
												<span class="font-semibold text-emerald-700 dark:text-emerald-300">
													{formatAmount(grossAmountStr)}
													{ticker}
												</span>
												en Sepolia
											</p>
										</div>
									</div>
								{:else}
									<div class="flex flex-1 items-center justify-center py-6">
										<p class="text-center text-sm text-muted-foreground">
											Ingresá un monto para ver cuánto se debita y cuánto recibís en tu wallet.
										</p>
									</div>
								{/if}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/snippet}
</Modal>
