<script lang="ts">
	import { formatDate } from '$lib/utils/format_utils';
	import type { Transaction } from '$lib/types/endpoints/transactions.types';

	interface Props {
		transactionsArray: Transaction[];
		loadingTransactions: boolean;
	}

	let { transactionsArray, loadingTransactions }: Props = $props();

	function translateTxType(type: string) {
		const types: Record<string, string> = {
			deposit: 'Depósito',
			withdraw: 'Retiro',
			Fund: 'Fondeo'
		};
		return types[type] || type;
	}
</script>

<section class="animate-in fade-in flex flex-col gap-6 duration-300">
	<div class="flex items-center justify-between">
		<h2 class="text-lg font-bold text-black">Actividad Reciente</h2>
	</div>

	<div class="flex flex-col gap-3">
		{#if loadingTransactions}
			<div class="flex justify-center py-8">
				<p class="text-sm text-gray-500">Cargando transacciones...</p>
			</div>
		{:else if transactionsArray.length === 0}
			<div class="flex justify-center py-8">
				<p class="text-sm text-gray-500">No hay transacciones recientes.</p>
			</div>
		{:else}
			{#each transactionsArray as tx}
				<div
					class="flex items-center justify-between rounded-xl border border-gray-200 bg-white p-4 transition hover:border-gray-300"
				>
					<div class="flex flex-col gap-0.5">
						<span class="font-bold text-black capitalize">{translateTxType(tx.tx_type)}</span>
						<span class="text-sm text-gray-500">
							{tx.description ? tx.description : ''}
						</span>
					</div>
					<div class="flex flex-col items-end gap-0.5">
						<span class="font-bold text-black">
							{tx.tx_type === 'withdraw' ? '+' : '-'} ${tx.amount}
						</span>
						<span class="text-sm text-gray-500">{formatDate(tx.created_at)}</span>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</section>
