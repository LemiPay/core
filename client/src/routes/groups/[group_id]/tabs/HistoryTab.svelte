<script lang="ts">
	import type { GroupState } from '../group.svelte';
	import { formatAmount, formatDateTimeShort, formatTxType } from '$lib/utils/format_utils';

	let { groupState } = $props<{
		groupState: GroupState;
	}>();
</script>

<div class="space-y-3">
	<div class="flex flex-wrap items-center gap-2">
		<h3 class="text-sm font-medium text-black">Actividad y registros</h3>
		<span
			class="rounded-full bg-gray-100 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-gray-700 uppercase"
		>
			Listados
		</span>
	</div>

	{#if groupState.loadingBalancesDetail}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
	{:else}
		<div class="space-y-6">
			<div class="space-y-2">
				<div class="flex items-center gap-2">
					<h4 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
						Transacciones
					</h4>
					<span
						class="rounded-full bg-sky-100 px-2 py-0.5 text-[10px] font-semibold text-sky-900 uppercase"
					>
						Transacción
					</span>
				</div>
				{#if groupState.transactionsDetailError}
					<p class="text-xs text-rose-600">{groupState.transactionsDetailError}</p>
				{:else if groupState.sortedGroupTransactions.length === 0}
					<p class="text-xs text-gray-500">No hay transacciones en este grupo.</p>
				{:else}
					<div class="space-y-2">
						{#each groupState.sortedGroupTransactions as tx (tx.id)}
							<div
								class="flex flex-col gap-2 rounded-xl border border-gray-200 bg-white px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
							>
								<div class="min-w-0 space-y-1">
									<div class="flex flex-wrap items-center gap-2">
										<span
											class="rounded-full bg-sky-100 px-2 py-0.5 text-[10px] font-semibold text-sky-900 uppercase"
										>
											Transacción
										</span>
										<span class="text-xs font-medium text-gray-800">
											{formatTxType(tx.tx_type)}
										</span>
										<span class="text-[11px] text-gray-400">
											{formatDateTimeShort(tx.created_at)}
										</span>
									</div>
									{#if tx.description}
										<p class="truncate text-xs text-gray-600">{tx.description}</p>
									{/if}
									<p class="text-[11px] text-gray-400">
										Usuario: <span class="font-mono text-gray-600">{tx.address.slice(0, 8)}…</span>
									</p>
								</div>
								<p class="shrink-0 text-sm font-semibold text-black tabular-nums">
									${formatAmount(tx.amount)}
								</p>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<div class="space-y-2">
				<div class="flex items-center gap-2">
					<h4 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">Gastos</h4>
					<span
						class="rounded-full bg-violet-100 px-2 py-0.5 text-[10px] font-semibold text-violet-900 uppercase"
					>
						Gasto
					</span>
				</div>
				{#if groupState.expensesDetailError}
					<p class="text-xs text-rose-600">{groupState.expensesDetailError}</p>
				{:else if groupState.sortedGroupExpenses.length === 0}
					<p class="text-xs text-gray-500">No hay gastos registrados para este grupo.</p>
				{:else}
					<div class="space-y-2">
						{#each groupState.sortedGroupExpenses as ex (ex.expense_id)}
							<div
								class="flex flex-col gap-2 rounded-xl border border-gray-200 bg-white px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
							>
								<div class="min-w-0 space-y-1">
									<div class="flex flex-wrap items-center gap-2">
										<span
											class="rounded-full bg-violet-100 px-2 py-0.5 text-[10px] font-semibold text-violet-900 uppercase"
										>
											Gasto
										</span>
										<span class="text-[11px] text-gray-400">
											{formatDateTimeShort(ex.created_at)}
										</span>
										<span
											class="rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 text-[10px] font-medium text-gray-600 uppercase"
										>
											{ex.status}
										</span>
									</div>
									{#if ex.description}
										<p class="text-xs text-gray-800">{ex.description}</p>
									{:else}
										<p class="text-xs text-gray-400">Sin descripción</p>
									{/if}
									<p class="text-[11px] text-gray-400">
										Cargado por:
										<span class="font-mono text-gray-600">{ex.user_id.slice(0, 8)}…</span>
									</p>
								</div>
								<p class="shrink-0 text-sm font-semibold text-black tabular-nums">
									${formatAmount(ex.amount)}
								</p>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
