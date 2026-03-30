import { apiFetch } from './client';

import type {
	RegisterData,
	LoginData,
	User,
	ApiResponse,
	Web3AuthPayload,
	Web3AuthResult
} from '$lib/types/auth.types';

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

export function me(): ApiResponse<{ id: string }> {
	return apiFetch('/auth/me', {
		method: 'GET',
		credentials: 'include'
	});
}

export function web3Auth(data: Web3AuthPayload, signal?: AbortSignal): ApiResponse<Web3AuthResult> {
	return apiFetch('/auth/web3', {
		method: 'POST',
		body: JSON.stringify(data),
		signal
	});
}
