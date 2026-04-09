<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { authStore } from '$lib/stores/auth';
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import { onMount } from 'svelte';

	let { children } = $props();
	let initialized = $state(false);

	onMount(async () => {
		await authStore.init();
		initialized = true;
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

{#if initialized}
	{#if $authStore.isAuthenticated}
		<Navbar />
	{/if}
	{@render children()}
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<div
			class="h-10 w-10 animate-spin rounded-full border-4 border-gray-200 border-t-blue-500"
		></div>
	</div>
{/if}
