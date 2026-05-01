import { isSuccess } from '$lib/types/client.types';
import type { ApiResponse, SuccessResponse, FailedResponse } from '$lib/types/client.types';

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

	// Guardamos la referencia del timeout
	private timeoutId?: ReturnType<typeof setTimeout>;

	// Corregimos la firma: acepta la Promesa (ApiResponse) o los objetos directos resueltos
	async submit<T>(
		apiFn: () => ApiResponse<T>,
		{ successMsg, onSuccess }: SubmitOptions<T>
	): Promise<void> {
		this.attempted = true;
		this.error = '';
		this.success = '';
		this.loading = true;

		// Limpiamos timeouts previos si el usuario hace clics rápidos
		if (this.timeoutId) {
			clearTimeout(this.timeoutId);
		}

		// await maneja perfectamente tanto la Promesa como el objeto directo
		const result = await apiFn();
		this.loading = false;

		if (!isSuccess(result)) {
			this.error = result.message || 'Ocurrió un error.';
			return;
		}

		this.success = successMsg;

		// Guardamos el ID del timeout
		this.timeoutId = setTimeout(() => onSuccess?.(result.body), SUCCESS_DELAY_MS);
	}

	setAttempted() {
		this.attempted = true;
	}

	reset() {
		this.loading = false;
		this.error = '';
		this.success = '';
		this.attempted = false;

		// Cancelamos la ejecución de onSuccess si el modal se resetea/cierra antes de tiempo
		if (this.timeoutId) {
			clearTimeout(this.timeoutId);
			this.timeoutId = undefined;
		}
	}
}
