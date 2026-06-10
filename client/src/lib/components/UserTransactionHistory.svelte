<script lang="ts">
	import SideBar from './ui/SideBar.svelte';
	import { roundBalance } from '$lib/utils/money_utils';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess?: () => void;
		transactionsArray: Transaction[];
		blockchainEvents: BlockchainEvent[];
		loadingTransactions: boolean;
	}

	const {
		open,
		onclose,
		onsuccess,
		transactionsArray,
		blockchainEvents,
		loadingTransactions
	}: Props = $props();

	import { formatDate } from '$lib/utils/format_utils';
	import type { BlockchainEvent, Transaction } from '$lib/types/endpoints/transactions.types';

	function translateTxType(type: string) {
		const types: Record<string, string> = {
			deposit: 'Depósito',
			withdraw: 'Retiro',
			expense: 'Gasto',
			investment: 'Inversión',
			Fund: 'Fondeo'
		};
		return types[type] || type;
	}

	function mergeAndSort() {
		const txItems = transactionsArray.map((t) => ({
			type: t.tx_type,
			label: t.description ?? '',
			amount: t.amount,
			sign: t.tx_type === 'withdraw' ? '-' : '+',
			date: t.created_at,
			sortKey: t.created_at
		}));

		const eventItems = blockchainEvents.map((e) => ({
			type: e.event_type,
			label: e.event_type === 'Fund' ? 'Fondeo' : 'Retiro',
			amount: e.net_amount,
			sign: e.event_type === 'Withdraw' ? '-' : '+',
			date: e.created_at,
			sortKey: e.created_at
		}));

		return [...txItems, ...eventItems].sort((a, b) => b.sortKey.localeCompare(a.sortKey));
	}
</script>

<div>
	{#if open}
		<SideBar title={'Actividad Reciente'} {onclose} {open}>
			<section class="flex animate-in flex-col gap-6 duration-300 fade-in">
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
									<span class="font-bold text-black capitalize"
										>{item.type === 'Fund'
											? 'Fondeo'
											: item.type === 'Withdraw'
												? 'Retiro'
												: item.label || translateTxType(item.type)}</span
									>
								</div>
								<div class="flex flex-col items-end gap-0.5">
									<span class="font-bold text-black">
										{item.sign} ${roundBalance(item.amount)}
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
		</SideBar>
	{/if}
</div>
