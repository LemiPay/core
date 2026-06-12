export type CoreUserBalance = {
	user_name: string;
	user_id: string;
	balance: string | number;
};

export type GroupBalancesResponse = {
	group_balance: string | number;
	balances: CoreUserBalance[];
};

export type SettlementItem = {
	from: string;
	to: string;
	amount: string;
	to_name: string | null;
	from_name: string | null;
};

export type GetSettlementsResponse = {
	settlements: SettlementItem[];
};
