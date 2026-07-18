import { authedApiFetch } from '../client';
import type { ApiResponse } from '$lib/types/client.types';
import type { AskRequest, AskResponse } from '$lib/types/endpoints/ai.types';

export function askAI(data: AskRequest, signal?: AbortSignal): ApiResponse<AskResponse> {
	return authedApiFetch('/ai/ask', {
		method: 'POST',
		body: JSON.stringify(data),
		signal
	});
}
