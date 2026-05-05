<script lang="ts">
	import { ArrowLeft, Wallet, Activity } from 'lucide-svelte';
	import type { User } from '$lib/types/endpoints/auth.types';
	import { me } from '$lib/api/auth';
	import { type FailedResponse, isSuccess, type SuccessResponse } from '$lib/types/client.types';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';
	import { getAllMyWallets } from '$lib/api/endpoints/user_wallet';
	import { listUserTransactions } from '$lib/api/endpoints/transactions';
	import type { Transaction } from '$lib/types/endpoints/transactions.types';

	// Importar los nuevos Tabs
	import WalletsTab from './tabs/WalletsTab.svelte';
	import ActivityTab from './tabs/ActivityTab.svelte';

	// Importar Modales
	import FaucetModal from '$lib/components/modals/user/FaucetModal.svelte';
	import TransferModal from '$lib/components/modals/user/TransferModal.svelte'; // Asumo que lo tenés
	import CreateWalletModal from '$lib/components/modals/user/CreateWalletModal.svelte';

	// --- ESTADOS DE UI ---
	let activeTab = $state<'wallets' | 'activity'>('wallets');

	// --- ESTADOS DE DATOS ---
	let loadingUserInfo = $state(true);
	let user = $state({} as User);

	let loadingWalletsInfo = $state(true);
	let walletsArray = $state([] as WalletInfo[]);

	let loadingTransactions = $state(true);
	let transactionsArray = $state([] as Transaction[]);

	// --- ESTADOS DE MODALES ---
	let faucetTarget = $state<{ wallet_id: string; ticker: string } | null>(null);
	let transferTarget = $state<{ sender_wallet_id: string; ticker: string } | null>(null);
	let openCreateWalletModal = $state(false);

	// --- ESTADO DERIVADO ---
	let totalBalance = $derived(
		walletsArray.reduce((acc, group) => {
			const groupSum = group.currencies.reduce((sum, curr) => sum + Number(curr.balance || 0), 0);
			return acc + groupSum;
		}, 0)
	);

	// --- CARGA DE DATOS ---
	async function loadUserProfile() {
		let result: SuccessResponse<User> | FailedResponse = await me();
		if (isSuccess(result)) user = result.body;
		loadingUserInfo = false;
	}

	async function loadWallets() {
		let result = await getAllMyWallets();
		if (isSuccess(result)) walletsArray = result.body;
		loadingWalletsInfo = false;
	}

	async function loadTransactions() {
		loadingTransactions = true;
		let result = await listUserTransactions();
		if (isSuccess(result)) transactionsArray = result.body.reverse();
		loadingTransactions = false;
	}

	let hasInitializedTabEffect = $state(false);

	// --- REFETCH LOGIC (Como en group.page) ---
	$effect(() => {
		if (!hasInitializedTabEffect) {
			hasInitializedTabEffect = true;
			return;
		}

		if (activeTab === 'wallets') loadWallets();
		if (activeTab === 'activity') loadTransactions();
	});

	// --- UTILIDADES ---
	function goBack() {
		if (typeof history !== 'undefined' && history.length > 1) {
			history.back();
		} else {
			window.location.href = '/dashboard';
		}
	}

	// --- INIT ---
	loadUserProfile();
	loadWallets();
	loadTransactions();
</script>

<svelte:head>
	<title>Lemipay - Perfil de {user.name}</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-3xl flex-col gap-2 p-6 pt-8 pb-16">
	<!-- HEADER Y NAVEGACIÓN -->
	<button
		onclick={goBack}
		class="mb-4 flex w-fit items-center gap-2 rounded-full border border-gray-200 px-3 py-1.5 text-xs font-medium text-gray-600 transition hover:border-gray-400 hover:text-black"
	>
		<ArrowLeft class="h-3.5 w-3.5" />
		Volver
	</button>

	<!-- HEADER: Usuario / Saldo -->
	<div class="mb-4 flex items-start justify-between">
		<div class="flex flex-col gap-1">
			{#if loadingUserInfo}
				<div class="h-8 w-48 animate-pulse rounded bg-gray-200"></div>
				<div class="h-5 w-32 animate-pulse rounded bg-gray-100"></div>
			{:else}
				<h1 class="text-3xl font-extrabold text-black">{user.name}</h1>
				<p class="text-base text-gray-500">{user.email}</p>
			{/if}
		</div>

		<div class="flex flex-col items-end gap-1">
			<span class="text-xs font-semibold tracking-wider text-gray-500 uppercase">Saldo Total</span>
			<div class="flex items-center gap-3">
				{#if loadingWalletsInfo}
					<div class="h-8 w-28 animate-pulse rounded bg-gray-200"></div>
					<div
						class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
					></div>
				{:else}
					<span class="text-4xl font-extrabold tracking-tight text-black">
						${totalBalance.toLocaleString('en-US', {
							minimumFractionDigits: 2,
							maximumFractionDigits: 2
						})}
					</span>
				{/if}
			</div>
		</div>
	</div>

	<!-- PESTAÑAS -->
	<div class="mt-4 mb-6 flex w-full gap-8 border-y border-gray-200">
		<button
			class="-mb-px flex items-center gap-2 border-b-2 py-3 text-sm font-semibold transition-colors {activeTab ===
			'wallets'
				? 'border-black text-black'
				: 'border-transparent text-gray-500 hover:border-gray-300 hover:text-black'}"
			onclick={() => (activeTab = 'wallets')}
		>
			<Wallet size={16} />
			Billeteras
		</button>
		<button
			class="-mb-px flex items-center gap-2 border-b-2 py-3 text-sm font-semibold transition-colors {activeTab ===
			'activity'
				? 'border-black text-black'
				: 'border-transparent text-gray-500 hover:border-gray-300 hover:text-black'}"
			onclick={() => (activeTab = 'activity')}
		>
			<Activity size={16} />
			Actividad
		</button>
	</div>

	<!-- CONTENIDO DINÁMICO (TABS) -->
	<div class="w-full">
		{#if activeTab === 'wallets'}
			<WalletsTab
				{walletsArray}
				{loadingWalletsInfo}
				onCreateWallet={() => (openCreateWalletModal = true)}
				onReceive={(wallet_id, ticker) => (faucetTarget = { wallet_id, ticker })}
				onSend={(sender_wallet_id, ticker) => (transferTarget = { sender_wallet_id, ticker })}
			/>
		{:else}
			<ActivityTab {transactionsArray} {loadingTransactions} />
		{/if}
	</div>

	<!-- MODALES -->
	<CreateWalletModal
		open={openCreateWalletModal}
		onclose={() => (openCreateWalletModal = false)}
		onsuccess={() => loadWallets()}
	/>

	<FaucetModal
		open={faucetTarget !== null}
		wallet_id={faucetTarget?.wallet_id ?? ''}
		ticker={faucetTarget?.ticker ?? ''}
		onclose={() => (faucetTarget = null)}
		onsuccess={() => loadWallets()}
	/>

	{#if typeof TransferModal !== 'undefined'}
		<TransferModal
			open={transferTarget !== null}
			sender_wallet_id={transferTarget?.sender_wallet_id ?? ''}
			ticker={transferTarget?.ticker ?? ''}
			onclose={() => (transferTarget = null)}
			onsuccess={() => loadWallets()}
		/>
	{/if}
</div>
