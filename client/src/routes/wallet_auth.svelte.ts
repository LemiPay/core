import { createAppKit } from '@reown/appkit';
import { type AppKitNetwork, sepolia } from '@reown/appkit/networks';
import { WagmiAdapter } from '@reown/appkit-adapter-wagmi';
import {
	connect,
	getAccount,
	getConnections,
	getConnectors,
	getWalletClient,
	reconnect,
	switchChain,
	writeContract,
	type Connector
} from '@wagmi/core';
import type { Abi } from 'viem';
import { writeContract as viemWriteContract } from 'viem/actions';

import { fallback, http } from 'wagmi';

const reown_project_id = import.meta.env.VITE_PUBLIC_REOWN_KEY || '';
const hasReownProjectId = reown_project_id.length > 0 && reown_project_id !== 'random key';

const appOrigin = typeof window !== 'undefined' ? window.location.origin : 'http://localhost:5173';

const networks: [AppKitNetwork, ...AppKitNetwork[]] = [sepolia];
const AUTH_CONNECTOR_ID = 'AUTH';

export type WagmiTxContext = {
	address: `0x${string}`;
	connector: Connector;
	chainId: number;
};

function isEmbeddedWalletSession(): boolean {
	return walletAuthState.isSocial || !!modal.getAccount()?.embeddedWalletInfo;
}

function getAuthConnector(): Connector | undefined {
	return getConnectors(wagmiAdapter.wagmiConfig).find(
		(c) => c.id === AUTH_CONNECTOR_ID || c.type === 'AUTH'
	);
}

export const wagmiAdapter = new WagmiAdapter({
	networks,
	projectId: reown_project_id,
	transports: {
		[sepolia.id]: fallback([
			http('https://sepolia.gateway.tenderly.co'),
			http('https://ethereum-sepolia.publicnode.com')
		])
	}
});

export const modal = createAppKit({
	adapters: [wagmiAdapter],
	networks,
	projectId: reown_project_id,
	defaultNetwork: sepolia,
	metadata: {
		name: 'Lemipay',
		description: 'Gestor de gastos Web3',
		url: appOrigin,
		icons: ['https://avatars.githubusercontent.com/u/37784886']
	},
	features: {
		email: true,
		socials: ['google']
	},
	// EOA evita problemas con approve/fund ERC20 vía wagmi; smartAccount usa flujos distintos.
	defaultAccountTypes: {
		eip155: 'eoa'
	}
});

export type WalletAccountType = 'eoa' | 'smartAccount' | undefined;

export const walletAuthState = $state({
	address: undefined as string | undefined,
	email: undefined as string | undefined | null,
	name: undefined as string | undefined,
	isSocial: false,
	isConnected: false,
	accountType: undefined as WalletAccountType
});

type WalletAuthListener = () => void;
const walletAuthListeners = new Set<WalletAuthListener>();

function notifyWalletAuth() {
	for (const listener of walletAuthListeners) {
		listener();
	}
}

export function onWalletAuthChange(listener: WalletAuthListener) {
	walletAuthListeners.add(listener);
	return () => {
		walletAuthListeners.delete(listener);
	};
}

const syncWallet = () => {
	const account = modal.getAccount();

	const userEmail = account?.embeddedWalletInfo?.user?.email;
	const userName = account?.embeddedWalletInfo?.user?.username;
	const address = account?.address;
	const isSocial = !!account?.embeddedWalletInfo;
	const wasSocial = walletAuthState.isSocial;
	const nextName = isSocial ? userName : !address || wasSocial ? undefined : walletAuthState.name;
	const nextConnected = !!address;
	const embeddedAccountType = account?.embeddedWalletInfo?.accountType;
	const nextAccountType: WalletAccountType =
		embeddedAccountType === 'smartAccount' || embeddedAccountType === 'eoa'
			? embeddedAccountType
			: isSocial
				? 'eoa'
				: undefined;

	if (
		walletAuthState.address === address &&
		walletAuthState.email === userEmail &&
		walletAuthState.name === nextName &&
		walletAuthState.isSocial === isSocial &&
		walletAuthState.isConnected === nextConnected &&
		walletAuthState.accountType === nextAccountType
	) {
		return;
	}

	walletAuthState.address = address;
	walletAuthState.email = userEmail;
	walletAuthState.isSocial = isSocial;
	walletAuthState.name = nextName;
	walletAuthState.isConnected = nextConnected;
	walletAuthState.accountType = nextAccountType;
	notifyWalletAuth();
};

// Suscripción a cambios
modal.subscribeAccount(() => {
	syncWallet();
});

let restoreInFlight: Promise<void> | null = null;

function getSepoliaWagmiChain() {
	return wagmiAdapter.wagmiChains?.find((chain) => chain.id === sepolia.id);
}

async function ensureSepoliaActiveNetwork(): Promise<void> {
	await modal.ready();
	try {
		await modal.switchNetwork(sepolia);
	} catch (err) {
		console.warn('No se pudo activar Sepolia en AppKit:', err);
	}
}

