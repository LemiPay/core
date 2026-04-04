export type RegisterData = NewUser;

export type LoginData = {
	email: string;
	password: string;
};

export type User = {
	id: string;
	email: string;
	password: string;
	name: string;
};

export type NewUser = {
	email: string;
	password: string;
	name: string;
};
