<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/client';
	import { ArrowLeft, Mail, User as UserIcon } from 'lucide-svelte';

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
			error = 'No se encontro el id del usuario.';
			isLoading = false;
			return;
		}

		const response = await apiFetch<UserSummary>(`/users/${userId}`);

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
	<title>Lemipay - Perfil</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-6 p-6 pt-8">
	<button
		onclick={goBack}
		class="flex w-fit items-center gap-2 rounded-full border border-gray-200 px-3 py-1.5 text-xs font-medium text-gray-600 transition hover:border-gray-400 hover:text-black"
	>
		<ArrowLeft class="h-3.5 w-3.5" />
		Volver
	</button>

	<h1 class="text-xl font-bold text-black">Perfil de usuario</h1>

	{#if isLoading}
		<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
			<div class="flex items-center gap-4">
				<div class="h-16 w-16 animate-pulse rounded-full bg-gray-200"></div>
				<div class="flex flex-1 flex-col gap-2">
					<div class="h-4 w-1/2 animate-pulse rounded bg-gray-200"></div>
					<div class="h-3 w-2/3 animate-pulse rounded bg-gray-200"></div>
				</div>
			</div>
		</div>
	{:else if error}
		<div class="rounded-lg border border-red-300 bg-red-50 p-4 text-red-700">
			<p class="font-medium">Hubo un problema al cargar el usuario.</p>
			<p class="text-sm">{error}</p>
		</div>
	{:else if user}
		<div class="flex flex-col gap-6 rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
			<div class="flex items-center gap-4">
				<div
					class="flex h-16 w-16 shrink-0 items-center justify-center rounded-full bg-linear-to-br from-gray-200 to-gray-300 text-lg font-semibold text-gray-700 select-none"
					aria-hidden="true"
				>
					{initials}
				</div>
				<div class="min-w-0">
					<p class="truncate text-lg font-semibold text-black">{user.name}</p>
					<p class="truncate text-sm text-gray-500">{user.email}</p>
				</div>
			</div>

			<div class="h-px w-full bg-gray-100"></div>

			<dl class="flex flex-col gap-4 text-sm">
				<div class="flex items-start gap-3">
					<UserIcon class="mt-0.5 h-4 w-4 text-gray-400" />
					<div class="flex flex-col">
						<dt class="text-xs font-medium text-gray-500">Nombre</dt>
						<dd class="text-sm text-black">{user.name}</dd>
					</div>
				</div>

				<div class="flex items-start gap-3">
					<Mail class="mt-0.5 h-4 w-4 text-gray-400" />
					<div class="flex flex-col">
						<dt class="text-xs font-medium text-gray-500">Email</dt>
						<dd class="text-sm text-black">{user.email}</dd>
					</div>
				</div>

				<div class="flex items-start gap-3">
					<span class="mt-0.5 text-xs font-semibold text-gray-400">ID</span>
					<div class="flex flex-col">
						<dt class="text-xs font-medium text-gray-500">Identificador</dt>
						<dd class="font-mono text-xs break-all text-gray-700">{user.id}</dd>
					</div>
				</div>
			</dl>
		</div>
	{:else}
		<p class="text-sm text-gray-500">No hay informacion disponible para este usuario.</p>
	{/if}
</div>
