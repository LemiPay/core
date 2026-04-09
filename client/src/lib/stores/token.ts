import { writable, get } from 'svelte/store';

const tokenStore = writable<string | null>(null);

export const token = {
	subscribe: tokenStore.subscribe,
	set: (value: string | null) => tokenStore.set(value),
	get: () => get(tokenStore)
};
