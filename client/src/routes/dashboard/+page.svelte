<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	import { Activity, ArrowUpRight, Landmark, Plus, Sparkles, UserPlus } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { resolve } from '$app/paths';

	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { getNotifications, markNotificationRead } from '$lib/api/endpoints/notifications';
	import { listGroupInvestments } from '$lib/api/endpoints/investments';
	import { isSuccess } from '$lib/types/client.types';
	import type { NotificationRecord } from '$lib/types/endpoints/notifications.types';
	import type { Investment } from '$lib/types/endpoints/investments.types';
	import NotificationActivityList from '$lib/components/NotificationActivityList.svelte';
	import DashboardGroupsList from '$lib/components/dashboard/DashboardGroupsList.svelte';
	import { authStore } from '$lib/stores/auth';
	import NewGroup from '$lib/components/modals/group/NewGroup.svelte';
	import DashboardLayout from './DashboardLayout.svelte';
	import { formatMoney } from '$lib/utils/format_utils';

	let loadingActivity = $state(true);
	let activityNotifications = $state<NotificationRecord[]>([]);
	let activityShowingRead = $state(false);
	let misGrupos = $state<GroupSummary[]>([]);
	let showNewGroup = $state(false);
	let realInvestments = $state<Investment[]>([]);

	const userName = $derived($authStore.user?.name?.split(' ')[0] ?? 'Mateo');

	const activeGroupsCount = $derived(
		misGrupos.filter((g) => g.status.toLowerCase() === 'active').length
	);

	const investmentMetrics = $derived.by(() => {
		const totalInvested = realInvestments.reduce((sum, inv) => sum + parseFloat(inv.amount), 0);
		const totalCurrentValue = realInvestments.reduce(
			(sum, inv) => sum + parseFloat(inv.current_value),
			0
		);
		const totalReturn = totalCurrentValue - totalInvested;
		const returnPercentage = totalInvested > 0 ? (totalReturn / totalInvested) * 100 : 0;
		const activeInvestments = realInvestments.filter((inv) => inv.status === 'active').length;

		return { totalInvested, totalCurrentValue, totalReturn, returnPercentage, activeInvestments };
	});

	function riskLabel(risk: string) {
		if (risk === 'low') return 'Bajo';
		if (risk === 'medium') return 'Medio';
		return 'Alto';
	}

	function statusLabel(status: string) {
		if (status === 'active') return 'Activo';
		if (status === 'matured') return 'Para retirar';
		return 'Retirado';
	}

	const activityLimit = 3;

	async function loadActivityNotifications() {
		loadingActivity = true;

		let unread: NotificationRecord[] = [];
		const unreadResponse = await getNotifications({ read: false, limit: activityLimit });
		if (isSuccess(unreadResponse)) {
			unread = unreadResponse.body;
		}

		const remaining = activityLimit - unread.length;
		let read: NotificationRecord[] = [];
		if (remaining > 0) {
			const readResponse = await getNotifications({ read: true, limit: remaining });
			if (isSuccess(readResponse)) {
				read = readResponse.body;
			}
		}

		activityNotifications = [...unread, ...read];
		activityShowingRead = unread.length === 0 && read.length > 0;
		loadingActivity = false;
	}

	async function handleMarkActivityRead(id: string) {
		const response = await markNotificationRead(id);
		if (!isSuccess(response)) return;
		await loadActivityNotifications();
	}

	async function load_my_groups() {
		const res = await getMyGroups();
		if (!isSuccess(res)) {
			console.error(res.message || 'Error al buscar grupos');
			return;
		}

		misGrupos = res.body;

		const investmentResults = await Promise.allSettled(
			misGrupos.map((g) => listGroupInvestments(g.group_id))
		);
		const allInv: Investment[] = [];
		for (const result of investmentResults) {
			if (result.status === 'fulfilled' && isSuccess(result.value)) {
				allInv.push(...result.value.body);
			}
		}
		realInvestments = allInv;
	}

	onMount(() => {
		load_my_groups();
		void loadActivityNotifications();
	});
</script>

<svelte:head>
	<title>Lemipay - Dashboard</title>
</svelte:head>

<NewGroup onclose={() => (showNewGroup = false)} open={showNewGroup} />

