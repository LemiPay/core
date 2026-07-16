type Uuid = string;
type DateTime = string;

export type InvestmentStatus = 'active' | 'matured' | 'withdrawn';

export type ValuationMode = 'simulated' | 'mark_to_market';

export type StrategyCategory = 'simulated' | 'crypto' | 'stocks' | 'mixed' | 'rwa';

export type PriceProvider = 'coingecko' | 'stooq' | 'mock' | string;

export type Allocation = {
	asset_id: Uuid;
	symbol: string;
	name: string;
	kind: string;
	weight_bps: number;
	price_provider?: PriceProvider;
	external_id?: string;
	price_source_url?: string;
};

export type Holding = {
	asset_id: Uuid;
	symbol: string;
	name: string;
	kind: string;
	units: string;
	weight_bps_at_entry: number;
	cost_basis_usd: string;
	price_provider?: PriceProvider;
	external_id?: string;
	price_source_url?: string;
	/** cost_basis / units at entry */
	entry_price_usd?: string | null;
	/** Latest oracle mark (mock or live) */
	current_price_usd?: string | null;
	/** units * current_price */
	current_value_usd?: string | null;
};

export type InvestmentStrategy = {
	id: Uuid;
	name: string;
	description: string;
	risk_level: string;
	expected_return_percentage: string;
	duration_days: number;
	created_at: DateTime;
	valuation_mode: ValuationMode;
	category: StrategyCategory;
	ragequit_fee_bps: number;
	allocations: Allocation[];
};

export type CreateInvestmentProposalData = {
	amount: string;
	strategy_id: Uuid;
	currency_id: Uuid;
};

export type InvestmentProposal = {
	proposal_id: Uuid;
	group_id: Uuid;
	created_by: Uuid;
	status: InvestmentStatus;
	created_at: DateTime;
	updated_at: DateTime;
	amount: string;
	strategy_id: Uuid;
	currency_id: Uuid;
	strategy_name: string;
};

export type ExecuteInvestmentData = {
	proposal_id: Uuid;
};

export type WithdrawInvestmentData = {
	investment_id: Uuid;
};

export type Investment = {
	id: Uuid;
	group_id: Uuid;
	proposal_id: Uuid;
	strategy_id: Uuid;
	currency_id: Uuid;
	amount: string;
	current_value: string;
	actual_return: string | null;
	status: InvestmentStatus;
	started_at: DateTime;
	matures_at?: DateTime;
	created_at: DateTime;
	updated_at: DateTime;
	strategy_name: string;
	risk_level: string;
	expected_return_percentage: string;
	valuation_mode?: ValuationMode;
	category?: StrategyCategory;
	ragequit_fee_bps?: number;
	exit_kind?: string | null;
	fee_amount?: string | null;
	holdings?: Holding[];
};

export type Snapshot = {
	investment_id: Uuid;
	value: string;
	snapshot_date: DateTime;
	created_at: DateTime;
};

export const categoryLabels: Record<string, string> = {
	simulated: 'Simulado',
	crypto: 'Crypto',
	stocks: 'Stocks (Ondo)',
	mixed: 'Mix',
	rwa: 'RWA'
};

export const providerLabels: Record<string, string> = {
	coingecko: 'CoinGecko',
	mock: 'Mock',
	// legacy labels (config is CoinGecko-only now)
	dexscreener: 'CoinGecko',
	ondo: 'CoinGecko',
	stooq: 'CoinGecko'
};

export function weightPct(bps: number): string {
	return (bps / 100).toFixed(bps % 100 === 0 ? 0 : 1);
}

/**
 * CoinGecko page. Prefer server `price_source_url` (built as en/{tag}/{id}
 * e.g. stocks/nvidia from config [stocks] NVDA = "nvidia").
 */
export function resolvePriceSourceUrl(asset: {
	price_provider?: string;
	external_id?: string;
	price_source_url?: string;
	symbol?: string;
	kind?: string;
}): string | null {
	if (asset.price_source_url) return asset.price_source_url;
	const id = asset.external_id || asset.symbol?.toLowerCase();
	if (!id) return null;
	// Fallback by kind if server did not send URL
	const tag =
		asset.kind === 'tokenized_stock' ? 'stocks' : asset.kind === 'rwa' ? 'commodities' : 'coins';
	return `https://www.coingecko.com/en/${tag}/${encodeURIComponent(id)}`;
}

export function providerShortLabel(_provider?: string, _kind?: string): string {
	return 'CoinGecko';
}
