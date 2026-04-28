<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormField from '$lib/components/ui/FormField.svelte';

	import { createGroupWallet } from '$lib/api/endpoints/groups';
	import { generateRandomAddress } from '$lib/utils/address_utils';

	import { ModalState } from '$lib/utils/modal_state.svelte';
	import type { ApiResponse } from '$lib/types/client.types';
	import type { GroupWallet } from '$lib/types/endpoints/groups.types';

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
	const tickerValid = $derived(
		currencyTicker.trim().length >= 2 && currencyTicker.trim().length <= 10
	);
	const formValid = $derived(addressValid && tickerValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(
			() =>
				createGroupWallet(group_id, {
					address: address.trim(),
					currency_ticker: currencyTicker.trim()
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
			<FormField
				id="wallet-address"
				label="Dirección de wallet"
				type="text"
				placeholder="0x..."
				minLength={3}
				maxLength={100}
				bind:value={address}
				attempted={form.attempted}
			/>
			<button
				type="button"
				onclick={() => (address = generateRandomAddress())}
				class="mt-1 text-xs font-medium text-gray-500 transition hover:text-black"
			>
				Generar dirección aleatoria
			</button>
			<FormField
				id="currency-ticker"
				label="Moneda (ticker)"
				type="text"
				placeholder="e.g. ETH, USDT, BTC"
				minLength={2}
				maxLength={10}
				bind:value={currencyTicker}
				attempted={form.attempted}
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
			loading={form.loading}
		/>
	{/snippet}
</Modal>
