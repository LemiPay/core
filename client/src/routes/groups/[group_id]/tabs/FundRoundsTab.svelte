<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Plus, HandCoins, Ban, ChevronDown } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount } from '$lib/utils/format_utils';
	import { getProposalStatusDisplay } from '$lib/utils/proposal_status';
	import { shortenAddress } from '$lib/utils/address_utils';
	import type { GroupState } from '../group.svelte';
	import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';

	let { groupState, onCreateFundRound, onCancelFundRound } = $props<{
		groupState: GroupState;
		onCreateFundRound: () => void;
		onCancelFundRound: (id: string) => void;
	}>();

	// Estado local UI
	let expandedFundRoundId = $state<string | null>(null);
	let selectedContribWalletId = $state('');

	function toggleAccordion(id: string) {
		expandedFundRoundId = expandedFundRoundId === id ? null : id;
		selectedContribWalletId = '';
		groupState.contribError = '';
	}

	async function handleContribute(status: FundRoundStatusResponse) {
		const success = await groupState.handleContribute(status, selectedContribWalletId);
		if (success) {
			expandedFundRoundId = null;
			selectedContribWalletId = '';
		}
	}
</script>

{#snippet fundRoundCard(status: FundRoundStatusResponse)}
	{@const proposalId = status.fund_round.proposal.id}
	{@const target = Number(status.target_amount)}
	{@const raised = Number(status.total_contributed)}
	{@const progress = target > 0 ? Math.min(100, Math.round((raised / target) * 100)) : 0}
	{@const currencyId = status.fund_round.fund_round_proposal?.currency_id}
	{@const ticker = groupState.getTickerForCurrency(currencyId)}
	{@const compatibleWallets = groupState.getCompatibleUserWallets(currencyId)}
	{@const isOpen = expandedFundRoundId === proposalId}
	{@const statusDisplay = getProposalStatusDisplay(status.fund_round.proposal.status)}

	<div
		class="group rounded-xl border border-border bg-card text-card-foreground transition hover:border-input hover:shadow-sm hover:shadow-black/5 dark:hover:shadow-black/20"
	>
		<div class="space-y-4 p-5">
			<div class="flex items-start justify-between gap-3">
				<div class="flex items-start gap-3">
					<div
						class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-border bg-muted text-muted-foreground"
					>
						<HandCoins class="h-5 w-5" />
					</div>
					<div>
						<div class="flex items-baseline gap-1.5">
							<span class="text-2xl font-bold text-foreground">${target}</span>
							<span
								class="rounded bg-primary px-1.5 py-0.5 text-[10px] font-bold text-primary-foreground uppercase"
								>{ticker}</span
							>
						</div>
					</div>
				</div>
				<div class="flex items-center gap-1.5">
					<span class="rounded-full border px-2.5 py-1 text-xs font-medium {statusDisplay.classes}"
						>{statusDisplay.label}</span
					>
					{#if groupState.currentUserId === status.fund_round.proposal.created_by && status.fund_round.proposal.status === 'Approved' && !status.is_completed}
						<button
							type="button"
							onclick={() => onCancelFundRound(proposalId)}
							class="flex h-7 w-7 items-center justify-center rounded-full border border-border text-muted-foreground transition hover:border-red-400/40 hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-400/10 dark:hover:text-red-300"
							aria-label="Cancelar ronda"
						>
							<Ban class="h-3.5 w-3.5" />
						</button>
					{/if}
				</div>
			</div>

			<div class="space-y-2">
				<div class="h-2 w-full overflow-hidden rounded-full bg-muted">
					<div
						class="h-full rounded-full bg-primary transition-all duration-700"
						style="width: {progress}%"
					></div>
				</div>
				<div class="flex justify-between text-xs font-medium">
					<span class="text-foreground">${formatAmount(raised)} / ${formatAmount(target)}</span>
					<span class="text-muted-foreground">{progress}%</span>
				</div>
			</div>

			{#if status.fund_round.proposal.status === 'Approved'}
				<div
					class="flex justify-between rounded-lg border border-border bg-muted/70 px-3 py-2.5 text-xs"
				>
					<p class="text-muted-foreground">
						Te toca aportar <span class="font-semibold text-foreground"
							>${formatAmount(groupState.recommendedAmount(status.target_amount))} {ticker}</span
						>
					</p>
					{#if !status.is_completed && !Number(groupState.myContributions[proposalId] ?? 0)}
						<button
							type="button"
							onclick={() => toggleAccordion(proposalId)}
							class="flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-primary-foreground transition hover:bg-primary/90"
						>
							Aportar <ChevronDown class="h-3 w-3 {isOpen ? 'rotate-180' : ''}" />
						</button>
					{/if}
				</div>
			{/if}
		</div>

		{#if isOpen}
			<div transition:slide class="space-y-3 border-t border-border bg-muted/50 p-4">
				<select
					bind:value={selectedContribWalletId}
					class="w-full rounded-md border border-input bg-background px-3 py-2.5 text-sm text-foreground transition focus:border-ring focus:ring-0 focus:outline-none"
				>
					<option value="" disabled>Seleccionar wallet personal...</option>
					{#each compatibleWallets as w (w.wallet_id)}
						<option value={w.wallet_id}
							>{shortenAddress(w.address)} — ${w.balance} {w.ticker}</option
						>
					{/each}
				</select>
				<div class="flex justify-end gap-2">
					<button
						type="button"
						onclick={() => toggleAccordion(proposalId)}
						class="px-3 py-2 text-xs text-muted-foreground transition hover:text-foreground"
						>Cancelar</button
					>
					<button
						type="button"
						onclick={() => handleContribute(status)}
						disabled={!selectedContribWalletId || groupState.contribLoading}
						class="rounded-md bg-primary px-4 py-2 text-xs text-primary-foreground transition hover:bg-primary/90 disabled:opacity-40"
					>
						{groupState.contribLoading ? 'Enviando...' : 'Confirmar Aporte'}
					</button>
				</div>
			</div>
		{/if}
	</div>
{/snippet}

<div class="animate-in space-y-4 duration-300 fade-in slide-in-from-bottom-2">
	<div class="flex items-start justify-between">
		<div>
			<h2 class="text-sm font-medium text-foreground">Rondas de Fondeo</h2>
		</div>
		<Button label="Nueva Ronda" variant="primary" onclick={onCreateFundRound}>
			{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
		</Button>
	</div>

	{#if groupState.loadingFundRounds}
		<div class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"></div>
	{:else if groupState.fundRounds.length === 0}
		<div
			class="rounded-lg border border-dashed border-border bg-card p-8 text-center text-card-foreground"
		>
			<HandCoins class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
			<p class="text-sm font-medium text-foreground">Sin rondas de fondeo</p>
			<Button label="Crear primera ronda" variant="secondary" onclick={onCreateFundRound} />
		</div>
	{:else}
		<div class="space-y-3">
			{#each groupState.activeFundRounds as status (status.fund_round.proposal.id)}
				{@render fundRoundCard(status)}
			{/each}
		</div>
	{/if}
</div>
