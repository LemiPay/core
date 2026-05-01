import { getGroup, getGroupMembers, getGroupWallets } from '$lib/api/endpoints/groups';
import {
	getGroupFundRoundProposals,
	getFundRoundProposal,
	getMyFundRoundContribution,
	getFundRoundRemaining,
	contributeFundRound
} from '$lib/api/endpoints/fund_rounds';
import { getGroupBalances } from '$lib/api/endpoints/core';
import { getGroupExpenses, getExpenses } from '$lib/api/endpoints/expenses';
import { listGroupTransactions } from '$lib/api/endpoints/transactions';
import { getMyWallets } from '$lib/api/endpoints/wallets';
import { authStore } from '$lib/stores/auth';
import { isSuccess } from '$lib/types/client.types';

// Types
import type { Group, GroupWallet } from '$lib/types/endpoints/groups.types';
import type { UserBadge } from '$lib/types/endpoints/auth.types';
import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';
import type { GroupBalancesResponse } from '$lib/types/endpoints/core.types';
import type { Transaction } from '$lib/types/endpoints/transactions.types';
import type { Expense } from '$lib/types/endpoints/expenses.types';
import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';
import { formatAmount, parseBalanceValue } from '$lib/utils/format_utils';

// --- CLASE DE ESTADO ---
export class GroupState {
	groupId: string;

	// --- STATES ---
	loading = $state(true);
	loadingMembers = $state(true);
	loadingWallets = $state(true);
	groupExists = $state(true);

	groupData = $state<Group>({} as Group);
	members = $state<UserBadge[]>([]);
	wallets = $state<GroupWallet[]>([]);

	// Fund Rounds State
	fundRounds = $state<FundRoundStatusResponse[]>([]);
	loadingFundRounds = $state(true);
	fundRoundsError = $state('');
	userWallets = $state<WalletCurrency[]>([]);
	myContributions = $state<Record<string, string>>({});
	contribLoading = $state(false);
	contribError = $state('');

	// Expenses State
	expenses = $state<Expense[]>([]);
	loadingExpenses = $state(true);
	expensesError = $state('');

	// Balances State
	coreBalancesData = $state<GroupBalancesResponse | null>(null);
	loadingCoreBalances = $state(true);
	coreBalancesError = $state('');
	groupTransactions = $state<Transaction[]>([]);
	groupExpenses = $state<Expense[]>([]);
	loadingBalancesDetail = $state(false);
	transactionsDetailError = $state('');
	expensesDetailError = $state('');

	constructor(groupId: string) {
		this.groupId = groupId;
	}

	// --- DERIVED / GETTERS ---
	get currentUserId() {
		return authStore.getUserId();
	}

	get groupWalletsBalance() {
		return this.wallets.reduce((acc, wallet) => acc + Number(wallet.balance || 0), 0);
	}

	get recentExpenses() {
		return [...this.expenses]
			.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
			.slice(0, 3);
	}

	get activeFundRounds() {
		return this.fundRounds.filter(
			(r) =>
				r.fund_round.proposal.status === 'Pending' || r.fund_round.proposal.status === 'Approved'
		);
	}

	get pastFundRounds() {
		return this.fundRounds.filter(
			(r) =>
				r.fund_round.proposal.status !== 'Pending' && r.fund_round.proposal.status !== 'Approved'
		);
	}

	get memberBalances() {
		if (!this.coreBalancesData?.balances?.length) return [];
		return this.coreBalancesData.balances.map((b) => {
			const m = this.members.find((x) => x.user_id === b.user_id);
			const user = m ?? { user_id: b.user_id, name: b.user_name, role: 'Miembro' };
			return { user, balance: parseBalanceValue(b.balance) };
		});
	}

	get maxAbsoluteBalance() {
		if (this.memberBalances.length === 0) return 1;
		return Math.max(1, ...this.memberBalances.map((m) => Math.abs(m.balance)));
	}

	get coreGroupBalance() {
		return this.coreBalancesData ? parseBalanceValue(this.coreBalancesData.group_balance) : 0;
	}

