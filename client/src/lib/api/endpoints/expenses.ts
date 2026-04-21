import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	CreateExpenseData,
	Expense,
	UpdateExpenseData
} from '$lib/types/endpoints/expenses.types';

export async function createExpense(
	group_id: string,
	data: CreateExpenseData
): ApiResponse<Expense> {
	return authedApiFetch(`/expense/new/${group_id}`, {
		method: 'POST',
		body: JSON.stringify(data)
	});
}

export async function getExpenses(group_id: string): ApiResponse<Expense[]> {
	return authedApiFetch(`/expense/${group_id}/list`, {
		method: 'GET'
	});
}

export async function updateExpense(
	group_id: string,
	expense_id: string,
	data: UpdateExpenseData
): ApiResponse<Expense> {
	return authedApiFetch(`/expense/${group_id}/${expense_id}`, {
		method: 'PUT',
		body: JSON.stringify(data)
	});
}

export async function deleteExpense(group_id: string, expense_id: string): ApiResponse<Expense> {
	return authedApiFetch(`/expense/${group_id}/${expense_id}`, {
		method: 'DELETE'
	});
}

export async function adminUpdateExpense(
	group_id: string,
	expense_id: string,
	data: UpdateExpenseData
): ApiResponse<Expense> {
	return authedApiFetch(`/expense/admin/${group_id}/${expense_id}`, {
		method: 'PUT',
		body: JSON.stringify(data)
	});
}

export async function adminDeleteExpense(
	group_id: string,
	expense_id: string
): ApiResponse<Expense> {
	return authedApiFetch(`/expense/admin/${group_id}/${expense_id}`, {
		method: 'DELETE'
	});
}
