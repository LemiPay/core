<script lang="ts">
	import { Wallet, Copy, HandCoins, Coins } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { shortenAddress, copyToClipboard } from '$lib/utils/address_utils';
	import type { GroupState } from '../group.svelte';
	import { roundBalance } from '$lib/utils/money_utils';

	let {
		groupState,
		readonly = false,
		onCreateWallet,
		onFundWallet,
		onWithdraw
	} = $props<{
		groupState: GroupState;
		readonly?: boolean;
		onCreateWallet: () => void;
		onFundWallet: (walletId: string, currencyId: string) => void;
		onWithdraw: (currencyId: string) => void;
	}>();
</script>

<div class="animate-in space-y-4 duration-300 fade-in slide-in-from-bottom-2">
	{#if groupState.wallets.length === 0}
		<div class="flex items-center justify-between">
			<h2 class="text-sm font-medium text-foreground">Billetera del Grupo</h2>
			{#if !readonly}
				<Button label="Nueva Billetera" variant="primary" onclick={onCreateWallet}>
					{#snippet icon()}<Wallet class="h-4 w-4" />{/snippet}
				</Button>
			{/if}
		</div>
	{/if}

	{#if groupState.loadingWallets}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"></div>
	{:else if groupState.wallets.length > 0}
		<div class="space-y-3 pt-2">
			{#each groupState.wallets as wallet (wallet.id)}
				<div
					class="flex flex-col items-start justify-between gap-4 rounded-lg border border-border bg-card p-4 text-card-foreground transition hover:border-input hover:shadow-sm hover:shadow-black/5 sm:flex-row sm:items-center dark:hover:shadow-black/20"
				>
					<div class="space-y-1">
						<div class="flex items-center gap-2">
							<span class="text-lg font-bold text-foreground">${roundBalance(wallet.balance)}</span>
							<span
								class="rounded bg-primary px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-primary-foreground uppercase"
							>
								{wallet.currency_ticker || 'USDC'}
							</span>
						</div>
						<div class="flex items-center gap-2 text-xs text-muted-foreground">
							<span>{shortenAddress(wallet.address)}</span>
							<button
								type="button"
								class="transition hover:text-foreground"
								aria-label="Copy address"
								onclick={() => copyToClipboard(wallet.address)}
							>
								<Copy class="h-3 w-3" />
							</button>
						</div>
					</div>

					<div class="flex items-center gap-2">
						{#if !readonly}
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
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div
			class="rounded-lg border border-dashed border-border bg-card p-8 text-center text-card-foreground"
		>
			<Wallet class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
			<p class="text-sm font-medium text-foreground">Sin billeteras</p>
			<p class="mb-4 text-sm text-muted-foreground">
				Este grupo no tiene ninguna billetera asociada aún.
			</p>
			{#if !readonly}
				<Button label="Crear primera wallet" variant="secondary" onclick={onCreateWallet} />
			{/if}
		</div>
	{/if}
</div>
