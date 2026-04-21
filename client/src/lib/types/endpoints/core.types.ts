export type CoreUserBalance = {
	user_name: string;
	user_id: string;
	balance: string | number;
};

export type GroupBalancesResponse = {
	group_balance: string | number;
	balances: CoreUserBalance[];
};
