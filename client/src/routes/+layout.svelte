<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { authStore } from '$lib/stores/auth';
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { children } = $props();
	let initialized = $state(false);

	const PUBLIC_ROUTES = ['/', '/login', '/register'];
	const AUTH_ONLY_ROUTES = ['/login', '/register'];

	function isPublic(pathname: string) {
		return PUBLIC_ROUTES.includes(pathname);
	}

	onMount(async () => {
		await authStore.init();
		initialized = true;
	});

	$effect(() => {
		if (!initialized) return;

		const pathname = page.url.pathname;
		const authed = $authStore.isAuthenticated;

		if (!authed && !isPublic(pathname)) {
			const redirectTo = encodeURIComponent(pathname + page.url.search);
			goto(`/login?redirectTo=${redirectTo}`, { replaceState: true });
			return;
		}

		if (authed && AUTH_ONLY_ROUTES.includes(pathname)) {
			goto('/dashboard', { replaceState: true });
		}
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
