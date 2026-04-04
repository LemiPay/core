import { apiFetch, authedApiFetch } from './client';

import type { ApiResponse } from '$lib/types/client.types';
import type { RegisterData, LoginData, User } from '$lib/types/auth.types';

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
	return await authedApiFetch('/auth/me', {
		method: 'GET'
	});
}
