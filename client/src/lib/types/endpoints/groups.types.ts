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
