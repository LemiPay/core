import { authedApiFetch } from '$lib/api/client';
import type { ApiResponse } from '$lib/types/client.types';
import type { Wallet, WalletInfo } from '$lib/types/endpoints/user_wallet.types';

export async function getAllMyWallets(): ApiResponse<WalletInfo[]> {
	return authedApiFetch('/wallet/get-all', {
		method: 'GET'
	});
}

export async function fundWallet(amount: string, wallet_id: string): ApiResponse<Wallet> {
	return authedApiFetch(`/wallet/fund/${wallet_id}`, {
		method: 'POST',
		body: JSON.stringify({ amount: amount })
	});
}
export async function transferToWallet(
	amount: string,
	sender_wallet_id: string,
	receiver_address: string
): ApiResponse<boolean> {
	return authedApiFetch('/wallet/transfer', {
		method: 'POST',
		body: JSON.stringify({ sender_wallet_id, receiver_address, amount })
	});
}

export async function createNewAddress(
	address: string,
	currency_ticker: string
): ApiResponse<Wallet> {
	return authedApiFetch('/wallet/create', {
		method: 'POST',
		body: JSON.stringify({ address, currency_ticker })
	});
}

export async function withdrawFromWallet(
	amount: string,
	wallet_id: string,
	signature: string,
	address: string
): ApiResponse<Wallet> {
	return authedApiFetch(`/wallet/withdraw/${wallet_id}`, {
		method: 'POST',
		body: JSON.stringify({ amount, signature, address })
	});
}
