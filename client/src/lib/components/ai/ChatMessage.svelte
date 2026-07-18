<script lang="ts">
	import { Bot, User } from 'lucide-svelte';

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
		class="max-w-[80%] rounded-2xl px-3.5 py-2.5 text-sm leading-relaxed {role === 'assistant'
			? 'rounded-bl-md border border-border bg-card text-card-foreground'
			: 'rounded-br-md bg-primary text-primary-foreground'}"
	>
		{content}
	</div>
</div>
