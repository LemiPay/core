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
