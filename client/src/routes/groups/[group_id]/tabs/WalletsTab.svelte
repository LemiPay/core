<script lang="ts">
	import { Wallet, Copy, HandCoins, Coins } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';
	import type { GroupState } from '../group.svelte';

	let { groupState, onCreateWallet, onFundWallet, onWithdraw } = $props<{
		groupState: GroupState;
		onCreateWallet: () => void;
		onFundWallet: (walletId: string, currencyId: string) => void;
		onWithdraw: (currencyId: string) => void;
	}>();
</script>

<div class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
	<div class="flex items-center justify-between">
		<h2 class="text-sm font-medium text-black">Billeteras del Grupo</h2>
		<Button label="Nueva Wallet" variant="primary" onclick={onCreateWallet}>
			{#snippet icon()}<Wallet class="h-4 w-4" />{/snippet}
		</Button>
	</div>

	{#if groupState.loadingWallets}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
	{:else if groupState.wallets.length > 0}
		<div class="space-y-3 pt-2">
			{#each groupState.wallets as wallet}
				<div
					class="flex flex-col items-start justify-between gap-4 rounded-lg border border-gray-200 bg-white p-4 sm:flex-row sm:items-center"
				>
					<div class="space-y-1">
						<div class="flex items-center gap-2">
							<span class="text-lg font-bold text-black">${wallet.balance}</span>
							<span
								class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
							>
								{wallet.currency_ticker || 'USDC'}
							</span>
						</div>
						<div class="flex items-center gap-2 text-xs text-gray-500">
							<span>{shortenAddress(wallet.address)}</span>
							<button class="transition hover:text-black" aria-label="Copy address">
								<Copy class="h-3 w-3" />
							</button>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<Button
							label="Retirar"
							variant="secondary"
							onclick={() => onWithdraw(wallet.currency_id)}
						>
							{#snippet icon()}<HandCoins class="h-4 w-4" />{/snippet}
						</Button>
						<Button
							label="Fondear"
							variant="secondary"
							onclick={() => onFundWallet(wallet.id, wallet.currency_id)}
						>
							{#snippet icon()}<Coins class="h-4 w-4" />{/snippet}
						</Button>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="rounded-lg border border-dashed border-gray-300 p-8 text-center">
			<Wallet class="mx-auto mb-3 h-8 w-8 text-gray-400" />
			<p class="text-sm font-medium text-black">Sin billeteras</p>
			<p class="mb-4 text-sm text-gray-500">Este grupo no tiene ninguna billetera asociada aún.</p>
			<Button label="Crear primera wallet" variant="secondary" onclick={onCreateWallet} />
		</div>
	{/if}
</div>
