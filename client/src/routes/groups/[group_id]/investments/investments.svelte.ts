import { getGroup, getGroupWallets } from '$lib/api/endpoints/groups';
import {
	listStrategies,
	listGroupInvestments,
	listApprovedProposals,
	createInvestmentProposal,
	executeInvestmentProposal,
	withdrawInvestment
} from '$lib/api/endpoints/investments';
import { getMyWallets } from '$lib/api/endpoints/wallets';
import { authStore } from '$lib/stores/auth';
import { isSuccess } from '$lib/types/client.types';

import type { Group, GroupWallet } from '$lib/types/endpoints/groups.types';
import type {
	Investment,
	InvestmentProposal,
	InvestmentStrategy,
	CreateInvestmentProposalData
} from '$lib/types/endpoints/investments.types';

export class InvestmentsState {
	groupId: string;

	loading = $state(true);
	strategyError = $state('');
	investmentError = $state('');
	walletsError = $state('');
	proposeError = $state('');
	executeError = $state('');
	withdrawError = $state('');

	groupData = $state<Group>({} as Group);
	groupWallets = $state<GroupWallet[]>([]);
	strategies = $state<InvestmentStrategy[]>([]);
	investments = $state<Investment[]>([]);
	proposals = $state<InvestmentProposal[]>([]);
	tickerMap = $state<Record<string, string>>({});

	proposing = $state(false);
	executing = $state(false);
	withdrawing = $state(false);

	constructor(groupId: string) {
		this.groupId = groupId;
	}

	get currentUserId() {
		return authStore.getUserId();
	}

	get activeInvestments() {
		return this.investments.filter((i) => i.status === 'active');
	}

	get maturedInvestments() {
		return this.investments.filter((i) => i.status === 'matured');
	}

	get withdrawnInvestments() {
		return this.investments.filter((i) => i.status === 'withdrawn');
	}

	getTicker(currencyId: string): string {
		return this.tickerMap[currencyId] ?? 'USDC';
	}

	get walletCurrencies() {
		return this.groupWallets
			.filter((w) => this.tickerMap[w.currency_id])
			.map((w) => ({
				currency_id: w.currency_id,
				ticker: this.tickerMap[w.currency_id]
			}));
	}

	async loadAll() {
		this.loading = true;
		await Promise.all([
			this.loadGroupData(),
			this.loadStrategies(),
			this.loadInvestments(),
			this.loadProposals(),
			this.loadTickerMap()
		]);
		this.loading = false;
	}

	async loadProposals() {
		const res = await listApprovedProposals(this.groupId);
		if (isSuccess(res)) this.proposals = res.body;
	}

	async loadGroupData() {
		const res = await getGroup(this.groupId);
		if (isSuccess(res)) this.groupData = res.body;
	}

	async loadTickerMap() {
		this.walletsError = '';

		const [groupWalletsRes, myWalletsRes] = await Promise.all([
			getGroupWallets(this.groupId),
			getMyWallets()
		]);

		if (isSuccess(groupWalletsRes)) {
			this.groupWallets = groupWalletsRes.body;
		}

		const map: Record<string, string> = {};

		if (isSuccess(myWalletsRes)) {
			for (const g of myWalletsRes.body) {
				for (const c of g.currencies) {
					map[c.currency_id] = c.ticker;
				}
			}
		}

		if (isSuccess(groupWalletsRes)) {
			for (const w of groupWalletsRes.body) {
				if (!map[w.currency_id] && w.currency_ticker) {
					map[w.currency_id] = w.currency_ticker;
				}
			}
		}

		this.tickerMap = map;
	}

	async loadStrategies() {
		this.strategyError = '';
		const res = await listStrategies();
		if (!isSuccess(res)) this.strategyError = res.message || 'Error al cargar estrategias.';
		else this.strategies = res.body;
	}

	async loadInvestments() {
		this.investmentError = '';
		const res = await listGroupInvestments(this.groupId);
		if (!isSuccess(res)) this.investmentError = res.message || 'Error al cargar inversiones.';
		else this.investments = res.body;
	}

	async propose(data: CreateInvestmentProposalData) {
		this.proposing = true;
		this.proposeError = '';
		const res = await createInvestmentProposal(this.groupId, data);
		this.proposing = false;
		if (!isSuccess(res)) {
			this.proposeError = res.message || 'Error al crear propuesta.';
			return null;
		}
		await this.loadProposals();
		return res.body;
	}

	async execute(proposalId: string) {
		this.executing = true;
		this.executeError = '';
		const res = await executeInvestmentProposal(this.groupId, { proposal_id: proposalId });
		this.executing = false;
		if (!isSuccess(res)) {
			this.executeError = res.message || 'Error al ejecutar inversión.';
			return false;
		}
		await this.loadInvestments();
		await this.loadProposals();
		return true;
	}

	async withdraw(investmentId: string) {
		this.withdrawing = true;
		this.withdrawError = '';
		const res = await withdrawInvestment(this.groupId, { investment_id: investmentId });
		this.withdrawing = false;
		if (!isSuccess(res)) {
			this.withdrawError = res.message || 'Error al retirar inversión.';
			return false;
		}
		await this.loadInvestments();
		return true;
	}
}
