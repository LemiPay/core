import { apiFetch, authedApiFetch } from './client';

import { type ApiResponse } from '$lib/types/client.types';
import type { RegisterData, LoginData, PostUser, User } from '$lib/types/endpoints/auth.types';

export async function register(data: RegisterData): ApiResponse<PostUser> {
	return apiFetch('/auth/register', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export function login(data: LoginData): ApiResponse<{ token: string }> {
	return apiFetch('/auth/login', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function me(): ApiResponse<User> {
	return authedApiFetch('/auth/me', {
		method: 'GET'
	});
}
export async function userInfo(id: string): ApiResponse<User> {
	return authedApiFetch(`/users/${id}`, {
		method: 'GET'
	});
}

export default {
	register,
	login,
	me,
	userInfo
};
