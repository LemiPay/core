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
	return authedApiFetch(`/transaction/${group_id}/withdraw/proposal`, {
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
	return authedApiFetch(`/proposal/withdraw/${group_id}`, {
		method: 'GET'
	});
}

export async function executeWithdrawProposal(
	group_id: string,
	execute: ExecuteWithdrawProposal
): ApiResponse<Transaction> {
	return authedApiFetch(`/transaction/${group_id}/withdraw/execute`, {
		method: 'POST',
		body: JSON.stringify(execute)
	});
}

export async function listGroupTransactions(group_id: string): ApiResponse<Transaction[]> {
	return authedApiFetch(`/transaction/${group_id}/list`, { method: 'GET' });
}
export async function listUserTransactions(): ApiResponse<Transaction[]> {
	let transactions = [
		{
			id: 'tx_1a2b3c4d',
			tx_hash: '0x123abc456def7890123abc456def7890123abc45',
			amount: '120.00',
			user_id: 'usr_88ab99',
			group_id: 'grp_asado_finde',
			currency_id: '33de6c7c-62a2-4182-813a-9005183be70d',
			address: '0x71C7656EC7ab88b098defB751B7401B5f6d8976F',
			description: null,
			tx_type: 'deposit',
			created_at: '2026-04-28T14:32:00Z',
			updated_at: '2026-04-28T14:32:00Z'
		},
		{
			id: 'tx_8f9e0d1c',
			tx_hash: '0xabcdef1234567890abcdef1234567890abcdef12',
			amount: '50.50',
			user_id: 'usr_88ab99',
			group_id: 'grp_viaje_costa',
			currency_id: '33de6c7c-62a2-4182-813a-9005183be70d',
			address: '0x71C7656EC7ab88b098defB751B7401B5f6d8976F',
			description: 'Fondo común para el viaje',
			tx_type: 'deposit',
			created_at: '2026-04-30T10:15:00Z',
			updated_at: '2026-04-30T10:15:00Z'
		},
		{
			id: 'tx_4b5c6d7e',
			tx_hash: '0x9876543210fedcba9876543210fedcba98765432',
			amount: '25.00',
			user_id: 'usr_88ab99',
			group_id: 'grp_viaje_costa',
			currency_id: '33de6c7c-62a2-4182-813a-9005183be70d',
			address: '0x71C7656EC7ab88b098defB751B7401B5f6d8976F',
			description: 'Reintegro por pago de peajes',
			tx_type: 'withdraw',
			created_at: '2026-05-01T12:00:00Z',
			updated_at: '2026-05-01T12:00:00Z'
		}
	];

	return {
		body: transactions,
		message: 'success',
		ok: true,
		status: 200
	};
}
