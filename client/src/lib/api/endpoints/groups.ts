import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	FundGroupWalletData,
	Group,
	GroupSummary,
	GroupWallet,
	NewGroupData,
	NewGroupWalletData,
	UpdateGroupData
} from '$lib/types/endpoints/groups.types';
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

export async function updateGroup(group_id: string, data: UpdateGroupData): ApiResponse<Group> {
	return authedApiFetch(`/group/${group_id}`, {
		method: 'PUT',
		body: JSON.stringify(data)
	});
}

export async function deleteGroup(group_id: string): ApiResponse<Group> {
	return authedApiFetch(`/group/${group_id}`, {
		method: 'DELETE'
	});
}

export async function createGroupWallet(
	group_id: string,
	data: NewGroupWalletData
): ApiResponse<GroupWallet> {
	return authedApiFetch(`/group-wallet/${group_id}/create`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}
export async function getGroupWallets(group_id: string): ApiResponse<GroupWallet[]> {
	return authedApiFetch(`group-wallet/${group_id}`, {
		method: 'GET'
	});
}

export async function fundGroupWallet(
	group_id: string,
	data: FundGroupWalletData
): ApiResponse<boolean> {
	return authedApiFetch(`/group/${group_id}/fund`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}
