<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount, formatDate } from '$lib/utils/format_utils';
	import type { GroupState } from '../group.svelte';

	let { groupState, onCreateExpense } = $props<{
		groupState: GroupState;
		onCreateExpense: () => void;
	}>();
</script>

<div class="animate-in space-y-4 duration-300 fade-in slide-in-from-bottom-2">
	<div class="flex items-center justify-between gap-3">
		<div>
			<h3 class="text-sm font-semibold text-foreground">Últimos Gastos</h3>
			<p class="text-xs text-muted-foreground">Se muestran los gastos más recientes del grupo.</p>
		</div>
		<Button label="Agregar Gasto" onclick={onCreateExpense} />
	</div>

	{#if groupState.loadingExpenses}
		<div
			class="flex items-center justify-center rounded-xl border border-border bg-card p-6 text-card-foreground"
		>
			<div
				class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"
			></div>
		</div>
	{:else if groupState.expensesError}
		<div
			class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-600 dark:border-red-400/20 dark:bg-red-400/10 dark:text-red-300"
		>
			{groupState.expensesError}
		</div>
	{:else if groupState.recentExpenses.length === 0}
		<div
			class="rounded-xl border border-dashed border-border bg-card p-6 text-center text-sm text-muted-foreground"
		>
			No hay gastos todavía. Creá el primero desde el botón de arriba.
		</div>
	{:else}
		<div class="space-y-2">
			{#each groupState.recentExpenses as expense (expense.expense_id)}
				<div class="rounded-xl border border-border bg-card p-4 text-card-foreground">
					<div class="flex items-start justify-between gap-3">
						<div class="space-y-1">
							<p class="text-sm font-semibold text-foreground">
								{expense.description || 'Sin descripción'}
							</p>
							<p class="text-xs text-muted-foreground">
								Creado por {groupState.getMemberName(expense.user_id)}
							</p>
						</div>
						<div class="text-right">
							<p class="text-sm font-semibold text-foreground">${formatAmount(expense.amount)}</p>
							<p class="text-xs text-muted-foreground">{formatDate(expense.created_at)}</p>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
