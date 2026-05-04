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

export async function wallet_login_mock(
	email: any,
	wallet: any
): ApiResponse<{ token: string; user_id: string }> {
	let res = {
		token:
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZDJlOTBhMS1kYjVkLTQ4NjUtYTNlMy04MjRiMWE3NGQwYjQiLCJleHAiOjE3NzgzMzY3MDF9.eEFmS31n_EnU-SiJa4n8pjnam4aQpbBm-stQm02F2m4',
		user_id: '3d2e90a1-db5d-4865-a3e3-824b1a74d0b4'
	};
	return {
		ok: true,
		status: 200,
		body: res,
		message: 'no message, mock function'
	};
}

export default {
	register,
	login,
	me,
	wallet_login_mock,
	userInfo
};
