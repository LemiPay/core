<script lang="ts">
	import { onMount } from 'svelte';
	import { Activity, CheckCircle2, Link2, RefreshCw, Server, XCircle } from 'lucide-svelte';

	import { getApiUrl } from '$lib/api/client';
	import { getHealth } from '$lib/api/endpoints/health';
	import { isSuccess } from '$lib/types/client.types';
	import Button from '$lib/components/ui/Button.svelte';

	type ServiceState = 'checking' | 'ok' | 'error';

	type ServiceStatus = {
		name: string;
		description: string;
		state: ServiceState;
		detail: string;
		latencyMs: number | null;
		url?: string | null;
	};

	let loading = $state(true);
	let lastCheckedAt = $state<Date | null>(null);
	let apiUrl = $state<string | null>(null);
	let services = $state<ServiceStatus[]>([
		{
			name: 'Frontend',
			description: 'Cliente web de LemiPay',
			state: 'ok',
			detail: 'Disponible',
			latencyMs: null
		},
		{
			name: 'API',
			description: 'Backend HTTP (Axum)',
			state: 'checking',
			detail: 'Comprobando…',
			latencyMs: null,
			url: null
		}
	]);

	const overallState = $derived.by((): ServiceState => {
		if (services.some((s) => s.state === 'checking')) return 'checking';
		if (services.some((s) => s.state === 'error')) return 'error';
		return 'ok';
	});

	const overallLabel = $derived(
		overallState === 'ok'
			? 'Todos los sistemas operativos'
			: overallState === 'checking'
				? 'Comprobando estado…'
				: 'Hay problemas en el sistema'
	);

	async function checkStatus() {
		loading = true;

		try {
			apiUrl = getApiUrl();
		} catch {
			apiUrl = null;
		}

		services = services.map((s) =>
			s.name === 'API'
				? {
						...s,
						state: 'checking',
						detail: 'Comprobando…',
						latencyMs: null,
						url: apiUrl ? `${apiUrl}/health` : null
					}
				: s
		);

		const started = performance.now();

		try {
			const res = await getHealth();
			const latencyMs = Math.round(performance.now() - started);

			if (isSuccess(res) && res.body?.status?.toLowerCase() === 'ok') {
				services = services.map((s) =>
					s.name === 'API'
						? {
								...s,
								state: 'ok',
								detail: `Respuesta: ${res.body.status.toUpperCase()}`,
								latencyMs,
								url: apiUrl ? `${apiUrl}/health` : null
							}
						: s
				);
			} else {
				const detail = res.ok
					? `Respuesta inesperada: ${JSON.stringify(res.body)}`
					: res.status === 0
						? `API no disponible: ${res.message}`
						: `Error HTTP ${res.status}: ${res.message}`;

				services = services.map((s) =>
					s.name === 'API'
						? {
								...s,
								state: 'error',
								detail,
								latencyMs,
								url: apiUrl ? `${apiUrl}/health` : null
							}
						: s
				);
			}
		} catch (err) {
			const latencyMs = Math.round(performance.now() - started);
			const message = err instanceof Error ? err.message : 'No se pudo contactar la API';
			services = services.map((s) =>
				s.name === 'API'
					? {
							...s,
							state: 'error',
							detail: message,
							latencyMs,
							url: apiUrl ? `${apiUrl}/health` : null
						}
					: s
			);
		} finally {
			lastCheckedAt = new Date();
			loading = false;
		}
	}

	function stateDotClass(state: ServiceState) {
		if (state === 'ok') return 'bg-emerald-500 shadow-emerald-500/40';
		if (state === 'error') return 'bg-red-500 shadow-red-500/40';
		return 'bg-amber-400 shadow-amber-400/40 animate-pulse';
	}

	function stateBadgeClass(state: ServiceState) {
		if (state === 'ok')
			return 'bg-emerald-500/10 text-emerald-700 border-emerald-500/20 dark:text-emerald-400';
		if (state === 'error') return 'bg-red-500/10 text-red-700 border-red-500/20 dark:text-red-400';
		return 'bg-amber-500/10 text-amber-700 border-amber-500/20 dark:text-amber-400';
	}

	function stateLabel(state: ServiceState) {
		if (state === 'ok') return 'Operativo';
		if (state === 'error') return 'Caído';
		return 'Comprobando';
	}

	onMount(() => {
		checkStatus();
	});
