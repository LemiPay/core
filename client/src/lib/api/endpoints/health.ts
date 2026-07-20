import { apiFetch } from '../client';

import type { ApiResponse } from '$lib/types/client.types';
import type { HealthResponse } from '$lib/types/endpoints/health.types';

export async function getHealth(): ApiResponse<HealthResponse> {
	return apiFetch('/health', { method: 'GET' });
}
