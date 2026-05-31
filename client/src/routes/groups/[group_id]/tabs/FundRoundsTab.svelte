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
		Wallet
	} from 'lucide-svelte';
	import { formatAmount } from '$lib/utils/format_utils';
	import { getProposalStatusDisplay } from '$lib/utils/proposal_status';
	import { shortenAddress } from '$lib/utils/address_utils';
	import type { GroupState } from '../group.svelte';
	import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';
	import FundRoundCard from '$lib/components/pages/fundRound/FundRoundCard.svelte';

	let { groupState, onCreateFundRound, onCancelFundRound } = $props<{
		groupState: GroupState;
		onCreateFundRound: () => void;
		onCancelFundRound: (id: string) => void;
	}>();

	let currentUserId = () => groupState.currentUserId;

	// Estado local UI
	let expandedFundRoundId = $state<string | null>(null);
	let selectedContribWalletId = $state('');

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

<div class="space-y-3 pt-2">
	{#if groupState.activeFundRounds.length > 0}
		{#each groupState.activeFundRounds as status (status.fund_round.proposal.id)}
			<FundRoundCard
				{status}
				{expandedFundRoundId}
				{selectedContribWalletId}
				recommended={groupState.recommendedAmount(status.target_amount)}
				myContribution={Number(groupState.myContributions[status.fund_round.proposal.id] ?? '0')}
				ticker={groupState.getTickerForCurrency(status.fund_round.fund_round_proposal.currency_id)}
				compatibleWallets={groupState.getCompatibleUserWallets(
					status.fund_round.fund_round_proposal.currency_id
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
				onCancelRound={(id) => groupState.openCancelFundRoundModal(id)}
			/>
		{/each}
	{:else}
		<div
			class="flex flex-col items-center gap-1 rounded-xl border border-dashed border-gray-300 p-6 text-center"
		>
			<HandCoins class="h-6 w-6 text-gray-400" />
			<p class="text-sm font-medium text-black">No hay rondas activas</p>
			<p class="text-xs text-gray-500">Todas las rondas están finalizadas o canceladas.</p>
		</div>
	{/if}

	{#if groupState.pastFundRounds.length > 0}
		<div class="flex items-center gap-3 pt-4 pb-1">
			<div class="h-px flex-1 bg-gray-200"></div>
			<button
				type="button"
				onclick={() => (groupState.showPastFundRounds = !groupState.showPastFundRounds)}
				class="inline-flex items-center gap-1.5 rounded-full border border-gray-200 bg-white px-3 py-1 text-[11px] font-medium text-gray-600 transition hover:border-gray-300 hover:text-black"
			>
				{groupState.showPastFundRounds ? 'Ocultar' : 'Ver'} rondas pasadas

				<span class="rounded-full bg-gray-100 px-1.5 text-[10px] font-semibold text-gray-600">
					{groupState.pastFundRounds.length}
				</span>

				<ChevronDown
					class={[
						'h-3 w-3 transition-transform duration-200',
						groupState.showPastFundRounds ? 'rotate-180' : ''
					].join(' ')}
				/>
			</button>
			<div class="h-px flex-1 bg-gray-200"></div>
		</div>

		{#if groupState.showPastFundRounds}
			<div transition:slide={{ duration: 220 }} class="space-y-3">
				{#each groupState.pastFundRounds as status (status.fund_round.proposal.id)}
					<FundRoundCard
						{status}
						{expandedFundRoundId}
						{selectedContribWalletId}
						recommended={groupState.recommendedAmount(status.target_amount)}
						myContribution={Number(
							groupState.myContributions[status.fund_round.proposal.id] ?? '0'
						)}
						ticker={groupState.getTickerForCurrency(
							status.fund_round.fund_round_proposal.currency_id
						)}
						compatibleWallets={groupState.getCompatibleUserWallets(
							status.fund_round.fund_round_proposal.currency_id
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
						onCancelRound={(id) => groupState.openCancelFundRoundModal(id)}
					/>
				{/each}
				{#if groupState.sortedGroupTransactions.length > 3}
					<button
						type="button"
						class="text-xs font-medium text-gray-600 underline-offset-2 transition hover:text-black hover:underline"
						onclick={() => (groupState.showAllTransactions = !groupState.showAllTransactions)}
					>
						{groupState.showAllTransactions
							? 'Ver menos'
							: `Ver todo (${groupState.sortedGroupTransactions.length})`}
					</button>
				{/if}
			</div>
		{/if}
	{/if}
</div>
