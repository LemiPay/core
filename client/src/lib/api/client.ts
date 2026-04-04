import { getToken } from '$lib/stores/store';
import type { ApiResponse } from '$lib/types/client.types';

const API_URL = 'http://localhost:3000';

export async function apiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	const res = await fetch(`${API_URL}${path}`, {
		headers: {
			'Content-Type': 'application/json',
			...options.headers
		},
		...options
	});

	return {
		status: res.status,
		message: res.statusText,
		body: (await res.json()) as T
	};
}

export async function authedApiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	const token = getToken();

	const res = await fetch(`${API_URL}${path}`, {
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`,
			...options.headers
		},
		...options
	});

	return {
		status: res.status,
		message: res.statusText,
		body: (await res.json()) as T
	};
}
