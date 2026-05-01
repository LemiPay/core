<script lang="ts">
	import { ArrowLeft, ArrowDownToLine, Copy, Plus, Send, Wallet, Activity } from 'lucide-svelte';
	import type { User } from '$lib/types/endpoints/auth.types';
	import { me } from '$lib/api/auth';
	import { type FailedResponse, isSuccess, type SuccessResponse } from '$lib/types/client.types';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';
	import { getAllMyWallets } from '$lib/api/endpoints/user_wallet';
	import FaucetModal from '$lib/components/modals/user/FaucetModal.svelte';
	import CreateWalletModal from '$lib/components/modals/user/CreateWalletModal.svelte';
	import { shortenAddress, copyToClipboard } from '$lib/utils/address_utils';
	import { formatDate } from '$lib/utils/format_utils';
	import { listUserTransactions } from '$lib/api/endpoints/transactions';
	import type { Transaction } from '$lib/types/endpoints/transactions.types';

	// --- ESTADOS DE UI ---
	let activeTab = $state<'wallets' | 'activity'>('wallets');

	// --- ESTADOS ---
	let loadingUserInfo = $state(true);
	let errorInLoadingProfile = $state('');
	let user = $state({} as User);

	let loadingWalletsInfo = $state(true);
	let errorInLoadingWallets = $state('');
	let walletsArray = $state([] as WalletInfo[]);

	let loadingTransactions = $state(true);
	let errorInTransactions = $state('');
	let transactionsArray = $state([] as Transaction[]);

	let faucetTarget = $state<{ wallet_id: string; ticker: string } | null>(null);
	let transferTarget = $state<{ sender_wallet_id: string; ticker: string } | null>(null);
	let openFaucetModal = $state(false);
	let openTransferModal = $state(false);
	let openCreateWalletModal = $state(false);

	// --- CARGA DE DATOS ---
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

	async function loadTransactions() {
		loadingTransactions = true;
		let result = await listUserTransactions();
		if (!isSuccess(result)) {
			errorInTransactions = 'error al cargar historial de transacciones';
			loadingTransactions = false;
			return;
		}
		transactionsArray = result.body;
		loadingTransactions = false;
	}

	// --- UTILIDADES ---
	function goBack() {
		if (typeof history !== 'undefined' && history.length > 1) {
			history.back();
		} else {
			window.location.href = '/dashboard';
		}
	}

	function translateTxType(type: string) {
		const types: Record<string, string> = {
			deposit: 'Depósito',
			withdraw: 'Retiro',
			Fund: 'Fondeo'
		};
		return types[type] || type;
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

	<!-- HEADER ESTILO GRUPOS (Sin caja, texto grande) -->
	<div class="mb-4 flex flex-col gap-1">
		{#if loadingUserInfo}
			<div class="h-8 w-48 animate-pulse rounded bg-gray-200"></div>
			<div class="h-5 w-32 animate-pulse rounded bg-gray-100"></div>
		{:else}
			<h1 class="text-3xl font-extrabold text-black">{user.name}</h1>
			<p class="text-base text-gray-500">{user.email}</p>
		{/if}
	</div>

	<!-- PESTAÑAS DE NAVEGACIÓN (Estilo Grupos) -->
	<!-- border-y crea la línea arriba y abajo, gap-8 las separa como en la foto -->
	<div class="mt-2 mb-6 flex w-full gap-8 border-y border-gray-200">
		<!-- El -mb-px hace que el borde negro se superponga perfectamente al borde gris del contenedor -->
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

	<!-- CONTENIDO DINÁMICO -->
	<div class="w-full">
		{#if activeTab === 'wallets'}
			<!-- SECCIÓN BILLETERAS -->
			<section class="animate-in fade-in flex flex-col gap-6 duration-300">
				<div class="flex items-center justify-between">
					<h2 class="text-lg font-bold text-black">Tus Direcciones</h2>
					<button
						class="flex items-center gap-2 rounded-full border border-gray-200 px-4 py-2 text-sm font-medium text-black transition hover:border-black hover:bg-gray-50"
						onclick={() => (openCreateWalletModal = true)}
					>
						<Plus size={16} />
						Nueva Dirección
					</button>
				</div>

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
													(faucetTarget = {
														wallet_id: currency.wallet_id,
														ticker: currency.ticker
													})}
											>
												<ArrowDownToLine size={14} />
												Recibir
											</button>
										</div>
									</div>
								{/each}
							</div>
						</div>
					{/each}

					{#if walletsArray.length === 0 && !loadingWalletsInfo}
						<p class="py-8 text-center text-sm text-gray-500">Aún no tienes billeteras creadas.</p>
					{/if}
				</div>
			</section>
		{:else}
			<!-- SECCIÓN HISTORIAL DE TRANSACCIONES -->
			<section class="animate-in fade-in flex flex-col gap-6 duration-300">
				<div class="flex items-center justify-between">
					<h2 class="text-lg font-bold text-black">Actividad Reciente</h2>
				</div>

				<div class="flex flex-col gap-3">
					{#if loadingTransactions}
						<div class="flex justify-center py-8">
							<p class="text-sm text-gray-500">Cargando transacciones...</p>
						</div>
					{:else if transactionsArray.length === 0}
						<div class="flex justify-center py-8">
							<p class="text-sm text-gray-500">No hay transacciones recientes.</p>
						</div>
					{:else}
						{#each transactionsArray as tx}
							<div
								class="flex items-center justify-between rounded-xl border border-gray-200 bg-white p-4 transition hover:border-gray-300"
							>
								<div class="flex flex-col gap-0.5">
									<span class="font-bold text-black capitalize">{translateTxType(tx.tx_type)}</span>
									<span class="text-sm text-gray-500">
										{tx.description ? tx.description : ''}
									</span>
								</div>
								<div class="flex flex-col items-end gap-0.5">
									<span class="font-bold text-black">
										{tx.tx_type === 'withdraw' ? '+' : '-'} ${tx.amount}
									</span>
									<span class="text-sm text-gray-500">{formatDate(tx.created_at)}</span>
								</div>
							</div>
						{/each}
					{/if}
				</div>
			</section>
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
</div>
