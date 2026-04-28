<script lang="ts">
	import Modal from "$lib/components/modals/Modal.svelte";
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import type { NewMemberData } from '$lib/types/endpoints/proposals.types';
	import { createNewMemberProposal } from '$lib/api/endpoints/proposals';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let email = $state('');

	const formValid = $derived(email.trim().length >= 4 && email.trim().length <= 30);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		const params: NewMemberData = {
			group_id: group_id,
			email: email.trim()
		};

		await form.submit(
				() => createNewMemberProposal(params),
				{
					successMsg: 'Invitación enviada correctamente',
					onSuccess: () => {
						onsuccess?.();
						handleClose();
					}
				}
		);
	}

	function handleClose() {
		email = '';
		form.reset();
		onclose();
	}
</script>

<Modal
		{open}
		title="Invitar nuevo miembro"
		description="Creá una propuesta para invitar a un usuario a este grupo."
		onclose={handleClose}
		error={form.error}
		success={form.success}
		loading={form.loading}
>
	{#snippet children()}
		<form id="add-member-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
					id="member-email"
					label="Email"
					type="email"
					placeholder="e.g. joe@doe.com"
					minLength={4}
					maxLength={30}
					bind:value={email}
					attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
				label="Enviar Invitación"
				type="submit"
				form="add-member-form"
				disabled={!formValid}
				loading={form.loading}
		/>
	{/snippet}
</Modal>