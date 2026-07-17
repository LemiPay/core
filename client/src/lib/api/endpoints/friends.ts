import { authedApiFetch } from '$lib/api/client';
import type { ApiResponse } from '$lib/types/client.types';
import type { FriendResponse, UserSearchResult } from '$lib/types/endpoints/friends.types';

export async function sendFriendRequest(userId: string): ApiResponse<FriendResponse> {
	return authedApiFetch(`/friend/request/${userId}`, {
		method: 'POST'
	});
}

export async function respondToFriendRequest(
	userId: string,
	action: string
): ApiResponse<FriendResponse> {
	return authedApiFetch(`/friend/respond/${userId}`, {
		method: 'POST',
		body: JSON.stringify({ action })
	});
}

export async function getFriends(): ApiResponse<FriendResponse[]> {
	return authedApiFetch('/friend/list');
}

export async function getReceivedRequests(): ApiResponse<FriendResponse[]> {
	return authedApiFetch('/friend/requests/received');
}

export async function getSentRequests(): ApiResponse<FriendResponse[]> {
	return authedApiFetch('/friend/requests/sent');
}

export async function unfriend(userId: string): ApiResponse<unknown> {
	return authedApiFetch(`/friend/${userId}`, {
		method: 'DELETE'
	});
}

export async function blockUser(userId: string): ApiResponse<unknown> {
	return authedApiFetch(`/friend/block/${userId}`, {
		method: 'POST'
	});
}

export async function searchUsers(query: string): ApiResponse<UserSearchResult[]> {
	return authedApiFetch(`/friend/search?q=${encodeURIComponent(query)}`);
}
