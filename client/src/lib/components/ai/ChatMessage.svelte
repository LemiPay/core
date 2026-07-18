<script lang="ts">
	import { Bot, User } from 'lucide-svelte';
	import MarkdownRenderer from './MarkdownRenderer.svelte';

	interface Props {
		role: 'user' | 'assistant';
		content: string;
	}

	const { role, content }: Props = $props();
</script>

<div class="flex gap-2.5 {role === 'user' ? 'flex-row-reverse' : ''}">
	<div
		class="mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-full {role ===
		'assistant'
			? 'bg-primary text-primary-foreground'
			: 'bg-muted text-muted-foreground'}"
	>
		{#if role === 'assistant'}
			<Bot class="size-3.5" />
		{:else}
			<User class="size-3.5" />
		{/if}
	</div>

	<div
		class="max-w-[80%] overflow-x-auto rounded-2xl px-3.5 py-2.5 text-sm leading-relaxed [&_code]:rounded [&_code]:bg-muted [&_code]:px-1 [&_code]:py-0.5 [&_code]:text-xs [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:bg-muted [&_pre]:p-3 [&_pre]:text-xs [&_pre_code]:bg-transparent [&_pre_code]:p-0 {role ===
		'assistant'
			? 'rounded-bl-md border border-border bg-card text-card-foreground'
			: 'rounded-br-md bg-primary text-primary-foreground'}"
	>
		{#if role === 'assistant'}
			<MarkdownRenderer {content} />
		{:else}
			{content}
		{/if}
	</div>
</div>
