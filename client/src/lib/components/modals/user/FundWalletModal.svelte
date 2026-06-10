<script lang="ts">
	import { readContract, writeContract } from '@wagmi/core';
	import { parseUnits, pad } from 'viem';
	import { env } from '$env/dynamic/public';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import CurrencySelectField from '$lib/components/input_fields/CurrencySelectField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import {
		wagmiAdapter,
		walletAuthState,
		authActions
	} from '../../../../routes/wallet_auth.svelte';

	const vaultAbi = [
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

	let payLabel = $state('Pagar');

	function handleClose() {
		amount = '';
		selectedTicker = '';
		form.reset();
		onclose();
	}

	async function handlePay(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		if (!walletAuthState.isConnected) {
			await authActions.openLogin();
			if (!walletAuthState.isConnected) {
				form.error = 'Necesitás conectar tu wallet para pagar.';
				return;
			}
		}

		await form.submit(
			async () => {
				const parsedAmount = String(amount).replace(',', '.');
				const config = tokenConfig;
				const userAddress = walletAuthState.address;
				if (!config || !userAddress) {
					return {
						ok: false as const,
						status: 400,
						message: !config
							? `Token ${selectedTicker} no soportado para fondeo on-chain.`
							: 'Wallet no conectada.',
						body: null
					};
				}

				const tokenAddress = config.address as `0x${string}`;
				const vaultAddress = env.PUBLIC_VAULT_CONTRACT_ADDRESS as `0x${string}`;
				const parsedUnits = parseUnits(parsedAmount, config.decimals);

				try {
					const currentAllowance = await readContract(wagmiAdapter.wagmiConfig, {
						address: tokenAddress,
						abi: erc20Abi,
						functionName: 'allowance',
						args: [userAddress as `0x${string}`, vaultAddress]
					});

					if (currentAllowance < parsedUnits) {
						payLabel = 'Aprobando...';
						await writeContract(wagmiAdapter.wagmiConfig, {
							address: tokenAddress,
							abi: erc20Abi,
							functionName: 'approve',
							args: [vaultAddress, parsedUnits]
						});
					}

					payLabel = 'Pagando...';
					const walletAddressBytes32 = pad(wallet_address as `0x${string}`, { size: 32 });
					await writeContract(wagmiAdapter.wagmiConfig, {
						address: vaultAddress,
						abi: vaultAbi,
						functionName: 'fund',
						args: [walletAddressBytes32, tokenAddress, parsedUnits]
					});
				} catch (err: any) {
					return {
						ok: false as const,
						status: 400,
						message: err?.message || 'Transacción rechazada o fallida.',
						body: null
					};
				}

				return {
					ok: true as const,
					status: 200,
					body: null,
					message: ''
				};
			},
			{
				successMsg: '¡Fondeo realizado exitosamente!',
				onSuccess: () => {
					onsuccess();
					handleClose();
				}
			}
		);

		payLabel = 'Pagar';
	}
</script>

<Modal
	{open}
	title="Fondear wallet"
	description="Primero aprobás el gasto de tokens y luego se ejecuta el fondeo."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="fund-wallet-form" onsubmit={handlePay} class="space-y-4">
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
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label={payLabel}
			type="submit"
			form="fund-wallet-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
