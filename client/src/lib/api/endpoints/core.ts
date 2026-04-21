import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { GroupBalancesResponse } from '$lib/types/endpoints/core.types';

export async function getGroupBalances(groupId: string): ApiResponse<GroupBalancesResponse> {
	return authedApiFetch(`/core/balances/${groupId}`, { method: 'GET' });
}
