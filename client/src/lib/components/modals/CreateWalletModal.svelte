<script lang="ts">
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';
	import { createNewAddress } from '$lib/api/endpoints/user_wallet';
	import { isSuccess } from '$lib/types/client.types';
	import { generateRandomAddress } from '$lib/utils/address_utils';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, onclose, onsuccess }: Props = $props();

	let address = $state('');
	let currency_ticker = $state('');
	let attempted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const formValid = $derived(address.trim().length > 0 && currency_ticker.trim().length > 0);

	function handleClose() {
		address = '';
		currency_ticker = '';
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

		let result = await createNewAddress(address.trim(), currency_ticker.trim().toUpperCase());

		if (!isSuccess(result)) {
			error = result.message;
			loading = false;
			return;
		}

		loading = false;
		success = 'Billetera creada exitosamente';
		setTimeout(() => {
			handleClose();
		}, 2000);
	}
</script>

<Modal
	{open}
	title="Nueva Dirección"
	description="Crea una nueva dirección y asignale un token inicial."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="create-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<div>
				<FormField
					id="address"
					label="Dirección"
					minLength={0}
					maxLength={255}
					type="text"
					placeholder="Ej. 0x123...abc"
					bind:value={address}
					{attempted}
				/>
				<button
					type="button"
					onclick={() => (address = generateRandomAddress())}
					class="mt-1 text-xs font-medium text-gray-500 transition hover:text-black"
				>
					Generar dirección aleatoria
				</button>
			</div>
			<FormField
				id="currency_ticker"
				label="Token (Ticker)"
				minLength={1}
				maxLength={10}
				type="text"
				placeholder="Ej. USDC"
				bind:value={currency_ticker}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button label="Crear" type="submit" form="create-wallet-form" disabled={!formValid} {loading} />
	{/snippet}
</Modal>
