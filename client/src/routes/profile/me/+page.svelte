<script lang="ts">
	import { ArrowLeft, ArrowDownToLine, Copy, Plus, Send, Wallet } from 'lucide-svelte';
	import type { User } from '$lib/types/endpoints/auth.types';
	import { me } from '$lib/api/auth';
	import { type FailedResponse, isSuccess, type SuccessResponse } from '$lib/types/client.types';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';
	import { getAllMyWallets } from '$lib/api/endpoints/user_wallet';
	import FaucetModal from '$lib/components/modals/FaucetModal.svelte';
	import TransferModal from '$lib/components/modals_old/modals/TransferModal.svelte';
	import CreateWalletModal from '$lib/components/modals_old/modals/CreateWalletModal.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';

	let loadingUserInfo = $state(true);
	let errorInLoadingProfile = $state('');
	let user = $state({} as User);
	let faucetTarget = $state<{ wallet_id: string; ticker: string } | null>(null);
	let transferTarget = $state<{ sender_wallet_id: string; ticker: string } | null>(null);
	let openFaucetModal = $state(false);
	let openTransferModal = $state(false);
	let openCreateWalletModal = $state(false);

	async function loadUserProfile() {
		let result: SuccessResponse<User> | FailedResponse = await me();
		if (!isSuccess(result)) {
			errorInLoadingProfile = "couldn't get user_id";
			loadingUserInfo = false;
			return;
		}
		loadingUserInfo = false;
		user = result.body;
	}

	let loadingWalletsInfo = $state(true);
	let errorInLoadingWallets = $state('');
	let walletsArray = $state([] as WalletInfo[]);

	async function loadWallets() {
		let result = await getAllMyWallets();
		if (!isSuccess(result)) {
			errorInLoadingWallets = 'failed to load wallets';
			loadingWalletsInfo = false;
			return;
		}
		loadingWalletsInfo = false;
		walletsArray = result.body;
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		// Acá podrías disparar un toast de "Copiado!"
	}

	function goBack() {
		if (typeof history !== 'undefined' && history.length > 1) {
			history.back();
		} else {
			window.location.href = '/dashboard';
		}
	}

	loadUserProfile();
	loadWallets();
</script>

<svelte:head>
	<title>Lemipay - Perfil de {user.name}</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 p-6 pt-8">
	<button
		onclick={goBack}
		class="flex w-fit items-center gap-2 rounded-full border border-gray-200 px-3 py-1.5 text-xs font-medium text-gray-600 transition hover:border-gray-400 hover:text-black"
	>
		<ArrowLeft class="h-3.5 w-3.5" />
		Volver
	</button>

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
		{#each walletsArray as group}
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
									onclick={() =>
										(faucetTarget = { wallet_id: currency.wallet_id, ticker: currency.ticker })}
								>
									<ArrowDownToLine size={14} />
									Recibir
								</button>

								<button
									class="flex items-center gap-1.5 rounded-full bg-black px-4 py-1.5 text-sm font-medium text-white transition hover:bg-gray-800"
									onclick={() =>
										(transferTarget = {
											sender_wallet_id: currency.wallet_id,
											ticker: currency.ticker
										})}
								>
									<Send size={14} />
									Enviar
								</button>
							</div>
						</div>
					{/each}
				</div>
				<!-- lo comento asi fabri no lo trata de clickear
				<div class="bg-white px-4 pt-1 pb-4">
					<button
						class="flex items-center gap-1 text-xs font-medium text-gray-400 transition hover:text-black"
					>
						<Plus size={12} /> Agregar token a esta dirección
					</button>
				</div>
				-->
			</div>
		{/each}

		{#if walletsArray.length === 0}
			<p class="text-sm text-gray-500">Aún no tienes billeteras creadas.</p>
		{/if}
		<FaucetModal
			open={faucetTarget !== null}
			wallet_id={faucetTarget?.wallet_id ?? ''}
			ticker={faucetTarget?.ticker ?? ''}
			onclose={() => (faucetTarget = null)}
			onsuccess={() => loadWallets()}
		/>

		<TransferModal
			open={transferTarget !== null}
			sender_wallet_id={transferTarget?.sender_wallet_id ?? ''}
			ticker={transferTarget?.ticker ?? ''}
			onclose={() => (transferTarget = null)}
			onsuccess={() => loadWallets()}
		/>
	</div>
</div>
