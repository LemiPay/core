import { authedApiFetch } from './client';

import type { ApiResponse } from '$lib/types/client.types';
import type { NewGroupData } from '$lib/types/groups.types';

export async function create_group(data: NewGroupData): ApiResponse<{ id: string }> {
	return authedApiFetch('/group/create', {
		method: 'POST',
		body: JSON.stringify(data)
	});
}
