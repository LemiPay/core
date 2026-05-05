import { apiFetch, authedApiFetch } from './client';

import { type ApiResponse } from '$lib/types/client.types';
import type { RegisterData, LoginData, PostUser, User } from '$lib/types/endpoints/auth.types';

export async function register(data: RegisterData): ApiResponse<PostUser> {
	return apiFetch('/auth/register', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export function login(data: LoginData): ApiResponse<{ token: string; user_id: string }> {
	return apiFetch('/auth/login', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function me(): ApiResponse<User> {
	return authedApiFetch('/user/me', {
		method: 'GET'
	});
}
export async function userInfo(id: string): ApiResponse<User> {
	return authedApiFetch(`/user/id/${id}`, {
		method: 'GET'
	});
}

export async function request_challenge(
	email: any,
	address: any
): ApiResponse<{ nonce: string; message: string }> {
	return authedApiFetch('/auth/request-challenge', {
		method: 'POST',
		body: JSON.stringify({
			email,
			address
		})
	});
}

export async function verify_signature(
	email: any,
	address: any,
	nonce: string,
	signature: string
): ApiResponse<boolean> {
	return {
		status: 200,
		ok: true,
		body: true,
		message: 'mock'
	};
}

export default {
	register,
	login,
	me,
	request_challenge,
	userInfo
};
