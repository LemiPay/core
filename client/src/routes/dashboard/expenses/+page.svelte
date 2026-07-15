<script lang="ts">
	import DashboardLayout from '../DashboardLayout.svelte';
	import { onMount } from 'svelte';
	import {
		getFriends,
		getReceivedRequests,
		getSentRequests,
		sendFriendRequest,
		respondToFriendRequest,
		unfriend
	} from '$lib/api/endpoints/friends';
	import { isSuccess } from '$lib/types/client.types';
	import type { FriendResponse } from '$lib/types/endpoints/friends.types';
	import {
		UserPlus,
		UserCheck,
		Clock,
		MessageCircle,
		Trash2,
		Check,
		X,
		Users,
		Search
	} from 'lucide-svelte';

	let friends = $state<FriendResponse[]>([]);
	let received = $state<FriendResponse[]>([]);
	let sent = $state<FriendResponse[]>([]);
	let loading = $state(true);
	let activeTab = $state<'friends' | 'received' | 'sent'>('friends');

	async function loadAll() {
		loading = true;
		const [fRes, rRes, sRes] = await Promise.all([
			getFriends(),
			getReceivedRequests(),
			getSentRequests()
		]);
		if (isSuccess(fRes)) friends = fRes.body;
		if (isSuccess(rRes)) received = rRes.body;
		if (isSuccess(sRes)) sent = sRes.body;
		loading = false;
	}

	async function handleAccept(userId: string) {
		const res = await respondToFriendRequest(userId, 'accept');
		if (isSuccess(res)) await loadAll();
	}

	async function handleReject(userId: string) {
		const res = await respondToFriendRequest(userId, 'reject');
		if (isSuccess(res)) await loadAll();
	}

	async function handleUnfriend(userId: string) {
		const res = await unfriend(userId);
		if (isSuccess(res)) await loadAll();
	}

	async function handleCancel(userId: string) {
		const res = await unfriend(userId);
		if (isSuccess(res)) await loadAll();
	}

	onMount(loadAll);
</script>

