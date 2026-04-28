import type { ApiResponse, SuccessResponse } from '$lib/types/client.types';
import type { Currency } from '$lib/types/endpoints/currency.types';

export async function getAllCurrencies(): ApiResponse<Currency[]> {
	let currencies: Currency[] = [
		{
			currency_id: '33de6c7c-62a2-4182-813a-9005183be70d',
			currency_ticker: 'USDC'
		}
	];
	return {
		ok: true,
		status: 200,
		body: currencies,
		message: 'currencies mockeadas'
	};
}
