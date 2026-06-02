<script lang="ts">
	import HeroOne from '$lib/components/blocks/hero/hero-one.svelte';
	import { authStore } from '$lib/stores/auth';
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<HeroOne isAuthed={authStore.getIsAuthed()} />
