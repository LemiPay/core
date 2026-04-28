<script lang="ts">
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import NumberField from "$lib/components/input_fields/NumberField.svelte";
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from "$lib/components/modals/Modal.svelte";

	import { transferToWallet } from '$lib/api/endpoints/user_wallet';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		sender_wallet_id: string;
		ticker: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, sender_wallet_id, ticker, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let amount = $state('');
	let receiver_address = $state('');

	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const addressValid = $derived(receiver_address.trim().length >= 3);

	const formValid = $derived(amountValid && addressValid);

	function handleClose() {
		amount = '';
		receiver_address = '';
		form.reset();
		onclose();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		const trimmedReceiverAddress = receiver_address.trim();
		receiver_address = trimmedReceiverAddress; // Limpiamos los espacios en el input

		await form.submit(
				() => transferToWallet(String(parsedAmount), sender_wallet_id, trimmedReceiverAddress),
				{
					successMsg: 'Transferencia enviada',
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
		title="Enviar {ticker}"
		description="Transfiere fondos a otra billetera ingresando su dirección."
		onclose={handleClose}
		error={form.error}
		success={form.success}
		loading={form.loading}
>
	{#snippet children()}
		<form id="transfer-money-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
					id="receiver_address"
					label="Dirección de destino"
					type="text"
					minLength={3}
					maxLength={100}
					placeholder="Ej. 0x123...abc"
					bind:value={receiver_address}
					attempted={form.attempted}
			/>

			<NumberField
					id="amount"
					label="Monto a enviar"
					min={0.01}
					placeholder="Ej. 10.50"
					bind:value={amount}
					attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
				label="Enviar"
				type="submit"
				form="transfer-money-form"
				disabled={!formValid}
				loading={form.loading}
		/>
	{/snippet}
</Modal>