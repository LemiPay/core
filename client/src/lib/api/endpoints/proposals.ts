import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	ExpandedProposal,
	NewMemberData,
	ReceivedProposalSummary
} from '$lib/types/endpoints/proposals.types';
import type { GroupSummary } from '$lib/types/endpoints/groups.types';

export async function createNewMemberProposal(data: NewMemberData): ApiResponse<ExpandedProposal> {
	return authedApiFetch(`/proposal/new-member/${data.group_id}`, {
		method: 'POST',
		body: JSON.stringify({ user_email: data.email })
	});
}

export async function getReceivedProposals(): ApiResponse<ExpandedProposal[]> {
	return authedApiFetch('/proposal/received', {
		method: 'GET'
	});
}
