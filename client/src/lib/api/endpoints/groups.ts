import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { Group, GroupSummary, NewGroupData } from '$lib/types/endpoints/groups.types';
import type { UserBadge } from '$lib/types/endpoints/auth.types';

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
export async function getGroupMembers(group_id: string): ApiResponse<UserBadge[]> {
	return authedApiFetch(`/group/${group_id}/members`, {
		method: 'GET'
	});
}
