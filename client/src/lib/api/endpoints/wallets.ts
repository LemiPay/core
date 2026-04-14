import { authedApiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { AddressGroup } from '$lib/types/endpoints/wallets.types';

export async function getMyWallets(): ApiResponse<AddressGroup[]> {
	return authedApiFetch('/wallet/get-all', {
		method: 'GET'
	});
}
