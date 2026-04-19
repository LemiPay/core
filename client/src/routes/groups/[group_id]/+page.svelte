<script lang="ts">
	import {
		Trash2,
		Pencil,
		Wallet,
		Coins,
		Plus,
		Copy,
		HandCoins,
		LogOut,
		ChevronDown
	} from 'lucide-svelte';
	import { slide } from 'svelte/transition';
	import { page } from '$app/state';

	// Api
	import {
		getGroup,
		getGroupMembers,
		updateGroup,
		deleteGroup,
		getGroupWallets,
		leaveGroup
	} from '$lib/api/endpoints/groups';

	// Helpers
	import { isSuccess } from '$lib/types/client.types';

	// Types
	import type { Group } from '$lib/types/endpoints/groups.types';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';
	import type { GroupWallet } from '$lib/types/endpoints/groups.types';

	// Components
	import UserIconBadge from '$lib/components/UserIconBadge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import InviteUserToGroup from '$lib/components/modals/InviteUserToGroup.svelte';
	import Confirm from '$lib/components/modals/Confirm.svelte';
	import EditGroup from '$lib/components/modals/EditGroup.svelte';
	import CreateGroupWallet from '$lib/components/modals/CreateGroupWallet.svelte';
	import FundGroupWallet from '$lib/components/modals/FundGroupWallet.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';
	import ProposeWithdrawModal from '$lib/components/modals/ProposeWithdrawModal.svelte';
	import WithdrawProposalDrawer from '$lib/components/WithdrawProposalDrawer.svelte';

	// --- STATES ---
	let loading = $state(true);
	let loadingMembers = $state(true);
	let loadingWallets = $state(true);
	let groupExists = $state(true);

	let groupData = $state({} as Group);
	let members = $state([] as UserBadge[]);
	let wallets = $state([] as GroupWallet[]);

	const groupId = page.params.group_id as string;

	// Sistema de Tabs
	type Tab = 'general' | 'wallets' | 'fund_rounds';
	let activeTab = $state<Tab>('general');

	// Modals
	let showNewMemberModal = $state(false);
	let showDeleteModal = $state(false);
	let showEditModal = $state(false);
	let showCreateWalletModal = $state(false);
	let showFundWalletModal = $state(false);
	let showLeaveModal = $state(false);
	let showWithdrawModal = $state(false);
	let showProposalsDrawer = $state(false);
	let selectedCurrencyIdToWithdraw = $state<string>('');
	let selectedWalletIdToFund = $state<string>('');
	let selectedCurrencyId = $state<string>('');

	let deleteLoading = $state(false);
	let deleteError = $state('');
	let leaveLoading = $state(false);
	let leaveError = $state('');

	// --- FUND ROUNDS MOCK STATE ---
	let showFundRoundAccordion = $state(false);
	let selectedFundWallet = $state('');

	// Mock de rondas de fondeo
	const mockFundRounds = [
		{ id: 1, title: 'Ronda #1', goal: 500, raised: 375, currency: 'USDC' },
		{ id: 2, title: 'Ronda #2', goal: 1000, raised: 200, currency: 'USDC' }
	];

	// --- LOGIC ---
	async function handleEditGroup(data: { name: string; description: string }) {
		const res = await updateGroup(groupId, data);
		if (!isSuccess(res)) throw new Error(res.message || 'Failed to update group.');
		groupData = res.body;
	}

	async function handleDeleteGroup() {
		deleteLoading = true;
		deleteError = '';
		const res = await deleteGroup(groupId);
		deleteLoading = false;
		if (!isSuccess(res)) {
			deleteError = res.message || 'Failed to delete group.';
			return;
		}
		window.location.href = '/dashboard';
	}

	async function handleLeaveGroup() {
		leaveLoading = true;
		leaveError = '';
		const res = await leaveGroup(groupId);
		leaveLoading = false;
		if (!isSuccess(res)) {
			leaveError = res.message || 'Error al salir del grupo.';
			return;
		}
		window.location.href = '/dashboard';
	}

	async function loadGroupData() {
		const res = await getGroup(groupId);
		if (!isSuccess(res)) {
			groupExists = false;
			loading = false;
			return;
		}
		groupData = res.body;
		loading = false;
	}

	async function loadMembersData() {
		try {
			const res = await getGroupMembers(groupId);
			if (isSuccess(res)) members = res.body;
		} finally {
			loadingMembers = false;
		}
	}

	async function loadWalletsData() {
		try {
			const res = await getGroupWallets(groupId);
			if (!isSuccess(res)) return;
			wallets = res.body;
		} finally {
			loadingWallets = false;
		}
	}

	function openFundModal(walletId: string, currencyId: string) {
		selectedWalletIdToFund = walletId;
		selectedCurrencyId = currencyId;
		showFundWalletModal = true;
	}

	function openWithdrawModal(currencyId: string) {
		selectedCurrencyIdToWithdraw = currencyId;
		showWithdrawModal = true;
	}

	loadGroupData();
	loadMembersData();
	loadWalletsData();