<DashboardLayout>
	<main class="min-w-0 space-y-6">
		<section
			class="overflow-hidden rounded-[2rem] border border-border/80 bg-card shadow-sm shadow-black/5 dark:shadow-none"
			in:fly={{ y: 14, duration: 420 }}
		>
			<div class="relative p-6 sm:p-8">
				<div
					class="absolute top-0 right-0 h-52 w-52 translate-x-16 -translate-y-20 rounded-full bg-lime-300/30 blur-3xl dark:bg-lime-400/10"
				></div>
				<div
					class="absolute right-24 bottom-0 h-40 w-40 translate-y-20 rounded-full bg-violet-400/20 blur-3xl dark:bg-violet-500/10"
				></div>

				<div class="relative grid gap-8 xl:grid-cols-[minmax(0,1fr)_320px]">
					<div class="space-y-6">
						<div
							class="inline-flex items-center gap-2 rounded-full border border-lime-300/60 bg-lime-200/30 px-3 py-1 text-xs font-medium text-lime-900 dark:border-lime-400/20 dark:bg-lime-400/10 dark:text-lime-200"
						>
							<Sparkles class="size-3.5" />
							Dashboard financiero vivo
						</div>

						<div>
							<h1
								class="flex max-w-lg items-baseline text-4xl font-semibold tracking-tight text-balance sm:text-5xl"
							>
								Hola, <span
									class="mx-2 inline-block max-w-sm overflow-hidden font-semibold tracking-tight text-balance text-ellipsis whitespace-nowrap sm:text-5xl"
									>{userName}</span
								> 👋
							</h1>
							<p class="mt-3 max-w-2xl text-base text-muted-foreground sm:text-lg">
								Gestioná tesorerías, gastos grupales y decisiones compartidas desde un solo lugar.
							</p>
						</div>

						<div class="flex flex-wrap gap-3">
							<button
								class="inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2.5 text-sm font-semibold text-background shadow-lg shadow-foreground/10 transition hover:-translate-y-0.5 hover:bg-foreground/90"
								onclick={() => (showNewGroup = true)}
								type="button"
							>
								<Plus class="size-4" />
								Crear grupo
							</button>
							<a
								class="inline-flex items-center gap-2 rounded-2xl border border-border bg-background/70 px-4 py-2.5 text-sm font-semibold transition hover:-translate-y-0.5 hover:bg-accent"
								href={resolve('/profile/me')}
							>
								Ver perfil
								<ArrowUpRight class="size-4" />
							</a>
						</div>
					</div>
					<div class="grid grid-cols-2 gap-3 sm:grid-cols-4 xl:grid-cols-2">
						<div></div>
						<div
							class="max-h-30 rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur"
						>
							<p class="text-xs font-medium text-muted-foreground">Grupos activos</p>
							<p class="mt-2 text-2xl font-semibold">{activeGroupsCount}</p>
						</div>
						<div></div>
						<div></div>
					</div>
				</div>
			</div>
		</section>

		<section class="grid gap-6 xl:grid-cols-[minmax(0,1fr)_340px]">
			<div class="space-y-6">
				<section class="grid gap-3 sm:grid-cols-2 lg:grid-cols-2" in:fade={{ duration: 300 }}>
					<button
						class="group rounded-3xl border border-border bg-card p-4 text-left shadow-sm transition hover:-translate-y-1 hover:border-lime-300 hover:shadow-xl hover:shadow-lime-500/10"
						onclick={() => (showNewGroup = true)}
						type="button"
					>
						<div
							class="mb-4 flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
						>
							<Plus class="size-4" />
						</div>
						<p class="font-semibold">Crear Grupo</p>
						<p class="mt-1 text-sm text-muted-foreground">Nueva tesorería compartida</p>
					</button>
					<!--
                    <a
                        href={resolve('/dashboard')}
                        class="group rounded-3xl border border-border bg-card p-4 shadow-sm transition hover:-translate-y-1 hover:border-sky-300 hover:shadow-xl hover:shadow-sky-500/10"
                    >
                        <div
                            class="mb-4 flex size-10 items-center justify-center rounded-2xl bg-sky-400/15 text-sky-700 dark:text-sky-300"
                        >
                            <ReceiptText class="size-4" />
                        </div>
                        <p class="font-semibold">Registrar Gasto</p>
                        <p class="mt-1 text-sm text-muted-foreground">Split automático</p>
                    </a>
                    <a
                        href={resolve('/dashboard')}
                        class="group rounded-3xl border border-border bg-card p-4 shadow-sm transition hover:-translate-y-1 hover:border-violet-300 hover:shadow-xl hover:shadow-violet-500/10"
                    >
                        <div
                            class="mb-4 flex size-10 items-center justify-center rounded-2xl bg-violet-400/15 text-violet-700 dark:text-violet-300"
                        >
                            <Activity class="size-4" />
                        </div>
                        <p class="font-semibold">Crear Proposal</p>
                        <p class="mt-1 text-sm text-muted-foreground">Votación del grupo</p>
                    </a>
                    -->

					<a
						class="group rounded-3xl border border-border bg-card p-4 shadow-sm transition hover:-translate-y-1 hover:border-amber-300 hover:shadow-xl hover:shadow-amber-500/10"
						href={resolve('/dashboard')}
					>
						<div
							class="mb-4 flex size-10 items-center justify-center rounded-2xl bg-amber-400/15 text-amber-700 dark:text-amber-300"
						>
							<UserPlus class="size-4" />
						</div>
						<p class="font-semibold">Invitar Miembro</p>
						<p class="mt-1 text-sm text-muted-foreground">Sumá colaboradores</p>
					</a>
				</section>

				<section class="rounded-[2rem] border border-border bg-card p-5 shadow-sm">
					<DashboardGroupsList onCreateGroup={() => (showNewGroup = true)} />
				</section>
			</div>

			<aside class="space-y-6">
				<section
					class="rounded-[2rem] border border-border bg-card p-5 shadow-sm"
					in:fly={{ x: 14, duration: 380 }}
				>
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium text-muted-foreground">Actividad reciente</p>
							<h2 class="mt-1 text-xl font-semibold">
								{activityShowingRead ? 'Recientes' : 'Sin leer'}
							</h2>
						</div>
						<div
							class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
						>
							<Activity class="size-4" />
						</div>
					</div>

					<div class="mt-5">
						{#if loadingActivity}
							<p class="text-sm text-muted-foreground">Cargando actividad...</p>
						{:else}
							<NotificationActivityList
								notifications={activityNotifications}
								markRead={handleMarkActivityRead}
								emptyMessage="No hay actividad para mostrar."
								compact={true}
							/>
						{/if}
					</div>

					<div class="mt-4 border-t border-border pt-3 text-center">
						<a
							href={resolve('/dashboard/activity')}
							class="text-xs font-medium text-muted-foreground transition hover:text-foreground hover:underline"
						>
							Ver toda la actividad
						</a>
					</div>
				</section>
				<!--
					<section
						class="rounded-[2rem] border border-border bg-card p-5 shadow-sm"
						in:fly={{ x: 14, duration: 380 }}
					>

						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium text-muted-foreground">Resumen de inversiones</p>
								<h2 class="mt-1 text-xl font-semibold">Portfolio</h2>
							</div>
							<div
								class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
							>
								<TrendingUp class="size-4" />
							</div>
						</div>


						<div class="mt-5 space-y-3">
							<div class="rounded-2xl bg-muted/60 p-3">
								<p class="text-xs text-muted-foreground">Total invertido</p>
								<p class="mt-1 font-semibold">{formatMoney(investmentMetrics.totalInvested)}</p>
							</div>
							<div class="rounded-2xl bg-muted/60 p-3">
								<p class="text-xs text-muted-foreground">Valor actual</p>
								<p class="mt-1 font-semibold">{formatMoney(investmentMetrics.totalCurrentValue)}</p>
							</div>
							<div class="rounded-2xl bg-muted/60 p-3">
								<p class="text-xs text-muted-foreground">Retorno total</p>
								<p
									class={investmentMetrics.totalReturn >= 0
										? 'mt-1 font-semibold text-emerald-600 dark:text-emerald-300'
										: 'mt-1 font-semibold text-rose-600 dark:text-rose-300'}
								>
									{investmentMetrics.totalReturn >= 0 ? '+' : ''}
									{formatMoney(investmentMetrics.totalReturn)}
									({investmentMetrics.returnPercentage >= 0 ? '+' : ''}{investmentMetrics.returnPercentage.toFixed(1)}%)
								</p>
							</div>
							<div class="flex items-center justify-between rounded-2xl bg-muted/60 p-3">
								<span class="text-sm text-muted-foreground">Inversiones activas</span>
								<span class="font-semibold">{investmentMetrics.activeInvestments}</span>
							</div>
						</div>
					</section>
					-->

				<!--
					<section
						class="rounded-[2rem] border border-border bg-card p-5 shadow-sm"
						in:fly={{ x: 14, duration: 420 }}
					>
						<div class="flex items-center gap-3">
							<div
								class="flex size-10 items-center justify-center rounded-2xl bg-violet-400/15 text-violet-700 dark:text-violet-300"
							>
								<TrendingUp class="size-4" />
							</div>
							<div>
								<p class="font-semibold">Balance social</p>
								<p class="text-sm text-muted-foreground">Splitwise + treasury + governance</p>
							</div>
						</div>
						<div class="mt-5 space-y-3">
							<div class="flex items-center justify-between rounded-2xl bg-muted/60 p-3">
								<span class="text-sm text-muted-foreground">Propuestas activas</span>
								<span class="font-semibold">—</span>
							</div>
							<div class="flex items-center justify-between rounded-2xl bg-muted/60 p-3">
								<span class="text-sm text-muted-foreground">Health promedio</span>
								<span class="font-semibold text-emerald-600 dark:text-emerald-300">Healthy</span>
							</div>
							<div class="flex items-center justify-between rounded-2xl bg-muted/60 p-3">
								<span class="text-sm text-muted-foreground">Última sync</span>
								<span class="inline-flex items-center gap-1 font-semibold">
									<Clock class="size-3.5" />
									Ahora
								</span>
							</div>
						</div>
					</section>
-->
				<section
					class="rounded-[2rem] border border-border bg-card p-5 shadow-sm"
					in:fly={{ x: 14, duration: 420 }}
				>
					<div class="flex items-center gap-3">
						<div
							class="flex size-10 items-center justify-center rounded-2xl bg-violet-400/15 text-violet-700 dark:text-violet-300"
						>
							<Landmark class="size-4" />
						</div>
						<div>
							<p class="font-semibold">Posiciones</p>
							<p class="text-sm text-muted-foreground">Estrategias activas</p>
						</div>
					</div>
					<div class="mt-5 space-y-3">
						{#each realInvestments as inv (inv.id)}
							<a
								href={resolve(`/groups/${inv.group_id}/investments/${inv.id}`)}
								class="block rounded-2xl border border-border/70 bg-background p-3 transition hover:bg-muted/60"
							>
								<div class="flex items-start justify-between gap-2">
									<p class="text-sm font-semibold">{inv.strategy_name}</p>
									<span
										class={parseFloat(inv.amount) > 0 &&
										parseFloat(inv.current_value) >= parseFloat(inv.amount)
											? 'shrink-0 text-xs font-semibold text-emerald-600 dark:text-emerald-300'
											: 'shrink-0 text-xs font-semibold text-rose-600 dark:text-rose-300'}
									>
										{parseFloat(inv.amount) > 0
											? (parseFloat(inv.current_value) >= parseFloat(inv.amount) ? '+' : '') +
												(
													((parseFloat(inv.current_value) - parseFloat(inv.amount)) /
														parseFloat(inv.amount)) *
													100
												).toFixed(1) +
												'%'
											: '—'}
									</span>
								</div>
								<p class="mt-1 text-xs text-muted-foreground">
									{formatMoney(inv.amount, 'USDC')} → {formatMoney(inv.current_value, 'USDC')}
								</p>
								<div class="mt-2 flex items-center gap-2">
									<span
										class={inv.risk_level === 'Low'
											? 'rounded-full bg-emerald-500/15 px-2 py-0.5 text-[11px] font-medium text-emerald-700 dark:text-emerald-300'
											: inv.risk_level === 'Medium'
												? 'rounded-full bg-amber-500/15 px-2 py-0.5 text-[11px] font-medium text-amber-700 dark:text-amber-300'
												: 'rounded-full bg-rose-500/15 px-2 py-0.5 text-[11px] font-medium text-rose-700 dark:text-rose-300'}
									>
										{riskLabel(inv.risk_level)}
									</span>
									<span
										class={inv.status === 'active'
											? 'rounded-full bg-emerald-500/15 px-2 py-0.5 text-[11px] font-medium text-emerald-700 dark:text-emerald-300'
											: 'rounded-full bg-muted px-2 py-0.5 text-[11px] font-medium text-muted-foreground'}
									>
										{statusLabel(inv.status)}
									</span>
								</div>
							</a>
						{:else}
							<div
								class="rounded-2xl border border-dashed border-border bg-background p-4 text-center"
							>
								<p class="text-sm text-muted-foreground">Sin inversiones activas</p>
								<p class="mt-1 text-xs text-muted-foreground">
									Las inversiones aparecerán aquí cuando algún grupo active una estrategia.
								</p>
							</div>
						{/each}
					</div>
				</section>
			</aside>
		</section>
	</main>

	<div class="group fixed right-5 bottom-5 z-40 flex items-center gap-3">
		<div
			class="pointer-events-none hidden translate-x-2 rounded-2xl border border-border bg-card px-3 py-2 text-sm font-semibold opacity-0 shadow-xl transition group-hover:translate-x-0 group-hover:opacity-100 sm:block"
		>
			Crear grupo
		</div>
		<button
			aria-label="Crear grupo"
			class="flex h-14 w-14 items-center justify-center rounded-full bg-foreground text-background shadow-2xl ring-4 shadow-lime-500/20 ring-lime-400/10 transition hover:scale-105 hover:shadow-lime-500/30 focus:ring-2 focus:ring-ring focus:outline-none active:scale-95"
			onclick={() => (showNewGroup = true)}
			type="button"
		>
			<Plus class="size-6 transition group-hover:rotate-90" />
		</button>
	</div>
</DashboardLayout>
