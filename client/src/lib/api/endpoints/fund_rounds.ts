import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	ContributeFundRoundData,
	CreateFundRoundData,
	FundRoundContribution,
	FundRoundProposalExpanded,
	FundRoundRemainingResponse,
	FundRoundStatusResponse
} from '$lib/types/endpoints/fund_rounds.types';

export async function createFundRoundProposal(
	data: CreateFundRoundData
): ApiResponse<FundRoundProposalExpanded> {
	return authedApiFetch(`/group-wallet/fund-round/create/${data.group_id}`, {
		method: 'POST',
		body: JSON.stringify({
			target_amount: data.target_amount,
			currency_id: data.currency_id
		})
	});
}

export async function contributeFundRound(
	fund_round_id: string,
	data: ContributeFundRoundData
): ApiResponse<FundRoundStatusResponse> {
	return authedApiFetch(`/group-wallet/fund-round/${fund_round_id}/contribute`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function getMyFundRoundContribution(
	fund_round_id: string
): ApiResponse<FundRoundContribution> {
	return authedApiFetch(`/group-wallet/fund-round/${fund_round_id}/contribute`, {
		method: 'GET'
	});
}

export async function getGroupFundRoundProposals(
	group_id: string
): ApiResponse<FundRoundProposalExpanded[]> {
	return authedApiFetch(`/group-wallet/fund-round/${group_id}/get-all`, {
		method: 'GET'
	});
}

export async function getFundRoundProposal(
	fund_round_id: string
): ApiResponse<FundRoundStatusResponse> {
	return authedApiFetch(`/group-wallet/fund-round/${fund_round_id}`, {
		method: 'GET'
	});
}

export async function cancelFundRoundProposal(
	fund_round_id: string
): ApiResponse<FundRoundProposalExpanded> {
	return authedApiFetch(`/group-wallet/fund-round/${fund_round_id}/cancel`, {
		method: 'DELETE'
	});
}

export async function getFundRoundRemaining(
	fund_round_id: string
): ApiResponse<FundRoundRemainingResponse> {
	return authedApiFetch(`/group-wallet/fund-round/${fund_round_id}/remaining`, {
		method: 'GET'
	});
}
