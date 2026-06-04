<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		icon: Snippet;
		ariaLabel: string;
		onclick: () => void;
	}

	const { icon, ariaLabel, onclick }: Props = $props();
</script>

<button
	type="button"
	{onclick}
	aria-label={ariaLabel}
	class="fixed right-6 bottom-6 z-40 flex h-14 w-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-xl transition hover:bg-primary/90 focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-95 active:bg-primary/80"
>
	<span class="flex h-6 w-6 items-center justify-center">
		{@render icon()}
	</span>
</button>
