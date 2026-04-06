<script lang="ts">
	import { me, userInfo } from '$lib/api/auth';
	import { isSuccess } from '$lib/types/client.types';
	import type { UserInfo } from '$lib/types/endpoints/auth.types';
	import { CircleUser } from 'lucide-svelte';

	const wallet = 'Wallet';
	const balance = 0.0;
	const currency = 'USDC';

	let userData = $state<UserInfo | null>(null);
	let isLoading = $state(true);
	let error = $state('');

	async function loadUserProfile() {
		isLoading = true;
		error = '';
		const meResponse = await me();
		if (!isSuccess(meResponse)) {
			error = meResponse.message || 'Error al validar sesión.';
			isLoading = false;
			return;
		}

		const userId = meResponse.body.id;

		const infoResponse = await userInfo(userId);
		if (!isSuccess(infoResponse)) {
			error = infoResponse.message || 'Error al obtener datos del usuario.';
			isLoading = false;
			return;
		}
		userData = {
			id: infoResponse.body.id,
			name: infoResponse.body.name,
			email: infoResponse.body.email,
			wallet: wallet,
			balance: balance
		};
		isLoading = false;
	}

	loadUserProfile();
</script>

<div
	class="flex w-full items-center justify-between rounded-md border border-gray-300 bg-white p-3 shadow-sm"
>
	<div class="flex items-center gap-3">
		<div class="flex h-10 w-10 items-center justify-center rounded-full">
			<CircleUser size={40} />
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
