import { createAppKit } from '@reown/appkit';
import { type AppKitNetwork, mainnet, sepolia } from '@reown/appkit/networks';
import { WagmiAdapter } from '@reown/appkit-adapter-wagmi';

import { http } from 'wagmi';

const reown_project_id = import.meta.env.VITE_PUBLIC_REOWN_KEY || 'random key';

const networks: [AppKitNetwork, ...AppKitNetwork[]] = [mainnet, sepolia];

export const wagmiAdapter = new WagmiAdapter({
	networks,
	projectId: reown_project_id,
	transports: {
		[mainnet.id]: http(),
		[sepolia.id]: http('https://sepolia.gateway.tenderly.co')
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
		url: 'http://localhost:5173',
		icons: ['https://avatars.githubusercontent.com/u/37784886']
	},
	features: {
		email: true,
		socials: ['google']
	}
});

export const walletAuthState = $state({
	address: undefined as string | undefined,
	email: undefined as string | undefined | null,
	name: undefined as string | undefined,
	isSocial: false,
	isConnected: false
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

	walletAuthState.address = address;
	walletAuthState.email = userEmail;
	walletAuthState.isSocial = isSocial;
	if (isSocial) {
		walletAuthState.name = userName;
	} else if (!address || wasSocial) {
		walletAuthState.name = undefined;
	}

	// Si hay address, para nosotros está conectado
	walletAuthState.isConnected = !!address;
	notifyWalletAuth();
};

// Suscripción a cambios
modal.subscribeAccount(() => {
	syncWallet();
});

// Check inicial
syncWallet();

export const authActions = {
	logout: async () => {
		await modal.disconnect();
		// Limpiamos el estado manualmente para asegurar feedback instantáneo
		walletAuthState.address = undefined;
		walletAuthState.email = undefined;
		walletAuthState.name = undefined;
		walletAuthState.isSocial = false;
		walletAuthState.isConnected = false;
		notifyWalletAuth();
	},
	openLogin: async () => {
		await modal.open();
		syncWallet();
	}
};
