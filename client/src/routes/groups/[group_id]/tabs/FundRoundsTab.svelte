<script lang="ts">
	import { slide } from 'svelte/transition';
	import {
		Ban,
		ChevronDown,
		HandCoins,
		Calendar,
		Target,
		Users,
		CircleCheckBig,
		Wallet,
		Plus
	} from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount } from '$lib/utils/format_utils';
	import { getProposalStatusDisplay } from '$lib/utils/proposal_status';
	import { shortenAddress } from '$lib/utils/address_utils';
	import type { GroupState } from '../group.svelte';
	import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';
	import FundRoundCard from '$lib/components/pages/fundRound/FundRoundCard.svelte';

	let {
		groupState,
		readonly = false,
		onCreateFundRound,
		onCancelFundRound
	} = $props<{
		groupState: GroupState;
		readonly?: boolean;
		onCreateFundRound: () => void;
		onCancelFundRound: (id: string) => void;
	}>();

	let currentUserId = () => groupState.currentUserId;

	let expandedFundRoundId = $state<string | null>(null);
	let selectedContribWalletId = $state('');
	let showPastFundRounds = $state(false);

	function toggleFundRoundAccordion(id: string) {
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

<div class="animate-in space-y-4 duration-300 fade-in slide-in-from-bottom-2">
	<div class="flex items-start justify-between gap-4">
		<div class="space-y-1">
			<h2 class="text-sm font-medium text-foreground">Rondas de Fondeo</h2>
			<p class="text-xs text-muted-foreground">
				Aportes colectivos para fondear una billetera del grupo.
			</p>
		</div>
		{#if !readonly}
			<Button label="Nueva Ronda" variant="primary" onclick={onCreateFundRound}>
				{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
			</Button>
		{/if}
	</div>

	{#if groupState.fundRoundsError}
		<div
			class="rounded-md border border-red-200 bg-red-50 p-3 text-sm text-red-600 dark:border-red-400/20 dark:bg-red-400/10 dark:text-red-300"
		>
			{groupState.fundRoundsError}
		</div>
	{/if}

	{#if groupState.loadingFundRounds}
		<div class="flex justify-center py-8">
			<div
				class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"
			></div>
		</div>
	{:else if groupState.fundRounds.length === 0 && !groupState.fundRoundsError}
		<div
			class="rounded-lg border border-dashed border-border bg-card p-8 text-center text-card-foreground"
		>
			<HandCoins class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
			<p class="text-sm font-medium text-foreground">Sin rondas de fondeo</p>
			<p class="mb-4 text-sm text-muted-foreground">Este grupo no tiene rondas de fondeo aún.</p>
			{#if !readonly}
				<Button label="Crear primera ronda" variant="secondary" onclick={onCreateFundRound}>
					{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
				</Button>
			{/if}
		</div>
	{:else}
		<div class="space-y-3 pt-2">
			{#if groupState.activeFundRounds.length > 0}
				{#each groupState.activeFundRounds as status (status.fund_round.proposal.id)}
					<FundRoundCard
						{status}
						{readonly}
						{expandedFundRoundId}
						bind:selectedContribWalletId
						recommended={groupState.recommendedAmount(status.target_amount)}
						myContribution={Number(
							groupState.myContributions[status.fund_round.proposal.id] ?? '0'
						)}
						ticker={groupState.getTickerForCurrency(status.fund_round.currency_id)}
						compatibleWallets={groupState.getCompatibleUserWallets(status.fund_round.currency_id)}
						contribLoading={groupState.contribLoading}
						contribError={groupState.contribError}
						memberCount={groupState.members.length}
						currentUserId={currentUserId()}
						statusDisplay={getProposalStatusDisplay(status.fund_round.proposal.status)}
						{formatAmount}
						{shortenAddress}
						onToggleAccordion={toggleFundRoundAccordion}
						onContribute={handleContribute}
						onCancelRound={(id) => onCancelFundRound(id)}
					/>
				{/each}
			{:else}
				<div
					class="flex flex-col items-center gap-1 rounded-xl border border-dashed border-border p-6 text-center"
				>
					<HandCoins class="h-6 w-6 text-muted-foreground" />
					<p class="text-sm font-medium text-foreground">No hay rondas activas</p>
					<p class="text-xs text-muted-foreground">
						Todas las rondas están finalizadas o canceladas.
					</p>
				</div>
			{/if}

			{#if groupState.pastFundRounds.length > 0}
				<div class="flex items-center gap-3 pt-4 pb-1">
					<div class="h-px flex-1 bg-border"></div>
					<button
						type="button"
						onclick={() => (showPastFundRounds = !showPastFundRounds)}
						class="inline-flex items-center gap-1.5 rounded-full border border-border bg-card px-3 py-1 text-[11px] font-medium text-muted-foreground transition hover:border-input hover:text-foreground"
					>
						{showPastFundRounds ? 'Ocultar' : 'Ver'} rondas pasadas
						<span
							class="rounded-full bg-muted px-1.5 text-[10px] font-semibold text-muted-foreground"
						>
							{groupState.pastFundRounds.length}
						</span>
						<ChevronDown
							class={[
								'h-3 w-3 transition-transform duration-200',
								showPastFundRounds ? 'rotate-180' : ''
							].join(' ')}
						/>
					</button>
					<div class="h-px flex-1 bg-border"></div>
				</div>

				{#if showPastFundRounds}
					<div transition:slide={{ duration: 220 }} class="space-y-3">
						{#each groupState.pastFundRounds as status (status.fund_round.proposal.id)}
							<FundRoundCard
								{status}
								{readonly}
								{expandedFundRoundId}
								bind:selectedContribWalletId
								recommended={groupState.recommendedAmount(status.target_amount)}
								myContribution={Number(
									groupState.myContributions[status.fund_round.proposal.id] ?? '0'
								)}
								ticker={groupState.getTickerForCurrency(status.fund_round.currency_id)}
								compatibleWallets={groupState.getCompatibleUserWallets(
									status.fund_round.currency_id
								)}
								contribLoading={groupState.contribLoading}
								contribError={groupState.contribError}
								memberCount={groupState.members.length}
								currentUserId={currentUserId()}
								statusDisplay={getProposalStatusDisplay(status.fund_round.proposal.status)}
								{formatAmount}
								{shortenAddress}
								onToggleAccordion={toggleFundRoundAccordion}
								onContribute={handleContribute}
								onCancelRound={(id) => onCancelFundRound(id)}
							/>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	{/if}
</div>