<DashboardLayout>
	<div class="space-y-6">
		<div class="flex items-center justify-between">
			<h1 class="text-2xl font-bold tracking-tight">Amigos</h1>
		</div>

		<div class="flex w-fit gap-1 rounded-xl bg-muted p-1">
			<button
				onclick={() => (activeTab = 'friends')}
				class="flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition"
				class:bg-background={activeTab === 'friends'}
				class:text-foreground={activeTab === 'friends'}
				class:text-muted-foreground={activeTab !== 'friends'}
			>
				<UserCheck class="size-4" />
				Amigos ({friends.length})
			</button>
			<button
				onclick={() => (activeTab = 'received')}
				class="flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition"
				class:bg-background={activeTab === 'received'}
				class:text-foreground={activeTab === 'received'}
				class:text-muted-foreground={activeTab !== 'received'}
			>
				<UserPlus class="size-4" />
				Recibidas ({received.length})
			</button>
			<button
				onclick={() => (activeTab = 'sent')}
				class="flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition"
				class:bg-background={activeTab === 'sent'}
				class:text-foreground={activeTab === 'sent'}
				class:text-muted-foreground={activeTab !== 'sent'}
			>
				<Clock class="size-4" />
				Enviadas ({sent.length})
			</button>
		</div>

		{#if loading}
			<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
				{#each Array(6) as _}
					<div class="animate-pulse space-y-3 rounded-2xl border border-border/60 bg-card p-5">
						<div class="flex items-center gap-3">
							<div class="size-12 rounded-full bg-muted"></div>
							<div class="flex-1 space-y-2">
								<div class="h-4 w-24 rounded bg-muted"></div>
								<div class="h-3 w-32 rounded bg-muted"></div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{:else if activeTab === 'friends'}
			{#if friends.length === 0}
				<div
					class="flex flex-col items-center gap-3 rounded-2xl border border-dashed border-border bg-card/50 p-12 text-center"
				>
					<Users class="size-10 text-muted-foreground/50" />
					<p class="text-sm text-muted-foreground">Todavía no tenés amigos.</p>
					<p class="text-xs text-muted-foreground/70">
						Buscá usuarios desde el perfil o al invitar a un grupo.
					</p>
				</div>
			{:else}
				<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
					{#each friends as f}
						<div
							class="group rounded-2xl border border-border/60 bg-card p-5 transition hover:border-border hover:shadow-sm"
						>
							<div class="flex items-center gap-3">
								<a href="/users/{f.user_id}" class="shrink-0">
									<div
										class="flex size-12 items-center justify-center rounded-full bg-linear-to-br from-violet-200 to-lime-200 text-sm font-bold text-violet-800 dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
									>
										{f.name.slice(0, 2).toUpperCase()}
									</div>
								</a>
								<div class="min-w-0 flex-1">
									<a href="/users/{f.user_id}" class="hover:underline">
										<p class="truncate text-sm font-semibold text-foreground">{f.name}</p>
									</a>
									<p class="truncate text-xs text-muted-foreground">{f.email}</p>
									<div class="mt-1 flex items-center gap-1">
										<UserCheck class="size-3 text-emerald-500" />
										<span class="text-[10px] font-medium text-emerald-600 dark:text-emerald-400"
											>Amigos</span
										>
									</div>
								</div>
								<button
									onclick={() => handleUnfriend(f.user_id)}
									class="flex size-8 shrink-0 items-center justify-center rounded-full text-muted-foreground opacity-0 transition group-hover:opacity-100 hover:bg-red-50 hover:text-red-500"
									title="Eliminar amigo"
								>
									<Trash2 class="size-4" />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{:else if activeTab === 'received'}
			{#if received.length === 0}
				<div
					class="flex flex-col items-center gap-3 rounded-2xl border border-dashed border-border bg-card/50 p-12 text-center"
				>
					<UserPlus class="size-10 text-muted-foreground/50" />
					<p class="text-sm text-muted-foreground">No tenés solicitudes de amistad pendientes.</p>
				</div>
			{:else}
				<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
					{#each received as r}
						<div class="rounded-2xl border border-border/60 bg-card p-5">
							<div class="flex items-center gap-3">
								<a href="/users/{r.user_id}" class="shrink-0">
									<div
										class="flex size-12 items-center justify-center rounded-full bg-linear-to-br from-violet-200 to-lime-200 text-sm font-bold text-violet-800 dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
									>
										{r.name.slice(0, 2).toUpperCase()}
									</div>
								</a>
								<div class="min-w-0 flex-1">
									<a href="/users/{r.user_id}" class="hover:underline">
										<p class="truncate text-sm font-semibold text-foreground">{r.name}</p>
									</a>
									<p class="truncate text-xs text-muted-foreground">{r.email}</p>
								</div>
								<div class="flex shrink-0 gap-1">
									<button
										onclick={() => handleAccept(r.user_id)}
										class="flex size-8 items-center justify-center rounded-full bg-emerald-100 text-emerald-600 transition hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400"
										title="Aceptar"
									>
										<Check class="size-4" />
									</button>
									<button
										onclick={() => handleReject(r.user_id)}
										class="flex size-8 items-center justify-center rounded-full bg-red-100 text-red-500 transition hover:bg-red-200 dark:bg-red-900/30 dark:text-red-400"
										title="Rechazar"
									>
										<X class="size-4" />
									</button>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{:else if activeTab === 'sent'}
			{#if sent.length === 0}
				<div
					class="flex flex-col items-center gap-3 rounded-2xl border border-dashed border-border bg-card/50 p-12 text-center"
				>
					<Clock class="size-10 text-muted-foreground/50" />
					<p class="text-sm text-muted-foreground">No enviaste solicitudes de amistad.</p>
				</div>
			{:else}
				<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
					{#each sent as s}
						<div class="rounded-2xl border border-border/60 bg-card p-5">
							<div class="flex items-center gap-3">
								<a href="/users/{s.user_id}" class="shrink-0">
									<div
										class="flex size-12 items-center justify-center rounded-full bg-linear-to-br from-violet-200 to-lime-200 text-sm font-bold text-violet-800 dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
									>
										{s.name.slice(0, 2).toUpperCase()}
									</div>
								</a>
								<div class="min-w-0 flex-1">
									<a href="/users/{s.user_id}" class="hover:underline">
										<p class="truncate text-sm font-semibold text-foreground">{s.name}</p>
									</a>
									<p class="truncate text-xs text-muted-foreground">{s.email}</p>
									<div class="mt-1 flex items-center gap-1">
										<Clock class="size-3 text-amber-500" />
										<span class="text-[10px] font-medium text-amber-600 dark:text-amber-400"
											>Pendiente</span
										>
									</div>
								</div>
								<button
									onclick={() => handleCancel(s.user_id)}
									class="flex size-8 shrink-0 items-center justify-center rounded-full text-muted-foreground transition hover:bg-red-50 hover:text-red-500"
									title="Cancelar solicitud"
								>
									<X class="size-4" />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</DashboardLayout>
