export type NotificationEvent = {
	id: string;
	name: string;
};

export type NotificationChannel = {
	id: string;
	name: string;
};

export type UserNotificationPreference = {
	user_id: string;
	event_id: string;
	channel_id: string;
	enabled: boolean;
};

export type GroupNotificationPreference = {
	user_id: string;
	group_id: string;
	event_id: string;
	channel_id: string;
	enabled: boolean;
};

export type UpsertPreferenceRequest = {
	event_id: string;
	channel_id: string;
	enabled: boolean;
};

export type NotificationRecord = {
	id: string;
	event_name: string;
	group_id: string | null;
	group_name: string | null;
	read: boolean;
	created_at: string;
};

export type GetNotificationsParams = {
	read?: boolean;
	limit?: number;
};
