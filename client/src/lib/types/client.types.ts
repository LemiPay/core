export type SuccessResponse<T> = {
	ok: true;
	status: 200;
	body: T;
	message: string;
};

export type FailedResponse = {
	ok: false;
	status: number;
	message: string;
	body: unknown;
};

export type ApiResponse<T> = Promise<SuccessResponse<T> | FailedResponse>;

export function isSuccess<T>(res: SuccessResponse<T> | FailedResponse): res is SuccessResponse<T> {
	return res.ok;
}
