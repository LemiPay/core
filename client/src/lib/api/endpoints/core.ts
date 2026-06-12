import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	GroupBalancesResponse,
	GetSettlementsResponse
} from '$lib/types/endpoints/core.types';

export async function getGroupBalances(groupId: string): ApiResponse<GroupBalancesResponse> {
	return authedApiFetch(`/core/balances/${groupId}`, { method: 'GET' });
}

export async function getGroupSettlements(groupId: string): ApiResponse<GetSettlementsResponse> {
	return authedApiFetch(`/core/get-settlements/${groupId}`, { method: 'GET' });
}
