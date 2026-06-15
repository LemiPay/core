<script lang="ts">
	import { BellRing, Inbox, MailOpen } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import DashboardLayout from '../DashboardLayout.svelte';
	import NotificationActivityList from '$lib/components/NotificationActivityList.svelte';
	import {
		getNotifications,
		markAllNotificationsRead,
		markNotificationRead
	} from '$lib/api/endpoints/notifications';
	import { isSuccess } from '$lib/types/client.types';
	import type { NotificationRecord } from '$lib/types/endpoints/notifications.types';

	type ReadFilter = 'all' | 'unread' | 'read';

	let loading = $state(true);
	let error = $state('');
	let filter = $state<ReadFilter>('unread');
	let notifications = $state<NotificationRecord[]>([]);

	const filterOptions: { value: ReadFilter; label: string; icon: typeof Inbox }[] = [
		{ value: 'all', label: 'Todas', icon: Inbox },
		{ value: 'unread', label: 'No leídas', icon: BellRing },
		{ value: 'read', label: 'Leídas', icon: MailOpen }
	];

	async function loadNotifications() {
		loading = true;
		error = '';

		const params = filter === 'all' ? undefined : { read: filter === 'read' };

		const response = await getNotifications(params);

		if (!isSuccess(response)) {
			error = 'No se pudieron cargar las notificaciones.';
			loading = false;
			return;
		}

		notifications = response.body;
		loading = false;
	}

	async function handleMarkRead(id: string) {
		const response = await markNotificationRead(id);
		if (!isSuccess(response)) {
			error = 'No se pudo marcar la notificación como leída.';
			return;
		}

		notifications = notifications.map((n) => (n.id === id ? { ...n, read: true } : n));
		if (filter === 'unread') {
			notifications = notifications.filter((n) => n.id !== id);
		}
	}

	async function handleMarkAllRead() {
		const response = await markAllNotificationsRead();
		if (!isSuccess(response)) {
			error = 'No se pudieron marcar todas como leídas.';
			return;
		}

		if (filter === 'unread') {
			notifications = [];
		} else {
			notifications = notifications.map((n) => ({ ...n, read: true }));
		}
	}

	$effect(() => {
		filter;
		void loadNotifications();
	});
</script>

<DashboardLayout>
	<section class="space-y-6">
		<div
			class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
			in:fly={{ y: 12, duration: 360 }}
		>
			<div>
				<p class="text-sm font-medium text-muted-foreground">Tu historial</p>
				<h1 class="mt-1 text-3xl font-semibold tracking-tight">Actividad</h1>
				<p class="mt-2 max-w-xl text-sm text-muted-foreground">
					Todas las notificaciones de tus grupos en un solo lugar.
				</p>
			</div>
			<div
				class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
			>
				<BellRing class="size-4" />
			</div>
		</div>

		<div class="flex flex-wrap gap-2">
			{#each filterOptions as option (option.value)}
				{@const Icon = option.icon}
				<button
					type="button"
					onclick={() => (filter = option.value)}
					class={filter === option.value
						? 'inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2.5 text-sm font-semibold text-background shadow-sm'
						: 'inline-flex items-center gap-2 rounded-2xl border border-border px-4 py-2.5 text-sm font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground'}
				>
					<Icon class="size-4" />
					{option.label}
				</button>
			{/each}
		</div>

		<section class="rounded-[2rem] border border-border bg-card p-5 shadow-sm sm:p-6">
			{#if loading}
				<p class="text-sm text-muted-foreground">Cargando actividad...</p>
			{:else if error}
				<p class="text-sm text-destructive">{error}</p>
			{:else}
				<NotificationActivityList
					{notifications}
					markRead={handleMarkRead}
					showMarkAll={filter !== 'read'}
					markAllRead={handleMarkAllRead}
					emptyMessage="No hay notificaciones en esta categoría."
				/>
			{/if}
		</section>
	</section>
</DashboardLayout>
