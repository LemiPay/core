<script lang="ts">
	import { Trash2, Pencil, HandCoins, LogOut } from 'lucide-svelte';

	import { page } from '$app/state';

	// API UI bindings (Solo las que borran/salen)
	import { deleteGroup, leaveGroup } from '$lib/api/endpoints/groups';
	import { cancelFundRoundProposal } from '$lib/api/endpoints/fund_rounds';

	// Helpers y Estado Global
	import { GroupState } from './group.svelte';

	// Components
	import Button from '$lib/components/ui/Button.svelte';
	import InviteUserToGroup from '$lib/components/modals/group/InviteUserToGroup.svelte';
	import Confirm from '$lib/components/modals/Confirm.svelte';
	import EditGroup from '$lib/components/modals/group/EditGroup.svelte';
	import CreateGroupWallet from '$lib/components/modals/group_wallet/CreateGroupWallet.svelte';
	import FundGroupWallet from '$lib/components/modals/group_wallet/FundGroupWallet.svelte';
	import CreateFundRound from '$lib/components/modals/group_wallet/CreateFundRound.svelte';
	import ProposeWithdrawModal from '$lib/components/modals/group_wallet/ProposeWithdrawModal.svelte';
	import WithdrawProposalDrawer from '$lib/components/WithdrawProposalDrawer.svelte';
	import CreateExpenseModal from '$lib/components/modals/group_wallet/CreateExpenseModal.svelte';

	// Tabs
	import GeneralTab from './tabs/GeneralTab.svelte';
	import WalletsTab from './tabs/WalletsTab.svelte';
	import FundRoundsTab from './tabs/FundRoundsTab.svelte';
	import BalancesTab from './tabs/BalancesTab.svelte';
	import ExpensesTab from './tabs/ExpensesTab.svelte';

	const groupId = page.params.group_id as string;
	const groupState = new GroupState(groupId);

	// UI States (Strictly UI Orchestration)
	type Tab = 'general' | 'wallets' | 'fund_rounds' | 'balances' | 'expenses';
	let activeTab = $state<Tab>('general');

	let showNewMemberModal = $state(false);
	let showDeleteModal = $state(false);
	let showEditModal = $state(false);
	let showCreateWalletModal = $state(false);
	let showFundWalletModal = $state(false);
	let showLeaveModal = $state(false);
	let showWithdrawModal = $state(false);
	let showProposalsDrawer = $state(false);
	let showCreateFundRoundModal = $state(false);
	let showCreateExpenseModal = $state(false);
	let showCancelFundRoundModal = $state(false);

	let selectedCurrencyIdToWithdraw = $state<string>('');
	let selectedWalletIdToFund = $state<string>('');
	let selectedCurrencyId = $state<string>('');
	let fundRoundToCancel = $state<string>('');

	// Local UI interactions for tabs
	let expandedFundRoundId = $state<string | null>(null);
	let selectedContribWalletId = $state('');
	let showPastFundRounds = $state(false);
	let showAllTransactions = $state(false);
	let showAllExpenses = $state(false);

	const visibleGroupTransactions = $derived(
		showAllTransactions
			? groupState.sortedGroupTransactions
			: groupState.sortedGroupTransactions.slice(0, 3)
	);
	const visibleGroupExpenses = $derived(
		showAllExpenses ? groupState.sortedGroupExpenses : groupState.sortedGroupExpenses.slice(0, 3)
	);

	const GROUP_TABS: { key: Tab; label: string }[] = [
		{ key: 'general', label: 'General' },
		{ key: 'wallets', label: 'Billeteras' },
		{ key: 'fund_rounds', label: 'Rondas de Fondeo' },
		{ key: 'balances', label: 'Balances' },
		{ key: 'expenses', label: 'División de Gastos' }
	];

	// Initial load
	groupState.loadGroupData();
	groupState.loadMembersData();
	groupState.loadWalletsData();
	groupState.loadExpensesData();
	groupState.loadCoreBalances();

	// Tab Refetching Logic
	$effect(() => {
		if (activeTab === 'wallets') groupState.loadWalletsData();
	});
	$effect(() => {
		if (activeTab === 'fund_rounds') groupState.loadFundRoundsData();
	});
	$effect(() => {
		if (activeTab === 'expenses') groupState.loadExpensesData();
	});
	$effect(() => {
		if (activeTab === 'balances') {
			groupState.loadCoreBalances();
			groupState.loadBalancesMovimientos();
		}
	});

	// UI Handlers
	function openFundModal(walletId: string, currencyId: string) {
		selectedWalletIdToFund = walletId;
		selectedCurrencyId = currencyId;
		showFundWalletModal = true;
	}

	function openWithdrawModal(currencyId: string) {
		selectedCurrencyIdToWithdraw = currencyId;
		showWithdrawModal = true;
	}

	function openCancelFundRoundModal(fundRoundId: string) {
		fundRoundToCancel = fundRoundId;
		showCancelFundRoundModal = true;
	}

	function toggleFundRoundAccordion(fundRoundId: string) {
		expandedFundRoundId = expandedFundRoundId === fundRoundId ? null : fundRoundId;
		selectedContribWalletId = '';
		groupState.contribError = '';
	}

	async function submitContribute(status: any) {
		const success = await groupState.handleContribute(status, selectedContribWalletId);
		if (success) {
			expandedFundRoundId = null;
			selectedContribWalletId = '';
		}
	}
