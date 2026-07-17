<script lang="ts">
	import { Check } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	export type StepItem = {
		title: string;
		description?: string;
	};

	interface Props {
		steps: StepItem[];
		/** 1-based current step index */
		current: number;
		class?: string;
	}

	const { steps, current, class: className = '' }: Props = $props();

	const progressPct = $derived(
		steps.length <= 1 ? 0 : ((Math.min(current, steps.length) - 1) / (steps.length - 1)) * 100
	);
</script>

<nav aria-label="Progreso" class={cn('w-full', className)}>
	<!-- Track + progress line (aligned to circle centers) -->
	<div class="relative mb-2 px-4">
		<div
			class="absolute top-4 right-4 left-4 h-0.5 rounded-full bg-border"
			aria-hidden="true"
		></div>
		<div
			class="absolute top-4 left-4 h-0.5 rounded-full bg-primary transition-all duration-300"
			style="width: calc((100% - 2rem) * {progressPct / 100})"
			aria-hidden="true"
		></div>

		<ol class="relative flex w-full items-start justify-between">
			{#each steps as step, i (step.title)}
				{@const index = i + 1}
				{@const done = index < current}
				{@const active = index === current}

				<li class="flex flex-col items-center gap-1.5">
					<div
						class={cn(
							'flex size-8 items-center justify-center rounded-full border-2 text-xs font-semibold transition-colors',
							done && 'border-primary bg-primary text-primary-foreground',
							active &&
								'border-primary bg-background text-primary shadow-[0_0_0_4px] shadow-primary/15',
							!done && !active && 'border-border bg-background text-muted-foreground'
						)}
						aria-current={active ? 'step' : undefined}
					>
						{#if done}
							<Check class="size-4" strokeWidth={2.5} />
						{:else}
							{index}
						{/if}
					</div>
					<div class="max-w-24 text-center sm:max-w-none">
						<p
							class={cn(
								'text-xs font-semibold',
								active || done ? 'text-foreground' : 'text-muted-foreground'
							)}
						>
							{step.title}
						</p>
						{#if step.description}
							<p class="mt-0.5 text-[11px] leading-snug text-muted-foreground">
								{step.description}
							</p>
						{/if}
					</div>
				</li>
			{/each}
		</ol>
	</div>
</nav>
