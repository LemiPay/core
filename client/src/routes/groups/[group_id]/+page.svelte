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
		ChevronDown,
		CircleCheckBig,
		Users,
		Calendar,
		Target,
		Ban,
		TrendingUp,
		TrendingDown,
		ArrowRight,
		Scale,
		CircleUser,
		Info
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
	import {
		cancelFundRoundProposal,
		contributeFundRound,
		getFundRoundProposal,
		getFundRoundRemaining,
		getGroupFundRoundProposals,
		getMyFundRoundContribution
	} from '$lib/api/endpoints/fund_rounds';
	import { getGroupBalances } from '$lib/api/endpoints/core';
	import { getGroupExpenses } from '$lib/api/endpoints/expenses';
	import { listGroupTransactions } from '$lib/api/endpoints/transactions';
	import { getExpenses } from '$lib/api/endpoints/expenses';
	import { getMyWallets } from '$lib/api/endpoints/wallets';

	// Stores
	import { authStore } from '$lib/stores/auth';

	// Helpers
	import { isSuccess } from '$lib/types/client.types';

	// Types
	import type { Group } from '$lib/types/endpoints/groups.types';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';
	import type { GroupWallet } from '$lib/types/endpoints/groups.types';
	import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';
	import type { GroupBalancesResponse } from '$lib/types/endpoints/core.types';
	import type { Transaction } from '$lib/types/endpoints/transactions.types';
	import type { Expense } from '$lib/types/endpoints/expenses.types';
	import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';

	// Components
	import UserIconBadge from '$lib/components/UserIconBadge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import InviteUserToGroup from '$lib/components/modals/InviteUserToGroup.svelte';
	import Confirm from '$lib/components/modals/Confirm.svelte';
	import EditGroup from '$lib/components/modals/EditGroup.svelte';
	import CreateGroupWallet from '$lib/components/modals/CreateGroupWallet.svelte';
	import FundGroupWallet from '$lib/components/modals/FundGroupWallet.svelte';
	import CreateFundRound from '$lib/components/modals/CreateFundRound.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';
	import { getProposalStatusDisplay } from '$lib/utils/proposal_status';
	import ProposeWithdrawModal from '$lib/components/modals/ProposeWithdrawModal.svelte';
	import WithdrawProposalDrawer from '$lib/components/WithdrawProposalDrawer.svelte';
	import CreateExpenseModal from '$lib/components/modals/CreateExpenseModal.svelte';

	// --- STATES ---
	let loading = $state(true);
	let loadingMembers = $state(true);
	let loadingWallets = $state(true);
	let groupExists = $state(true);

	let groupData = $state({} as Group);
	let members = $state([] as UserBadge[]);
	let wallets = $state([] as GroupWallet[]);
	const groupWalletsBalance = $derived(
		wallets.reduce((acc, wallet) => acc + Number(wallet.balance || 0), 0)
	);

	const groupId = page.params.group_id as string;

	// Sistema de Tabs
	type Tab = 'general' | 'wallets' | 'fund_rounds' | 'balances' | 'expenses';
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
	let showCreateFundRoundModal = $state(false);
	let showCreateExpenseModal = $state(false);
	let selectedCurrencyIdToWithdraw = $state<string>('');
	let selectedWalletIdToFund = $state<string>('');
	let selectedCurrencyId = $state<string>('');

	let deleteLoading = $state(false);
	let deleteError = $state('');
	let leaveLoading = $state(false);
	let leaveError = $state('');

	// --- FUND ROUNDS STATE ---
	let fundRounds = $state<FundRoundStatusResponse[]>([]);
	let loadingFundRounds = $state(true);
	let fundRoundsError = $state('');
	let userWallets = $state<WalletCurrency[]>([]);
	// proposal_id -> amount aportado por el usuario actual (string BigDecimal)
	let myContributions = $state<Record<string, string>>({});
	let expandedFundRoundId = $state<string | null>(null);
	let selectedContribWalletId = $state('');
	let contribLoading = $state(false);
	let contribError = $state('');

	let showCancelFundRoundModal = $state(false);
	let fundRoundToCancel = $state<string>('');
	let cancelFundRoundLoading = $state(false);
	let cancelFundRoundError = $state('');

	let showPastFundRounds = $state(false);
	let expenses = $state<Expense[]>([]);
	let loadingExpenses = $state(true);
	let expensesError = $state('');

	const currentUserId = $derived($authStore.user?.id);
	const recentExpenses = $derived(
		[...expenses]
			.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
			.slice(0, 3)
	);
	// Activas: las que todavía pueden recibir aportes o están a la espera.
	// Pasadas: finalizadas, canceladas o que no pueden avanzar.
	const activeFundRounds = $derived(
		fundRounds.filter((r) => {
			const s = r.fund_round.proposal.status;
			return s === 'Pending' || s === 'Approved';
		})
	);
	const pastFundRounds = $derived(
		fundRounds.filter((r) => {
			const s = r.fund_round.proposal.status;
			return s !== 'Pending' && s !== 'Approved';
		})
	);

	function recommendedAmount(target: string): number {
		const n = Math.max(1, members.length);
		return Number(target) / n;
	}

	function formatAmount(value: number): string {
		return value.toFixed(2);
	}

	function formatExpenseDate(value: string): string {
		return new Date(value).toLocaleDateString('es-AR', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		});
	}

	function getMemberName(userId: string): string {
		const member = members.find((item) => item.user_id === userId);
		return member?.name ?? 'Usuario';
	}

	function parseBalanceValue(v: string | number): number {
		if (typeof v === 'number') return Number.isFinite(v) ? v : 0;
		const n = Number(v);
		return Number.isFinite(n) ? n : 0;
	}

	function formatTxType(t: string): string {
		const map: Record<string, string> = {
			Deposit: 'Depósito',
			Withdraw: 'Retiro',
			Expense: 'Gasto',
			Investment: 'Inversión'
		};
		return map[t] ?? t;
	}

	function formatDateTimeShort(iso: string): string {
		const d = new Date(iso);
		if (Number.isNaN(d.getTime())) return iso;
		return d.toLocaleString('es-AR', { dateStyle: 'short', timeStyle: 'short' });
	}

	function userBadgeFromCoreRow(
		row: { user_id: string; user_name: string },
		memberList: UserBadge[]
	): UserBadge {
		const m = memberList.find((x) => x.user_id === row.user_id);
		return m ?? { user_id: row.user_id, name: row.user_name, role: 'Miembro' };
	}

	// --- BALANCES (API /core/balances/:groupId) ---
	// Balance positivo: al miembro le deben. Balance negativo: el miembro debe.
	let coreBalancesData = $state(null as GroupBalancesResponse | null);
	let loadingCoreBalances = $state(true);
	let coreBalancesError = $state('');

	let groupTransactions = $state([] as Transaction[]);
	let groupExpenses = $state([] as Expense[]);
	let loadingBalancesDetail = $state(false);
	let transactionsDetailError = $state('');
	let expensesDetailError = $state('');
	let showAllTransactions = $state(false);
	let showAllExpenses = $state(false);

	type MemberBalance = { user: UserBadge; balance: number };
	type Settlement = { from: UserBadge; to: UserBadge; amount: number };

	const memberBalances = $derived.by<MemberBalance[]>(() => {
		if (!coreBalancesData?.balances?.length) return [];
		return coreBalancesData.balances.map((b) => ({
			user: userBadgeFromCoreRow(b, members),
			balance: parseBalanceValue(b.balance)
		}));
	});

	const coreGroupBalance = $derived(
		coreBalancesData ? parseBalanceValue(coreBalancesData.group_balance) : 0
	);

	const sortedGroupTransactions = $derived(
		[...groupTransactions].sort(
			(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		)
	);
	const sortedGroupExpenses = $derived(
		[...groupExpenses].sort(
			(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		)
	);
	const visibleGroupTransactions = $derived(
		showAllTransactions ? sortedGroupTransactions : sortedGroupTransactions.slice(0, 3)
	);
	const visibleGroupExpenses = $derived(
		showAllExpenses ? sortedGroupExpenses : sortedGroupExpenses.slice(0, 3)
	);

	const sortedMemberBalances = $derived([...memberBalances].sort((a, b) => b.balance - a.balance));

	const totalToReceive = $derived(
		memberBalances.filter((m) => m.balance > 0).reduce((acc, x) => acc + x.balance, 0)
	);
	const totalToPay = $derived(
		memberBalances.filter((m) => m.balance < 0).reduce((acc, x) => acc + Math.abs(x.balance), 0)
	);

	// Algoritmo greedy: al que más le deben contra el que más debe, hasta saldar todo.
	const settlements = $derived.by<Settlement[]>(() => {
		const creditors = memberBalances
			.filter((m) => m.balance > 0.01)
			.map((m) => ({ user: m.user, remaining: m.balance }))
			.sort((a, b) => b.remaining - a.remaining);
		const debtors = memberBalances
			.filter((m) => m.balance < -0.01)
			.map((m) => ({ user: m.user, remaining: -m.balance }))
			.sort((a, b) => b.remaining - a.remaining);

		const result: Settlement[] = [];
		let i = 0;
		let j = 0;
		while (i < debtors.length && j < creditors.length) {
			const amount = Math.min(debtors[i].remaining, creditors[j].remaining);
			if (amount > 0.01) {
				result.push({
					from: debtors[i].user,
					to: creditors[j].user,
					amount: Math.round(amount * 100) / 100
				});
			}
			debtors[i].remaining -= amount;
			creditors[j].remaining -= amount;
			if (debtors[i].remaining < 0.01) i++;
			if (creditors[j].remaining < 0.01) j++;
		}
		return result;
	});

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
		loadingWallets = true;
		try {
			const res = await getGroupWallets(groupId);
			if (!isSuccess(res)) return;
			wallets = res.body;
		} finally {
			loadingWallets = false;
		}
	}

	async function loadCoreBalances() {
		loadingCoreBalances = true;
		coreBalancesError = '';
		try {
			const res = await getGroupBalances(groupId);

			if (!isSuccess(res)) {
				coreBalancesError = res.message || 'No se pudieron cargar los balances del grupo.';
				coreBalancesData = null;
				return;
			}
			coreBalancesData = res.body;
		} finally {
			loadingCoreBalances = false;
		}
	}

	async function loadBalancesMovimientos() {
		loadingBalancesDetail = true;
		transactionsDetailError = '';
		expensesDetailError = '';
		showAllTransactions = false;
		showAllExpenses = false;
		const [txRes, expRes] = await Promise.all([
			listGroupTransactions(groupId),
			getGroupExpenses(groupId)
		]);
		if (isSuccess(txRes)) {
			groupTransactions = txRes.body;
		} else {
			groupTransactions = [];
			transactionsDetailError = txRes.message || 'No se pudieron cargar las transacciones.';
		}
		if (isSuccess(expRes)) {
			groupExpenses = expRes.body;
		} else {
			groupExpenses = [];
			expensesDetailError = expRes.message || 'No se pudieron cargar los gastos.';
		}
		loadingBalancesDetail = false;
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

	async function loadFundRoundsData() {
		loadingFundRounds = true;
		fundRoundsError = '';

		const [roundsRes, walletsRes] = await Promise.all([
			getGroupFundRoundProposals(groupId),
			getMyWallets()
		]);

		if (!isSuccess(roundsRes)) {
			fundRoundsError = roundsRes.message || 'No se pudieron cargar las rondas de fondeo.';
			loadingFundRounds = false;
			return;
		}

		if (isSuccess(walletsRes)) {
			userWallets = walletsRes.body.flatMap((group) => group.currencies);
		}

		// Traemos los totales aportados por cada ronda en paralelo
		const statuses = await Promise.all(
			roundsRes.body.map((round) => getFundRoundProposal(round.proposal.id))
		);

		fundRounds = statuses
			.filter(isSuccess)
			.map((res) => res.body)
			.sort(
				(a, b) =>
					new Date(b.fund_round.proposal.created_at).getTime() -
					new Date(a.fund_round.proposal.created_at).getTime()
			);

		// Traemos mi aporte actual para cada ronda activa (el endpoint solo responde en Approved)
		const approved = fundRounds.filter((s) => s.fund_round.proposal.status === 'Approved');
		const contribResponses = await Promise.all(
			approved.map((s) => getMyFundRoundContribution(s.fund_round.proposal.id))
		);

		const nextContributions: Record<string, string> = {};
		approved.forEach((s, i) => {
			const res = contribResponses[i];
			if (isSuccess(res)) {
				nextContributions[s.fund_round.proposal.id] = res.body.amount;
			}
		});
		myContributions = nextContributions;

		loadingFundRounds = false;
	}

	async function loadExpensesData() {
		loadingExpenses = true;
		expensesError = '';
		try {
			const res = await getExpenses(groupId);
			if (!isSuccess(res)) {
				expensesError = res.message || 'No se pudieron cargar los gastos.';
				return;
			}
			expenses = res.body;
		} finally {
			loadingExpenses = false;
		}
	}

	function toggleFundRoundAccordion(fundRoundId: string) {
		if (expandedFundRoundId === fundRoundId) {
			expandedFundRoundId = null;
		} else {
			expandedFundRoundId = fundRoundId;
		}
		selectedContribWalletId = '';
		contribError = '';
	}

	async function handleContribute(status: FundRoundStatusResponse) {
		if (!selectedContribWalletId) return;

		const proposalId = status.fund_round.proposal.id;

		contribLoading = true;
		contribError = '';

		// Le preguntamos al backend (1) el monto EXACTO que falta y (2) si este usuario
		// es el último miembro que aún no aportó. Si es el último, manda el remaining
		// exacto para cerrar la ronda sin dejar centavos colgados por redondeos previos,
		// aunque termine aportando un poquito más que los demás.
		const remainingRes = await getFundRoundRemaining(proposalId);
		if (!isSuccess(remainingRes)) {
			contribLoading = false;
			contribError = remainingRes.message || 'No se pudo calcular el monto a aportar.';
			return;
		}

		const recommended = recommendedAmount(status.target_amount);
		const { remaining: remainingStr, is_last_contributor } = remainingRes.body;

		const amount = is_last_contributor ? remainingStr : formatAmount(recommended);

		const res = await contributeFundRound(proposalId, {
			amount,
			sender_wallet_id: selectedContribWalletId
		});

		contribLoading = false;

		if (!isSuccess(res)) {
			contribError = res.message || 'Error al aportar a la ronda de fondeo.';
			return;
		}

		expandedFundRoundId = null;
		selectedContribWalletId = '';
		await loadFundRoundsData();
	}

	function openCancelFundRoundModal(fundRoundId: string) {
		fundRoundToCancel = fundRoundId;
		cancelFundRoundError = '';
		showCancelFundRoundModal = true;
	}

	async function handleCancelFundRound() {
		if (!fundRoundToCancel) return;

		cancelFundRoundLoading = true;
		cancelFundRoundError = '';

		const res = await cancelFundRoundProposal(fundRoundToCancel);

		cancelFundRoundLoading = false;

		if (!isSuccess(res)) {
			cancelFundRoundError = res.message || 'Error al cancelar la ronda de fondeo.';
			return;
		}

		showCancelFundRoundModal = false;
		fundRoundToCancel = '';
		await loadFundRoundsData();
	}

	// Recargamos las wallets del grupo cada vez que se entra a la pestaña "Billeteras"
	$effect(() => {
		if (activeTab === 'wallets') {
			loadWalletsData();
		}
	});

	// Recargamos las rondas de fondeo cada vez que se entra a la pestaña "Rondas de Fondeo"
	$effect(() => {
		if (activeTab === 'fund_rounds') {
			loadFundRoundsData();
		}
	});

	$effect(() => {
		if (activeTab === 'expenses') {
			loadExpensesData();
		}
	});

	// Transacciones y gastos al abrir la pestaña Balances
	$effect(() => {
		if (activeTab === 'balances') {
			loadCoreBalances();
			loadBalancesMovimientos();
		}
	});

	loadGroupData();
	loadMembersData();
	loadWalletsData();
	loadExpensesData();
	loadCoreBalances();
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
				{#each [{ key: 'general', label: 'General' }, { key: 'wallets', label: 'Billeteras' }, { key: 'fund_rounds', label: 'Rondas de Fondeo' }, { key: 'balances', label: 'Balances' }, { key: 'expenses', label: 'División de Gastos' }] as const as tab}
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
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-8 duration-300">
						<!-- Sección: Miembros -->
						<div class="space-y-4">
							<div class="flex items-center justify-between gap-3">
								<div class="flex items-center gap-2">
									<h2 class="text-sm font-medium text-black">Miembros</h2>
									{#if !loadingMembers && members.length > 0}
										<span
											class="rounded-full bg-gray-100 px-2 py-0.5 text-[11px] font-semibold text-gray-600"
										>
											{members.length}
										</span>
									{/if}
								</div>
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
								<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
									{#each members as member (member.user_id)}
										{@const isAdmin = member.role === 'Admin'}
										{@const initials =
											member.name
												.trim()
												.split(/\s+/)
												.slice(0, 2)
												.map((p) => p[0]?.toUpperCase() ?? '')
												.join('') || '?'}
										<a
											href={`/users/${member.user_id}`}
											class="group flex items-center gap-3 rounded-xl border border-gray-200 bg-white px-3 py-2.5 transition hover:border-gray-300 hover:shadow-sm"
										>
											<div
												class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isAdmin
													? 'border-gray-900 bg-gray-900 text-white'
													: 'border-gray-200 bg-gray-50 text-gray-700'} transition group-hover:border-black"
											>
												{initials}
											</div>
											<div class="min-w-0 flex-1 space-y-0.5">
												<p class="truncate text-sm font-medium text-black group-hover:underline">
													{member.name}
												</p>
												<p class="text-[11px] text-gray-400">
													{isAdmin ? 'Admin' : 'Miembro'}
												</p>
											</div>
										</a>
									{/each}

									<!-- Card para invitar más miembros -->
									<button
										type="button"
										onclick={() => (showNewMemberModal = true)}
										class="group flex items-center gap-3 rounded-xl border border-dashed border-gray-300 bg-white px-3 py-2.5 text-left transition hover:border-black hover:bg-gray-50"
									>
										<div
											class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-dashed border-gray-300 bg-white text-gray-400 transition group-hover:border-black group-hover:text-black"
										>
											<Plus class="h-4 w-4" />
										</div>
										<div class="min-w-0 space-y-0.5">
											<p class="truncate text-sm font-medium text-gray-500 group-hover:text-black">
												Invitar miembro
											</p>
											<p class="text-[11px] text-gray-400">Sumá a alguien al grupo</p>
										</div>
									</button>
								</div>
							{:else}
								<div
									class="rounded-xl border border-dashed border-gray-300 bg-white p-8 text-center"
								>
									<Users class="mx-auto mb-3 h-8 w-8 text-gray-400" />
									<p class="text-sm font-medium text-black">Sin miembros aún</p>
									<p class="mb-4 text-sm text-gray-500">
										Invitá a alguien para empezar a mover plata en grupo.
									</p>
									<Button
										label="Invitar primer miembro"
										variant="secondary"
										onclick={() => (showNewMemberModal = true)}
									>
										{#snippet icon()}
											<Plus class="h-4 w-4" />
										{/snippet}
									</Button>
								</div>
							{/if}
						</div>

						{#if coreBalancesError && !loadingCoreBalances}
							<div
								class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
							>
								<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
								<span>{coreBalancesError}</span>
							</div>
						{/if}

						<!-- BALANCE DASHBOARD -->
						{#if !loadingMembers && !loadingCoreBalances && memberBalances.length > 0 && !coreBalancesError}
							{@const maxAbs = Math.max(1, ...memberBalances.map((m) => Math.abs(m.balance)))}
							{@const topMovers = sortedMemberBalances.slice(0, 5)}

							<div class="space-y-5">
								<!-- Header del dashboard -->
								<div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
									<div class="space-y-1">
										<h2 class="text-sm font-medium text-black">Balance del grupo</h2>
										<p class="text-xs text-gray-500">
											Resumen de cuánto debe o tiene a favor cada miembro (motor contable).
										</p>
									</div>
								</div>

								<!-- Balance actual del grupo -->
								<div
									class="flex items-center justify-between gap-4 rounded-xl border border-gray-200 bg-white p-5 transition hover:border-gray-300 hover:shadow-sm"
								>
									<div class="space-y-1">
										<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
											Billeteras del grupo
										</p>
										<p class="flex items-baseline gap-2">
											<span class="text-3xl font-bold tracking-tight text-black">
												${formatAmount(groupWalletsBalance)}
											</span>
										</p>
										<p class="text-xs text-gray-500">Suma de balances en las wallets del grupo</p>
										<p class="text-xs text-gray-500">
											Balance según movimientos (core):
											<span class="font-medium text-black">${formatAmount(coreGroupBalance)}</span>
										</p>
									</div>
									<div
										class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-600"
									>
										<Scale class="h-5 w-5" />
									</div>
								</div>

								<!-- Top movers -->
								<div class="space-y-3">
									<div class="flex items-center justify-between">
										<h3 class="text-xs font-medium tracking-wider text-gray-500 uppercase">
											{memberBalances.length > 5 ? 'Top movimientos' : 'Detalle por miembro'}
										</h3>
										<span class="text-[11px] text-gray-400">Ordenado por balance</span>
									</div>

									<div
										class="divide-y divide-gray-100 overflow-hidden rounded-xl border border-gray-200 bg-white"
									>
										{#each topMovers as mb (mb.user.user_id)}
											{@const isCredit = mb.balance > 0.01}
											{@const isDebt = mb.balance < -0.01}
											{@const pct = Math.min(100, (Math.abs(mb.balance) / maxAbs) * 100)}
											<a
												href={`/users/${mb.user.user_id}`}
												class="flex items-center gap-4 px-4 py-3 transition hover:bg-gray-50"
											>
												<!-- Avatar con iniciales -->
												<div
													class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isCredit
														? 'border-emerald-200 bg-emerald-50 text-emerald-700'
														: isDebt
															? 'border-rose-200 bg-rose-50 text-rose-700'
															: 'border-gray-200 bg-gray-50 text-gray-500'}"
												>
													{mb.user.name
														.trim()
														.split(/\s+/)
														.slice(0, 2)
														.map((p) => p[0]?.toUpperCase() ?? '')
														.join('') || '?'}
												</div>

												<!-- Nombre + barra centrada en cero -->
												<div class="min-w-0 flex-1 space-y-1.5">
													<div class="flex items-center gap-2">
														<span class="truncate text-sm font-medium text-black">
															{mb.user.name}
														</span>
														{#if mb.user.role === 'Admin'}
															<span
																class="rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 text-[9px] font-semibold tracking-wider text-gray-500 uppercase"
															>
																Admin
															</span>
														{/if}
													</div>

													<div class="relative h-1.5 w-full rounded-full bg-gray-100">
														<div class="absolute top-0 bottom-0 left-1/2 w-px bg-gray-300"></div>
														{#if isCredit}
															<div
																class="absolute top-0 bottom-0 left-1/2 rounded-r-full bg-linear-to-r from-emerald-400 to-emerald-600 transition-all duration-700"
																style="width: {pct / 2}%"
															></div>
														{:else if isDebt}
															<div
																class="absolute top-0 right-1/2 bottom-0 rounded-l-full bg-linear-to-l from-rose-400 to-rose-600 transition-all duration-700"
																style="width: {pct / 2}%"
															></div>
														{/if}
													</div>
												</div>

												<!-- Monto -->
												<div class="shrink-0 text-right">
													<p
														class="text-sm font-semibold tabular-nums {isCredit
															? 'text-emerald-700'
															: isDebt
																? 'text-rose-700'
																: 'text-gray-500'}"
													>
														{isCredit ? '+' : isDebt ? '-' : ''}${formatAmount(
															Math.abs(mb.balance)
														)}
													</p>
													<p
														class="text-[10px] font-medium tracking-wider uppercase {isCredit
															? 'text-emerald-600/70'
															: isDebt
																? 'text-rose-600/70'
																: 'text-gray-400'}"
													>
														{isCredit ? 'a favor' : isDebt ? 'debe' : 'saldado'}
													</p>
												</div>
											</a>
										{/each}
									</div>
								</div>

								<!-- Call-to-action hacia la tab detallada -->
								<button
									type="button"
									onclick={() => (activeTab = 'balances')}
									class="group flex w-full items-center justify-between rounded-xl border border-gray-200 bg-white px-4 py-3 text-left transition hover:border-gray-300 hover:shadow-sm"
								>
									<div class="min-w-0 space-y-0.5">
										<p class="flex items-center gap-2 text-sm font-medium text-black">
											<Scale class="h-4 w-4 text-gray-500" />
											Ver detalle de deudas
										</p>
										<p class="text-[11px] text-gray-500">
											{#if settlements.length > 0}
												{settlements.length}
												{settlements.length === 1
													? 'transferencia sugerida'
													: 'transferencias sugeridas'} para saldar todo
											{:else}
												Nadie le debe nada a nadie. ¡Todo saldado!
											{/if}
										</p>
									</div>
									<ArrowRight
										class="h-4 w-4 shrink-0 text-gray-400 transition group-hover:translate-x-0.5 group-hover:text-black"
									/>
								</button>
							</div>
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
						<div class="flex items-start justify-between gap-4">
							<div class="space-y-1">
								<h2 class="text-sm font-medium text-black">Rondas de Fondeo</h2>
								<p class="text-xs text-gray-500">
									Aportes colectivos para fondear una billetera del grupo.
								</p>
							</div>
							<Button
								label="Nueva Ronda"
								variant="primary"
								onclick={() => (showCreateFundRoundModal = true)}
							>
								{#snippet icon()}
									<Plus class="h-4 w-4" />
								{/snippet}
							</Button>
						</div>

						{#if fundRoundsError}
							<div class="rounded-md bg-red-50 p-3 text-sm text-red-600">
								{fundRoundsError}
							</div>
						{/if}

						{#if loadingFundRounds}
							<div
								class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
							></div>
						{:else if fundRounds.length === 0}
							<div class="rounded-lg border border-dashed border-gray-300 p-8 text-center">
								<HandCoins class="mx-auto mb-3 h-8 w-8 text-gray-400" />
								<p class="text-sm font-medium text-black">Sin rondas de fondeo</p>
								<p class="mb-4 text-sm text-gray-500">Este grupo no tiene rondas de fondeo aún.</p>
								<Button
									label="Crear primera ronda"
									variant="secondary"
									onclick={() => (showCreateFundRoundModal = true)}
								/>
							</div>
						{:else}
							{#snippet fundRoundCard(status: FundRoundStatusResponse)}
								{@const proposalId = status.fund_round.proposal.id}
								{@const proposalStatus = status.fund_round.proposal.status}
								{@const target = Number(status.target_amount)}
								{@const raised = Number(status.total_contributed)}
								{@const progress =
									target > 0 ? Math.min(100, Math.round((raised / target) * 100)) : 0}
								{@const remaining = Math.max(0, target - raised)}
								{@const ticker =
									wallets.find(
										(w) => w.currency_id === status.fund_round.fund_round_proposal.currency_id
									)?.currency_ticker ?? 'USDC'}
								{@const compatibleWallets = userWallets.filter(
									(w) => w.currency_id === status.fund_round.fund_round_proposal.currency_id
								)}
								{@const recommended = recommendedAmount(status.target_amount)}
								{@const myContribution = Number(myContributions[proposalId] ?? '0')}
								{@const hasContributed = myContribution > 0}
								{@const myRemaining = Math.max(0, recommended - myContribution)}
								{@const canContribute =
									proposalStatus === 'Approved' && !status.is_completed && !hasContributed}
								{@const isCreator =
									!!currentUserId && status.fund_round.proposal.created_by === currentUserId}
								{@const canCancel =
									isCreator && proposalStatus === 'Approved' && !status.is_completed}
								{@const statusDisplay = getProposalStatusDisplay(proposalStatus)}
								{@const isOpen = expandedFundRoundId === proposalId}

								<div
									class="group rounded-xl border border-gray-200 bg-white transition hover:border-gray-300 hover:shadow-sm"
								>
									<div class="space-y-4 p-5">
										<div class="flex items-start justify-between gap-3">
											<div class="flex items-start gap-3">
												<div
													class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-700"
												>
													<HandCoins class="h-5 w-5" />
												</div>
												<div class="space-y-1">
													<div class="flex items-baseline gap-1.5">
														<span class="text-2xl font-bold tracking-tight text-black"
															>${target}</span
														>
														<span
															class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
														>
															{ticker}
														</span>
													</div>
													<p
														class="flex items-center gap-1 text-[11px] font-medium tracking-wide text-gray-400"
													>
														<Calendar class="h-3 w-3" />
														{new Date(status.fund_round.proposal.created_at).toLocaleDateString(
															'es-AR',
															{
																day: '2-digit',
																month: 'short',
																year: 'numeric'
															}
														)}
													</p>
												</div>
											</div>

											<div class="flex shrink-0 items-center gap-1.5">
												<span
													class="rounded-full border px-2.5 py-1 text-xs font-medium {statusDisplay.classes}"
												>
													{statusDisplay.label}
												</span>

												{#if canCancel}
													<div class="group/cancel relative flex">
														<button
															type="button"
															onclick={() => openCancelFundRoundModal(proposalId)}
															aria-label="Cancelar ronda"
															class="flex h-7 w-7 items-center justify-center rounded-full border border-gray-200 bg-white text-gray-500 transition hover:border-red-200 hover:bg-red-50 hover:text-red-600 active:scale-95"
														>
															<Ban class="h-3.5 w-3.5" />
														</button>

														<span
															class="pointer-events-none invisible absolute top-full right-0 z-50 mt-2 rounded-md bg-[#222327] px-2.5 py-1 text-xs font-medium whitespace-nowrap text-white opacity-0 shadow-sm transition-all duration-200 group-hover/cancel:visible group-hover/cancel:opacity-100"
														>
															Cancelar ronda
														</span>
													</div>
												{/if}
											</div>
										</div>

										<div class="space-y-2">
											<div class="h-2 w-full overflow-hidden rounded-full bg-gray-100">
												<div
													class="h-full rounded-full bg-linear-to-r from-gray-800 to-black transition-all duration-700"
													style="width: {progress}%"
												></div>
											</div>

											<div class="flex items-center justify-between text-xs">
												<span class="font-medium text-gray-700">
													${formatAmount(raised)}
													<span class="text-gray-400">/ ${formatAmount(target)} {ticker}</span>
												</span>
												<span class="text-gray-500">
													<span class="font-medium text-gray-700">{progress}%</span>
													{#if remaining > 0}
														— faltan ${formatAmount(remaining)} {ticker}
													{:else}
														— completado
													{/if}
												</span>
											</div>
										</div>

										{#if proposalStatus === 'Approved'}
											<div
												class="flex flex-col gap-3 rounded-lg border border-gray-100 bg-gray-50/70 px-3 py-2.5 text-xs sm:flex-row sm:items-center sm:justify-between"
											>
												<div class="flex items-start gap-2">
													<Target class="mt-0.5 h-3.5 w-3.5 shrink-0 text-gray-400" />
													<div class="space-y-0.5">
														<p class="text-gray-500">
															Te toca aportar
															<span class="font-semibold text-black"
																>${formatAmount(recommended)} {ticker}</span
															>
														</p>
														{#if members.length > 0}
															<p class="flex items-center gap-1 text-[11px] text-gray-400">
																<Users class="h-3 w-3" />
																${formatAmount(target)} entre {members.length}
																{members.length === 1 ? 'miembro' : 'miembros'}
															</p>
														{/if}
													</div>
												</div>

												<div class="flex items-center gap-4 self-stretch sm:self-auto">
													{#if hasContributed}
														<span
															class="inline-flex flex-1 items-center justify-center gap-1 rounded-md border border-green-200 bg-green-50 px-2 py-1 font-medium text-green-700 sm:flex-none"
														>
															<CircleCheckBig class="h-3 w-3" />
															Aportaste ${formatAmount(myContribution)}
															{ticker}
														</span>
													{:else}
														<span
															class="flex-1 text-right font-medium text-gray-700 sm:flex-none sm:text-left"
														>
															Te falta ${formatAmount(myRemaining)}
															{ticker}
														</span>
													{/if}

													{#if canContribute}
														<button
															onclick={() => toggleFundRoundAccordion(proposalId)}
															class="flex shrink-0 items-center gap-1.5 rounded-md bg-black px-3 py-1.5 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95"
														>
															Aportar
															<ChevronDown
																class={[
																	'h-3.5 w-3.5 transition-transform duration-200',
																	isOpen ? 'rotate-180' : ''
																].join(' ')}
															/>
														</button>
													{/if}
												</div>
											</div>
										{/if}
									</div>

									{#if isOpen && canContribute}
										<div
											transition:slide={{ duration: 220 }}
											class="space-y-4 overflow-hidden rounded-b-xl border-t border-gray-100 bg-gray-50/60 px-5 py-4"
										>
											<div class="space-y-1">
												<p class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
													Aportar a esta ronda
												</p>
												<p class="text-xs text-gray-500">
													Elegí desde qué wallet personal salen los fondos.
												</p>
											</div>

											{#if compatibleWallets.length === 0}
												<div
													class="flex items-start gap-2 rounded-md border border-gray-200 bg-white p-3 text-xs text-gray-500"
												>
													<Wallet class="mt-0.5 h-3.5 w-3.5 shrink-0 text-gray-400" />
													<span>No tenés wallets compatibles con la moneda de esta ronda.</span>
												</div>
											{:else}
												<div class="space-y-3">
													<select
														bind:value={selectedContribWalletId}
														class="w-full rounded-md border border-gray-200 bg-white px-3 py-2.5 text-sm text-black transition outline-none focus:border-black focus:ring-1 focus:ring-black"
													>
														<option value="" disabled>Seleccionar wallet personal...</option>
														{#each compatibleWallets as wallet (wallet.wallet_id)}
															<option value={wallet.wallet_id}>
																{shortenAddress(wallet.address)} — ${wallet.balance}
																{wallet.ticker}
															</option>
														{/each}
													</select>

													<div
														class="flex items-center justify-between rounded-md border border-gray-200 bg-white px-3 py-2.5 text-sm"
													>
														<span class="text-gray-500">Monto a aportar</span>
														<span class="flex items-baseline gap-1.5">
															<span class="font-semibold text-black">
																${formatAmount(recommended)}
															</span>
															<span
																class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
															>
																{ticker}
															</span>
														</span>
													</div>

													{#if contribError}
														<p class="text-xs text-red-500">{contribError}</p>
													{/if}

													<div class="flex items-center justify-end gap-2">
														<button
															onclick={() => toggleFundRoundAccordion(proposalId)}
															class="rounded-md px-3.5 py-2 text-xs font-medium text-gray-500 transition hover:text-black"
															disabled={contribLoading}
														>
															Cancelar
														</button>
														<button
															onclick={() => handleContribute(status)}
															class="rounded-md bg-black px-4 py-2 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95 disabled:opacity-40"
															disabled={!selectedContribWalletId || contribLoading}
														>
															{contribLoading
																? 'Enviando...'
																: `Confirmar aporte de $${formatAmount(recommended)} ${ticker}`}
														</button>
													</div>
												</div>
											{/if}
										</div>
									{/if}
								</div>
							{/snippet}

							<div class="space-y-3 pt-2">
								{#if activeFundRounds.length > 0}
									{#each activeFundRounds as status (status.fund_round.proposal.id)}
										{@render fundRoundCard(status)}
									{/each}
								{:else}
									<div
										class="flex flex-col items-center gap-1 rounded-xl border border-dashed border-gray-300 p-6 text-center"
									>
										<HandCoins class="h-6 w-6 text-gray-400" />
										<p class="text-sm font-medium text-black">No hay rondas activas</p>
										<p class="text-xs text-gray-500">
											Todas las rondas están finalizadas o canceladas.
										</p>
									</div>
								{/if}

								{#if pastFundRounds.length > 0}
									<div class="flex items-center gap-3 pt-4 pb-1">
										<div class="h-px flex-1 bg-gray-200"></div>
										<button
											type="button"
											onclick={() => (showPastFundRounds = !showPastFundRounds)}
											class="inline-flex items-center gap-1.5 rounded-full border border-gray-200 bg-white px-3 py-1 text-[11px] font-medium text-gray-600 transition hover:border-gray-300 hover:text-black"
										>
											{showPastFundRounds ? 'Ocultar' : 'Ver'} rondas pasadas
											<span
												class="rounded-full bg-gray-100 px-1.5 text-[10px] font-semibold text-gray-600"
											>
												{pastFundRounds.length}
											</span>
											<ChevronDown
												class={[
													'h-3 w-3 transition-transform duration-200',
													showPastFundRounds ? 'rotate-180' : ''
												].join(' ')}
											/>
										</button>
										<div class="h-px flex-1 bg-gray-200"></div>
									</div>

									{#if showPastFundRounds}
										<div transition:slide={{ duration: 220 }} class="space-y-3">
											{#each pastFundRounds as status (status.fund_round.proposal.id)}
												{@render fundRoundCard(status)}
											{/each}
											{#if sortedGroupTransactions.length > 3}
												<button
													type="button"
													class="text-xs font-medium text-gray-600 underline-offset-2 transition hover:text-black hover:underline"
													onclick={() => (showAllTransactions = !showAllTransactions)}
												>
													{showAllTransactions
														? 'Ver menos'
														: `Ver todo (${sortedGroupTransactions.length})`}
												</button>
											{/if}
										</div>
									{/if}
								{/if}
							</div>
						{/if}
					</div>
				{/if}

				<!-- BALANCES TAB -->
				{#if activeTab === 'balances'}
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-8 duration-300">
						<div class="space-y-1">
							<h2 class="text-sm font-medium text-black">Balances del grupo</h2>
							<p class="text-xs text-gray-500">
								Balances por integrante, sugerencias para saldar, transacciones y gastos cargados.
							</p>
						</div>

						{#if loadingMembers || loadingCoreBalances}
							<div
								class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
							></div>
						{:else if coreBalancesError}
							<div
								class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
							>
								<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
								<div class="space-y-2">
									<p>{coreBalancesError}</p>
									<button
										type="button"
										class="text-xs font-medium text-rose-700 underline-offset-2 transition hover:underline"
										onclick={loadCoreBalances}
									>
										Reintentar balances
									</button>
								</div>
							</div>
						{:else if memberBalances.length === 0}
							<div class="rounded-lg border border-dashed border-gray-300 p-8 text-center">
								<Scale class="mx-auto mb-3 h-8 w-8 text-gray-400" />
								<p class="text-sm font-medium text-black">Sin balances para mostrar</p>
								<p class="text-sm text-gray-500">
									Todavía no hay integrantes o movimientos que generen saldos en el core.
								</p>
							</div>
						{:else}
							<div
								class="flex flex-wrap items-end justify-between gap-3 rounded-xl border border-gray-200 bg-white px-4 py-3"
							>
								<div class="space-y-0.5">
									<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
										Balance grupal (core)
									</p>
									<p class="text-xl font-bold text-black tabular-nums">
										${formatAmount(coreGroupBalance)}
									</p>
								</div>
							</div>

							<!-- Lista de miembros con balance -->
							<div class="space-y-2">
								<div class="flex flex-wrap items-center gap-2">
									<h3 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
										Por integrante
									</h3>
									<span
										class="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-slate-700 uppercase"
									>
										Balance
									</span>
								</div>
								<div
									class="divide-y divide-gray-100 overflow-hidden rounded-xl border border-gray-200 bg-white"
								>
									{#each sortedMemberBalances as mb (mb.user.user_id)}
										{@const isPositive = mb.balance > 0.01}
										{@const isNegative = mb.balance < -0.01}
										<div class="flex items-center justify-between gap-3 px-4 py-3">
											<a
												href={`/users/${mb.user.user_id}`}
												class="group flex min-w-0 items-center gap-3"
											>
												<div
													class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-700 transition group-hover:border-gray-300 group-hover:text-black"
												>
													<CircleUser
														class="h-5 w-5"
														strokeWidth={mb.user.role === 'Admin' ? 2.5 : 2}
													/>
												</div>
												<div class="min-w-0 space-y-0.5">
													<p class="truncate text-sm font-medium text-black group-hover:underline">
														{mb.user.name}
													</p>
													<p class="text-[11px] text-gray-400">
														{mb.user.role === 'Admin' ? 'Admin' : 'Miembro'}
													</p>
												</div>
											</a>

											<div class="flex shrink-0 items-center gap-2">
												{#if isPositive}
													<span class="hidden text-[11px] text-gray-500 sm:inline">le deben</span>
													<span
														class="inline-flex items-center gap-1 rounded-md border border-green-200 bg-green-50 px-2 py-1 text-xs font-semibold text-green-700"
													>
														<TrendingUp class="h-3 w-3" />
														+${formatAmount(mb.balance)}
													</span>
												{:else if isNegative}
													<span class="hidden text-[11px] text-gray-500 sm:inline">debe</span>
													<span
														class="inline-flex items-center gap-1 rounded-md border border-red-200 bg-red-50 px-2 py-1 text-xs font-semibold text-red-600"
													>
														<TrendingDown class="h-3 w-3" />
														-${formatAmount(Math.abs(mb.balance))}
													</span>
												{:else}
													<span
														class="inline-flex items-center gap-1 rounded-md border border-gray-200 bg-gray-50 px-2 py-1 text-xs font-semibold text-gray-500"
													>
														<CircleCheckBig class="h-3 w-3" />
														Saldado
													</span>
												{/if}
											</div>
										</div>
									{/each}
								</div>
							</div>

							<!-- Sugerencias para saldar -->
							<div class="space-y-2">
								<div class="flex flex-wrap items-center gap-2">
									<h3 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
										Sugerencias para saldar
									</h3>
									<span
										class="rounded-full bg-amber-100 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-amber-900 uppercase"
									>
										Sugerencia
									</span>
								</div>

								{#if settlements.length === 0}
									<div
										class="flex items-center gap-3 rounded-xl border border-green-200 bg-green-50/60 p-4"
									>
										<div
											class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-green-100 text-green-700"
										>
											<CircleCheckBig class="h-5 w-5" />
										</div>
										<div class="space-y-0.5">
											<p class="text-sm font-medium text-black">Todo al día</p>
											<p class="text-xs text-gray-500">Nadie le debe nada a nadie en este grupo.</p>
										</div>
									</div>
								{:else}
									<div class="space-y-2">
										{#each settlements as s, idx (idx)}
											<div
												class="flex items-center justify-between gap-3 rounded-xl border border-gray-200 bg-white p-3"
											>
												<div class="flex min-w-0 flex-1 items-center gap-2">
													<div class="flex min-w-0 items-center gap-2">
														<div
															class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-red-100 bg-red-50 text-red-500"
														>
															<CircleUser class="h-4 w-4" />
														</div>
														<span class="truncate text-sm font-medium text-black">
															{s.from.name}
														</span>
													</div>

													<ArrowRight class="h-4 w-4 shrink-0 text-gray-300" />

													<div class="flex min-w-0 items-center gap-2">
														<div
															class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-green-100 bg-green-50 text-green-600"
														>
															<CircleUser class="h-4 w-4" />
														</div>
														<span class="truncate text-sm font-medium text-black">
															{s.to.name}
														</span>
													</div>
												</div>

												<span class="shrink-0 text-sm font-semibold text-black tabular-nums">
													${formatAmount(s.amount)}
												</span>
											</div>
										{/each}
									</div>
								{/if}
							</div>
						{/if}

						<div class="space-y-3 border-t border-gray-200 pt-6">
							<div class="flex flex-wrap items-center gap-2">
								<h3 class="text-sm font-medium text-black">Actividad y registros</h3>
								<span
									class="rounded-full bg-gray-100 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-gray-700 uppercase"
								>
									Listados
								</span>
							</div>

							{#if loadingBalancesDetail}
								<div
									class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
								></div>
							{:else}
								<div class="space-y-6">
									<div class="space-y-2">
										<div class="flex items-center gap-2">
											<h4 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
												Transacciones
											</h4>
											<span
												class="rounded-full bg-sky-100 px-2 py-0.5 text-[10px] font-semibold text-sky-900 uppercase"
											>
												Transacción
											</span>
										</div>
										{#if transactionsDetailError}
											<p class="text-xs text-rose-600">{transactionsDetailError}</p>
										{:else if sortedGroupTransactions.length === 0}
											<p class="text-xs text-gray-500">No hay transacciones en este grupo.</p>
										{:else}
											<div class="space-y-2">
												{#each visibleGroupTransactions as tx (tx.id)}
													<div
														class="flex flex-col gap-2 rounded-xl border border-gray-200 bg-white px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
													>
														<div class="min-w-0 space-y-1">
															<div class="flex flex-wrap items-center gap-2">
																<span
																	class="rounded-full bg-sky-100 px-2 py-0.5 text-[10px] font-semibold text-sky-900 uppercase"
																>
																	Transacción
																</span>
																<span class="text-xs font-medium text-gray-800">
																	{formatTxType(tx.tx_type)}
																</span>
																<span class="text-[11px] text-gray-400">
																	{formatDateTimeShort(tx.created_at)}
																</span>
															</div>
															{#if tx.description}
																<p class="truncate text-xs text-gray-600">{tx.description}</p>
															{/if}
															<p class="text-[11px] text-gray-400">
																Usuario: <span class="font-mono text-gray-600"
																	>{tx.user_id.slice(0, 8)}…</span
																>
															</p>
														</div>
														<p class="shrink-0 text-sm font-semibold text-black tabular-nums">
															${parseBalanceValue(tx.amount).toFixed(2)}
														</p>
													</div>
												{/each}
											</div>
										{/if}
									</div>

									<div class="space-y-2">
										<div class="flex items-center gap-2">
											<h4 class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
												Gastos
											</h4>
											<span
												class="rounded-full bg-violet-100 px-2 py-0.5 text-[10px] font-semibold text-violet-900 uppercase"
											>
												Gasto
											</span>
										</div>
										{#if expensesDetailError}
											<p class="text-xs text-rose-600">{expensesDetailError}</p>
										{:else if sortedGroupExpenses.length === 0}
											<p class="text-xs text-gray-500">
												No hay gastos registrados para este grupo.
											</p>
										{:else}
											<div class="space-y-2">
												{#each visibleGroupExpenses as ex (ex.expense_id)}
													<div
														class="flex flex-col gap-2 rounded-xl border border-gray-200 bg-white px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
													>
														<div class="min-w-0 space-y-1">
															<div class="flex flex-wrap items-center gap-2">
																<span
																	class="rounded-full bg-violet-100 px-2 py-0.5 text-[10px] font-semibold text-violet-900 uppercase"
																>
																	Gasto
																</span>
																<span class="text-[11px] text-gray-400">
																	{formatDateTimeShort(ex.created_at)}
																</span>
																<span
																	class="rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 text-[10px] font-medium text-gray-600 uppercase"
																>
																	{ex.status}
																</span>
															</div>
															{#if ex.description}
																<p class="text-xs text-gray-800">{ex.description}</p>
															{:else}
																<p class="text-xs text-gray-400">Sin descripción</p>
															{/if}
															<p class="text-[11px] text-gray-400">
																Cargado por:
																<span class="font-mono text-gray-600"
																	>{ex.user_id.slice(0, 8)}…</span
																>
															</p>
														</div>
														<p class="shrink-0 text-sm font-semibold text-black tabular-nums">
															${parseBalanceValue(ex.amount).toFixed(2)}
														</p>
													</div>
												{/each}
											</div>
										{/if}
									</div>
								</div>
							{/if}
						</div>
					</div>
				{/if}

				{#if activeTab === 'expenses'}
					<div class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
						<div class="flex items-center justify-between gap-3">
							<div>
								<h3 class="text-sm font-semibold text-black">Ultimos Gastos</h3>
								<p class="text-xs text-gray-500">Se muestran los gastos mas recientes del grupo.</p>
							</div>
							<Button label="Agregar Gasto" onclick={() => (showCreateExpenseModal = true)} />
						</div>

						{#if loadingExpenses}
							<div
								class="flex items-center justify-center rounded-xl border border-gray-200 bg-white p-6"
							>
								<div
									class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
								></div>
							</div>
						{:else if expensesError}
							<div class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-600">
								{expensesError}
							</div>
						{:else if recentExpenses.length === 0}
							<div
								class="rounded-xl border border-dashed border-gray-300 bg-white p-6 text-center text-sm text-gray-500"
							>
								No hay expenses todavia. Crea la primera desde el boton de arriba.
							</div>
						{:else}
							<div class="space-y-2">
								{#each recentExpenses as expense (expense.expense_id)}
									<div class="rounded-xl border border-gray-200 bg-white p-4">
										<div class="flex items-start justify-between gap-3">
											<div class="space-y-1">
												<p class="text-sm font-semibold text-black">
													{expense.description || 'Sin descripcion'}
												</p>
												<p class="text-xs text-gray-500">
													Creado por {getMemberName(expense.user_id)}
												</p>
											</div>
											<div class="text-right">
												<p class="text-sm font-semibold text-black">{expense.amount}</p>
												<p class="text-xs text-gray-500">
													{formatExpenseDate(expense.created_at)}
												</p>
											</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
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
			onsuccess={loadWalletsData}
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
		<CreateFundRound
			open={showCreateFundRoundModal}
			group_id={groupData.id}
			onclose={() => (showCreateFundRoundModal = false)}
			onsuccess={loadFundRoundsData}
		/>
		<CreateExpenseModal
			open={showCreateExpenseModal}
			group_id={groupData.id}
			{members}
			onclose={() => (showCreateExpenseModal = false)}
			onsuccess={loadExpensesData}
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
		<Confirm
			open={showCancelFundRoundModal}
			title="Cancelar ronda de fondeo"
			description="Los aportes realizados no se devuelven automáticamente."
			message="¿Seguro que querés cancelar esta ronda de fondeo?"
			onclose={() => {
				showCancelFundRoundModal = false;
				cancelFundRoundError = '';
				fundRoundToCancel = '';
			}}
			onconfirm={handleCancelFundRound}
			loading={cancelFundRoundLoading}
			error={cancelFundRoundError}
		/>
	{/if}
</div>
