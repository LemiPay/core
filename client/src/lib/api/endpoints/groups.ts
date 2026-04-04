import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { GroupSummary, NewGroupData } from '$lib/types/endpoints/groups.types';
import { login, me, register, user_info } from '$lib/api/auth';

export async function createGroup(data: NewGroupData): ApiResponse<{ id: string }> {
	return authedApiFetch('/group/create', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function getMyGroups(): ApiResponse<GroupSummary[]> {
	return authedApiFetch('/group/my-groups', {
		method: 'GET'
	});
}

export default {
	createGroup,
	getMyGroups
};
