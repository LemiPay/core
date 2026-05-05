import { authedApiFetch } from '../client';

import type { ApiResponse, SuccessResponse } from '$lib/types/client.types';
import type {
	ExecuteWithdrawProposal,
	Transaction,
	WithdrawProposalExpanded,
	WithdrawProposalRequest
} from '$lib/types/endpoints/transactions.types';

export async function proposeWithdraw(
	request: WithdrawProposalRequest,
	group_id: string
): ApiResponse<WithdrawProposalExpanded> {
	return authedApiFetch(`/governance/${group_id}/withdraw/proposal`, {
		method: 'POST',
		body: JSON.stringify({
			currency_id: request.currency_id,
			address: request.user_address,
			amount: request.amount
		})
	});
}

export async function getAllWithdrawProposals(
	group_id: string
): ApiResponse<WithdrawProposalExpanded[]> {
	return authedApiFetch(`/governance/withdraw/${group_id}`, {
		method: 'GET'
	});
}

export async function executeWithdrawProposal(
	group_id: string,
	execute: ExecuteWithdrawProposal
): ApiResponse<Transaction> {
	return authedApiFetch(`/governance/${group_id}/withdraw/execute`, {
		method: 'POST',
		body: JSON.stringify(execute)
	});
}

export async function listGroupTransactions(group_id: string): ApiResponse<Transaction[]> {
	return authedApiFetch(`/transaction/${group_id}/list`, { method: 'GET' });
}
export async function listUserTransactions(): ApiResponse<Transaction[]> {
	return await authedApiFetch<Transaction[]>(`/transaction/me`, {
		method: 'GET'
	});
}
