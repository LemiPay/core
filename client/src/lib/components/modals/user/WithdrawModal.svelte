<script lang="ts">
	import { signMessage } from '@wagmi/core';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	import { ModalState } from '$lib/utils/modal_state.svelte';
	import {
		wagmiAdapter,
		walletAuthState,
		authActions
	} from '../../../../routes/wallet_auth.svelte';
	import { withdrawFromWallet } from '$lib/api/endpoints/user_wallet';

	interface Props {
		open: boolean;
		wallet_id: string;
		wallet_address: string;
		ticker: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, wallet_id, wallet_address, ticker, onclose, onsuccess }: Props = $props();

	const form = new ModalState();
	let amount = $state('');

	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const formValid = $derived(amountValid);

	function handleClose() {
		amount = '';
		form.reset();
		onclose();
	}

	async function handleWithdraw(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		if (!walletAuthState.isConnected) {
			await authActions.openLogin();
			if (!walletAuthState.isConnected) {
				form.error = 'Necesitás conectar tu wallet para firmar.';
				return;
			}
		}

		await form.submit(
			async () => {
				const parsedAmount = String(amount).replace(',', '.');
				const userAddress = walletAuthState.address;
				if (!userAddress) {
					return {
						ok: false as const,
						status: 400,
						message: 'Wallet no conectada.',
						body: null
					};
				}

				const message = [
					'lemipay.app quiere autorizar un retiro:',
					'',
					`Wallet: ${wallet_id}`,
					`Monto: ${parsedAmount}`,
					`Address: ${userAddress}`,
					'',
					'URI: https://localhost:5173'
				].join('\n');

				let signature: string;
				try {
					signature = await signMessage(wagmiAdapter.wagmiConfig, { message });
				} catch (err: any) {
					return {
						ok: false as const,
						status: 400,
						message: err?.message || 'Firma rechazada por el usuario.',
						body: null
					};
				}

				return await withdrawFromWallet(parsedAmount, wallet_id, signature, userAddress);
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
	description="Firmá un mensaje con tu wallet para autorizar el retiro."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="withdraw-form" onsubmit={handleWithdraw} class="space-y-4">
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
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Retirar"
			type="submit"
			form="withdraw-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
