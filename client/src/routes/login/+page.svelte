<script lang="ts">
	import {
		authActions,
		onWalletAuthChange,
		wagmiAdapter,
		walletAuthState
	} from '../wallet_auth.svelte';
	import { signMessage } from '@wagmi/core';

	import api from '$lib/api/auth';
	import { authStore } from '$lib/stores/auth';
	import { isSuccess } from '$lib/types/client.types';
	import AuthLayout from '$lib/components/layouts/AuthLayout.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';

	let mounted = $state(false);

	let data = $state({
		email: '',
		password: ''
	});

	// false: idle | true: loading | null: end
	let status: boolean | null = $state(false);
	let error = $state('');

	let socialModalOpen = $state(false);
	let socialEmail = $state('');
	let socialName = $state('');
	let socialAttempted = $state(false);

	const socialEmailTrimmed = $derived(socialEmail.trim());
	const socialEmailValid = $derived(
		socialEmailTrimmed.length >= 4 && socialEmailTrimmed.length <= 30
	);
	const socialFormValid = $derived(socialEmailValid);

	type PendingChallenge = {
		nonce: string;
		message: string;
		is_linked: boolean;
		address: string;
	};

	// NUEVO: Memoria para saber si ya le pedimos la firma a esta address
	let lastHandledAddress = $state('' as string | undefined);
	let pendingChallenge = $state(null as PendingChallenge | null);
	let walletNotice = $state('');
	let challengeInFlight = $state(false);
	let challengeInFlightAddress = $state('' as string | undefined);
	let signingInFlight = $state(false);
	let signingAddress = $state('' as string | undefined);

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
			status = false; // Lo pasamos a false para permitir reintentos manuales
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

	async function fetch_challenge(address: string) {
		if (challengeInFlight && challengeInFlightAddress === address) return;
		const requestedAddress = address;
		challengeInFlight = true;
		challengeInFlightAddress = requestedAddress;
		walletNotice = '';
		error = '';
		status = true;

		try {
			const response = await api.request_challenge(address);
			if (!isSuccess(response)) {
				error = response.message;
				status = false; // Permitimos reintentar si el challenge falla
				return;
			}
			return response.body;
		} finally {
			if (challengeInFlightAddress === requestedAddress) {
				challengeInFlight = false;
				challengeInFlightAddress = '';
			}
		}
	}

	async function complete_challenge(nonce: string, message: string) {
		error = '';
		status = true;

		const address = walletAuthState.address;
		if (!address) {
			error = 'Wallet no conectada.';
			status = false;
			return;
		}
		if (signingInFlight && signingAddress === address) return;
		const signingFor = address;
		signingInFlight = true;
		signingAddress = signingFor;

		try {
			const signature = await signMessage(wagmiAdapter.wagmiConfig, {
				message: message
			});

			const res = await api.verify_signature(
				walletAuthState.email ?? null,
				walletAuthState.name ?? null,
				address,
				nonce,
				signature,
				walletAuthState.isSocial
			);

			if (!isSuccess(res)) {
				error = res.message || 'Invalid credentials.';
				status = false; // Evitamos el estado zombi 'null' cuando falla la verificación
				return;
			}

			await authStore.login(res.body.token);
			status = null;
			lastHandledAddress = signingFor;
			walletNotice = '';

			const redirectTo = getSafeRedirectPath(page.url.searchParams.get('redirectTo'));

			setTimeout(() => {
				window.location.href = redirectTo;
			}, 1000);
		} catch (err: any) {
			error = 'Firma rechazada por el usuario.';
			status = false;
			console.error('Error al firmar:', err);
		} finally {
			if (signingAddress === signingFor) {
				signingInFlight = false;
				signingAddress = '';
			}
		}
	}

	async function request_and_complete_challenge(address: string | undefined) {
		if (!address) {
			error = 'Wallet no conectada.';
			status = false;
			return;
		}
		const challenge = await fetch_challenge(address);
		if (!challenge) return;
		await complete_challenge(challenge.nonce, challenge.message);
	}

	function openSocialModal() {
		socialEmail = walletAuthState.email ?? '';
		socialName = walletAuthState.name ?? '';
		socialAttempted = false;
		socialModalOpen = true;
	}

	function cancelWalletLoginModal() {
		if (!socialModalOpen && !pendingChallenge) return;
		socialModalOpen = false;
		socialEmail = '';
		socialName = '';
		socialAttempted = false;
		pendingChallenge = null;
		walletNotice = '';
		if (challengeInFlight) {
			challengeInFlight = false;
			challengeInFlightAddress = '';
		}
	}

	function handleSocialClose() {
		cancelWalletLoginModal();
		challengeInFlight = false;
		challengeInFlightAddress = '';
		signingInFlight = false;
		signingAddress = '';
		void authActions.logout();
	}

	function handleSocialSubmit(e: SubmitEvent) {
		e.preventDefault();
		socialAttempted = true;
		if (!socialFormValid) return;

		walletAuthState.email = socialEmailTrimmed;
		walletAuthState.name = socialName.trim() ? socialName.trim() : undefined;
		lastHandledAddress = walletAuthState.address;
		const cachedChallenge = pendingChallenge;
		pendingChallenge = null;
		socialModalOpen = false;

		if (cachedChallenge && cachedChallenge.address === walletAuthState.address) {
			complete_challenge(cachedChallenge.nonce, cachedChallenge.message);
			return;
		}

		if (walletAuthState.address) {
			request_and_complete_challenge(walletAuthState.address);
		}
	}

	function handleWalletAuthChange() {
		// 1. Si el usuario se desconecta, limpiamos la memoria
		if (!walletAuthState.isConnected) {
			lastHandledAddress = '';
			pendingChallenge = null;
			walletNotice = '';
			challengeInFlight = false;
			challengeInFlightAddress = '';
			signingInFlight = false;
			signingAddress = '';
		}

		// 2. Evaluamos si hay que disparar el challenge
		if (!walletAuthState.isConnected) return;
		if (signingInFlight) return;

		if (walletAuthState.isSocial) {
			cancelWalletLoginModal();
		}

		if (walletAuthState.isSocial && walletAuthState.address !== lastHandledAddress) {
			// SOCIAL LOGIN !
			lastHandledAddress = walletAuthState.address;
			pendingChallenge = null;
			request_and_complete_challenge(walletAuthState.address);
			return;
		}

		if (pendingChallenge && pendingChallenge.address !== walletAuthState.address) {
			pendingChallenge = null;
		}

		if (!walletAuthState.isSocial && walletAuthState.email == undefined) {
			// WALLET LOGIN !
			if (socialModalOpen || pendingChallenge) return;
			if (walletAuthState.address) {
				fetch_challenge(walletAuthState.address).then((challenge) => {
					if (!challenge || !walletAuthState.address) return;
					if (challenge.is_linked) {
						walletNotice = 'Wallet ya vinculada. Firmá para iniciar sesión.';
						pendingChallenge = null;
						complete_challenge(challenge.nonce, challenge.message);
						return;
					}
					walletNotice = '';
					pendingChallenge = { ...challenge, address: walletAuthState.address };
					status = false;
					if (!socialModalOpen) {
						openSocialModal();
					}
				});
			}
			return;
		}

		if (
			!walletAuthState.isSocial &&
			walletAuthState.email &&
			walletAuthState.address !== lastHandledAddress
		) {
			lastHandledAddress = walletAuthState.address;
			pendingChallenge = null;
			request_and_complete_challenge(walletAuthState.address);
		}
	}

	onMount(() => {
		mounted = true;
		const unsubscribe = onWalletAuthChange(handleWalletAuthChange);
		handleWalletAuthChange();
		return () => {
			unsubscribe();
		};
	});
