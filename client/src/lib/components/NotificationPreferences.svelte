<script lang="ts">
	import { onMount } from 'svelte';
	import { Mail, Bell, Loader2 } from 'lucide-svelte';
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
		investment_matured: 'Inversión madurada',
		new_member_added: 'Invitación a un grupo',
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

	const isGroupMode = $derived(!!groupId);
	const contextLabel = $derived(isGroupMode ? 'para este grupo' : 'globales');

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

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-start gap-3">
		<div
			class="mt-0.5 flex h-9 w-9 items-center justify-center rounded-2xl border border-border bg-muted/50 text-muted-foreground"
		>
			<Mail class="h-4 w-4" />
		</div>
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2">
				<h3 class="text-base font-semibold tracking-tight text-foreground">
					Preferencias de notificaciones
				</h3>
				<span
					class="rounded-full border border-border bg-muted px-2.5 py-0.5 text-[10px] font-medium text-muted-foreground"
				>
					{contextLabel}
				</span>
			</div>
			<p class="mt-1 text-sm text-muted-foreground">
				Elige qué notificaciones recibir y por qué medio. Los cambios se guardan automáticamente.
			</p>
		</div>
	</div>

	{#if loading}
		<div class="rounded-3xl border border-border bg-card p-8">
			<div class="flex flex-col items-center gap-3 text-muted-foreground">
				<Loader2 class="h-5 w-5 animate-spin" />
				<p class="text-sm">Cargando preferencias...</p>
			</div>
		</div>
	{:else if error && !events.length}
		<div
			class="rounded-3xl border border-red-200 bg-red-50 p-5 text-sm text-red-700 dark:border-red-400/30 dark:bg-red-400/10 dark:text-red-300"
		>
			{error}
		</div>
	{:else if events.length === 0}
		<div
			class="rounded-3xl border border-dashed border-border bg-card p-8 text-center text-sm text-muted-foreground"
		>
			No hay tipos de notificación configurables todavía.
		</div>
	{:else}
		<!-- Preferences Table -->
		<div class="overflow-hidden rounded-3xl border border-border bg-card shadow-sm">
			<!-- Column headers -->
			<div
				class="grid grid-cols-[minmax(0,1fr)_80px_80px] items-center border-b border-border bg-muted/30 px-5 py-3 text-xs font-medium tracking-wide text-muted-foreground"
			>
				<div class="pl-1">Tipo de notificación</div>
				<div class="flex items-center justify-center gap-1.5">
					<Mail class="h-3.5 w-3.5" />
					<span>Email</span>
				</div>
				<div class="flex items-center justify-center gap-1.5">
					<Bell class="h-3.5 w-3.5" />
					<span>Web/App</span>
				</div>
			</div>

			<!-- Rows -->
			<div class="divide-y divide-border/60">
				{#each events as ev (ev.id)}
					<div
						class="grid grid-cols-[minmax(0,1fr)_80px_80px] items-center px-5 py-4 transition-colors hover:bg-muted/20"
					>
						<!-- Event name -->
						<div class="pr-4">
							<p class="text-sm leading-tight font-medium text-foreground">
								{getLabel(ev.name)}
							</p>
						</div>

						<!-- Toggles -->
						{#each channels as ch (ch.id)}
							{@const key = getKey(ev.id, ch.id)}
							{@const isOn = enabledMap[key] ?? true}

							<div class="flex justify-center">
								<!-- Pretty toggle switch -->
								<button
									type="button"
									class="group relative flex h-5 w-9 cursor-pointer items-center rounded-full border border-border bg-muted p-0.5 transition-all hover:border-border/80 focus:outline-none focus-visible:ring-2 focus-visible:ring-ring/50 active:scale-[0.985] {isOn
										? 'border-emerald-500/90 bg-emerald-500/90'
										: ''}"
									onclick={() => toggle(ev.id, ch.id)}
									aria-pressed={isOn}
									aria-label={`${getLabel(ev.name)} - ${getChannelLabel(ch.name)}`}
								>
									<span
										class="block h-3.5 w-3.5 rounded-full bg-white shadow-sm transition-all duration-150 {isOn
											? 'translate-x-4'
											: 'translate-x-0 group-hover:scale-105'}"
									></span>
								</button>
							</div>
						{/each}
					</div>
				{/each}
			</div>
		</div>

		<!-- Feedback -->
		<div class="flex items-center justify-between px-1 text-xs">
			<p class="text-muted-foreground">Los cambios se aplican de inmediato.</p>

			<div class="min-h-[1rem]">
				{#if saveMessage}
					<span
						class="inline-flex items-center gap-1 rounded-full bg-emerald-500/10 px-2.5 py-0.5 font-medium text-emerald-600 dark:bg-emerald-400/10 dark:text-emerald-400"
					>
						✓ {saveMessage}
					</span>
				{:else if error}
					<span class="text-red-600 dark:text-red-400">{error}</span>
				{/if}
			</div>
		</div>
	{/if}
</div>
