import azure from 'svelte-adapter-azure-swa';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: azure({
			esbuildOptions: {
				external: [
					'@coinbase/wallet-sdk',
					'@metamask/connect-evm',
					'@metamask/sdk',
					'porto',
					'porto/internal',
					'@safe-global/safe-apps-sdk',
					'@safe-global/safe-apps-provider',
					'@walletconnect/ethereum-provider'
				]
			}
		})
	},
	vitePlugin: {
		dynamicCompileOptions: ({ filename }) =>
			filename.includes('node_modules') ? undefined : { runes: true }
	}
};

export default config;
