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

export type PaySettlementData = {
	amount: string;
	address: string;
	currency_id: string;
};

export type PaySettlementResponse = {
	id: string;
	amount: string;
	user_id: string;
	group_id: string;
	currency_id: string;
	address: string;
	description: string | null;
	tx_type: 'settlement_payment';
	created_at: string;
};
