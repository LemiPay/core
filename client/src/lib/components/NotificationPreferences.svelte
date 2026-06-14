<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth';
	import {
		getNotificationEvents,
		getNotificationChannels,
		getUserPreferences,
		upsertUserPreference,
		getGroupPreferences,
		upsertGroupPreference
	} from '$lib/api/endpoints/notifications';
	import { isSuccess } from '$lib/types/client.types';
	import type {
		NotificationEvent,
		NotificationChannel,
		UpsertPreferenceRequest
	} from '$lib/types/endpoints/notifications.types';

	interface Props {
		groupId?: string;
	}

	let { groupId }: Props = $props();

	let loading = $state(true);
	let error = $state('');
	let saveMessage = $state('');

	let events = $state<NotificationEvent[]>([]);
	let channels = $state<NotificationChannel[]>([]);

	// key = `${eventId}:${channelId}` -> enabled
	let enabledMap = $state<Record<string, boolean>>({});

	const EVENT_LABELS: Record<string, string> = {
		proposal_created: 'Nueva propuesta creada',
		proposal_approved: 'Propuesta aprobada',
		proposal_rejected: 'Propuesta rechazada',
		proposal_executed: 'Propuesta ejecutada',
		fund_round_created: 'Ronda de fondeo creada',
		investment_created: 'Inversión creada',
		new_member_added: 'Nuevo miembro agregado',
		expense_created: 'Gasto creado'
	};

	const CHANNEL_LABELS: Record<string, string> = {
		email: 'Email',
		web: 'Web / App'
	};

	function getKey(eventId: string, channelId: string) {
		return `${eventId}:${channelId}`;
	}

	function getLabel(name: string) {
		return EVENT_LABELS[name] || name;
	}

	function getChannelLabel(name: string) {
		return CHANNEL_LABELS[name] || name;
	}

	async function load() {
		loading = true;
		error = '';
		try {
			const [evRes, chRes] = await Promise.all([
				getNotificationEvents(),
				getNotificationChannels()
			]);

			if (!isSuccess(evRes) || !isSuccess(chRes)) {
				error = 'No se pudieron cargar las opciones de notificaciones.';
				loading = false;
				return;
			}

			events = evRes.body;
			channels = chRes.body;

			let prefs: any[] = [];
			if (groupId) {
				const pRes = await getGroupPreferences(groupId);
				if (isSuccess(pRes)) prefs = pRes.body;
			} else {
				const pRes = await getUserPreferences();
				if (isSuccess(pRes)) prefs = pRes.body;
			}

			const map: Record<string, boolean> = {};
			for (const ev of events) {
				for (const ch of channels) {
					const key = getKey(ev.id, ch.id);
					const found = prefs.find((p: any) => p.event_id === ev.id && p.channel_id === ch.id);
					map[key] = found ? found.enabled : true;
				}
			}
			enabledMap = map;
		} catch (e) {
			console.error(e);
			error = 'Error cargando preferencias.';
		} finally {
			loading = false;
		}
	}

	async function toggle(eventId: string, channelId: string) {
		const key = getKey(eventId, channelId);
		const current = enabledMap[key] ?? true;
		const next = !current;

		// optimistic
		enabledMap = { ...enabledMap, [key]: next };

		const payload: UpsertPreferenceRequest = {
			event_id: eventId,
			channel_id: channelId,
			enabled: next
		};

		try {
			let res;
			if (groupId) {
				res = await upsertGroupPreference(groupId, payload);
			} else {
				res = await upsertUserPreference(payload);
			}

			if (!isSuccess(res)) {
				throw new Error(res.message || 'Error al guardar');
			}

			saveMessage = 'Guardado';
			setTimeout(() => {
				if (saveMessage === 'Guardado') saveMessage = '';
			}, 1400);
		} catch (e) {
			// revert
			enabledMap = { ...enabledMap, [key]: current };
			error = 'No se pudo guardar el cambio. Intenta de nuevo.';
			setTimeout(() => (error = ''), 2500);
			console.error('upsert pref failed', e);
		}
	}

	onMount(() => {
		load();
	});
</script>

<div class="space-y-4">
	<div>
		<h3 class="text-sm font-semibold text-foreground">Preferencias de notificaciones</h3>
		<p class="mt-1 text-xs text-muted-foreground">
			Elige qué notificaciones recibir y por qué medio. Los cambios se guardan automáticamente.
		</p>
	</div>

	{#if loading}
		<div class="flex justify-center py-6">
			<div
				class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"
			></div>
		</div>
	{:else if error && !events.length}
		<div
			class="rounded-xl border border-red-200 bg-red-50 p-3 text-sm text-red-600 dark:border-red-400/30 dark:bg-red-400/10"
		>
			{error}
		</div>
	{:else if events.length === 0}
		<div
			class="rounded-xl border border-dashed border-border bg-card p-4 text-sm text-muted-foreground"
		>
			No hay tipos de notificación configurables todavía.
		</div>
	{:else}
		<div class="space-y-3">
			{#each events as ev (ev.id)}
				<div class="rounded-xl border border-border bg-card p-3">
					<div class="mb-2 text-sm font-medium text-foreground">
						{getLabel(ev.name)}
					</div>

					<div class="flex flex-wrap items-center gap-x-6 gap-y-2">
						{#each channels as ch (ch.id)}
							{@const key = getKey(ev.id, ch.id)}
							{@const isOn = enabledMap[key] ?? true}

							<label class="flex cursor-pointer items-center gap-2 text-sm text-muted-foreground">
								<input
									type="checkbox"
									checked={isOn}
									onchange={() => toggle(ev.id, ch.id)}
									class="h-4 w-4 rounded border-border text-foreground accent-foreground focus:ring-0"
								/>
								<span>{getChannelLabel(ch.name)}</span>
							</label>
						{/each}
					</div>
				</div>
			{/each}
		</div>

		{#if saveMessage}
			<p class="text-right text-xs text-emerald-600 dark:text-emerald-400">{saveMessage}</p>
		{/if}

		{#if error}
			<p class="text-xs text-red-600">{error}</p>
		{/if}
	{/if}
</div>
