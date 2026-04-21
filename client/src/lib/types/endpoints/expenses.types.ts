export type Expense = {
	expense_id: string;
	user_id: string;
	currency_id: string;
	group_id: string;
	description: string | null;
	amount: string;
	status: string;
	created_at: string;
	updated_at: string;
};
