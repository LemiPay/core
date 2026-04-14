export type NewGroupData = {
	name: string;
	description: string;
};

export type UpdateGroupData = {
	name?: string;
	description?: string;
};

export type GroupSummary = {
	user_id: string;
	group_id: string;
	role: string;
	group_name: string;
	group_description: string;
	status: string;
};

export type Group = {
	id: string;
	name: string;
	description: string;
	status: string;
	created_at: string;
};

export type NewGroupWalletData = {
	address: string;
	currency_ticker: string;
};

export type GroupWallet = {
	id: string;
	address: string;
	group_id: string;
	currency_id: string;
	balance: number;
	created_at: string;
	updated_at: string;
};
