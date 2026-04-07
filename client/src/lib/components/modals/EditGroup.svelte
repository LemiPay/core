<script lang="ts">
	import Modal from './Modal.svelte';
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import type { Group } from '$lib/types/endpoints/groups.types';

	interface Props {
		open: boolean;
		group: Group;
		onclose: () => void;
		onedit: (data: { name: string; description: string }) => Promise<void>;
	}

	const { open, group, onclose, onedit }: Props = $props();

	let name = $derived(group.name);
	let description = $derived(group.description);
	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const nameValid = $derived(name.trim().length >= 4 && name.trim().length <= 30);
	const descValid = $derived(description.trim().length >= 8 && description.trim().length <= 30);
	const formValid = $derived(nameValid && descValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;
		error = '';
		success = '';
		loading = true;
		try {
			await onedit({ name: name.trim(), description: description.trim() });
			success = 'Group updated successfully!';
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : 'An error occurred while updating the group.';
		} finally {
			loading = false;

			setTimeout(() => {
				handleClose();
			}, 2000);
		}
	}

	function handleClose() {
		name = group.name;
		description = group.description;
		attempted = false;
		error = '';
		success = '';
		loading = false;
		onclose();
	}
</script>

<Modal
	{open}
	title="Edit group"
	description="Update the group's name and description."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="edit-group-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="group-name"
				label="Name"
				type="text"
				placeholder="e.g. Trip to Rome"
				minLength={4}
				maxLength={30}
				bind:value={name}
				{attempted}
			/>
			<FormField
				id="group-description"
				label="Description"
				type="textarea"
				placeholder="What is this group for?"
				minLength={8}
				maxLength={30}
				rows={3}
				bind:value={description}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancel" variant="secondary" onclick={handleClose} />

		<Button
			label="Save changes"
			type="submit"
			form="edit-group-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
