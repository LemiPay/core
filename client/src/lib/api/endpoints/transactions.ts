import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	WithdrawProposalExpanded,
	WithdrawProposalRequest
} from '$lib/types/endpoints/transactions.types';

export async function proposeWithdraw(
	request: WithdrawProposalRequest,
	group_id: string
): ApiResponse<WithdrawProposalExpanded> {
	return authedApiFetch(`/transaction/${group_id}/withdraw/proposal`, {
		method: 'POST',
		body: JSON.stringify({
			currency_id: request.currency_id,
			address: request.user_address,
			amount: request.amount
		})
	});
}
