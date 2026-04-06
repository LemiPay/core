type Uuid = string;

enum ProposalStatus {
	Pending = 'Pending',
	Approved = 'Approved',
	Rejected = 'Rejected'
}

enum ProposalType {
	NewMember = 'NewMember'
}

type DateTime = string;

export type Proposal = {
	id: Uuid;
	group_id: Uuid;
	created_by: Uuid;
	status: ProposalStatus;
	created_at: DateTime;
	updated_at: DateTime;
};

export type NewMemberProposal = {
	proposal_id: Uuid;
	new_member_id: Uuid;
};

export type ExpandedProposal = {
	proposal: Proposal;
	new_member_proposal: NewMemberProposal;
	proposal_type: ProposalType;
};

export type NewMemberData = {
	group_id: string;
	email: string;
};
