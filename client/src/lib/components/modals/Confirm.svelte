<script lang="ts">
	import Modal from './Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	interface Props {
		open: boolean;
		title: string;
		description?: string;
		message?: string;
		onclose: () => void;
		onconfirm: () => void;
		loading?: boolean;
		error?: string;
	}

	const {
		open,
		title,
		description,
		message,
		onclose,
		onconfirm,
		loading = false,
		error = ''
	}: Props = $props();
</script>

<Modal {open} {title} {description} {onclose} {error} {loading}>
	{#snippet children()}
		{#if message}
			<p class="text-sm text-gray-600">{message}</p>
		{/if}
	{/snippet}

	{#snippet footer()}
		<Button label="Cancel" variant="secondary" onclick={onclose} />
		<Button label="Confirm" onclick={onconfirm} {loading} />
	{/snippet}
</Modal>
