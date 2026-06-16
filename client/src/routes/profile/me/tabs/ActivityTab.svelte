<script lang="ts">
	import { formatAmount, formatDate, formatTxType } from '$lib/utils/format_utils';
	import type { BlockchainEvent, Transaction } from '$lib/types/endpoints/transactions.types';

	interface Props {
		transactionsArray: Transaction[];
		blockchainEvents: BlockchainEvent[];
		loadingTransactions: boolean;
	}

	let { transactionsArray, blockchainEvents, loadingTransactions }: Props = $props();

	function mergeAndSort() {
		const txItems = transactionsArray.map((t) => ({
			type: t.tx_type,
			amount: t.amount,
			sign: t.tx_type === 'withdraw' || t.tx_type === 'settlement_payment' ? '-' : '+',
			date: t.created_at,
			sortKey: t.created_at
		}));

		const eventItems = blockchainEvents.map((e) => ({
			type: e.event_type,
			amount: e.net_amount,
			sign: e.event_type === 'Withdraw' ? '-' : '+',
			date: e.created_at,
			sortKey: e.created_at
		}));

		return [...txItems, ...eventItems].sort((a, b) => b.sortKey.localeCompare(a.sortKey));
	}
</script>

<section class="flex animate-in flex-col gap-6 duration-300 fade-in">
	<div class="flex items-center justify-between">
		<h2 class="text-lg font-bold text-black">Actividad Reciente</h2>
	</div>

	<div class="flex flex-col gap-3">
		{#if loadingTransactions}
			<div class="flex justify-center py-8">
				<p class="text-sm text-gray-500">Cargando transacciones...</p>
			</div>
		{:else}
			{#each mergeAndSort() as item}
				<div
					class="flex items-center justify-between rounded-xl border border-gray-200 bg-white p-4 transition hover:border-gray-300"
				>
					<div class="flex flex-col gap-0.5">
						<span class="font-bold text-black">{formatTxType(item.type)}</span>
					</div>
					<div class="flex flex-col items-end gap-0.5">
						<span class="font-bold text-black">
							{item.sign} ${formatAmount(item.amount)}
						</span>
						<span class="text-sm text-gray-500">{formatDate(item.date)}</span>
					</div>
				</div>
			{/each}
			{#if mergeAndSort().length === 0}
				<div class="flex justify-center py-8">
					<p class="text-sm text-gray-500">No hay transacciones recientes.</p>
				</div>
			{/if}
		{/if}
	</div>
</section>
