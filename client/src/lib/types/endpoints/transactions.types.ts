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

export type ExecuteWithdrawProposal = {
	currency_id: string;
	proposal_id: string;
	address: string;
};

export type WithdrawProposalExpanded = {
	proposal: Proposal;
	withdraw_proposal: WithdrawProposal;
	proposal_type: string;
};

export type Transaction = {
	id: string;
	tx_hash: string | null;
	amount: string;
	user_id: string;
	group_id: string;
	currency_id: string;
	address: string;
	description: string | null;
	tx_type: string;
	created_at: string;
	updated_at: string;
};
