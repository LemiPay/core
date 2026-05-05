import { createAppKit } from '@reown/appkit';
import { type AppKitNetwork, mainnet, sepolia } from '@reown/appkit/networks';
import { WagmiAdapter } from '@reown/appkit-adapter-wagmi';
import { http } from 'wagmi';

const reown_project_id = '85e92565831777780ce074f6065bdc7f';

const networks: [AppKitNetwork, ...AppKitNetwork[]] = [mainnet, sepolia];

const wagmiAdapter = new WagmiAdapter({
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
	isConnected: false
});

const syncWallet = () => {
	const account = modal.getAccount();

	const userEmail = account?.embeddedWalletInfo?.user?.email;
	const address = account?.address;

	walletAuthState.address = address;
	walletAuthState.email = userEmail;

	// Si hay address, para nosotros está conectado
	walletAuthState.isConnected = !!address;

	console.log('Sincronizado:', {
		email: walletAuthState.email,
		address: walletAuthState.address
	});
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
		walletAuthState.isConnected = false;
	},
	openLogin: async () => {
		await modal.open();
	}
};
