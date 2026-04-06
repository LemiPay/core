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
//este componente hay que modificarlo para agregar lo que necesitemos en el perfil y eso
export type UserInfo = {
	id: string;
	name: string;
	wallet: string; //para tener algo hardcodeado ahora
	email: string;
	balance: number; //lo mismo
};

export type UserBadge = {
	user_id: string;
	name: string;
	role: string;
};
