<script lang="ts">
	import { Copy, Plus, Send, ArrowDownToLine, Wallet } from 'lucide-svelte';
	import FAB from '$lib/components/ui/FAB.svelte';
	import type { User } from '$lib/types/endpoints/auth.types';
	import { me } from '$lib/api/auth';
	import { type FailedResponse, isSuccess, type SuccessResponse } from '$lib/types/client.types';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';
	import { getAllMyWallets } from '$lib/api/endpoints/user_wallet';
	import FaucetModal from '$lib/components/modals/FaucetModal.svelte';
	import TransferModal from '$lib/components/modals/TransferModal.svelte';
	import CreateWalletModal from '$lib/components/modals/CreateWalletModal.svelte';

	let loadingUserInfo = $state(true);
	let error_in_loading_profile = $state('');
	let user = $state({} as User);
	let openFaucetModal = $state(false);
	let openTransferModal = $state(false);
	let openCreateWalletModal = $state(false);

	async function loadUserProfile() {
		let result: SuccessResponse<User> | FailedResponse = await me();
		if (!isSuccess(result)) {
			error_in_loading_profile = "couldn't get user_id";
			loadingUserInfo = false;
			return;
		}
		loadingUserInfo = false;
		user = result.body;
	}

	let loadingWalletsInfo = $state(true);
	let error_in_loading_wallets = $state('');
	let wallets_array = $state([] as WalletInfo[]);

	async function loadWallets() {
		let result = await getAllMyWallets();
		if (!isSuccess(result)) {
			error_in_loading_wallets = 'failed to load wallets';
			loadingWalletsInfo = false;
			return;
		}
		loadingWalletsInfo = false;
		wallets_array = result.body;
	}

	// Función auxiliar para acortar la address visualmente (estilo 0x123...abc)
	function shortenAddress(address: string) {
		return `${address.slice(0, 6)}...${address.slice(-4)}`;
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		// Acá podrías disparar un toast de "Copiado!"
	}

	loadUserProfile();
	loadWallets();
</script>

<svelte:head>
	<title>Lemipay - Perfil de {user.name}</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 p-6 pt-8">
	<div class="flex items-center gap-4 rounded-xl border border-gray-200 bg-white p-6">
		<div class="flex flex-col">
			<h1 class="text-2xl font-bold text-black">{user.name}</h1>
			<p class="text-sm text-gray-500">{user.email}</p>
		</div>
	</div>

	<div class="flex items-center justify-between">
		<h2 class="text-xl font-bold text-black">Mis Billeteras</h2>
		<button
			class="flex items-center gap-2 rounded-full border border-gray-200 px-4 py-2 text-sm font-medium text-black transition hover:border-black hover:bg-gray-50"
			onclick={() => (openCreateWalletModal = true)}
		>
			<Plus size={16} />
			Nueva Dirección
		</button>
	</div>

	<CreateWalletModal
		open={openCreateWalletModal}
		onclose={() => (openCreateWalletModal = false)}
		onsuccess={() => loadWallets()}
	/>

	<div class="flex w-full flex-col gap-6">
		{#each wallets_array as group}
			<div class="flex flex-col overflow-hidden rounded-xl border border-gray-200 bg-white">
				<div
					class="flex items-center justify-between border-b border-gray-100 bg-gray-50 px-4 py-3"
				>
					<div class="flex items-center gap-2 text-gray-500">
						<Wallet size={16} />
						<span class="font-mono text-sm">{shortenAddress(group.address)}</span>
					</div>
					<button
						onclick={() => copyToClipboard(group.address)}
						class="flex items-center gap-1.5 rounded-md px-2 py-1 text-xs font-medium text-gray-500 transition hover:bg-gray-200 hover:text-black"
					>
						<Copy size={14} />
						Copiar
					</button>
				</div>

				<div class="flex flex-col px-4 py-2">
					{#each group.currencies as currency}
						<div
							class="flex items-center justify-between border-b border-gray-50 py-3 last:border-0"
						>
							<div class="flex flex-col">
								<span class="text-2xl font-bold text-black">
									{currency.balance}
									<span class="text-base font-medium text-gray-500">{currency.ticker}</span>
								</span>
							</div>

							<div class="flex gap-2">
								<button
									class="flex items-center gap-1.5 rounded-full border border-gray-200 px-4 py-1.5 text-sm font-medium text-black transition hover:border-gray-400 hover:bg-gray-50"
									onclick={() => (openFaucetModal = true)}
								>
									<ArrowDownToLine size={14} />
									Recibir
								</button>
								<FaucetModal
									open={openFaucetModal}
									wallet_id={currency.wallet_id}
									ticker={currency.ticker}
									onclose={() => (openFaucetModal = false)}
									onsuccess={() => loadWallets()}
								/>

								<button
									class="flex items-center gap-1.5 rounded-full bg-black px-4 py-1.5 text-sm font-medium text-white transition hover:bg-gray-800"
									onclick={() => (openTransferModal = true)}
								>
									<Send size={14} />
									Enviar
								</button>

								<TransferModal
									open={openTransferModal}
									sender_wallet_id={currency.wallet_id}
									ticker={currency.ticker}
									onclose={() => (openTransferModal = false)}
									onsuccess={() => loadWallets()}
								/>
							</div>
						</div>
					{/each}
				</div>

				<div class="bg-white px-4 pt-1 pb-4">
					<button
						class="flex items-center gap-1 text-xs font-medium text-gray-400 transition hover:text-black"
					>
						<Plus size={12} /> Agregar token a esta dirección
					</button>
				</div>
			</div>
		{/each}

		{#if wallets_array.length === 0}
			<p class="text-sm text-gray-500">Aún no tienes billeteras creadas.</p>
		{/if}
	</div>

	<div class="md:hidden">
		<FAB ariaLabel="Crear Wallet" onclick={() => console.log('Nueva Wallet')}>
			{#snippet icon()}
				<Plus />
			{/snippet}
		</FAB>
	</div>
</div>
