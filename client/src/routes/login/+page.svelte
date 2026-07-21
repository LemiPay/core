<script lang="ts">
	import { authActions, onWalletAuthChange, walletAuthState } from '../wallet_auth.svelte';

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

	// false: idle | true: loading | null: success
	let status: boolean | null = $state(false);
	let error = $state('');

	let socialModalOpen = $state(false);
	let socialEmail = $state('');
	let socialName = $state('');
	let socialAttempted = $state(false);

	// Credenciales de asociación locales (NO mutar walletAuthState: syncWallet las pisa en EOA).
	let associationEmail = $state('' as string | null);
	let associationName = $state('' as string | null);
	let associationAllowLinking = $state(false);

	const socialEmailTrimmed = $derived(socialEmail.trim());
	const socialEmailValid = $derived(
		socialEmailTrimmed.includes('@') &&
			socialEmailTrimmed.length >= 5 &&
			socialEmailTrimmed.length <= 254 &&
			!socialEmailTrimmed.startsWith('@') &&
			!socialEmailTrimmed.endsWith('@')
	);
	const socialFormValid = $derived(socialEmailValid);

	type PendingChallenge = {
		nonce: string;
		message: string;
		issued_at: string;
		is_linked: boolean;
		address: string;
	};

	/** Solo se setea tras login web3 exitoso, para no re-disparar el challenge. */
	let lastHandledAddress = $state('' as string | undefined);
	let pendingChallenge = $state(null as PendingChallenge | null);
	let walletNotice = $state('');
	/** Lock del flujo completo request+sign+verify (evita races en Google/Reown). */
	let authFlowInFlight = $state(false);

	function normalizeAddress(address: string): string {
		return address.trim().toLowerCase();
	}

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

	function resetAssociationState() {
		associationEmail = null;
		associationName = null;
		associationAllowLinking = false;
		pendingChallenge = null;
		walletNotice = '';
	}

	async function login_user(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		status = true;

		const response = await api.login(data);

		if (!isSuccess(response)) {
			error = response.message || 'Invalid credentials.';
			status = false;
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

	type Web3LoginOpts = {
		email?: string | null;
		name?: string | null;
		allowLinking?: boolean;
		/** Si true, no pide email aunque la wallet no esté linkeada (no debería usarse). */
		skipAssociationModal?: boolean;
	};

	/**
	 * Flujo atómico: resolver address de firma → challenge → firmar → verify.
	 * Usa SIEMPRE la misma address (la de wagmi listo para firmar).
	 */
	async function runWeb3Login(opts: Web3LoginOpts = {}) {
		if (authFlowInFlight) return;
		authFlowInFlight = true;
		error = '';
		status = true;
		walletNotice = '';

		try {
			// 1) Wallet lista para firmar (crítico en Google / embedded)
			const ctx = await authActions.getWagmiTxContext();
			if (!ctx?.address) {
				error =
					'La wallet no está lista para firmar. Reconectá con Google/Wallet e intentá de nuevo.';
				status = false;
				return;
			}

			const address = normalizeAddress(ctx.address);

			// 2) Challenge para ESA address (no la de un snapshot viejo de AppKit)
			const challengeRes = await api.request_challenge(address);
			if (!isSuccess(challengeRes)) {
				error = challengeRes.message || 'No se pudo solicitar el challenge.';
				status = false;
				return;
			}

			const challenge = challengeRes.body;

			// 3) Wallet no vinculada y sin email → modal de asociación
			const email =
				opts.email !== undefined ? opts.email : (associationEmail ?? walletAuthState.email ?? null);
			const name =
				opts.name !== undefined ? opts.name : (associationName ?? walletAuthState.name ?? null);
			const allowLinking =
				opts.allowLinking !== undefined
					? opts.allowLinking
					: associationAllowLinking || walletAuthState.isSocial;

			if (!challenge.is_linked && !email?.trim() && !opts.skipAssociationModal) {
				pendingChallenge = {
					...challenge,
					address
				};
				status = false;
				if (!socialModalOpen) openSocialModal();
				return;
			}

			if (challenge.is_linked) {
				walletNotice = 'Wallet ya vinculada. Firmá para iniciar sesión.';
			}

			// 4) Firmar con la misma address del challenge
			const { signature, address: signedAddress } = await authActions.signAuthMessage(
				challenge.message,
				address
			);

			const verifyAddress = normalizeAddress(signedAddress);

			// 5) Verify con issued_at (funciona aunque el cache del server se haya perdido)
			const res = await api.verify_signature(
				email?.trim() ? email.trim() : null,
				name?.trim() ? name.trim() : null,
				verifyAddress,
				challenge.nonce,
				signature,
				allowLinking,
				challenge.issued_at
			);

			if (!isSuccess(res)) {
				error = res.message || 'No se pudo verificar la firma.';
				status = false;
				return;
			}

			await authStore.login(res.body.token);
			status = null;
			lastHandledAddress = verifyAddress;
			walletNotice = '';
			pendingChallenge = null;

			const redirectTo = getSafeRedirectPath(page.url.searchParams.get('redirectTo'));
			setTimeout(() => {
				window.location.href = redirectTo;
			}, 1000);
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : String(err ?? '');
			if (msg.includes('WALLET_NOT_READY') || msg.toLowerCase().includes('connector')) {
				error =
					'La wallet no está lista para firmar. Reconectá con Google/Wallet e intentá de nuevo.';
			} else if (msg.includes('ADDRESS_MISMATCH')) {
				error =
					'La wallet cambió durante el login. Desconectá, volvé a conectar e intentá de nuevo.';
			} else if (
				msg.toLowerCase().includes('user rejected') ||
				msg.toLowerCase().includes('rejected') ||
				msg.toLowerCase().includes('denied')
			) {
				error = 'Firma rechazada por el usuario.';
			} else {
				error = 'No se pudo firmar el mensaje. Intentá de nuevo.';
			}
			status = false;
			console.error('Error en login web3:', err);
		} finally {
			authFlowInFlight = false;
		}
	}

	function openSocialModal() {
		socialEmail = associationEmail ?? walletAuthState.email ?? '';
		socialName = associationName ?? walletAuthState.name ?? '';
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
	}

	function handleSocialClose() {
		cancelWalletLoginModal();
		resetAssociationState();
		// No marcar lastHandled: el usuario puede reconectar la misma wallet.
		void authActions.logout();
	}

	function handleSocialSubmit(e: SubmitEvent) {
		e.preventDefault();
		socialAttempted = true;
		if (!socialFormValid) return;

		associationEmail = socialEmailTrimmed;
		associationName = socialName.trim() ? socialName.trim() : null;
		// Solo social confía el email del proveedor para linkear cuentas existentes.
		associationAllowLinking = walletAuthState.isSocial;

		pendingChallenge = null;
		socialModalOpen = false;

		void runWeb3Login({
			email: associationEmail,
			name: associationName,
			allowLinking: associationAllowLinking
		});
	}

	function handleWalletAuthChange() {
		if (!walletAuthState.isConnected || !walletAuthState.address) {
			lastHandledAddress = '';
			resetAssociationState();
			socialModalOpen = false;
			return;
		}

		if (authFlowInFlight || socialModalOpen) return;

		const address = normalizeAddress(walletAuthState.address);
		if (lastHandledAddress && normalizeAddress(lastHandledAddress) === address) return;

		// SOCIAL (Google / email Reown)
		if (walletAuthState.isSocial) {
			const socialEmailFromProvider = walletAuthState.email?.trim() || null;
			if (socialEmailFromProvider) {
				associationEmail = socialEmailFromProvider;
				associationName = walletAuthState.name ?? null;
				associationAllowLinking = true;
				void runWeb3Login({
					email: socialEmailFromProvider,
					name: walletAuthState.name ?? null,
					allowLinking: true
				});
				return;
			}
			// Social sin email del proveedor → pedir en modal (runWeb3Login lo abre)
			void runWeb3Login({ allowLinking: true });
			return;
		}

		// EOA: si ya hay email de asociación, login directo; si no, runWeb3Login abre modal
		if (associationEmail) {
			void runWeb3Login({
				email: associationEmail,
				name: associationName,
				allowLinking: associationAllowLinking
			});
			return;
		}

		void runWeb3Login();
	}

	async function retryWalletLogin() {
		if (authFlowInFlight) return;
		error = '';
		if (walletAuthState.isSocial && walletAuthState.email?.trim()) {
			void runWeb3Login({
				email: walletAuthState.email.trim(),
				name: walletAuthState.name ?? null,
				allowLinking: true
			});
			return;
		}
		if (associationEmail) {
			void runWeb3Login({
				email: associationEmail,
				name: associationName,
				allowLinking: associationAllowLinking
			});
			return;
		}
		void runWeb3Login();
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
				maxLength={254}
				minLength={5}
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
				<div class="w-full rounded-lg border border-green-200 bg-green-50 p-4">
					<div class="flex flex-col gap-1">
						<span class="text-[10px] font-bold text-green-700 uppercase">Wallet Conectada</span>
						<p class="truncate font-mono text-xs text-green-900">{walletAuthState.address}</p>
						{#if walletAuthState.email || associationEmail}
							<p class="text-xs text-green-800">
								<strong>Email:</strong>
								{walletAuthState.email ?? associationEmail}
							</p>
						{/if}
					</div>

					<div class="mt-4 flex gap-2">
						<button
							type="button"
							onclick={() => authActions.openLogin()}
							class="flex-1 rounded-md border border-gray-300 bg-white py-2 text-xs transition hover:bg-gray-50"
						>
							Ver Perfil
						</button>

						<button
							type="button"
							onclick={() => authActions.logout()}
							class="flex-1 rounded-md border border-red-200 bg-red-50 py-2 text-xs text-red-600 transition hover:bg-red-100"
						>
							Desconectar
						</button>
					</div>

					{#if error && status === false}
						<button
							type="button"
							onclick={() => retryWalletLogin()}
							class="mt-3 w-full rounded-md border border-blue-200 bg-blue-50 py-2 text-xs font-medium text-blue-700 transition hover:bg-blue-100"
						>
							Reintentar firma / login con wallet
						</button>
					{/if}
				</div>
			{:else}
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

		{#if error && status !== true}
			<div
				class="rounded-lg border border-red-300 bg-red-100 p-3 text-sm font-medium text-red-700 dark:border-red-700 dark:bg-red-900 dark:text-red-200"
			>
				{error}
			</div>
		{/if}

		<div class="space-y-4">
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
