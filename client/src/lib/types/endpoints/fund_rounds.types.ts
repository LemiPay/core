import type { Proposal } from '$lib/types/endpoints/proposals.types';

type ProposalType = 'FundRound';

export type FundRoundProposal = {
	proposal_id: string;
	target_amount: string;
	currency_id: string;
};

export type FundRoundProposalExpanded = {
	proposal: Proposal;
	fund_round_proposal: FundRoundProposal;
	proposal_type: ProposalType;
};

export type FundRoundStatusResponse = {
	fund_round: FundRoundProposalExpanded;
	total_contributed: string;
	target_amount: string;
	is_completed: boolean;
};

export type FundRoundContribution = {
	fund_round_proposal_id: string;
	user_id: string;
	amount: string;
	transaction_id: string;
	created_at: string;
	updated_at: string;
};

export type CreateFundRoundData = {
	group_id: string;
	target_amount: string;
	currency_id: string;
};

export type ContributeFundRoundData = {
	amount: string;
	sender_wallet_id: string;
};

export type FundRoundRemainingResponse = {
	remaining: string;
	is_last_contributor: boolean;
};
