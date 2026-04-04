<script lang="ts">
	import api from '$lib/api/auth';
	import { authStore } from '$lib/stores/auth';
	import { isSuccess } from '$lib/types/client.types';
	import type { User } from '$lib/types/endpoints/auth.types';

	const wallet = 'Wallet';
	const balance = '0.00';
	const currency = 'USDC';

	let userData = $state<User | null>(null);
	let isLoading = $state(true);
	let error = $state('');

	async function loadUserProfile() {
		isLoading = true;
		error = '';
		const meResponse = await api.me();
		if (!isSuccess(meResponse)) {
			error = meResponse.message || 'Error al validar sesión.';
			isLoading = false;
			return;
		}

		const userId = meResponse.body.id;

		const infoResponse = await api.user_info(userId);
		if (!isSuccess(infoResponse)) {
			error = infoResponse.message || 'Error al obtener datos del usuario.';
			isLoading = false;
			return;
		}
		userData = infoResponse.body;
		isLoading = false;
	}
	$effect(() => {
		if ($authStore.token) {
			loadUserProfile();
		} else {
			isLoading = true;
		}
	});
</script>

<div
	class="flex w-full items-center justify-between rounded-md border border-gray-300 bg-white p-3 shadow-sm"
>
	<div class="flex items-center gap-3">
		<div
			class="flex h-10 w-10 items-center justify-center rounded-full border border-gray-300 bg-gray-50 text-black"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
				<circle cx="12" cy="7" r="4"></circle>
			</svg>
		</div>

		<div class="flex flex-col">
			<span class="text-sm font-medium text-black">{userData ? userData.name : error}</span>
			<span class="text-xs text-gray-500">{wallet}</span>
		</div>
	</div>

	<div class="text-sm font-semibold text-black">
		${balance} <span class="text-xs font-normal text-gray-500">{currency}</span>
	</div>
</div>
