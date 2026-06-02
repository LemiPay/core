<script lang="ts">
	import { Wallet } from 'lucide-svelte';
	import type { SidebarItem } from './DashboardLayout.svelte';

	const {
		sidebarItems
	}: {
		sidebarItems: SidebarItem[];
	} = $props();
</script>

<aside class="hidden lg:block">
	<div
		class="sticky top-28 rounded-3xl border border-border/80 bg-card/80 p-3 shadow-sm backdrop-blur-xl"
	>
		<div
			class="mb-4 rounded-2xl bg-linear-to-br from-lime-200/70 via-background to-violet-200/60 p-4 dark:from-lime-400/10 dark:via-card dark:to-violet-500/10"
		>
			<div class="flex items-center gap-2">
				<div
					class="flex size-9 items-center justify-center rounded-xl bg-foreground text-background"
				>
					<Wallet class="size-4" />
				</div>
				<div>
					<p class="text-sm font-semibold">Lemipay</p>
					<p class="text-xs text-muted-foreground">Money coordination</p>
				</div>
			</div>
		</div>

		<nav class="space-y-1">
			{#each sidebarItems as item (item.label)}
				{@const Icon = item.icon}
				<a
					href={item.link}
					class={item.active
						? 'flex items-center gap-3 rounded-2xl bg-foreground px-3 py-2.5 text-sm font-medium text-background shadow-sm'
						: 'flex items-center gap-3 rounded-2xl px-3 py-2.5 text-sm font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground'}
				>
					<Icon class="size-4" />
					{item.label}
				</a>
			{/each}
		</nav>
	</div>
</aside>