</script>

<svelte:head>
	<title>Lemipay - {groupData.name || 'Group'}</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4">
	{#if loading}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-black"
		></div>
	{:else if !groupExists}
		<div class="mt-20 space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-black">404 - Group not found</h1>
			<p class="text-sm text-gray-500">The group you are looking for does not exist.</p>
		</div>
	{:else}
		<!-- HEADER: fluye sobre el fondo, ancho extendido -->
		<div class="w-full max-w-4xl border-b border-gray-200 pt-8 pb-6">
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
				<div class="space-y-1">
					<div class="flex items-center gap-3">
						<h1 class="text-2xl font-bold tracking-tight text-black">{groupData.name}</h1>
						{#if groupData.status}
							<span
								class="rounded border border-gray-200 bg-gray-50 px-2.5 py-1 text-xs font-medium text-gray-500"
							>
								{groupData.status}
							</span>
						{/if}
					</div>
					{#if groupData.description}
						<p class="text-sm leading-relaxed text-gray-500">{groupData.description}</p>
					{/if}
				</div>

				<!-- Botones de acción -->
				<div class="flex items-center gap-1 self-start">
					<Button
						label="Propuestas"
						variant="secondary"
						onclick={() => (showProposalsDrawer = true)}
					>
						{#snippet icon()}
							<HandCoins class="h-4 w-4" />
						{/snippet}
					</Button>

					<button
						onclick={() => (showEditModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-gray-100 hover:text-gray-700"
						title="Editar grupo"
					>
						<Pencil class="h-4 w-4" />
					</button>

					<button
						onclick={() => (showLeaveModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-orange-50 hover:text-orange-500"
						title="Salir del grupo"
					>
						<LogOut class="h-4 w-4" />
					</button>

					<button
						onclick={() => (showDeleteModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-red-50 hover:text-red-500"
						title="Eliminar grupo"
					>
						<Trash2 class="h-4 w-4" />
					</button>
				</div>
			</div>
		</div>

		<!-- TABS NAV -->
		<div class="w-full max-w-4xl">
			<div class="flex border-b border-gray-200">
				{#each [{ key: 'general', label: 'General' }, { key: 'wallets', label: 'Billeteras' }, { key: 'fund_rounds', label: 'Rondas de Fondeo' }] as const as tab}
					<button
						onclick={() => (activeTab = tab.key)}
						class={[
							'px-4 py-3 text-sm font-medium transition-colors',
							activeTab === tab.key
								? 'border-b-2 border-black text-black'
								: 'text-gray-500 hover:text-black'
						].join(' ')}
					>
						{tab.label}
					</button>
				{/each}
			</div>

			<!-- TAB CONTENT -->
			<div class="py-8">
				<!-- GENERAL TAB -->
				{#if activeTab === 'general'}
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-6 duration-300">
						<div class="flex items-center justify-between">
							<h2 class="text-sm font-medium text-black">Miembros</h2>
							<Button
								label="Invitar"
								variant="secondary"
								onclick={() => (showNewMemberModal = true)}
							>
								{#snippet icon()}
									<Plus class="h-4 w-4" />
								{/snippet}
							</Button>
						</div>

						{#if loadingMembers}
							<div
								class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
							></div>
						{:else if members.length > 0}
							<div class="flex flex-wrap gap-2 pt-1">
								{#each members as member}
									<UserIconBadge user={member} />
								{/each}
							</div>
						{:else}
							<p class="text-sm text-gray-400">No hay miembros aún.</p>
						{/if}
					</div>
				{/if}

				<!-- WALLETS TAB -->
				{#if activeTab === 'wallets'}
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
						<div class="flex items-center justify-between">
							<h2 class="text-sm font-medium text-black">Billeteras del Grupo</h2>
							<Button
								label="Nueva Wallet"
								variant="primary"
								onclick={() => (showCreateWalletModal = true)}
							>
								{#snippet icon()}
									<Wallet class="h-4 w-4" />
								{/snippet}
							</Button>
						</div>

						{#if loadingWallets}
							<div
								class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
							></div>
						{:else if wallets.length > 0}
							<div class="space-y-3 pt-2">
								{#each wallets as wallet}
									<div
										class="flex flex-col items-start justify-between gap-4 rounded-lg border border-gray-200 bg-white p-4 sm:flex-row sm:items-center"
									>
										<div class="space-y-1">
											<div class="flex items-center gap-2">
												<span class="text-lg font-bold text-black">${wallet.balance}</span>
												<span
													class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
												>
													{wallet.currency_ticker ? wallet.currency_ticker : 'USDC'}
												</span>
											</div>
											<div class="flex items-center gap-2 text-xs text-gray-500">
												<span>{shortenAddress(wallet.address)}</span>
												<button class="transition hover:text-black" aria-label="Copy address">
													<Copy class="h-3 w-3" />
												</button>
											</div>
										</div>

										<div class="flex items-center gap-2">
											<Button
												label="Retirar"
												variant="secondary"
												onclick={() => openWithdrawModal(wallet.currency_id)}
											>
												{#snippet icon()}
													<HandCoins class="h-4 w-4" />
												{/snippet}
											</Button>
											<Button
												label="Fondear"
												variant="secondary"
												onclick={() => openFundModal(wallet.id, wallet.currency_id)}
											>
												{#snippet icon()}
													<Coins class="h-4 w-4" />
												{/snippet}
											</Button>
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<div class="rounded-lg border border-dashed border-gray-300 p-8 text-center">
								<Wallet class="mx-auto mb-3 h-8 w-8 text-gray-400" />
								<p class="text-sm font-medium text-black">Sin billeteras</p>
								<p class="mb-4 text-sm text-gray-500">
									Este grupo no tiene ninguna billetera asociada aún.
								</p>
								<Button
									label="Crear primera wallet"
									variant="secondary"
									onclick={() => (showCreateWalletModal = true)}
								/>
							</div>
						{/if}
					</div>
				{/if}

				<!-- RONDAS DE FONDEO TAB -->
				{#if activeTab === 'fund_rounds'}
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
						<div class="flex items-center justify-between">
							<h2 class="text-sm font-medium text-black">Rondas de Fondeo</h2>
							<Button label="Nueva Ronda" variant="primary">
								{#snippet icon()}
									<Plus class="h-4 w-4" />
								{/snippet}
							</Button>
						</div>

						<div class="space-y-3 pt-2">
							{#each mockFundRounds as round}
								{@const progress = Math.round((round.raised / round.goal) * 100)}
								{@const remaining = round.goal - round.raised}
								{@const isOpen = showFundRoundAccordion && round.id === 1}

								<div class="overflow-hidden rounded-lg border border-gray-200 bg-white">
									<!-- Card principal -->
									<div class="space-y-4 p-5">
										<!-- Fila superior: título + objetivo -->
										<div class="flex items-start justify-between gap-2">
											<div>
												<p class="text-xs font-medium tracking-wider text-gray-400 uppercase">
													{round.title}
												</p>
												<p class="mt-0.5 text-xl font-bold text-black">
													${round.goal}
													<span class="ml-1 text-sm font-medium text-gray-500"
														>{round.currency}</span
													>
												</p>
											</div>
											<span
												class="rounded-full bg-gray-100 px-2.5 py-1 text-xs font-medium text-gray-600"
											>
												{progress}%
											</span>
										</div>

										<!-- Progress bar minimalista -->
										<div class="h-2 w-full overflow-hidden rounded-full bg-gray-100">
											<div
												class="h-full rounded-full bg-black transition-all duration-700"
												style="width: {progress}%"
											></div>
										</div>

										<!-- Fila inferior: info + botón -->
										<div class="flex items-center justify-between gap-4">
											<p class="text-xs text-gray-500">
												<span class="font-medium text-gray-700">{progress}% completado</span>
												&mdash; Faltan
												<span class="font-medium text-gray-700">${remaining} {round.currency}</span>
											</p>

											<button
												onclick={() =>
													(showFundRoundAccordion =
														round.id === 1 ? !showFundRoundAccordion : true)}
												class="flex items-center gap-1.5 rounded-md bg-black px-3.5 py-2 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95"
											>
												Aportar mi parte
												<ChevronDown
													class={[
														'h-3.5 w-3.5 transition-transform duration-200',
														isOpen ? 'rotate-180' : ''
													].join(' ')}
												/>
											</button>
										</div>
									</div>

									<!-- Panel acordeón -->
									{#if isOpen}
										<div
											transition:slide={{ duration: 220 }}
											class="space-y-4 border-t border-gray-100 bg-gray-50 px-5 py-4"
										>
											<p class="text-xs font-medium tracking-wider text-gray-500 uppercase">
												Elegí tu wallet para aportar
											</p>

											<div class="space-y-3">
												<select
													bind:value={selectedFundWallet}
													class="w-full rounded-md border border-gray-200 bg-white px-3 py-2.5 text-sm text-black transition outline-none focus:border-black focus:ring-1 focus:ring-black"
												>
													<option value="" disabled selected>Seleccionar wallet personal...</option>
													{#each wallets as wallet}
														<option value={wallet.id}>
															{shortenAddress(wallet.address)} — ${wallet.balance}
															{wallet.currency_ticker ?? 'USDC'}
														</option>
													{/each}
													<!-- Fallback mock si no hay wallets cargadas -->
													{#if wallets.length === 0}
														<option value="mock-1">0xABCD...1234 — $200.00 USDC</option>
														<option value="mock-2">0xEFGH...5678 — $50.00 USDC</option>
													{/if}
												</select>

												<div class="flex items-center justify-end gap-2">
													<button
														onclick={() => (showFundRoundAccordion = false)}
														class="rounded-md px-3.5 py-2 text-xs font-medium text-gray-500 transition hover:text-black"
													>
														Cancelar
													</button>
													<button
														class="rounded-md bg-black px-4 py-2 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95 disabled:opacity-40"
														disabled={!selectedFundWallet}
													>
														Confirmar Aporte
													</button>
												</div>
											</div>
										</div>
									{/if}
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</div>

		<!-- Volver -->
		<div class="w-full max-w-4xl pb-10">
			<a
				href="/dashboard"
				class="text-sm font-medium text-gray-400 transition hover:text-black hover:underline"
			>
				← Volver al Dashboard
			</a>
		</div>

		<!-- MODALS & DRAWERS (sin cambios) -->
		<InviteUserToGroup
			group_id={groupData.id}
			open={showNewMemberModal}
			onclose={() => (showNewMemberModal = false)}
			onsuccess={loadMembersData}
		/>
		<CreateGroupWallet
			open={showCreateWalletModal}
			group_id={groupData.id}
			onclose={() => (showCreateWalletModal = false)}
		/>
		<FundGroupWallet
			open={showFundWalletModal}
			currency_id={selectedCurrencyId}
			group_id={groupData.id}
			wallet_id={selectedWalletIdToFund}
			onclose={() => {
				showFundWalletModal = false;
				selectedWalletIdToFund = '';
				selectedCurrencyId = '';
			}}
			onsuccess={loadWalletsData}
		/>
		<ProposeWithdrawModal
			open={showWithdrawModal}
			group_id={groupData.id}
			currency_id={selectedCurrencyIdToWithdraw}
			onclose={() => {
				showWithdrawModal = false;
				selectedCurrencyIdToWithdraw = '';
			}}
			onsuccess={loadWalletsData}
		/>
		<EditGroup
			open={showEditModal}
			group={groupData}
			onclose={() => (showEditModal = false)}
			onedit={handleEditGroup}
		/>
		<WithdrawProposalDrawer
			open={showProposalsDrawer}
			group_id={groupData.id}
			onclose={() => (showProposalsDrawer = false)}
			onsuccess={loadWalletsData}
		/>
		<Confirm
			open={showLeaveModal}
			title="Salir del grupo"
			description="Dejarás de tener acceso a las billeteras y transacciones."
			message="¿Estás seguro de que querés salir de este grupo?"
			onclose={() => {
				showLeaveModal = false;
				leaveError = '';
			}}
			onconfirm={handleLeaveGroup}
			loading={leaveLoading}
			error={leaveError}
		/>
		<Confirm
			open={showDeleteModal}
			title="Delete group"
			description="This action cannot be undone."
			message="Are you sure you want to delete this group?"
			onclose={() => {
				showDeleteModal = false;
				deleteError = '';
			}}
			onconfirm={handleDeleteGroup}
			loading={deleteLoading}
			error={deleteError}
		/>
	{/if}
</div>
