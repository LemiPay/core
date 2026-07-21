<script lang="ts">
	export const ssr = false;

	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { authStore } from '$lib/stores/auth';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	import Navbar from '$lib/components/ui/Navbar.svelte';
	import FooterTwo from '$lib/components/blocks/footer/footer-two.svelte';

	let { children } = $props();
	let initialized = $state(false);

	const PUBLIC_ROUTES = ['/', '/login', '/register', '/status'];
	const AUTH_ONLY_ROUTES = ['/login', '/register'];

	function isPublic(pathname: string) {
		return PUBLIC_ROUTES.includes(pathname);
	}

	// Rutas públicas no bloquean el render por la API (landing, status, login, register).
	const canRender = $derived(initialized || isPublic(page.url.pathname));

	onMount(async () => {
		try {
			await authStore.init();
		} catch (error) {
			console.error('auth init failed:', error);
		} finally {
			initialized = true;
		}
	});

	$effect(() => {
		if (!initialized) return;

		const pathname = page.url.pathname;

		if (pathname.startsWith('/api')) return;

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

<Navbar isAuthenticated={$authStore.isAuthenticated} user={$authStore.user} />

{#if canRender}
	<div class="flex min-h-screen flex-col">
		{@render children()}
	</div>
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<div
			class="h-10 w-10 animate-spin rounded-full border-4 border-gray-200 border-t-blue-500"
		></div>
	</div>
{/if}

<FooterTwo />
