<script lang="ts">
	import Modal from './Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import type { ApiResponse } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		title: string;
		description?: string;
		message?: string;
		successMsg?: string;
		onclose: () => void;
		onsuccess?: () => void;
		onconfirm: () => ApiResponse<unknown>;
	}

	const {
		open,
		title,
		description,
		message,
		successMsg = 'Operación exitosa',
		onclose,
		onsuccess,
		onconfirm
	}: Props = $props();

	const form = new ModalState();

	function handleClose() {
		form.reset();
		onclose();
	}

	async function handleConfirm() {
		// Delegamos toda la ejecución, carga y manejo de errores a nuestra clase
		await form.submit(onconfirm, {
			successMsg,
			onSuccess: () => {
				onsuccess?.();
				handleClose();
			}
		});
	}
</script>

<Modal
	{open}
	{title}
	{description}
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		{#if message}
			<p class="text-sm text-gray-600">{message}</p>
		{/if}
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} disabled={form.loading} />
		<Button label="Confirmar" onclick={handleConfirm} loading={form.loading} />
	{/snippet}
</Modal>
