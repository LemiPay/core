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

export type SuccessResponse<T> = {
	status: 200;
	body: T;
	message: string;
};

export type FailedResponse = {
	status: number;
	message: string;
	body: unknown;
};

export type ApiResponse<T> = Promise<SuccessResponse<T> | FailedResponse>;

export type Web3AuthPayload = {
	wallet: string;
	message: string;
	signature: `0x${string}`;
};

export type Web3AuthResult = {
	/** `true` if the wallet was not in your DB yet. */
	isNew: boolean;
};
