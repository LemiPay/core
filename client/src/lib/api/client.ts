import type { ApiResponse } from '$lib/types/auth.types';

const API_URL = 'http://localhost:3000';

export async function apiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	const res = await fetch(`${API_URL}${path}`, {
		// Include {credentials: 'include'} in options if Auth required
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
