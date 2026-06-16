import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type {
	ExpandedProposal,
	NewMemberData,
	NewMemberProposalResponse,
	ReceivedNewMemberProposalExpanded
} from '$lib/types/endpoints/proposals.types';

export async function createNewMemberProposal(data: NewMemberData): ApiResponse<ExpandedProposal> {
	return authedApiFetch(`/governance/new-member/${data.group_id}`, {
		method: 'POST',
		body: JSON.stringify({ user_email: data.email })
	});
}

export async function getReceivedProposals(): ApiResponse<ReceivedNewMemberProposalExpanded[]> {
	return authedApiFetch('/governance/received', {
		method: 'GET'
	});
}

export async function getGroupNewMemberProposals(
	group_id: string
): ApiResponse<NewMemberProposalResponse[]> {
	return authedApiFetch(`/governance/group/${group_id}`, {
		method: 'GET'
	});
}
export async function respondToReceivedProposal(
	response: boolean,
	proposal_id: string
): ApiResponse<ExpandedProposal> {
	return authedApiFetch(`/governance/respond/${proposal_id}`, {
		method: 'PUT',
		body: JSON.stringify({ response })
	});
}
