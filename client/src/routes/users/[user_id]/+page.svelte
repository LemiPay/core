<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth';
	import { ArrowLeft, Mail, User as UserIcon, Hash, UserPlus, ShieldCheck } from 'lucide-svelte';
	import { userInfo } from '$lib/api/auth';
	import { fly, fade, scale } from 'svelte/transition';

	type UserSummary = {
		id: string;
		name: string;
		email: string;
	};

	const userId = $derived(page.params.user_id);

	let isLoading = $state(true);
	let error = $state('');
	let user = $state<UserSummary | null>(null);

	const initials = $derived(
		(user?.name ?? '')
			.trim()
			.split(/\s+/)
			.filter(Boolean)
			.slice(0, 2)
			.map((part) => part[0]?.toUpperCase() ?? '')
			.join('') || '?'
	);

	function goBack() {
		if (typeof history !== 'undefined' && history.length > 1) {
			history.back();
		} else {
			window.location.href = '/dashboard';
		}
	}

	onMount(async () => {
		if (!userId) {
			error = 'No se encontró el id del usuario.';
			isLoading = false;
			return;
		}

		const currentUserId = $authStore.user?.id;
		if (currentUserId && userId === currentUserId) {
			await goto('/profile/me', { replaceState: true });
			return;
		}

		const response = await userInfo(userId);

		if (!response.ok) {
			error = response.message || 'No se pudo cargar el usuario.';
			isLoading = false;
			return;
		}

		user = response.body;
		isLoading = false;
	});
</script>

<svelte:head>
	<title>Lemipay – {user?.name ?? 'Perfil'}</title>
</svelte:head>

