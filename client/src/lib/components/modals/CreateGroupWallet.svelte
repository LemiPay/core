<script lang="ts">
	import Modal from './Modal.svelte';
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import { createGroupWallet } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	let address = $state('');
	let currencyTicker = $state('');
	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const addressValid = $derived(address.trim().length >= 3 && address.trim().length <= 100);
	const tickerValid = $derived(
		currencyTicker.trim().length >= 2 && currencyTicker.trim().length <= 10
	);
	const formValid = $derived(addressValid && tickerValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;
		error = '';
		success = '';
		loading = true;

		const response = await createGroupWallet(group_id, {
			address: address.trim(),
			currency_ticker: currencyTicker.trim()
		});

		loading = false;

		if (!isSuccess(response)) {
			error = response.message || 'Error al crear la wallet del grupo.';
			return;
		}

		success = 'Wallet creada exitosamente!';

		setTimeout(() => {
			handleClose();
			onsuccess?.();
		}, 1200);
	}

	function handleClose() {
		address = '';
		currencyTicker = '';
		attempted = false;
		error = '';
		success = '';
		loading = false;
		onclose();
	}
</script>

<Modal
	{open}
	title="Crear wallet del grupo"
	description="Asociá una dirección de wallet y una moneda al grupo."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="create-group-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="wallet-address"
				label="Dirección de wallet"
				type="text"
				placeholder="0x..."
				minLength={3}
				maxLength={100}
				bind:value={address}
				{attempted}
			/>
			<FormField
				id="currency-ticker"
				label="Moneda (ticker)"
				type="text"
				placeholder="e.g. ETH, USDT, BTC"
				minLength={2}
				maxLength={10}
				bind:value={currencyTicker}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Crear wallet"
			type="submit"
			form="create-group-wallet-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
