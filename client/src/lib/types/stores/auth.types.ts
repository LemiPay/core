// lo que devuelve el /me
export type MyUser = {
	id: string;
	// después podés agregar name, email, etc
};

export type AuthState = {
	token: string | null;
	isAuthenticated: boolean;
	user: MyUser | null;
	loading: boolean;
};
