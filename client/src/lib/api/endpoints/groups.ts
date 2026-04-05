import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { Group, GroupSummary, NewGroupData } from '$lib/types/endpoints/groups.types';

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
export async function getGroup(group_id: string): ApiResponse<Group> {
	return authedApiFetch(`/group/${group_id}`, {
		method: 'GET'
	});
}
export default {
	createGroup,
	getMyGroups,
	getGroup
};
