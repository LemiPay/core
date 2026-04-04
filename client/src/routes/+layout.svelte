<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { authStore } from '$lib/stores/auth';
	import { onMount } from 'svelte';

	let { children } = $props();

	//aca se podria hacer que si no esta autenticado lo redireccione solo
	onMount(() => {
		authStore.init();
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
