type Uuid = string;
type DateTime = string;

export type InvestmentStatus = 'active' | 'matured' | 'withdrawn';

export type InvestmentStrategy = {
	id: Uuid;
	name: string;
	description: string;
	risk_level: string;
	expected_return_percentage: string;
	duration_days: number;
	created_at: DateTime;
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
	created_at: DateTime;
	updated_at: DateTime;
	strategy_name: string;
	risk_level: string;
	expected_return_percentage: string;
};

export type Snapshot = {
	investment_id: Uuid;
	value: string;
	snapshot_date: DateTime;
	created_at: DateTime;
};
