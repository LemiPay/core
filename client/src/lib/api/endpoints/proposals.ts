import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { ExpandedProposal, NewMemberData } from '$lib/types/endpoints/proposals';

export async function createNewMemberProposal(data: NewMemberData): ApiResponse<ExpandedProposal> {
	return authedApiFetch(`/proposal/new-member/${data.group_id}`, {
		method: 'POST',
		body: JSON.stringify({ user_email: data.email })
	});
}
