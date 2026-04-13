import { authedApiFetch } from '$lib/api/client';
import type { ApiResponse } from '$lib/types/client.types';
import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';

export async function getAllMyWallets(): ApiResponse<WalletInfo[]> {
	return authedApiFetch('/wallet/get-all', {
		method: 'GET'
	});
}
