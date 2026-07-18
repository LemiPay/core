import { authedApiFetch } from '../client';
import type { ApiResponse } from '$lib/types/client.types';
import type {
	AskRequest,
	AskResponse,
	ExplainRequest,
	ExplainResponse
} from '$lib/types/endpoints/ai.types';

export function askAI(data: AskRequest, signal?: AbortSignal): ApiResponse<AskResponse> {
	return authedApiFetch('/ai/ask', {
		method: 'POST',
		body: JSON.stringify(data),
		signal
	});
}

export function explainAI(
	data: ExplainRequest,
	signal?: AbortSignal
): ApiResponse<ExplainResponse> {
	return authedApiFetch('/ai/explain', {
		method: 'POST',
		body: JSON.stringify(data),
		signal
	});
}
