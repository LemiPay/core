<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	import { searchUsers, sendFriendRequest } from '$lib/api/endpoints/friends';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let email = $state('');

	const formValid = $derived(email.trim().length >= 4 && email.trim().length <= 30);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		form.error = '';
		form.success = '';
		form.loading = true;

		const query = email.trim();
		const res = await searchUsers(query);

		if (!isSuccess(res)) {
			form.loading = false;
			form.error = res.message || 'No se pudo buscar el usuario.';
			return;
		}

		const match = res.body.find((u) => u.email.toLowerCase() === query.toLowerCase());
		if (!match) {
			form.loading = false;
			form.error = 'No se encontró un usuario con ese email.';
			return;
		}
		if (match.is_friend) {
			form.loading = false;
			form.error = 'Ya son amigos.';
			return;
		}

		form.loading = false;
		await form.submit(() => sendFriendRequest(match.user_id), {
			successMsg: 'Solicitud enviada',
			onSuccess: () => {
				onsuccess?.();
				handleClose();
			}
		});
	}

	function handleClose() {
		email = '';
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Agregar amigo"
	description="Ingresá el email de la persona a la que querés agregar."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	<form id="add-friend-form" onsubmit={handleSubmit} class="space-y-4">
		<FormField
			id="friend-email"
			label="Email"
			type="email"
			placeholder="e.g. joe@doe.com"
			minLength={4}
			maxLength={30}
			bind:value={email}
			attempted={form.attempted}
		/>
	</form>

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />
		<Button
			label="Enviar solicitud"
			type="submit"
			form="add-friend-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
