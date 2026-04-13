<script lang="ts">
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';
	import { faucetFundWallet } from '$lib/api/endpoints/user_wallet';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		wallet_id: string;
		ticker: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, wallet_id, ticker, onclose, onsuccess }: Props = $props();

	// Estados del formulario
	let amount = $state('');
	let attempted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const formValid = $derived(
		amount != null &&
			amount !== '' &&
			!isNaN(Number(String(amount).replace(',', '.'))) &&
			Number(String(amount).replace(',', '.')) > 0
	);
	function handleClose() {
		amount = '';
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
		error = '';
		success = '';
		loading = true;
		let result = await faucetFundWallet(String(amount).replace(',', '.'), wallet_id);
		if (!isSuccess(result)) {
			error = result.message;
			loading = false;
			return;
		}
		loading = false;
		success = 'Billetera fondeada correctamente';
		setTimeout(() => {
			handleClose();
		}, 2000);
	}
</script>

<Modal
	{open}
	title="Recibir Dinero"
	description="Dinero mágico de otra dimensión será enviado a tu dirección."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="receive-money-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="amount"
				label="Monto de {ticker} a recibir"
				minLength={0}
				maxLength={3}
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
			label="Recibir"
			type="submit"
			form="receive-money-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
