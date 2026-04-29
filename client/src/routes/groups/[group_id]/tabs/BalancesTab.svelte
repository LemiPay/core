<script lang="ts">
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
	} from '../group.svelte';
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

<div class="animate-in fade-in slide-in-from-bottom-2 space-y-8 duration-300">
	<div class="space-y-1">
		<h2 class="text-sm font-medium text-black">Balances del grupo</h2>
		<p class="text-xs text-gray-500">
			Balances por integrante, sugerencias para saldar y registros.
		</p>
	</div>

	{#if groupState.loadingMembers || groupState.loadingCoreBalances}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
	{:else if groupState.coreBalancesError}
		<div
			class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
		>
			<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
			<div class="space-y-2">
				<p>{groupState.coreBalancesError}</p>
				<button
					type="button"
					class="text-xs font-medium text-rose-700 underline-offset-2 transition hover:underline"
					onclick={() => groupState.loadCoreBalances()}>Reintentar</button
				>
			</div>
		</div>
	{:else if groupState.memberBalances.length === 0}
		<div class="rounded-lg border border-dashed border-gray-300 p-8 text-center">
			<Scale class="mx-auto mb-3 h-8 w-8 text-gray-400" />
			<p class="text-sm font-medium text-black">Sin balances</p>
			<p class="text-sm text-gray-500">No hay movimientos que generen saldos en el core.</p>
		</div>
	{:else}
		<div
			class="flex flex-wrap items-end justify-between gap-3 rounded-xl border border-gray-200 bg-white px-4 py-3"
		>
			<div class="space-y-0.5">
				<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
					Balance grupal (core)
				</p>
				<p class="text-xl font-bold text-black tabular-nums">
					${formatAmount(groupState.coreGroupBalance)}
				</p>
			</div>
		</div>

		<div class="space-y-2">
			<h3 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">Por integrante</h3>
			<div
				class="divide-y divide-gray-100 overflow-hidden rounded-xl border border-gray-200 bg-white"
			>
				{#each groupState.sortedMemberBalances as mb (mb.user.user_id)}
					{@const isPositive = mb.balance > 0.01}
					{@const isNegative = mb.balance < -0.01}
					<div class="flex items-center justify-between gap-3 px-4 py-3">
						<a href={`/users/${mb.user.user_id}`} class="group flex min-w-0 items-center gap-3">
							<div
								class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-700"
							>
								<CircleUser class="h-5 w-5" />
							</div>
							<div class="min-w-0 space-y-0.5">
								<p class="truncate text-sm font-medium text-black group-hover:underline">
									{mb.user.name}
								</p>
							</div>
						</a>
						<div class="flex shrink-0 items-center gap-2">
							{#if isPositive}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-green-200 bg-green-50 px-2 py-1 text-xs font-semibold text-green-700"
								>
									<TrendingUp class="h-3 w-3" /> +${formatAmount(mb.balance)}
								</span>
							{:else if isNegative}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-red-200 bg-red-50 px-2 py-1 text-xs font-semibold text-red-600"
								>
									<TrendingDown class="h-3 w-3" /> -${formatAmount(Math.abs(mb.balance))}
								</span>
							{:else}
								<span
									class="inline-flex items-center gap-1 rounded-md border border-gray-200 bg-gray-50 px-2 py-1 text-xs font-semibold text-gray-500"
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
			<h3 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
				Sugerencias para saldar
			</h3>
			{#if groupState.settlements.length === 0}
				<div class="flex items-center gap-3 rounded-xl border border-green-200 bg-green-50/60 p-4">
					<CircleCheckBig class="h-5 w-5 text-green-700" />
					<p class="text-sm font-medium text-black">Todo al día</p>
				</div>
			{:else}
				<div class="space-y-2">
					{#each groupState.settlements as s, idx (idx)}
						<div
							class="flex items-center justify-between gap-3 rounded-xl border border-gray-200 bg-white p-3"
						>
							<div class="flex items-center gap-2">
								<span class="text-sm font-medium text-black">{s.from.name}</span>
								<ArrowRight class="h-4 w-4 text-gray-300" />
								<span class="text-sm font-medium text-black">{s.to.name}</span>
							</div>
							<span class="text-sm font-semibold text-black">${formatAmount(s.amount)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>