</script>

<svelte:head>
	<title>Estado del sistema · LemiPay</title>
</svelte:head>

<div class="flex flex-1 items-start justify-center bg-background px-4 py-12 text-foreground">
	<div class="w-full max-w-2xl space-y-8">
		<header class="space-y-3">
			<div class="flex items-center gap-2 text-sm font-medium text-muted-foreground">
				<Activity class="size-4" />
				<span>Healthcheck</span>
			</div>
			<h1 class="text-3xl font-bold tracking-tight">Estado del sistema</h1>
			<p class="text-sm text-muted-foreground">
				Vista del estado de los componentes principales de LemiPay.
			</p>
		</header>

		<section
			class="rounded-2xl border border-border bg-card p-6 shadow-sm shadow-black/5 dark:shadow-none"
		>
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div class="flex items-start gap-3">
					{#if overallState === 'ok'}
						<CheckCircle2 class="mt-0.5 size-6 shrink-0 text-emerald-500" />
					{:else if overallState === 'error'}
						<XCircle class="mt-0.5 size-6 shrink-0 text-red-500" />
					{:else}
						<RefreshCw class="mt-0.5 size-6 shrink-0 animate-spin text-amber-500" />
					{/if}
					<div>
						<p class="text-lg font-semibold">{overallLabel}</p>
						{#if apiUrl}
							<p class="mt-1 flex items-center gap-1.5 text-xs text-muted-foreground">
								<Link2 class="size-3 shrink-0" />
								<span class="font-mono break-all">{apiUrl}</span>
							</p>
						{/if}
						{#if lastCheckedAt}
							<p class="mt-1 text-xs text-muted-foreground">
								Última comprobación: {lastCheckedAt.toLocaleString()}
							</p>
						{/if}
					</div>
				</div>

				<Button label="Actualizar" variant="secondary" size="sm" {loading} onclick={checkStatus}>
					{#snippet icon()}
						<RefreshCw class="size-4 {loading ? 'animate-spin' : ''}" />
					{/snippet}
				</Button>
			</div>
		</section>

		<section class="space-y-3">
			<h2 class="text-sm font-semibold tracking-wide text-muted-foreground uppercase">Servicios</h2>

			<ul class="space-y-3">
				{#each services as service (service.name)}
					<li
						class="rounded-xl border border-border bg-card p-4 shadow-sm shadow-black/5 dark:shadow-none"
					>
						<div class="flex items-start justify-between gap-4">
							<div class="flex items-start gap-3">
								<div
									class="mt-1.5 size-2.5 shrink-0 rounded-full shadow-sm {stateDotClass(
										service.state
									)}"
								></div>
								<div class="space-y-1">
									<div class="flex items-center gap-2">
										{#if service.name === 'API'}
											<Server class="size-4 text-muted-foreground" />
										{:else}
											<Activity class="size-4 text-muted-foreground" />
										{/if}
										<span class="font-medium">{service.name}</span>
									</div>
									<p class="text-sm text-muted-foreground">{service.description}</p>
									{#if service.url}
										<p class="flex items-start gap-1.5 text-xs text-muted-foreground">
											<Link2 class="mt-0.5 size-3 shrink-0" />
											<span class="font-mono break-all">{service.url}</span>
										</p>
									{/if}
									<p class="text-sm text-foreground/80">{service.detail}</p>
									{#if service.latencyMs !== null}
										<p class="text-xs text-muted-foreground">
											Latencia: {service.latencyMs} ms
										</p>
									{/if}
								</div>
							</div>

							<span
								class="shrink-0 rounded-full border px-2.5 py-0.5 text-xs font-medium {stateBadgeClass(
									service.state
								)}"
							>
								{stateLabel(service.state)}
							</span>
						</div>
					</li>
				{/each}
			</ul>
		</section>
	</div>
</div>
