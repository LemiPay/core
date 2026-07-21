import { PUBLIC_API_URL } from '$env/static/public';
import { token } from '$lib/stores/token';
import type { ApiResponse } from '$lib/types/client.types';

/**
 * Backend base URL from PUBLIC_API_URL.
 *
 * Uses `$env/static/public` so the value is inlined into the client bundle at
 * build/dev time. `$env/dynamic/public` only works when SSR injects env into
 * `globalThis.__sveltekit_*.env`; with vite preview / Azure SWA that object is
 * often empty on the client, and the old code silently fell back to localhost.
 */
export function getApiUrl(): string {
	const fromEnv = PUBLIC_API_URL?.trim();
	if (!fromEnv) {
		throw new Error(
			'PUBLIC_API_URL is not set. Define it in the repo-root .env or client/.env (e.g. PUBLIC_API_URL=http://localhost:3000).'
		);
	}
	// Normalize trailing slash so paths like `/health` always join cleanly.
	return fromEnv.replace(/\/+$/, '');
}

function networkErrorResponse(error: unknown) {
	const message =
		error instanceof Error && error.message ? error.message : 'No se pudo conectar con la API';

	return {
		ok: false as const,
		status: 0,
		message,
		body: null
	};
}

export async function apiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	try {
		const res = await fetch(`${getApiUrl()}${path}`, {
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
	} catch (error) {
		return networkErrorResponse(error);
	}
}

export async function authedApiFetch<T>(path: string, options: RequestInit = {}): ApiResponse<T> {
	try {
		const res = await fetch(`${getApiUrl()}${path}`, {
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${token.get()}`,
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

		let errorMessage = res.statusText;

		if (data && typeof data === 'object' && 'message' in data) {
			errorMessage = String((data as Record<string, unknown>).message);
		}

		return {
			ok: false,
			status: res.status,
			message: errorMessage,
			body: data
		};
	} catch (error) {
		return networkErrorResponse(error);
	}
}
