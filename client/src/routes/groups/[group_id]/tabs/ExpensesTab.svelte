<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import { formatExpenseDate } from '$lib/utils/format_utils';
	import type { GroupState } from '../group.svelte';

	let { groupState, onCreateExpense } = $props<{
		groupState: GroupState;
		onCreateExpense: () => void;
	}>();
</script>

<div class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
	<div class="flex items-center justify-between gap-3">
		<div>
			<h3 class="text-sm font-semibold text-black">Últimos Gastos</h3>
			<p class="text-xs text-gray-500">Se muestran los gastos más recientes del grupo.</p>
		</div>
		<Button label="Agregar Gasto" onclick={onCreateExpense} />
	</div>

	{#if groupState.loadingExpenses}
		<div class="flex items-center justify-center rounded-xl border border-gray-200 bg-white p-6">
			<div class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
		</div>
	{:else if groupState.expensesError}
		<div class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-600">
			{groupState.expensesError}
		</div>
	{:else if groupState.recentExpenses.length === 0}
		<div
			class="rounded-xl border border-dashed border-gray-300 bg-white p-6 text-center text-sm text-gray-500"
		>
			No hay gastos todavía. Creá el primero desde el botón de arriba.
		</div>
	{:else}
		<div class="space-y-2">
			{#each groupState.recentExpenses as expense (expense.expense_id)}
				<div class="rounded-xl border border-gray-200 bg-white p-4">
					<div class="flex items-start justify-between gap-3">
						<div class="space-y-1">
							<p class="text-sm font-semibold text-black">
								{expense.description || 'Sin descripción'}
							</p>
							<p class="text-xs text-gray-500">
								Creado por {groupState.getMemberName(expense.user_id)}
							</p>
						</div>
						<div class="text-right">
							<p class="text-sm font-semibold text-black">${expense.amount}</p>
							<p class="text-xs text-gray-500">{formatExpenseDate(expense.created_at)}</p>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