<div class="min-h-screen bg-background text-foreground">
	<!-- Ambient blobs (más sutiles para vista de perfil) -->
	<div
		class="pointer-events-none fixed inset-0 -z-10 bg-[radial-gradient(circle_at_50%_-20%,rgba(168,85,247,0.15),transparent_45%)]"
	></div>

	<div class="mx-auto w-full max-w-2xl px-4 pt-24 pb-16 sm:px-6">
		<!-- Back button -->
		<div in:fly={{ y: -8, duration: 280 }}>
			<button
				onclick={goBack}
				class="group mb-8 inline-flex items-center gap-2 text-sm font-medium text-muted-foreground transition hover:text-foreground"
			>
				<div
					class="flex size-8 items-center justify-center rounded-full border border-border bg-card shadow-sm transition group-hover:-translate-x-1 group-hover:border-foreground/20 group-hover:bg-muted"
				>
					<ArrowLeft class="size-4" />
				</div>
				Volver
			</button>
		</div>

		<!-- Loading skeleton -->
		{#if isLoading}
			<div
				class="overflow-hidden rounded-[2.5rem] border border-border/60 bg-card shadow-xl"
				in:fade={{ duration: 200 }}
			>
				<div class="h-32 animate-pulse bg-muted/50"></div>
				<div class="px-8 pb-8">
					<div
						class="-mt-12 mb-4 size-24 animate-pulse rounded-full border-[6px] border-card bg-muted"
					></div>
					<div class="h-6 w-48 animate-pulse rounded-lg bg-muted"></div>
					<div class="mt-2 h-4 w-32 animate-pulse rounded-lg bg-muted"></div>
				</div>
			</div>

			<!-- Error state -->
		{:else if error}
			<div
				class="rounded-[2.5rem] border border-rose-200 bg-rose-50 p-10 text-center dark:border-rose-900/30 dark:bg-rose-900/10"
				in:scale={{ duration: 220 }}
			>
				<div
					class="mx-auto flex size-14 items-center justify-center rounded-full bg-rose-100 dark:bg-rose-900/50"
				>
					<UserIcon class="size-6 text-rose-600 dark:text-rose-400" />
				</div>
				<h2 class="mt-4 text-xl font-semibold text-rose-700 dark:text-rose-300">
					Usuario no encontrado
				</h2>
				<p class="mt-2 text-sm text-rose-600/80 dark:text-rose-400/80">{error}</p>
			</div>

			<!-- User profile -->
		{:else if user}
			<div in:fly={{ y: 14, duration: 400 }} class="space-y-6">
				<!-- Tarjeta Principal (Hero Cover) -->
				<section
					class="relative overflow-hidden rounded-[2.5rem] border border-border/80 bg-card shadow-xl shadow-black/5 dark:shadow-none"
				>
					<!-- Banner Header -->
					<div
						class="h-32 w-full bg-linear-to-r from-violet-500/20 via-sky-400/20 to-lime-500/20 dark:from-violet-500/10 dark:via-sky-400/10 dark:to-lime-500/10"
					></div>

					<!-- Floating Action: Add Friend (Top Right) -->
					<div class="absolute top-4 right-4 sm:top-6 sm:right-6">
						<button
							title="Enviar solicitud de amistad"
							aria-label="Agregar amigo"
							class="group flex size-11 items-center justify-center rounded-full bg-background/80 text-foreground shadow-sm backdrop-blur-md transition hover:scale-105 hover:bg-foreground hover:text-background focus:ring-2 focus:ring-ring focus:outline-none"
						>
							<UserPlus class="size-5 transition-transform group-hover:rotate-6" />
						</button>
					</div>

					<div class="px-6 pt-0 pb-8 sm:px-10">
						<!-- Avatar Overlapping -->
						<div class="flex items-end gap-5">
							<div
								class="-mt-14 flex size-28 shrink-0 items-center justify-center rounded-full border-[6px] border-card bg-linear-to-br from-violet-200 to-lime-200 text-4xl font-bold text-violet-800 shadow-md select-none dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
							>
								{initials}
							</div>
						</div>

						<!-- Identidad -->
						<div class="mt-4">
							<h1 class="text-3xl font-bold tracking-tight">{user.name}</h1>
							<div class="mt-1 flex items-center gap-2 text-muted-foreground">
								<ShieldCheck class="size-4 text-emerald-500" />
								<span class="text-sm font-medium">Cuenta LemiPay Verificada</span>
							</div>
						</div>
					</div>
				</section>

				<!-- Bloques de Información -->
				<section class="grid gap-4 sm:grid-cols-2" in:fly={{ y: 10, duration: 360, delay: 80 }}>
					<!-- Email Card -->
					<div
						class="group flex items-center gap-4 rounded-3xl border border-border/60 bg-card/50 p-5 backdrop-blur transition hover:border-border hover:bg-muted/50"
					>
						<div
							class="flex size-12 shrink-0 items-center justify-center rounded-2xl bg-sky-400/10 text-sky-600 transition group-hover:scale-105 dark:text-sky-400"
						>
							<Mail class="size-5" />
						</div>
						<div class="min-w-0 flex-1">
							<p class="text-xs font-medium text-muted-foreground">Contacto</p>
							<p class="mt-0.5 truncate text-sm font-semibold">{user.email}</p>
						</div>
					</div>

					<!-- ID Card -->
					<div
						class="group flex items-center gap-4 rounded-3xl border border-border/60 bg-card/50 p-5 backdrop-blur transition hover:border-border hover:bg-muted/50"
					>
						<div
							class="flex size-12 shrink-0 items-center justify-center rounded-2xl bg-muted text-muted-foreground transition group-hover:scale-105"
						>
							<Hash class="size-5" />
						</div>
						<div class="min-w-0 flex-1">
							<p class="text-xs font-medium text-muted-foreground">LemiPay ID</p>
							<p class="mt-0.5 truncate font-mono text-xs font-medium text-foreground/80">
								{user.id}
							</p>
						</div>
					</div>
				</section>
			</div>

			<!-- Fallback -->
		{:else}
			<div class="rounded-[2.5rem] border border-dashed border-border bg-card p-12 text-center">
				<div class="mx-auto flex size-14 items-center justify-center rounded-3xl bg-muted">
					<UserIcon class="size-6 text-muted-foreground" />
				</div>
				<h3 class="mt-5 text-lg font-semibold">Sin información disponible</h3>
				<p class="mt-2 text-sm text-muted-foreground">No hay datos para mostrar de este usuario.</p>
			</div>
		{/if}
	</div>
</div>
