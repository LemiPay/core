<script lang="ts">
	import Modal from '../Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import { updateGroup } from '$lib/api/endpoints/groups';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import type { Group } from '$lib/types/endpoints/groups.types';

	interface Props {
		open: boolean;
		group: Group;
		onclose: () => void;
		// Cambiamos onedit por onsuccess, y devuelve el grupo actualizado
		onsuccess?: (updatedGroup: Group) => void;
	}

	const { open, group, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let name = $state(group.name);
	let description = $state(group.description);

	const nameValid = $derived(name.trim().length >= 4 && name.trim().length <= 30);
	const descValid = $derived(description.trim().length >= 8 && description.trim().length <= 30);
	const formValid = $derived(nameValid && descValid);

	// Resetea los campos a los valores originales cada vez que se abre el modal
	$effect(() => {
		if (open) {
			name = group.name;
			description = group.description;
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(
				() => updateGroup(group.id, {
					name: name.trim(),
					description: description.trim()
				}),
				{
					successMsg: 'Actualización completada',
					onSuccess: (updatedGroup) => {
						onsuccess?.(updatedGroup);
						handleClose();
					}
				}
		);
	}

	function handleClose() {
		name = group.name;
		description = group.description;
		form.reset();
		onclose();
	}
</script>

<Modal
		{open}
		title="Editar grupo"
		description="Actualiza el nombre y la descripción del grupo"
		onclose={handleClose}
		error={form.error}
		success={form.success}
		loading={form.loading}
>
	{#snippet children()}
		<form id="edit-group-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
					id="group-name"
					label="Nombre"
					type="text"
					placeholder="e.g. Trip to Rome"
					minLength={4}
					maxLength={30}
					bind:value={name}
					attempted={form.attempted}
			/>
			<FormField
					id="group-description"
					label="Descripción"
					type="textarea"
					placeholder="What is this group for?"
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
				label="Guardar Cambios"
				type="submit"
				form="edit-group-form"
				disabled={!formValid}
				loading={form.loading}
		/>
	{/snippet}
</Modal>