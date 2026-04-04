import { authStore } from '$lib/stores/auth';
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

	let data: unknown = null;

	try {
		data = await res.json();
	} catch {
		// (ej: 204)
	}

	if (res.ok) {
		return {
			ok: true,
			status: 200,
			message: 'Success',
			body: data as T
		};
	}

	return {
		ok: false,
		status: res.status,
		message: res.statusText,
		body: data
	};
}

export async function authedApiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	const token = authStore.getToken();

	const res = await fetch(`${API_URL}${path}`, {
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`,
			...options.headers
		},
		...options
	});

	let data: unknown = null;

	try {
		data = await res.json();
	} catch {
		// (ej: 204)
	}

	if (res.ok) {
		return {
			ok: true,
			status: 200,
			message: 'Success',
			body: data as T
		};
	}

	return {
		ok: false,
		status: res.status,
		message: res.statusText,
		body: data
	};
}
