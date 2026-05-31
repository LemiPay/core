import { authedApiFetch } from '../client';
import type { ApiResponse } from '$lib/types/client.types';
import type {
	CreateInvestmentProposalData,
	ExecuteInvestmentData,
	Investment,
	InvestmentProposal,
	InvestmentStrategy,
	Snapshot,
	WithdrawInvestmentData
} from '$lib/types/endpoints/investments.types';

export async function listStrategies(): ApiResponse<InvestmentStrategy[]> {
	return authedApiFetch('/investment/strategies', { method: 'GET' });
}

export async function createInvestmentProposal(
	group_id: string,
	data: CreateInvestmentProposalData
): ApiResponse<InvestmentProposal> {
	return authedApiFetch(`/investment/proposal/${group_id}`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function executeInvestmentProposal(
	group_id: string,
	data: ExecuteInvestmentData
): ApiResponse<Investment> {
	return authedApiFetch(`/investment/execute/${group_id}`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function withdrawInvestment(
	group_id: string,
	data: WithdrawInvestmentData
): ApiResponse<Investment> {
	return authedApiFetch(`/investment/withdraw/${group_id}`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function listGroupInvestments(group_id: string): ApiResponse<Investment[]> {
	return authedApiFetch(`/investment/${group_id}`, { method: 'GET' });
}

export async function listInvestmentSnapshots(investment_id: string): ApiResponse<Snapshot[]> {
	return authedApiFetch(`/investment/${investment_id}/snapshots`, { method: 'GET' });
}
