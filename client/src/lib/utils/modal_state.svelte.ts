import { isSuccess } from '$lib/types/client.types';
import type { ApiResponse } from '$lib/types/client.types';

const SUCCESS_DELAY_MS = 1200;

interface SubmitOptions<T> {
	successMsg: string;
	onSuccess?: (data: T) => void;
}

export class ModalState {
	loading = $state(false);
	error = $state('');
	success = $state('');
	attempted = $state(false);

	// Agregamos "| ApiResponse<T>" para que acepte ambos casos
	async submit<T>(
		apiFn: () => Promise<ApiResponse<T>> | ApiResponse<T>,
		{ successMsg, onSuccess }: SubmitOptions<T>
	): Promise<void> {
		this.attempted = true;
		this.error = '';
		this.success = '';
		this.loading = true;

		const result = await apiFn();
		this.loading = false;

		if (!isSuccess(result)) {
			this.error = result.message || 'Ocurrió un error.';
			return;
		}

		this.success = successMsg;
		setTimeout(() => onSuccess?.(result.body), SUCCESS_DELAY_MS);
	}

	setAttempted() {
		this.attempted = true;
	}

	reset() {
		this.loading = false;
		this.error = '';
		this.success = '';
		this.attempted = false;
	}
}
