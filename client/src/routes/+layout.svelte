<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
    import {checkAuth} from "$lib/stores/store";
    import {onMount} from "svelte";

    let { children } = $props();
    //aca se podria hacer que si no esta autenticado lo redireccione solo
    onMount(checkAuth);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
