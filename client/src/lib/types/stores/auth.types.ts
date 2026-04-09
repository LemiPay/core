// lo que devuelve el /me
export type MyUser = {
	id: string;
	name: string;
	email: string;
};

export type AuthState = {
	token: string | null;
	isAuthenticated: boolean;
	user: MyUser | null;
	loading: boolean;
};
