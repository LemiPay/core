<script lang="ts">
	import { Wallet, Copy, Plus, ArrowDownToLine, Send } from 'lucide-svelte';
	import { shortenAddress, copyToClipboard } from '$lib/utils/address_utils';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';

	interface Props {
		walletsArray: WalletInfo[];
		loadingWalletsInfo: boolean;
		onCreateWallet: () => void;
		onReceive: (wallet_id: string, ticker: string) => void;
		onSend: (sender_wallet_id: string, ticker: string) => void;
	}

	let { walletsArray, loadingWalletsInfo, onCreateWallet, onReceive, onSend }: Props = $props();
</script>

<section class="animate-in fade-in flex flex-col gap-6 duration-300">
	<div class="flex items-center justify-between">
		<h2 class="text-lg font-bold text-black">Tus Direcciones</h2>
		<button
			class="flex items-center gap-2 rounded-full border border-gray-200 px-4 py-2 text-sm font-medium text-black transition hover:border-black hover:bg-gray-50"
			onclick={onCreateWallet}
		>
			<Plus size={16} />
			Nueva Dirección
		</button>
	</div>

	<div class="flex w-full flex-col gap-6">
		{#each walletsArray as group}
			<div class="flex flex-col overflow-hidden rounded-xl border border-gray-200 bg-white">
				<div
					class="flex items-center justify-between border-b border-gray-100 bg-gray-50 px-4 py-3"
				>
					<div class="flex items-center gap-2 text-gray-500">
						<Wallet size={16} />
						<span class="font-mono text-sm">{shortenAddress(group.address)}</span>
					</div>
					<button
						onclick={() => copyToClipboard(group.address)}
						class="flex items-center gap-1.5 rounded-md px-2 py-1 text-xs font-medium text-gray-500 transition hover:bg-gray-200 hover:text-black"
					>
						<Copy size={14} />
						Copiar
					</button>
				</div>

				<div class="flex flex-col px-4 py-2">
					{#each group.currencies as currency}
						<div
							class="flex items-center justify-between border-b border-gray-50 py-3 last:border-0"
						>
							<div class="flex flex-col">
								<span class="text-2xl font-bold text-black">
									{currency.balance}
									<span class="text-base font-medium text-gray-500">{currency.ticker}</span>
								</span>
							</div>
							<div class="flex gap-2">
								<button
									class="flex items-center gap-1.5 rounded-full border border-gray-200 px-4 py-1.5 text-sm font-medium text-black transition hover:border-gray-400 hover:bg-gray-50"
									onclick={() => onReceive(currency.wallet_id, currency.ticker)}
								>
									<ArrowDownToLine size={14} />
									Recibir
								</button>
								<button
									class="flex items-center gap-1.5 rounded-full bg-black px-4 py-1.5 text-sm font-medium text-white transition hover:bg-gray-800"
									onclick={() => onSend(currency.wallet_id, currency.ticker)}
								>
									<Send size={14} />
									Enviar
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/each}

		{#if walletsArray.length === 0 && !loadingWalletsInfo}
			<p class="py-8 text-center text-sm text-gray-500">Aún no tienes billeteras creadas.</p>
		{/if}
	</div>
</section>
