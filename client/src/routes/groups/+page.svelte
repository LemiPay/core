<script lang="ts">
	import IconButton from '$lib/components/IconButton.svelte';
	import NewGroup from '$lib/components/modals/NewGroup.svelte';

	let showNewGroup = $state(false);
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<IconButton variant="primary" ariaLabel="Create group" onclick={() => (showNewGroup = true)}>
	{#snippet icon()}
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2.5"
			stroke-linecap="round"
			stroke-linejoin="round"
		>
			<line x1="12" y1="5" x2="12" y2="19" />
			<line x1="5" y1="12" x2="19" y2="12" />
		</svg>
	{/snippet}
</IconButton>

<NewGroup open={showNewGroup} onclose={() => (showNewGroup = false)} />