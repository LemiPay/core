<script lang="ts">
	import { walletAuthState, authActions } from '../wallet_auth.svelte';

	//NUESTRAS API
	import api from '$lib/api/auth';
	import { authStore } from '$lib/stores/auth';
	import { isSuccess } from '$lib/types/client.types';
	import AuthLayout from '$lib/components/layouts/AuthLayout.svelte';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	let mounted = $state(false);

	let data = $state({
		email: '',
		password: ''
	});

	// false: idle | true: loading | null: end
	let status: boolean | null = $state(false);
	let error = $state('');

	function getSafeRedirectPath(redirectTo: string | null): string {
		if (!redirectTo) return '/dashboard';

		const trimmed = redirectTo.trim();
		if (!trimmed.startsWith('/') || trimmed.startsWith('//')) {
			return '/dashboard';
		}

		try {
			const parsed = new URL(trimmed, window.location.origin);
			if (parsed.origin !== window.location.origin) return '/dashboard';
			if (!parsed.pathname.startsWith('/')) return '/dashboard';
			return `${parsed.pathname}${parsed.search}${parsed.hash}`;
		} catch {
			return '/dashboard';
		}
	}

	async function login_user() {
		error = '';
		status = true;

		const response = await api.login(data);

		if (!isSuccess(response)) {
			error = response.message || 'Invalid credentials.';
			status = null;
			return;
		}

		await authStore.login(response.body.token);
		status = null;

		data = {
			email: '',
			password: ''
		};

		const redirectTo = getSafeRedirectPath(page.url.searchParams.get('redirectTo'));

		setTimeout(() => {
			window.location.href = redirectTo;
		}, 1000);
	}
	async function wallet_login_user() {
		error = '';
		status = true;

		const response = await api.wallet_login_mock(walletAuthState.email, walletAuthState.address);

		if (!isSuccess(response)) {
			error = response.message || 'Invalid credentials.';
			status = null;
			return;
		}

		await authStore.login(response.body.token);
		status = null;

		data = {
			email: '',
			password: ''
		};

		const redirectTo = getSafeRedirectPath(page.url.searchParams.get('redirectTo'));

		setTimeout(() => {
			window.location.href = redirectTo;
		}, 1000);
	}

	onMount(() => {
		mounted = true;
	});

	$effect(() => {
		if (walletAuthState.isConnected && walletAuthState.email && status === false) {
			console.log('Gatillando login de wallet para:', walletAuthState.email);
			wallet_login_user();
		}
	});
</script>

<AuthLayout title="Log in to your account" description="Enter your details to access the platform.">
	{#if mounted}
		<div class="mb-6 flex w-full flex-col items-center gap-4">
			{#if walletAuthState.isConnected}
				<!-- Estado: Conectado -->
				<div class="w-full rounded-lg border border-green-200 bg-green-50 p-4">
					<div class="flex flex-col gap-1">
						<span class="text-[10px] font-bold text-green-700 uppercase">Wallet Conectada</span>
						<p class="truncate font-mono text-xs text-green-900">{walletAuthState.address}</p>
						{#if walletAuthState.email}
							<p class="text-xs text-green-800"><strong>Email:</strong> {walletAuthState.email}</p>
						{/if}
					</div>

					<div class="mt-4 flex gap-2">
						<!-- Botón para abrir el modal de Reown (ajustes, cambiar red, etc) -->
						<button
							type="button"
							onclick={() => authActions.openLogin()}
							class="flex-1 rounded-md border border-gray-300 bg-white py-2 text-xs transition hover:bg-gray-50"
						>
							Ver Perfil
						</button>

						<!-- Botón para DESLOGUEARSE (limpia la sesión de Reown) -->
						<button
							type="button"
							onclick={() => authActions.logout()}
							class="flex-1 rounded-md border border-red-200 bg-red-50 py-2 text-xs text-red-600 transition hover:bg-red-100"
						>
							Desconectar
						</button>
					</div>
				</div>
			{:else}
				<!-- Estado: Desconectado -->
				<button
					type="button"
					onclick={() => authActions.openLogin()}
					class="flex w-full items-center justify-center gap-2 rounded-md border border-gray-300 bg-white px-4 py-2.5 font-medium text-black shadow-sm transition hover:bg-gray-50"
				>
					<img src="https://authjs.dev/img/providers/google.svg" alt="G" class="h-4 w-4" />
					Continuar con Google o Wallet
				</button>
			{/if}
		</div>

		<!-- Separador visual si vas a mantener el form de password abajo -->
		<div class="relative my-6">
			<div class="absolute inset-0 flex items-center">
				<span class="w-full border-t border-gray-300"></span>
			</div>
			<div class="relative flex justify-center text-xs uppercase">
				<span class="bg-white px-2 text-gray-500">O usar contraseña</span>
			</div>
		</div>
	{/if}

	<form onsubmit={login_user} onchange={() => (status = false)} class="flex flex-col space-y-6">
		{#if status === null && !error}
			<div
				class="rounded-lg border border-green-300 bg-green-100 p-3 text-sm font-medium text-green-700"
			>
				Login successful! Redirecting...
			</div>
		{/if}

		<!-- Error Message -->
		{#if status === null && error}
			<div class="rounded-lg border border-red-300 bg-red-100 p-3 text-sm font-medium text-red-700">
				{error}
			</div>
		{/if}

		<div class="space-y-4">
			<!-- Email -->
			<div class="flex flex-col gap-1.5">
				<label for="email" class="text-sm font-medium">Email</label>
				<input
					id="email"
					bind:value={data.email}
					type="email"
					required
					placeholder="name@example.com"
					class="rounded-md border border-gray-300 p-2 focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>

			<!-- Password -->
			<div class="flex flex-col gap-1.5">
				<label for="password" class="text-sm font-medium">Password</label>
				<input
					id="password"
					bind:value={data.password}
					type="password"
					required
					placeholder="••••••••"
					class="rounded-md border border-gray-300 p-2 focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>
		</div>

		<button
			type="submit"
			disabled={status === true}
			class="w-full rounded-md bg-black px-4 py-2 font-medium text-white transition hover:bg-gray-800 disabled:bg-gray-400"
		>
			{status === true ? 'Logging in...' : 'Log in'}
		</button>
		<a
			href="/register"
			class="w-full rounded-md border border-gray-300 bg-white px-4 py-2 text-center font-medium text-black transition hover:bg-gray-50"
		>
			Create account
		</a>
	</form>
</AuthLayout>
