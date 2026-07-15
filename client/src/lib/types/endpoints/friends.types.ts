export type FriendStatus = 'pending' | 'accepted' | 'rejected' | 'blocked';

export type FriendResponse = {
	user_id: string;
	name: string;
	email: string;
	status: string;
	created_at: string;
};

export type UserSearchResult = {
	user_id: string;
	name: string;
	email: string;
	is_friend: boolean;
};

export type RespondRequestData = {
	action: 'accept' | 'reject';
};

export type FriendRequestData = {
	user_id: string;
};
