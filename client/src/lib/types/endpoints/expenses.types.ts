type Uuid = string;
type DateTime = string;

export type ExpenseStatus = 'Created' | 'Verified' | 'Updated' | 'Deleted';

export type ExpenseParticipantInput = {
	user_id: Uuid;
};

export type CreateExpenseData = {
	currency_id: Uuid;
	amount: string;
	description?: string | null;
	participants: ExpenseParticipantInput[];
};

export type UpdateExpenseData = {
	currency_id?: Uuid;
	amount?: string;
	description?: string | null;
	participants?: ExpenseParticipantInput[];
};

export type Expense = {
	expense_id: Uuid;
	user_id: Uuid;
	currency_id: Uuid;
	group_id: Uuid;
	description: string | null;
	amount: string;
	status: ExpenseStatus;
	created_at: DateTime;
	updated_at: DateTime;
};
