<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import { fade, slide } from 'svelte/transition';

	interface Props {
		title: string;
		open: boolean;
		onclose: () => void;
		children: Snippet;
	}

	const { title, open, onclose, children }: Props = $props();

	function handleWindowKeydown(e: KeyboardEvent) {
		if (open && e.key === 'Escape') {
			onclose();
		}
	}
</script>

<svelte:window onkeydown={handleWindowKeydown} />
{#if open}
	<div
		role="presentation"
		tabindex="-1"
		class="fixed inset-0 z-40 bg-black/20 backdrop-blur-sm transition-opacity"
		transition:fade={{ duration: 200 }}
		onclick={onclose}
	></div>

	<div
		class="fixed inset-y-0 right-0 z-50 flex w-full max-w-md flex-col border-l border-gray-200 bg-white shadow-2xl"
		transition:slide={{ axis: 'x', duration: 300 }}
	>
		<div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
			<h2 class="text-lg font-bold text-black">{title}</h2>

			<button
				onclick={onclose}
				class="rounded-md p-1.5 text-gray-400 transition hover:bg-gray-100 hover:text-black"
			>
				<X class="h-5 w-5" />
			</button>
		</div>

		<div class="flex-1 overflow-y-auto p-6">
			{@render children()}
		</div>
	</div>
{/if}
