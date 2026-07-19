import { writable, get } from 'svelte/store';
import { me } from '$lib/api/auth';
import type { AuthState } from '$lib/types/stores/auth.types';
import { isSuccess } from '$lib/types/client.types';
import { token } from '$lib/stores/token';
import { authActions } from '../../routes/wallet_auth.svelte';

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		token: null,
		isAuthenticated: false,
		user: null,
		loading: false
	});

	return {
		subscribe,

		// --- 🔄 Inicializar desde localStorage ---
		async init() {
			const storedToken = localStorage.getItem('token');

			if (!storedToken) {
				token.set(null);
				set({
					token: null,
					isAuthenticated: false,
					user: null,
					loading: false
				});
				return;
			}

			token.set(storedToken);
			set({
				token: storedToken,
				isAuthenticated: true,
				user: null,
				loading: true
			});

			await this.fetchMe();
		},

		// --- 🔐 Login ---
		async login(newToken: string) {
			localStorage.setItem('token', newToken);

			token.set(newToken);
			set({
				token: newToken,
				isAuthenticated: true,
				user: null,
				loading: true
			});

			await this.fetchMe();
		},

		// --- Limpiar sesión local sin redirigir (API caída, 401, etc.) ---
		clearSession() {
			localStorage.removeItem('token');
			token.set(null);

			set({
				token: null,
				isAuthenticated: false,
				user: null,
				loading: false
			});
		},

		// --- 🚪 Logout explícito del usuario ---
		async logout() {
			this.clearSession();
			try {
				await authActions.logout();
			} catch (error) {
				console.error('Wallet logout failed:', error);
			}

			window.location.href = '/login';
		},

		// --- 👤 Traer usuario ---
		async fetchMe() {
			try {
				const response = await me();

				if (!isSuccess(response)) {
					// Solo invalidar sesión si el token es rechazado.
					// Si la API está caída (status 0) u otro error de servidor,
					// no redirigimos ni rompemos rutas públicas.
					if (response.status === 401) {
						this.clearSession();
						return;
					}

					console.error('fetchMe failed:', response.message);
					update((s) => ({
						...s,
						user: null,
						loading: false
					}));
					return;
				}

				const user = response.body;

				update((s) => ({
					...s,
					user,
					loading: false
				}));
			} catch (error) {
				console.error('fetchMe error:', error);
				update((s) => ({
					...s,
					user: null,
					loading: false
				}));
			}
		},

		// --- 📦 helpers ---
		getToken() {
			return get(this).token;
		},

		getIsAuthed() {
			return get(this).isAuthenticated;
		},

		getUserId() {
			return get(this).user?.id;
		}
	};
}

export const authStore = createAuthStore();