async function ensureWagmiConnectedForTx(): Promise<Connector | null> {
	const config = wagmiAdapter.wagmiConfig;
	let account = getAccount(config);

	if (
		isEmbeddedWalletSession() &&
		walletAuthState.address &&
		(account.status !== 'connected' || !account.connector)
	) {
		const authConnector = getAuthConnector();
		if (authConnector) {
			try {
				await reconnect(config, { connectors: [authConnector] });
			} catch (err) {
				console.warn('Reown AUTH reconnect failed:', err);
			}
		}
		account = getAccount(config);
	}

	if (account.status !== 'connected' || !account.connector) {
		if (getConnections(config).length === 0) {
			try {
				await reconnect(config);
			} catch (err) {
				console.warn('Wallet reconnect failed:', err);
			}
		}
		account = getAccount(config);
	}

	if (account.status !== 'connected' || !account.connector) {
		const authConnector = getAuthConnector();
		if (isEmbeddedWalletSession() && walletAuthState.address && authConnector) {
			try {
				await connect(config, {
					connector: authConnector,
					chainId: sepolia.id
				});
			} catch (err) {
				console.warn('Auth connector connect failed:', err);
			}
			account = getAccount(config);
		}
	}

	if (account.status === 'connected' && account.connector) {
		if (account.chainId !== sepolia.id) {
			try {
				await switchChain(config, { chainId: sepolia.id });
			} catch (err) {
				console.warn('Chain switch failed:', err);
			}
		}
		return account.connector;
	}

	return null;
}

async function getWagmiTxContext(): Promise<WagmiTxContext | null> {
	if (!hasReownProjectId) return null;

	await modal.ready();
	syncWallet();
	await ensureSepoliaActiveNetwork();

	const connector = await ensureWagmiConnectedForTx();
	const account = getAccount(wagmiAdapter.wagmiConfig);
	const address = (account.address ?? walletAuthState.address) as `0x${string}` | undefined;

	if (!address || !connector) return null;

	return { address, connector, chainId: sepolia.id };
}

async function restoreWalletSession(): Promise<void> {
	if (!hasReownProjectId) return;
	if (restoreInFlight) return restoreInFlight;

	restoreInFlight = (async () => {
		await modal.ready();
		await ensureWagmiConnectedForTx();
		syncWallet();
	})().finally(() => {
		restoreInFlight = null;
	});

	return restoreInFlight;
}

void modal.ready().then(() => syncWallet());

async function ensureWalletReadyForTx(): Promise<string | null> {
	if (!hasReownProjectId) {
		console.warn('VITE_PUBLIC_REOWN_KEY no configurada.');
		return null;
	}

	const ctx = await getWagmiTxContext();
	return ctx?.address ?? null;
}

export type WriteSepoliaContractParams = {
	address: `0x${string}`;
	abi: Abi | readonly unknown[];
	functionName: string;
	args: readonly unknown[];
};

async function writeSepoliaContract(params: WriteSepoliaContractParams): Promise<`0x${string}`> {
	await ensureSepoliaActiveNetwork();
	const connector = await ensureWagmiConnectedForTx();
	const account = getAccount(wagmiAdapter.wagmiConfig);
	const address = (account.address ?? walletAuthState.address) as `0x${string}` | undefined;

	if (!address || !connector) {
		throw new Error('WALLET_NOT_READY');
	}

	const chain = getSepoliaWagmiChain();
	if (!chain) {
		throw new Error('SEPOLIA_NOT_CONFIGURED');
	}

	if (isEmbeddedWalletSession()) {
		const authConnector = getAuthConnector();
		if (authConnector) {
			try {
				await switchChain(wagmiAdapter.wagmiConfig, {
					chainId: sepolia.id,
					connector: authConnector
				});
			} catch (err) {
				console.warn('Auth connector switchChain failed:', err);
			}
		}

		const walletClient = await getWalletClient(wagmiAdapter.wagmiConfig, {
			chainId: sepolia.id,
			connector,
			account: address
		});
		if (!walletClient) {
			throw new Error('WALLET_NOT_READY');
		}

		return viemWriteContract(walletClient, {
			address: params.address,
			abi: params.abi,
			functionName: params.functionName,
			args: params.args,
			account: address,
			chain
		} as Parameters<typeof viemWriteContract>[1]);
	}

	return writeContract(wagmiAdapter.wagmiConfig, {
		address: params.address,
		abi: params.abi,
		functionName: params.functionName,
		args: params.args,
		account: address,
		connector,
		chain,
		chainId: sepolia.id
	} as Parameters<typeof writeContract>[1]);
}

export const authActions = {
	logout: async () => {
		await modal.disconnect();
		// Limpiamos el estado manualmente para asegurar feedback instantáneo
		walletAuthState.address = undefined;
		walletAuthState.email = undefined;
		walletAuthState.name = undefined;
		walletAuthState.isSocial = false;
		walletAuthState.isConnected = false;
		walletAuthState.accountType = undefined;
		notifyWalletAuth();
	},
	openLogin: async () => {
		if (!hasReownProjectId) {
			console.warn('VITE_PUBLIC_REOWN_KEY no configurada.');
			return;
		}
		modal.setPreferredAccountType('eoa', 'eip155');
		await modal.open();
		await restoreWalletSession();
	},
	restoreWalletSession,
	ensureWalletReadyForTx,
	getWagmiTxContext,
	writeSepoliaContract,
	hasReownProjectId
};
