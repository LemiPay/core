<script lang="ts">
	import FormField from '$lib/components/ui/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	import type { NewMemberData } from '$lib/types/endpoints/proposals.types';
	import { createNewMemberProposal } from '$lib/api/endpoints/proposals';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess?: () => void;
		group_id: string;
	}

	const { open, onclose, onsuccess, group_id }: Props = $props();

	let email = $state('');

	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const emailValid = $derived(email.trim().length >= 4 && email.trim().length <= 30);
	const formValid = $derived(emailValid);

	async function handleCreateInvite() {
		const params: NewMemberData = {
			group_id: group_id,
			email: email
		};

		error = '';

		const response = await createNewMemberProposal(params);

		if (!isSuccess(response)) {
			error = response.message ?? 'Failed to send invite';
			return;
		}

		success = 'Invite sent successfully';

		email = '';
		attempted = false;

		setTimeout(() => {
			onsuccess?.();
			handleClose();
		}, 2000);
	}

	function handleClose() {
		email = '';
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

		try {
			await handleCreateInvite();
		} finally {
			loading = false;
		}
	}
</script>

<Modal
	{open}
	title="Invite new Member"
	description="Create a new member proposal to invite a user to this group"
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="add-member-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="member-email"
				label="Email"
				type="email"
				placeholder="e.g. joe@doe"
				minLength={4}
				maxLength={30}
				bind:value={email}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancel" variant="secondary" onclick={handleClose} />

		<Button
			label="Send Invite"
			type="submit"
			form="add-member-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
