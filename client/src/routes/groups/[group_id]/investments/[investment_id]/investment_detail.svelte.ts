import { getGroupWallets } from '$lib/api/endpoints/groups';
import { listGroupInvestments, listInvestmentSnapshots } from '$lib/api/endpoints/investments';
import { getMyWallets } from '$lib/api/endpoints/wallets';
import { isSuccess } from '$lib/types/client.types';

import type { GroupWallet } from '$lib/types/endpoints/groups.types';
import type { Investment, Snapshot } from '$lib/types/endpoints/investments.types';

export class InvestmentDetailState {
	groupId: string;
	investmentId: string;

	loadingDetail = $state(true);
	detailError = $state('');

	tickerMap = $state<Record<string, string>>({});
	investment = $state<Investment | null>(null);
	snapshots = $state<Snapshot[]>([]);

	constructor(groupId: string, investmentId: string) {
		this.groupId = groupId;
		this.investmentId = investmentId;
	}

	getTicker(currencyId: string): string {
		return this.tickerMap[currencyId] ?? 'USDC';
	}

	get chartData() {
		if (this.snapshots.length === 0) return [];
		return this.snapshots
			.slice()
			.sort((a, b) => new Date(a.snapshot_date).getTime() - new Date(b.snapshot_date).getTime())
			.map((s) => ({
				date: s.snapshot_date,
				value: Number(s.value),
				label: new Date(s.snapshot_date).toLocaleDateString('es-AR', {
					day: '2-digit',
					month: '2-digit'
				})
			}));
	}

	get investedAmount() {
		return this.investment ? Number(this.investment.amount) : 0;
	}

	get currentValue() {
		return this.investment ? Number(this.investment.current_value) : 0;
	}

	get actualReturn() {
		return this.investment ? Number(this.investment.actual_return ?? 0) : 0;
	}

	get pctChange() {
		if (this.investedAmount === 0) return 0;
		return ((this.currentValue - this.investedAmount) / this.investedAmount) * 100;
	}

	get isUp() {
		return this.currentValue >= this.investedAmount;
	}

	async loadAll() {
		this.loadingDetail = true;
		this.detailError = '';

		const [walletsRes, myWalletsRes, investmentsRes, snapshotsRes] = await Promise.all([
			getGroupWallets(this.groupId),
			getMyWallets(),
			listGroupInvestments(this.groupId),
			listInvestmentSnapshots(this.investmentId)
		]);

		const map: Record<string, string> = {};
		if (isSuccess(myWalletsRes)) {
			for (const g of myWalletsRes.body) {
				for (const c of g.currencies) {
					map[c.currency_id] = c.ticker;
				}
			}
		}
		if (isSuccess(walletsRes)) {
			for (const w of walletsRes.body) {
				if (!map[w.currency_id] && w.currency_ticker) {
					map[w.currency_id] = w.currency_ticker;
				}
			}
		}
		this.tickerMap = map;

		if (isSuccess(snapshotsRes)) this.snapshots = snapshotsRes.body;

		if (isSuccess(investmentsRes)) {
			const found = investmentsRes.body.find((i) => i.id === this.investmentId);
			if (found) this.investment = found;
			else this.detailError = 'Inversión no encontrada.';
		} else {
			this.detailError = investmentsRes.message || 'Error al cargar inversión.';
		}

		this.loadingDetail = false;
	}
}
