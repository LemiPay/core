<script lang="ts">
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals_old/modals/Modal.svelte';

	import { faucetFundWallet } from '$lib/api/endpoints/user_wallet';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		wallet_id: string;
		ticker: string;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, wallet_id, ticker, onclose, onsuccess }: Props = $props();

	const form = new ModalState();
	let amount = $state('');

	const formValid = $derived(
		amount != null &&
			amount !== '' &&
			!isNaN(Number(String(amount).replace(',', '.'))) &&
			Number(String(amount).replace(',', '.')) > 0
	);

	function handleClose() {
		amount = '';
		form.reset();
		onclose();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(() => faucetFundWallet(String(amount).replace(',', '.'), wallet_id), {
			successMsg: 'Billetera fondeada correctamente',
			onSuccess: () => {
				onsuccess(); // Refresca la data en la vista padre
				handleClose(); // Resetea y cierra el modal
			}
		});
	}
</script>

<Modal
	{open}
	title="Recibir Dinero"
	description="Dinero mágico de otra dimensión será enviado a tu dirección."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="receive-money-form" onsubmit={handleSubmit} class="space-y-4">
			<NumberField
				id="amount"
				label="Monto de {ticker} a recibir"
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
			label="Recibir"
			type="submit"
			form="receive-money-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
