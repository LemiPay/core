import { writable, get } from 'svelte/store';
import { me } from '$lib/api/auth';
import type { AuthState } from '$lib/types/stores/auth.types';
import { isSuccess } from '$lib/types/client.types';
import { token } from '$lib/stores/token';

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

		// --- 🚪 Logout ---
		logout() {
			localStorage.removeItem('token');
			token.set(null);

			set({
				token: null,
				isAuthenticated: false,
				user: null,
				loading: false
			});

			window.location.href = '/login';
		},

		// --- 👤 Traer usuario ---
		async fetchMe() {
			const response = await me();

			if (!isSuccess(response)) {
				console.error(response.message);
				this.logout();
				return;
			}

			const user = response.body;

			update((s) => ({
				...s,
				user,
				loading: false
			}));
		},

		// --- 📦 helpers ---
		getToken() {
			return get(this).token;
		},

		getUserId() {
			return get(this).user?.id;
		}
	};
}

export const authStore = createAuthStore();
