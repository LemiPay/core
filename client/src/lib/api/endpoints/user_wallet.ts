import { authedApiFetch } from '$lib/api/client';
import type { ApiResponse } from '$lib/types/client.types';
import type { Wallet, WalletInfo } from '$lib/types/endpoints/user_wallet.types';

export async function getAllMyWallets(): ApiResponse<WalletInfo[]> {
	return authedApiFetch('/wallet/get-all', {
		method: 'GET'
	});
}

export async function faucet_fund_wallet(amount: string, wallet_id: string): ApiResponse<Wallet> {
	return authedApiFetch(`/wallet/fund/${wallet_id}`, {
		method: 'POST',
		body: JSON.stringify({ amount: amount })
	});
}
