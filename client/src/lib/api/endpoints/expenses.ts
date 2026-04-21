import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { Expense } from '$lib/types/endpoints/expenses.types';

export async function getGroupExpenses(groupId: string): ApiResponse<Expense[]> {
	return authedApiFetch(`/expense/${groupId}/list`, { method: 'GET' });
}
