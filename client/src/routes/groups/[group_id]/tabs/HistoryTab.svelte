<script lang="ts">
	import type { GroupState } from '../group.svelte';
	import { formatAmount, formatDateTimeShort, formatTxType } from '$lib/utils/format_utils';

	let { groupState } = $props<{
		groupState: GroupState;
	}>();
</script>

<div class="space-y-3">
	<div class="flex flex-wrap items-center gap-2">
		<h3 class="text-sm font-medium text-foreground">Actividad y registros</h3>
		<span
			class="rounded-full bg-muted px-2 py-0.5 text-[10px] font-semibold tracking-wide text-muted-foreground uppercase"
		>
			Listados
		</span>
	</div>

	{#if groupState.loadingBalancesDetail}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"></div>
	{:else}
		<div class="space-y-6">
			<div class="space-y-2">
				<div class="flex items-center gap-2">
					<h4 class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Transacciones
					</h4>
					<span
						class="rounded-full border border-sky-200 bg-sky-50 px-2 py-0.5 text-[10px] font-semibold text-sky-800 uppercase dark:border-sky-400/20 dark:bg-sky-400/10 dark:text-sky-300"
					>
						Transacción
					</span>
				</div>
				{#if groupState.transactionsDetailError}
					<p class="text-xs text-rose-600 dark:text-rose-300">
						{groupState.transactionsDetailError}
					</p>
				{:else if groupState.sortedGroupTransactions.length === 0}
					<p class="text-xs text-muted-foreground">No hay transacciones en este grupo.</p>
				{:else}
					<div class="space-y-2">
						{#each groupState.sortedGroupTransactions as tx (tx.id)}
							<div
								class="flex flex-col gap-2 rounded-xl border border-border bg-card px-4 py-3 text-card-foreground sm:flex-row sm:items-center sm:justify-between"
							>
								<div class="min-w-0 space-y-1">
									<div class="flex flex-wrap items-center gap-2">
										<span
											class="rounded-full border border-sky-200 bg-sky-50 px-2 py-0.5 text-[10px] font-semibold text-sky-800 uppercase dark:border-sky-400/20 dark:bg-sky-400/10 dark:text-sky-300"
										>
											Transacción
										</span>
										<span class="text-xs font-medium text-foreground">
											{formatTxType(tx.tx_type)}
										</span>
										<span class="text-[11px] text-muted-foreground">
											{formatDateTimeShort(tx.created_at)}
										</span>
									</div>
									{#if tx.description}
										<p class="truncate text-xs text-muted-foreground">{tx.description}</p>
									{/if}
									<p class="text-[11px] text-muted-foreground">
										Usuario:
										<span class="font-mono text-foreground/80">{tx.address.slice(0, 8)}…</span>
									</p>
								</div>
								<p class="shrink-0 text-sm font-semibold text-foreground tabular-nums">
									${formatAmount(tx.amount)}
								</p>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<div class="space-y-2">
				<div class="flex items-center gap-2">
					<h4 class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Gastos
					</h4>
					<span
						class="rounded-full border border-violet-200 bg-violet-50 px-2 py-0.5 text-[10px] font-semibold text-violet-800 uppercase dark:border-violet-400/20 dark:bg-violet-400/10 dark:text-violet-300"
					>
						Gasto
					</span>
				</div>
				{#if groupState.expensesDetailError}
					<p class="text-xs text-rose-600 dark:text-rose-300">{groupState.expensesDetailError}</p>
				{:else if groupState.sortedGroupExpenses.length === 0}
					<p class="text-xs text-muted-foreground">No hay gastos registrados para este grupo.</p>
				{:else}
					<div class="space-y-2">
						{#each groupState.sortedGroupExpenses as ex (ex.expense_id)}
							<div
								class="flex flex-col gap-2 rounded-xl border border-border bg-card px-4 py-3 text-card-foreground sm:flex-row sm:items-center sm:justify-between"
							>
								<div class="min-w-0 space-y-1">
									<div class="flex flex-wrap items-center gap-2">
										<span
											class="rounded-full border border-violet-200 bg-violet-50 px-2 py-0.5 text-[10px] font-semibold text-violet-800 uppercase dark:border-violet-400/20 dark:bg-violet-400/10 dark:text-violet-300"
										>
											Gasto
										</span>
										<span class="text-[11px] text-muted-foreground">
											{formatDateTimeShort(ex.created_at)}
										</span>
										<span
											class="rounded border border-border bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground uppercase"
										>
											{ex.status}
										</span>
									</div>
									{#if ex.description}
										<p class="text-xs text-foreground">{ex.description}</p>
									{:else}
										<p class="text-xs text-muted-foreground">Sin descripción</p>
									{/if}
									<p class="text-[11px] text-muted-foreground">
										Cargado por:
										<span class="font-mono text-foreground/80">{ex.user_id.slice(0, 8)}…</span>
									</p>
								</div>
								<p class="shrink-0 text-sm font-semibold text-foreground tabular-nums">
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
