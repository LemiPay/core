<script lang="ts">
	import { Trash2, Pencil, Wallet, Coins, Plus, Copy } from 'lucide-svelte';
	import { page } from '$app/state';

	// Api
	import {
		getGroup,
		getGroupMembers,
		updateGroup,
		deleteGroup,
		getGroupWallets
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

	// --- STATES ---
	let loading = $state(true);
	let loadingMembers = $state(true);
	let loadingWallets = $state(true); // Nuevo estado de carga para wallets
	let groupExists = $state(true);

	let groupData = $state({} as Group);
	let members = $state([] as UserBadge[]);
	let wallets = $state([] as GroupWallet[]); // Nuevo estado para las wallets

	const groupId = page.params.group_id as string;

	// Sistema de Tabs nativo
	type Tab = 'general' | 'wallets';
	let activeTab = $state<Tab>('general');

	// Modals
	let showNewMemberModal = $state(false);
	let showDeleteModal = $state(false);
	let showEditModal = $state(false);
	let showCreateWalletModal = $state(false);
	let showFundWalletModal = $state(false);
	let selectedWalletIdToFund = $state<string>('');
	let selectedCurrencyId = $state<string>('');

	let deleteLoading = $state(false);
	let deleteError = $state('');

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
			if (!isSuccess(res)) {
				return;
			}
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

	loadGroupData();
	loadMembersData();
	loadWalletsData();
</script>

<svelte:head>
	<title>Lemipay - {groupData.name || 'Group'}</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center p-4 py-8">
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
		<div class="w-full max-w-2xl rounded-xl border border-gray-200 bg-white shadow-sm">
			<div class="p-6 pb-4 sm:p-8">
				<div class="space-y-2">
					<div class="flex items-start justify-between gap-4">
						<h1 class="text-2xl font-bold tracking-tight text-black">{groupData.name}</h1>
						<div class="flex items-center gap-2">
							{#if groupData.status}
								<span
									class="rounded border border-gray-200 bg-gray-50 px-2.5 py-1 text-xs font-medium text-gray-600"
								>
									{groupData.status}
								</span>
							{/if}
							<button
								onclick={() => (showEditModal = true)}
								class="rounded-md p-1 text-gray-400 transition hover:bg-gray-100 hover:text-gray-600"
							>
								<Pencil class="h-5 w-5" />
							</button>
							<button
								onclick={() => (showDeleteModal = true)}
								class="rounded-md p-1 text-gray-400 transition hover:bg-red-50 hover:text-red-500"
							>
								<Trash2 class="h-5 w-5" />
							</button>
						</div>
					</div>
					{#if groupData.description}
						<p class="text-sm leading-relaxed text-gray-500">{groupData.description}</p>
					{/if}
				</div>
			</div>

			<div class="flex border-b border-gray-200 px-6 sm:px-8">
				<button
					onclick={() => (activeTab = 'general')}
					class={[
						'px-4 py-3 text-sm font-medium transition-colors',
						activeTab === 'general'
							? 'border-b-2 border-black text-black'
							: 'text-gray-500 hover:text-black'
					].join(' ')}
				>
					General
				</button>
				<button
					onclick={() => (activeTab = 'wallets')}
					class={[
						'px-4 py-3 text-sm font-medium transition-colors',
						activeTab === 'wallets'
							? 'border-b-2 border-black text-black'
							: 'text-gray-500 hover:text-black'
					].join(' ')}
				>
					Billeteras
				</button>
			</div>

			<div class="p-6 sm:p-8">
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
										class="flex flex-col items-start justify-between gap-4 rounded-lg border border-gray-200 bg-gray-50/50 p-4 sm:flex-row sm:items-center"
									>
										<div class="space-y-1">
											<div class="flex items-center gap-2">
												<span class="text-lg font-bold text-black">${wallet.balance}</span>
												<span
													class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
												>
													<!-- ESTO ESTA HARDCODEADO TODO hacer que el back entregue tmb el ticker asi lo vemos aca-->
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

										<div>
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
			</div>
		</div>

		<div class="pt-6">
			<a
				href="/dashboard"
				class="text-sm font-medium text-gray-500 transition hover:text-black hover:underline"
			>
				← Back to Dashboard
			</a>
		</div>

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

		<EditGroup
			open={showEditModal}
			group={groupData}
			onclose={() => (showEditModal = false)}
			onedit={handleEditGroup}
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
