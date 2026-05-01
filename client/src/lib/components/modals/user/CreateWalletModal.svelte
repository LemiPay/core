<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import NewWalletField from '$lib/components/input_fields/NewWalletField.svelte';
	import CurrencySelectField from '$lib/components/input_fields/CurrencySelectField.svelte';

	import { createNewAddress } from '$lib/api/endpoints/user_wallet';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let address = $state('');
	let currency_ticker = $state('');

	// La dirección asumo mínimo 3 caracteres, y el ticker solo tiene que estar seleccionado
	const formValid = $derived(address.trim().length >= 3 && currency_ticker !== '');

	function handleClose() {
		address = '';
		currency_ticker = '';
		form.reset();
		onclose();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(() => createNewAddress(address.trim(), currency_ticker), {
			successMsg: 'Billetera creada exitosamente',
			onSuccess: () => {
				onsuccess();
				handleClose();
			}
		});
	}
</script>

<Modal
	{open}
	title="Nueva Dirección"
	description="Creá una nueva dirección y asignale un token inicial."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="create-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<NewWalletField bind:value={address} attempted={form.attempted} />

			<CurrencySelectField
				label="Token (Ticker)"
				bind:value={currency_ticker}
				attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Crear"
			type="submit"
			form="create-wallet-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
