import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';

export interface PermissionEntry {
	action: string;
	description: string;
	category: string;
}

export interface RolePermissions {
	role: string;
	permissions: PermissionEntry[];
}

export interface GroupPermissionsResponse {
	group_id: string;
	roles: RolePermissions[];
}

export async function getGroupPermissions(groupId: string): ApiResponse<GroupPermissionsResponse> {
	return authedApiFetch(`/permission/${groupId}`, { method: 'GET' });
}

export async function addGroupPermission(
	groupId: string,
	action: string,
	role: string = 'Member'
): ApiResponse<void> {
	return authedApiFetch(`/permission/${groupId}`, {
		method: 'POST',
		body: JSON.stringify({ action, role })
	});
}

export async function removeGroupPermission(
	groupId: string,
	action: string,
	role: string = 'Member'
): ApiResponse<void> {
	const params = new URLSearchParams({ action, role });
	return authedApiFetch(`/permission/${groupId}?${params}`, { method: 'DELETE' });
}