</script>

<AuthLayout description="Enter your details to access the platform." title="Log in to your account">
	<Modal
		description="Ingresá un mail para asociar la cuenta y un nombre opcional."
		onclose={handleSocialClose}
		open={socialModalOpen}
		title="Asociar cuenta"
	>
		<form class="space-y-4" id="social-auth-form" onsubmit={handleSocialSubmit}>
			<FormField
				attempted={socialAttempted}
				bind:value={socialEmail}
				id="social-email"
				label="Email"
				maxLength={30}
				minLength={4}
				placeholder="name@example.com"
				type="email"
			/>

			<div>
				<label class="mb-1.5 block text-sm font-medium text-foreground" for="social-name">
					Nombre (opcional)
				</label>
				<input
					bind:value={socialName}
					class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground transition placeholder:text-muted-foreground focus:border-ring focus:ring-0 focus:outline-none"
					id="social-name"
					maxlength="50"
					placeholder="Tu nombre"
					type="text"
				/>
			</div>
		</form>

		{#snippet footer()}
			<Button label="Cancelar" variant="secondary" onclick={handleSocialClose} />
			<Button label="Continuar" type="submit" form="social-auth-form" disabled={!socialFormValid} />
		{/snippet}
	</Modal>

	{#if mounted}
		<div class="mb-6 flex w-full flex-col items-center gap-4">
			{#if walletNotice}
				<div class="w-full rounded-lg border border-blue-200 bg-blue-50 p-3 text-xs text-blue-700">
					{walletNotice}
				</div>
			{/if}
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
				<span class="w-full border-t border-muted"></span>
			</div>
			<div class="relative flex justify-center text-xs uppercase">
				<span class="bg-card px-2 text-primary">O usar contraseña</span>
			</div>
		</div>
	{/if}

	<form class="flex flex-col space-y-6" onchange={() => (status = false)} onsubmit={login_user}>
		{#if status === null && !error}
			<div
				class="rounded-lg border border-green-300 bg-green-100 p-3 text-sm font-medium text-green-700 dark:border-green-700 dark:bg-green-900 dark:text-green-200"
			>
				Login successful! Redirecting...
			</div>
		{/if}

		<!-- Error Message -->
		{#if error && status !== true}
			<div
				class="rounded-lg border border-red-300 bg-red-100 p-3 text-sm font-medium text-red-700 dark:border-red-700 dark:bg-red-900 dark:text-red-200"
			>
				{error}
			</div>
		{/if}

		<div class="space-y-4">
			<!-- Email -->
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium" for="email">Email</label>
				<input
					bind:value={data.email}
					class="rounded-md border border-input bg-background p-2 text-foreground placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:outline-none"
					id="email"
					placeholder="name@example.com"
					required
					type="email"
				/>
			</div>

			<!-- Password -->
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium" for="password">Password</label>
				<input
					bind:value={data.password}
					class="rounded-md border border-input bg-background p-2 text-foreground placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:outline-none"
					id="password"
					placeholder="••••••••"
					required
					type="password"
				/>
			</div>
		</div>

		<button
			class="w-full rounded-md bg-primary px-4 py-2 font-medium text-primary-foreground transition hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
			disabled={status === true}
			type="submit"
		>
			{status === true ? 'Logging in...' : 'Log in'}
		</button>
		<a
			class="w-full rounded-md border border-input bg-background px-4 py-2 text-center font-medium text-foreground transition hover:bg-accent hover:text-accent-foreground"
			href={resolve('/register')}
		>
			Create account
		</a>
	</form>
</AuthLayout>
