import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	NotificationEvent,
	NotificationChannel,
	UserNotificationPreference,
	GroupNotificationPreference,
	UpsertPreferenceRequest,
	NotificationRecord,
	GetNotificationsParams
} from '$lib/types/endpoints/notifications.types';

export async function getNotifications(
	params?: GetNotificationsParams
): ApiResponse<NotificationRecord[]> {
	const search = new URLSearchParams();
	if (params?.read !== undefined) search.set('read', String(params.read));
	if (params?.limit !== undefined) search.set('limit', String(params.limit));
	const query = search.toString();
	return authedApiFetch(`/notifications${query ? `?${query}` : ''}`, { method: 'GET' });
}

export async function markNotificationRead(id: string): ApiResponse<void> {
	return authedApiFetch(`/notifications/${id}/read`, {
		method: 'PATCH',
		body: JSON.stringify({})
	});
}

export async function markAllNotificationsRead(): ApiResponse<void> {
	return authedApiFetch('/notifications/read-all', {
		method: 'PATCH',
		body: JSON.stringify({})
	});
}

export async function getNotificationEvents(): ApiResponse<NotificationEvent[]> {
	return authedApiFetch('/notifications/events', { method: 'GET' });
}

export async function getNotificationChannels(): ApiResponse<NotificationChannel[]> {
	return authedApiFetch('/notifications/channels', { method: 'GET' });
}

export async function getUserPreferences(): ApiResponse<UserNotificationPreference[]> {
	return authedApiFetch('/notifications/preferences', { method: 'GET' });
}

export async function upsertUserPreference(
	data: UpsertPreferenceRequest
): ApiResponse<UserNotificationPreference> {
	return authedApiFetch('/notifications/preferences', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function getGroupPreferences(
	group_id: string
): ApiResponse<GroupNotificationPreference[]> {
	return authedApiFetch(`/group/${group_id}/notification-preferences`, { method: 'GET' });
}

export async function upsertGroupPreference(
	group_id: string,
	data: UpsertPreferenceRequest
): ApiResponse<GroupNotificationPreference> {
	return authedApiFetch(`/group/${group_id}/notification-preferences`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}
