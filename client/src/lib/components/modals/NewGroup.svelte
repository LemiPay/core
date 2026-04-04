<script lang="ts">
	import Modal from './Modal.svelte';
	import FormField from '$lib/components/FormField.svelte';
	import Button from '../Button.svelte';

	import type { NewGroupData } from '$lib/types/groups.types';
	import { create_group } from '$lib/api/groups';

	interface Props {
		open: boolean;
		onclose: () => void;
	}

	const { open, onclose }: Props = $props();

	let name = $state('');
	let description = $state('');
	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const nameValid = $derived(name.trim().length >= 4 && name.trim().length <= 30);
	const descValid = $derived(description.trim().length >= 8 && description.trim().length <= 30);
	const formValid = $derived(nameValid && descValid);

	async function createGroup() {
		const params: NewGroupData = {
			name: name.trim(),
			description: description.trim()
		};

		loading = true;
		error = '';

		const response = await create_group(params);
		loading = false;

		if (response.status !== 200) {
			error = response.message || 'An error occurred while creating the group.';
			return;
		}

		success = 'Group created successfully!';

		setTimeout(() => {
			window.location.href = '/groups';
		}, 1000);
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;
		error = '';
		success = '';
		loading = true;
		try {
			await createGroup();
		} finally {
			loading = false;
		}
	}

	function handleClose() {
		name = '';
		description = '';
		attempted = false;
		error = '';
		success = '';
		loading = false;
		onclose();
	}
</script>

<Modal
	{open}
	title="New group"
	description="Create a group to start splitting expenses with others."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="new-group-form" onsubmit={handleSubmit} class="space-y-4">
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
			label="Create group"
			type="submit"
			form="new-group-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
