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
	amount: string;
	currency_id: string;
	kind: string;
	/**
	 * Backward compatibility for older API payloads.
	 * New governance DTOs return flattened fields.
	 */
	withdraw_proposal?: WithdrawProposal;
};

export type BlockchainEvent = {
	id: string;
	event_type: string;
	sender: string;
	wallet_address: string;
	token_address: string;
	currency_id: string;
	gross_amount: string;
	fee_amount: string;
	net_amount: string;
	tx_hash: string;
	block_number: number;
	created_at: string;
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
