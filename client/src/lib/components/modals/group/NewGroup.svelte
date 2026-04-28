<script lang="ts">
	import Modal from '../Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import type { NewGroupData } from '$lib/types/endpoints/groups.types';
	import { createGroup } from '$lib/api/endpoints/groups';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		onclose: () => void;
	}

	const { open, onclose }: Props = $props();

	const form = new ModalState();

	let name = $state('');
	let description = $state('');

	const nameValid = $derived(name.trim().length >= 4 && name.trim().length <= 30);
	const descValid = $derived(description.trim().length >= 8 && description.trim().length <= 30);
	const formValid = $derived(nameValid && descValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		const params: NewGroupData = {
			name: name.trim(),
			description: description.trim()
		};

		await form.submit(
				() => createGroup(params),
				{
					successMsg: '¡Grupo creado exitosamente!',
					// onSuccess recibe automáticamente el body de la respuesta gracias a nuestro ModalState
					onSuccess: (createdGroup: any) => {
						window.location.href = `/groups/${createdGroup.id}`;
					}
				}
		);
	}

	function handleClose() {
		name = '';
		description = '';
		form.reset();
		onclose();
	}
</script>

<Modal
		{open}
		title="Nuevo grupo"
		description="Creá un grupo para empezar a dividir gastos con otros."
		onclose={handleClose}
		error={form.error}
		success={form.success}
		loading={form.loading}
>
	{#snippet children()}
		<form id="new-group-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
					id="group-name"
					label="Nombre"
					type="text"
					placeholder="Ej. Viaje a Roma"
					minLength={4}
					maxLength={30}
					bind:value={name}
					attempted={form.attempted}
			/>
			<FormField
					id="group-description"
					label="Descripción"
					type="textarea"
					placeholder="¿Para qué es este grupo?"
					minLength={8}
					maxLength={30}
					rows={3}
					bind:value={description}
					attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
				label="Crear grupo"
				type="submit"
				form="new-group-form"
				disabled={!formValid}
				loading={form.loading}
		/>
	{/snippet}
</Modal>