	get sortedGroupTransactions() {
		return [...this.groupTransactions].sort(
			(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		);
	}

	get sortedGroupExpenses() {
		return [...this.groupExpenses].sort(
			(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		);
	}

	get sortedMemberBalances() {
		return [...this.memberBalances].sort((a, b) => b.balance - a.balance);
	}

	// Algoritmo Greedy
	get settlements() {
		const creditors = this.memberBalances
			.filter((m) => m.balance > 0.01)
			.map((m) => ({ user: m.user, remaining: m.balance }))
			.sort((a, b) => b.remaining - a.remaining);
		const debtors = this.memberBalances
			.filter((m) => m.balance < -0.01)
			.map((m) => ({ user: m.user, remaining: -m.balance }))
			.sort((a, b) => b.remaining - a.remaining);

		const result = [];
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
	}

	// --- METHODS ---
	getMemberName(userId: string): string {
		const member = this.members.find((item) => item.user_id === userId);
		return member?.name ?? 'Usuario';
	}

	getTickerForCurrency(currencyId: string): string {
		const wallet = this.wallets.find((w) => w.currency_id === currencyId);
		return wallet?.currency_ticker ?? 'USDC';
	}

	getCompatibleUserWallets(currencyId: string): WalletCurrency[] {
		return this.userWallets.filter((w) => w.currency_id === currencyId);
	}

	recommendedAmount(target: string): number {
		const n = Math.max(1, this.members.length);
		return Number(target) / n;
	}

	async loadGroupData() {
		const res = await getGroup(this.groupId);
		if (!isSuccess(res)) {
			this.groupExists = false;
		} else {
			this.groupData = res.body;
		}
		this.loading = false;
	}

	async loadMembersData() {
		this.loadingMembers = true;
		try {
			const res = await getGroupMembers(this.groupId);
			if (isSuccess(res)) this.members = res.body;
		} finally {
			this.loadingMembers = false;
		}
	}

	async loadWalletsData() {
		this.loadingWallets = true;
		try {
			const res = await getGroupWallets(this.groupId);
			if (isSuccess(res)) this.wallets = res.body;
		} finally {
			this.loadingWallets = false;
		}
	}

	async loadCoreBalances() {
		this.loadingCoreBalances = true;
		this.coreBalancesError = '';
		try {
			const res = await getGroupBalances(this.groupId);
			if (!isSuccess(res)) {
				this.coreBalancesError = res.message || 'No se pudieron cargar los balances.';
				this.coreBalancesData = null;
			} else {
				this.coreBalancesData = res.body;
			}
		} finally {
			this.loadingCoreBalances = false;
		}
	}

	async loadBalancesMovimientos() {
		this.loadingBalancesDetail = true;
		this.transactionsDetailError = '';
		this.expensesDetailError = '';
		const [txRes, expRes] = await Promise.all([
			listGroupTransactions(this.groupId),
			getGroupExpenses(this.groupId)
		]);
		if (isSuccess(txRes)) this.groupTransactions = txRes.body;
		else this.transactionsDetailError = txRes.message || 'Error cargando transacciones.';

		if (isSuccess(expRes)) this.groupExpenses = expRes.body;
		else this.expensesDetailError = expRes.message || 'Error cargando gastos.';

		this.loadingBalancesDetail = false;
	}

	async loadFundRoundsData() {
		this.loadingFundRounds = true;
		this.fundRoundsError = '';

		const [roundsRes, walletsRes] = await Promise.all([
			getGroupFundRoundProposals(this.groupId),
			getMyWallets()
		]);

		if (!isSuccess(roundsRes)) {
			this.fundRoundsError = roundsRes.message || 'Error al cargar rondas.';
			this.loadingFundRounds = false;
			return;
		}

		if (isSuccess(walletsRes)) {
			this.userWallets = walletsRes.body.flatMap((g) => g.currencies);
		}

		const statuses = await Promise.all(
			roundsRes.body.map((r) => getFundRoundProposal(r.proposal.id))
		);
		this.fundRounds = statuses
			.filter(isSuccess)
			.map((res) => res.body)
			.sort(
				(a, b) =>
					new Date(b.fund_round.proposal.created_at).getTime() -
					new Date(a.fund_round.proposal.created_at).getTime()
			);

		const approved = this.fundRounds.filter((s) => s.fund_round.proposal.status === 'Approved');
		const contribResponses = await Promise.all(
			approved.map((s) => getMyFundRoundContribution(s.fund_round.proposal.id))
		);

		const nextContributions: Record<string, string> = {};
		approved.forEach((s, i) => {
			const res = contribResponses[i];
			if (isSuccess(res)) nextContributions[s.fund_round.proposal.id] = res.body.amount;
		});
		this.myContributions = nextContributions;
		this.loadingFundRounds = false;
	}

	async loadExpensesData() {
		this.loadingExpenses = true;
		this.expensesError = '';
		try {
			const res = await getExpenses(this.groupId);
			if (!isSuccess(res)) this.expensesError = res.message || 'Error al cargar gastos.';
			else this.expenses = res.body;
		} finally {
			this.loadingExpenses = false;
		}
	}

	async handleContribute(status: FundRoundStatusResponse, selectedContribWalletId: string) {
		if (!selectedContribWalletId) return false;
		this.contribLoading = true;
		this.contribError = '';

		const proposalId = status.fund_round.proposal.id;
		const remainingRes = await getFundRoundRemaining(proposalId);

		if (!isSuccess(remainingRes)) {
			this.contribLoading = false;
			this.contribError = remainingRes.message || 'Error al calcular monto.';
			return false;
		}

		const recommended = this.recommendedAmount(status.target_amount);
		const { remaining: remainingStr, is_last_contributor } = remainingRes.body;
		const amount = is_last_contributor ? remainingStr : formatAmount(recommended);

		const res = await contributeFundRound(proposalId, {
			amount,
			sender_wallet_id: selectedContribWalletId
		});
		this.contribLoading = false;

		if (!isSuccess(res)) {
			this.contribError = res.message || 'Error al aportar.';
			return false;
		}

		await this.loadFundRoundsData();
		return true;
	}
}
