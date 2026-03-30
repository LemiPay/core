import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { createAppKit } from '@reown/appkit';
import { arbitrum, mainnet, sepolia } from '@reown/appkit/networks';
import { WagmiAdapter } from '@reown/appkit-adapter-wagmi';
import { getConnection, watchConnection } from '@wagmi/core';
import type { Config } from '@wagmi/core';
import type { Address } from 'viem';

let appKit: ReturnType<typeof createAppKit> | undefined = undefined;

/** Wagmi config from the adapter; use with `signMessage` / other `@wagmi/core` actions. SSR: undefined. */
export let wagmiConfig: Config | undefined = undefined;

/** EVM address from Wagmi; updates when the user connects, disconnects, or switches account. */
export const walletAddress = writable<Address | undefined>(undefined);

if (browser) {
	const projectId = import.meta.env.VITE_PROJECT_ID;
	if (!projectId) {
		throw new Error('VITE_PROJECT_ID is not set');
	}

	const networks = [sepolia, arbitrum, mainnet];

	// Create adapter
	const wagmiAdapter = new WagmiAdapter({
		networks,
		projectId
	});

	wagmiConfig = wagmiAdapter.wagmiConfig;

	function syncWalletAddress() {
		const c = getConnection(wagmiAdapter.wagmiConfig);
		walletAddress.set(c.status === 'connected' ? c.address : undefined);
	}

	syncWalletAddress();
	watchConnection(wagmiAdapter.wagmiConfig, {
		onChange() {
			syncWalletAddress();
		}
	});

	// Initialize AppKit
	appKit = createAppKit({
		adapters: [wagmiAdapter],
		networks: [arbitrum, mainnet, sepolia],
		defaultNetwork: sepolia,
		projectId,
		metadata: {
			name: 'LemiPay',
			description: 'LemiPay Auth',
			url: 'http://localhost:5174',
			icons: ['https://avatars.githubusercontent.com/u/179229932?s=200&v=4']
		},
		features: {
			analytics: true,
			connectMethodsOrder: ['social', 'email', 'wallet']
		}
	});
}

export const appKitStore = writable(appKit);
