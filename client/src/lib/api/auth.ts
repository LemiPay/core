import { apiFetch, authedApiFetch } from './client';

import { type ApiResponse } from '$lib/types/client.types';
import type { LoginData, PostUser, RegisterData, User } from '$lib/types/endpoints/auth.types';

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

export type ChallengeResponse = {
	nonce: string;
	message: string;
	is_linked: boolean;
	issued_at: string;
};

export async function request_challenge(address: string): ApiResponse<ChallengeResponse> {
	return apiFetch('/auth/request-challenge', {
		method: 'POST',
		body: JSON.stringify({
			address
		})
	});
}

export async function verify_signature(
	email: string | null,
	name: string | null,
	address: string,
	nonce: string,
	signature: string,
	allow_linking: boolean = false,
	issued_at?: string | null
): ApiResponse<{ token: string; user_id: string }> {
	return apiFetch('/auth/verify-challenge', {
		method: 'POST',
		body: JSON.stringify({
			email,
			name,
			address,
			signature,
			nonce,
			allow_linking,
			issued_at: issued_at ?? null
		})
	});
}

export default {
	register,
	login,
	me,
	request_challenge,
	verify_signature,
	userInfo
};
