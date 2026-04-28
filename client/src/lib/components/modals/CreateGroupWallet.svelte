<script lang="ts">
	import Modal from '$lib/components/modals_old/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import NewWalletField from '$lib/components/input_fields/NewWalletField.svelte';

	// 1. Importás el nuevo componente
	import CurrencySelectField from '$lib/components/input_fields/CurrencySelectField.svelte';

	import { createGroupWallet } from '$lib/api/endpoints/groups';
	import { ModalState } from '$lib/utils/modal_state.svelte';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let address = $state('');
	let currencyTicker = $state('');

	const addressValid = $derived(address.trim().length >= 3 && address.trim().length <= 100);

	// 2. Simplificás la validación del ticker (solo que no esté vacío)
	const tickerValid = $derived(currencyTicker !== '');

	const formValid = $derived(addressValid && tickerValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(
			() =>
				createGroupWallet(group_id, {
					address: address.trim(),
					currency_ticker: currencyTicker // <-- Se manda el ticker seleccionado
				}),
			{
				successMsg: 'Wallet creada exitosamente!',
				onSuccess: handleClose
			}
		);
	}

	function handleClose() {
		address = '';
		currencyTicker = '';
		form.reset();
		onclose();
		onsuccess?.();
	}
</script>

<Modal
	{open}
	title="Crear wallet del grupo"
	description="Asociá una dirección de wallet y una moneda al grupo."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="create-group-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<NewWalletField bind:value={address} attempted={form.attempted} />

			<CurrencySelectField bind:value={currencyTicker} attempted={form.attempted} />
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />
		<Button
			label="Crear wallet"
			type="submit"
			form="create-group-wallet-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
