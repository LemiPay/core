<script lang="ts">
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';
	import { transferToWallet } from '$lib/api/endpoints/user_wallet';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		sender_wallet_id: string;
		ticker: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, sender_wallet_id, ticker, onclose, onsuccess }: Props = $props();

	let amount = $state('');
	let receiver_address = $state('');
	let attempted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const formValid = $derived(
		amount != null &&
			amount !== '' &&
			!isNaN(Number(String(amount).replace(',', '.'))) &&
			Number(String(amount).replace(',', '.')) > 0 &&
			receiver_address.trim().length > 0
	);

	function handleClose() {
		amount = '';
		receiver_address = '';
		attempted = false;
		error = '';
		if (success != '') {
			onsuccess();
		}
		success = '';
		loading = false;
		onclose();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;
		const trimmedReceiverAddress = receiver_address.trim();
		receiver_address = trimmedReceiverAddress;
		error = '';
		success = '';
		loading = true;
		let result = await transferToWallet(String(amount), sender_wallet_id, trimmedReceiverAddress);
		if (!isSuccess(result)) {
			error = result.message;
			loading = false;
			return;
		}

		loading = false;
		success = 'Transferencia enviada';
		setTimeout(() => {
			handleClose();
		}, 2000);
	}
</script>

<Modal
	{open}
	title="Enviar {ticker}"
	description="Transfiere fondos a otra billetera ingresando su dirección."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="transfer-money-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="receiver_address"
				label="Dirección de destino"
				minLength={1}
				maxLength={255}
				type="text"
				placeholder="Ej. 0x123...abc"
				bind:value={receiver_address}
				{attempted}
			/>
			<FormField
				id="amount"
				label="Monto a enviar"
				minLength={1}
				maxLength={10}
				type="number"
				placeholder="Ej. 10.50"
				bind:value={amount}
				{attempted}
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
			{loading}
		/>
	{/snippet}
</Modal>
