<script lang="ts">
	import { Trash2, Pencil, HandCoins, LogOut, TrendingUp, ShieldAlert } from 'lucide-svelte';

	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	// API UI bindings (Solo las que borran/salen)
	import { deleteGroup, enterDebtResolution, leaveGroup } from '$lib/api/endpoints/groups';
	import { cancelFundRoundProposal } from '$lib/api/endpoints/fund_rounds';

	// Helpers y Estado Global
	import { GroupState } from './group.svelte';
	import { authStore } from '$lib/stores/auth';

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
	import DebtResolutionPanel from '$lib/components/pages/group/DebtResolutionPanel.svelte';

	// Tabs
	import GeneralTab from './tabs/GeneralTab.svelte';
	import WalletsTab from './tabs/WalletsTab.svelte';
	import FundRoundsTab from './tabs/FundRoundsTab.svelte';
	import BalancesTab from './tabs/BalancesTab.svelte';
	import ExpensesTab from './tabs/ExpensesTab.svelte';
	import HistoryTab from './tabs/HistoryTab.svelte';

	const groupId = page.params.group_id as string;
	const groupState = new GroupState(groupId);

	// UI States (Strictly UI Orchestration)
	type Tab = 'general' | 'wallets' | 'fund_rounds' | 'balances' | 'expenses' | 'history';
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
	let showDebtPanel = $state(true);
	let showConfirmDebtResolution = $state(false);

	let currentUserId = $derived($authStore.user?.id);

	let currentUserBalance = $derived(
		groupState.memberBalances.find((m) => m.user.user_id === currentUserId)?.balance ?? 0
	);

	let isCurrentUserAdmin = $derived(
		groupState.members.some((m) => m.user_id === currentUserId && m.role === 'Admin')
	);

	let settlementCurrencyId = $derived(groupState.wallets[0]?.currency_id ?? '');

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
		{ key: 'wallets', label: 'Billetera' },
		{ key: 'fund_rounds', label: 'Rondas de Fondeo' },
		{ key: 'balances', label: 'Balances' },
		{ key: 'expenses', label: 'Gastos' },
		{ key: 'history', label: 'Historial' }
	];

	// Initial load
	groupState.loadGroupData();
	groupState.loadMembersData();
	groupState.loadWalletsData();
	groupState.loadExpensesData();
	groupState.loadCoreBalances();
	groupState.loadSettlements();

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
	$effect(() => {
		if (activeTab === 'history') {
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

<div
	class="flex min-h-[calc(100vh-64px)] flex-col items-center bg-background px-4 pt-16 text-foreground"
>
	{#if groupState.loading}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-border border-t-foreground"
		></div>
	{:else if !groupState.groupExists}
		<div class="mt-20 space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-foreground">404 - Group not found</h1>
			<p class="text-sm text-muted-foreground">The group you are looking for does not exist.</p>
		</div>
	{:else}
		<div class="w-full max-w-4xl border-b border-border pt-8 pb-6">
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
				<div class="space-y-1">
					<div class="flex items-center gap-3">
						<h1 class="text-2xl font-bold tracking-tight text-foreground">
							{groupState.groupData.name}
						</h1>
						{#if groupState.groupData.status}
							<span
								class="rounded border border-border bg-muted px-2.5 py-1 text-xs font-medium text-muted-foreground"
							>
								{groupState.groupData.status}
							</span>
						{/if}
					</div>
					{#if groupState.groupData.description}
						<p class="text-sm leading-relaxed text-muted-foreground">
							{groupState.groupData.description}
						</p>
					{/if}
				</div>

				<div class="flex items-center gap-1 self-start">
					<a
						href={`/groups/${groupId}/investments`}
						class="inline-flex items-center gap-2 rounded-md border border-emerald-300 bg-emerald-50 px-4 py-2 text-sm font-medium text-emerald-700 transition hover:bg-emerald-100 hover:text-emerald-800"
					>
						<TrendingUp class="h-4 w-4" />
						Inversiones
					</a>
					<Button
						label="Propuestas"
						variant="secondary"
						onclick={() => (showProposalsDrawer = true)}
					>
						{#snippet icon()}
							<HandCoins class="h-4 w-4" />
						{/snippet}
					</Button>
					{#if !groupState.readonly && isCurrentUserAdmin}
						<button
							onclick={() => (showConfirmDebtResolution = true)}
							class="inline-flex items-center gap-1.5 rounded-md border border-amber-300 bg-amber-50 px-3 py-2 text-sm font-medium text-amber-700 transition hover:bg-amber-100 hover:text-amber-800"
							title="Iniciar resolución de deudas"
						>
							<ShieldAlert class="h-4 w-4" />
							Finalizar grupo
						</button>
					{/if}
					{#if !groupState.readonly}
						<button
							onclick={() => (showEditModal = true)}
							class="rounded-md p-2 text-muted-foreground transition hover:bg-accent hover:text-accent-foreground"
							title="Editar grupo"
						>
							<Pencil class="h-4 w-4" />
						</button>
						<button
							onclick={() => (showLeaveModal = true)}
							class="rounded-md p-2 text-muted-foreground transition hover:bg-orange-50 hover:text-orange-500 dark:hover:bg-orange-400/10 dark:hover:text-orange-300"
							title="Salir del grupo"
						>
							<LogOut class="h-4 w-4" />
						</button>
						<button
							onclick={() => (showDeleteModal = true)}
							class="rounded-md p-2 text-muted-foreground transition hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-400/10 dark:hover:text-red-300"
							title="Eliminar grupo"
						>
							<Trash2 class="h-4 w-4" />
						</button>
					{/if}
				</div>
			</div>
		</div>

		<div class="w-full max-w-4xl">
			<div class="flex border-b border-border">
				{#each GROUP_TABS as tab (tab.key)}
					<button
						onclick={() => (activeTab = tab.key)}
						class={[
							'px-4 py-3 text-sm font-medium transition-colors',
							activeTab === tab.key
								? 'border-b-2 border-foreground text-foreground'
								: 'text-muted-foreground hover:text-foreground'
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
						readonly={groupState.readonly}
						onInviteClick={() => (showNewMemberModal = true)}
						onGoToBalances={() => (activeTab = 'balances')}
					/>
				{:else if activeTab === 'wallets'}
					<WalletsTab
						{groupState}
						readonly={groupState.readonly}
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
						readonly={groupState.readonly}
						onCreateFundRound={() => (showCreateFundRoundModal = true)}
						onCancelFundRound={(id) => {
							fundRoundToCancel = id;
							showCancelFundRoundModal = true;
						}}
					/>
				{:else if activeTab === 'balances'}
					<BalancesTab {groupState} />
				{:else if activeTab === 'expenses'}
					<ExpensesTab
						{groupState}
						readonly={groupState.readonly}
						onCreateExpense={() => (showCreateExpenseModal = true)}
					/>
				{:else if activeTab === 'history'}
					<HistoryTab {groupState} />
				{/if}
			</div>
		</div>

		<div class="w-full max-w-4xl pb-10">
			<a
				href={resolve('/dashboard')}
				class="text-sm font-medium text-muted-foreground transition hover:text-foreground hover:underline"
				>← Volver al Dashboard</a
			>
		</div>

		{#if groupState.readonly && showDebtPanel && !groupState.loading}
			<DebtResolutionPanel
				debtAmount={groupState.currentUserDebtRaw}
				credits={groupState.userCredits}
				loading={groupState.settlementsLoading}
				error={groupState.settlementsError}
				{currentUserBalance}
				claimableAmount={groupState.currentUserBalanceRaw}
				hasDebtors={groupState.hasDebtors}
				currencyId={settlementCurrencyId}
				paying={groupState.settlementPaying}
				payError={groupState.settlementPayError}
				claiming={groupState.claimPaying}
				claimError={groupState.claimError}
				onPaySettlement={(amt, a, c) => groupState.paySettlement(amt, a, c)}
				onClaim={(a, c, amt) => groupState.claim(a, c, amt)}
				onClose={() => (showDebtPanel = false)}
			/>
		{/if}

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
			description="Los integrantes podrán usar la plata que hayas aportado"
			message="¿Estas seguro?"
			successMsg="Saliste del grupo exitosamente"
			onclose={() => (showLeaveModal = false)}
			onconfirm={() => leaveGroup(groupId)}
			onsuccess={() => (window.location.href = '/dashboard')}
		/>
		<Confirm
			open={showDeleteModal}
			title="Eliminar grupo"
			description="Vas a poder revisar la historia del grupo pero no hacer nada"
			message="Esta acción no se puede deshacer"
			successMsg="Grupo eliminado"
			onclose={() => (showDeleteModal = false)}
			onconfirm={() => deleteGroup(groupId)}
			onsuccess={() => (window.location.href = '/dashboard')}
		/>
		<Confirm
			open={showCancelFundRoundModal}
			title="Cancelar ronda"
			description="La plata que ya fue aportada quedará en la billetera de grupo"
			message="¿Estas seguro que quieres cancelarla?"
			successMsg="Ronda cancelada"
			onclose={() => {
				showCancelFundRoundModal = false;
				fundRoundToCancel = '';
			}}
			onconfirm={() => cancelFundRoundProposal(fundRoundToCancel)}
			onsuccess={() => groupState.loadFundRoundsData()}
		/>
		<Confirm
			open={showConfirmDebtResolution}
			title="Finalizar grupo"
			description="Se va a inhabilitar la creación de gastos, billeteras, rondas de fondeo y más. Solo se podrá ver la historia y los balances."
			message="¿Estás seguro de iniciar la resolución de deudas?"
			successMsg="Resolución de deudas iniciada"
			onclose={() => (showConfirmDebtResolution = false)}
			onconfirm={() => enterDebtResolution(groupId)}
			onsuccess={() => groupState.loadGroupData()}
		/>
	{/if}
</div>
