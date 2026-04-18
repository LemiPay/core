import type { Proposal } from '$lib/types/endpoints/proposals.types';

export type WithdrawProposal = {
	proposal_id: string;
	amount: string;
	currency_id: string;
};

export type WithdrawProposalRequest = {
	currency_id: string;
	user_address: string;
	amount: string;
};

export type WithdrawProposalExpanded = {
	proposal: Proposal;
	withdraw_proposal: WithdrawProposal;
	proposal_type: string;
};
