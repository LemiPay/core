import { apiFetch, authedApiFetch } from './client';

import { type ApiResponse } from '$lib/types/client.types';
import type { RegisterData, LoginData, User, UserInfo } from '$lib/types/endpoints/auth.types';

export async function register(data: RegisterData): ApiResponse<User> {
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

export async function me(): ApiResponse<{ id: string }> {
	return authedApiFetch('/auth/me', {
		method: 'GET'
	});
}
export async function userInfo(id: string): ApiResponse<UserInfo> {
	return authedApiFetch(`/users/${id}`, {
		method: 'GET'
	});
}

export default {
	register,
	login,
	me,
	user_info: userInfo
};
