<script lang="ts">
	import { resolve } from '$app/paths';
	import {
		Info,
		Scale,
		CircleUser,
		TrendingUp,
		TrendingDown,
		CircleCheckBig,
		ArrowRight
	} from 'lucide-svelte';
	import {
		formatAmount,
		parseBalanceValue,
		formatTxType,
		formatDateTimeShort
	} from '$lib/utils/format_utils';
	import type { GroupState } from '../group.svelte';

	let { groupState } = $props<{ groupState: GroupState }>();

	// Estado local UI
	let showAllTransactions = $state(false);
	let showAllExpenses = $state(false);

	let visibleGroupTransactions = $derived(
		showAllTransactions
			? groupState.sortedGroupTransactions
			: groupState.sortedGroupTransactions.slice(0, 3)
	);
	let visibleGroupExpenses = $derived(
		showAllExpenses ? groupState.sortedGroupExpenses : groupState.sortedGroupExpenses.slice(0, 3)
	);
</script>

<div class="animate-in space-y-8 duration-300 fade-in slide-in-from-bottom-2">
	<div class="space-y-1">
		<h2 class="text-sm font-medium text-foreground">Balances del grupo</h2>
		<p class="text-xs text-muted-foreground">
			Balances por integrante, sugerencias para saldar y registros.
		</p>
	</div>

	{#if groupState.loadingMembers || groupState.loadingCoreBalances}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"></div>
	{:else if groupState.coreBalancesError}
		<div
			class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
		>
			<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500 dark:text-rose-300" />
			<div class="space-y-2">
				<p>{groupState.coreBalancesError}</p>
				<button
					type="button"
					class="text-xs font-medium text-rose-700 underline-offset-2 transition hover:underline dark:text-rose-300"
					onclick={() => groupState.loadCoreBalances()}>Reintentar</button
				>
			</div>
		</div>
	{:else if groupState.memberBalances.length === 0}
		<div
			class="rounded-lg border border-dashed border-border bg-card p-8 text-center text-card-foreground"
		>
			<Scale class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
			<p class="text-sm font-medium text-foreground">Sin balances</p>
			<p class="text-sm text-muted-foreground">No hay movimientos que generen saldos en el core.</p>
		</div>
	{:else}
		<div
			class="flex flex-wrap items-end justify-between gap-3 rounded-xl border border-border bg-card px-4 py-3 text-card-foreground"
		>
			<div class="space-y-0.5">
				<p class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
					Balance grupal (core)
				</p>
				<p class="text-xl font-bold text-foreground tabular-nums">
					${formatAmount(groupState.coreGroupBalance)}
				</p>
			</div>
		</div>

		<div class="space-y-2">
			<h3 class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
				Por integrante
			</h3>
			<div
				class="divide-y divide-border overflow-hidden rounded-xl border border-border bg-card text-card-foreground"
			>
				{#each groupState.sortedMemberBalances as mb (mb.user.user_id)}
					{@const isPositive = mb.balance > 0.01}
					{@const isNegative = mb.balance < -0.01}
					<div class="flex items-center justify-between gap-3 px-4 py-3">
						<a
							href={resolve('/users/[user_id]', { user_id: mb.user.user_id })}
							class="group flex min-w-0 items-center gap-3"
						>
							<div
								class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-border bg-muted text-muted-foreground"
							>
								<CircleUser class="h-5 w-5" />
							</div>
							<div class="min-w-0 space-y-0.5">
								<p class="truncate text-sm font-medium text-foreground group-hover:underline">
									{mb.user.name}
								</p>
							</div>
						</a>
						<div class="flex shrink-0 items-center gap-2">
							{#if isPositive}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-green-200 bg-green-50 px-2 py-1 text-xs font-semibold text-green-700 dark:border-green-400/20 dark:bg-green-400/10 dark:text-green-300"
								>
									<TrendingUp class="h-3 w-3" /> +${formatAmount(mb.balance)}
								</span>
							{:else if isNegative}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-red-200 bg-red-50 px-2 py-1 text-xs font-semibold text-red-600 dark:border-red-400/20 dark:bg-red-400/10 dark:text-red-300"
								>
									<TrendingDown class="h-3 w-3" /> -${formatAmount(Math.abs(mb.balance))}
								</span>
							{:else}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-border bg-muted px-2 py-1 text-xs font-semibold text-muted-foreground"
								>
									<CircleCheckBig class="h-3 w-3" /> Saldado
								</span>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>

		<div class="space-y-2">
			<h3 class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
				Sugerencias para saldar
			</h3>
			{#if groupState.settlements.length === 0}
				<div
					class="flex items-center gap-3 rounded-xl border border-green-200 bg-green-50/60 p-4 dark:border-green-400/20 dark:bg-green-400/10"
				>
					<CircleCheckBig class="h-5 w-5 text-green-700 dark:text-green-300" />
					<p class="text-sm font-medium text-foreground">Todo al día</p>
				</div>
			{:else}
				<div class="space-y-2">
					{#each groupState.settlements as s, idx (idx)}
						<div
							class="flex items-center justify-between gap-3 rounded-xl border border-border bg-card p-3 text-card-foreground"
						>
							<div class="flex items-center gap-2">
								<span class="text-sm font-medium text-foreground">{s.from.name}</span>
								<ArrowRight class="h-4 w-4 text-muted-foreground" />
								<span class="text-sm font-medium text-foreground">{s.to.name}</span>
							</div>
							<span class="text-sm font-semibold text-foreground">${formatAmount(s.amount)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>