</script>

<svelte:head>
	<title>Lemipay - {groupState.groupData.name || 'Group'}</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4">
	{#if groupState.loading}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-black"
		></div>
	{:else if !groupState.groupExists}
		<div class="mt-20 space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-black">404 - Group not found</h1>
			<p class="text-sm text-gray-500">The group you are looking for does not exist.</p>
		</div>
	{:else}
		<div class="w-full max-w-4xl border-b border-gray-200 pt-8 pb-6">
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
				<div class="space-y-1">
					<div class="flex items-center gap-3">
						<h1 class="text-2xl font-bold tracking-tight text-black">
							{groupState.groupData.name}
						</h1>
						{#if groupState.groupData.status}
							<span
								class="rounded border border-gray-200 bg-gray-50 px-2.5 py-1 text-xs font-medium text-gray-500"
							>
								{groupState.groupData.status}
							</span>
						{/if}
					</div>
					{#if groupState.groupData.description}
						<p class="text-sm leading-relaxed text-gray-500">{groupState.groupData.description}</p>
					{/if}
				</div>

				<div class="flex items-center gap-1 self-start">
					<Button
						label="Propuestas"
						variant="secondary"
						onclick={() => (showProposalsDrawer = true)}
					>
						{#snippet icon()}<HandCoins class="h-4 w-4" />{/snippet}
					</Button>
					<button
						onclick={() => (showEditModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-gray-100 hover:text-gray-700"
						title="Editar grupo"><Pencil class="h-4 w-4" /></button
					>
					<button
						onclick={() => (showLeaveModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-orange-50 hover:text-orange-500"
						title="Salir del grupo"><LogOut class="h-4 w-4" /></button
					>
					<button
						onclick={() => (showDeleteModal = true)}
						class="rounded-md p-2 text-gray-400 transition hover:bg-red-50 hover:text-red-500"
						title="Eliminar grupo"><Trash2 class="h-4 w-4" /></button
					>
				</div>
			</div>
		</div>

		<div class="w-full max-w-4xl">
			<div class="flex border-b border-gray-200">
				{#each GROUP_TABS as tab}
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

			<div class="py-8">
				{#if activeTab === 'general'}
					<GeneralTab
						{groupState}
						onInviteClick={() => (showNewMemberModal = true)}
						onGoToBalances={() => (activeTab = 'balances')}
					/>
				{:else if activeTab === 'wallets'}
					<WalletsTab
						{groupState}
						onCreateWallet={() => (showCreateWalletModal = true)}
						onFundWallet={(wId, cId) => {
							selectedWalletIdToFund = wId;
							selectedCurrencyId = cId;
							showFundWalletModal = true;
						}}
						onWithdraw={(cId) => {
							selectedCurrencyIdToWithdraw = cId;
							showWithdrawModal = true;
						}}
					/>
				{:else if activeTab === 'fund_rounds'}
					<FundRoundsTab
						{groupState}
						onCreateFundRound={() => (showCreateFundRoundModal = true)}
						onCancelFundRound={(id) => {
							fundRoundToCancel = id;
							showCancelFundRoundModal = true;
						}}
					/>
				{:else if activeTab === 'balances'}
					<BalancesTab {groupState} />
				{:else if activeTab === 'expenses'}
					<ExpensesTab {groupState} onCreateExpense={() => (showCreateExpenseModal = true)} />
				{/if}
			</div>
		</div>

		<div class="w-full max-w-4xl pb-10">
			<a
				href="/dashboard"
				class="text-sm font-medium text-gray-400 transition hover:text-black hover:underline"
				>← Volver al Dashboard</a
			>
		</div>

		<InviteUserToGroup
			group_id={groupState.groupData.id}
			open={showNewMemberModal}
			onclose={() => (showNewMemberModal = false)}
			onsuccess={() => groupState.loadMembersData()}
		/>
		<CreateGroupWallet
			open={showCreateWalletModal}
			group_id={groupState.groupData.id}
			onclose={() => (showCreateWalletModal = false)}
			onsuccess={() => groupState.loadWalletsData()}
		/>
		<FundGroupWallet
			open={showFundWalletModal}
			currency_id={selectedCurrencyId}
			group_id={groupState.groupData.id}
			wallet_id={selectedWalletIdToFund}
			onclose={() => {
				showFundWalletModal = false;
				selectedWalletIdToFund = '';
				selectedCurrencyId = '';
			}}
			onsuccess={() => groupState.loadWalletsData()}
		/>
		<ProposeWithdrawModal
			open={showWithdrawModal}
			group_id={groupState.groupData.id}
			currency_id={selectedCurrencyIdToWithdraw}
			onclose={() => {
				showWithdrawModal = false;
				selectedCurrencyIdToWithdraw = '';
			}}
			onsuccess={() => groupState.loadWalletsData()}
		/>
		<EditGroup
			open={showEditModal}
			group={groupState.groupData}
			onclose={() => (showEditModal = false)}
			onsuccess={(nuevoGrupo) => (groupState.groupData = nuevoGrupo)}
		/>
		<WithdrawProposalDrawer
			open={showProposalsDrawer}
			group_id={groupState.groupData.id}
			onclose={() => (showProposalsDrawer = false)}
			onsuccess={() => groupState.loadWalletsData()}
		/>
		<CreateFundRound
			open={showCreateFundRoundModal}
			group_id={groupState.groupData.id}
			onclose={() => (showCreateFundRoundModal = false)}
			onsuccess={() => groupState.loadFundRoundsData()}
		/>
		<CreateExpenseModal
			open={showCreateExpenseModal}
			group_id={groupState.groupData.id}
			members={groupState.members}
			onclose={() => (showCreateExpenseModal = false)}
			onsuccess={() => groupState.loadExpensesData()}
		/>

		<Confirm
			open={showLeaveModal}
			title="Salir del grupo"
			description="..."
			message="..."
			successMsg="Saliste del grupo exitosamente"
			onclose={() => (showLeaveModal = false)}
			onconfirm={() => leaveGroup(groupId)}
			onsuccess={() => (window.location.href = '/dashboard')}
		/>
		<Confirm
			open={showDeleteModal}
			title="Eliminar grupo"
			description="..."
			message="..."
			successMsg="Grupo eliminado"
			onclose={() => (showDeleteModal = false)}
			onconfirm={() => deleteGroup(groupId)}
			onsuccess={() => (window.location.href = '/dashboard')}
		/>
		<Confirm
			open={showCancelFundRoundModal}
			title="Cancelar ronda"
			description="..."
			message="..."
			successMsg="Ronda cancelada"
			onclose={() => {
				showCancelFundRoundModal = false;
				fundRoundToCancel = '';
			}}
			onconfirm={() => cancelFundRoundProposal(fundRoundToCancel)}
			onsuccess={() => groupState.loadFundRoundsData()}
		/>
	{/if}
</div>
