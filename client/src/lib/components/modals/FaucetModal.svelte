<script lang="ts">
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	interface Props {
		open: boolean;
		wallet_id: string;
		onclose: () => void;
		onsuccess?: () => void; // Útil para recargar los datos en el perfil
	}

	const { open, wallet_id, onclose, onsuccess }: Props = $props();

	// Estados del formulario
	let amount = $state('');
	let attempted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const formValid = $derived(amount.trim().length > 0);

	function handleClose() {
		amount = '';
		attempted = false;
		error = '';
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
	}
</script>

<Modal
	{open}
	title="Recibir Dinero (Faucet)"
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
				label="Monto a recibir"
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
