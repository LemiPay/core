<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	import {
		Activity,
		ArrowUpRight,
		Clock,
		Landmark,
		Plus,
		Sparkles,
		TrendingUp,
		UserPlus,
		Users,
		Wallet
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { flip } from 'svelte/animate';
	import { fade, fly, scale } from 'svelte/transition';
	import { resolve } from '$app/paths';

	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';
	import { authStore } from '$lib/stores/auth';
	import NewGroup from '$lib/components/modals/group/NewGroup.svelte';
	import DashboardLayout from './DashboardLayout.svelte';

	type FilterRole = 'all' | 'Admin' | 'Member';
	type FilterStatus = 'all' | 'Active' | 'DebtResolution' | 'Ended';
	type GroupMeta = {
		members: number;
		treasury: number;
		expenses: number;
		proposals: number;
		activityLabel: string;
		currency: string;
		balance: number;
		yieldGenerated: number;
		icon: string;
		health: 'healthy' | 'warning' | 'debt';
	};

	type EnrichedGroupSummary = GroupSummary & { meta: GroupMeta };

	type InvestmentMock = {
		id: string;
		group_id: string;
		strategy_name: string;
		amount: number;
		current_value: number;
		actual_return: number | null;
		status: 'Active' | 'Closed';
		expected_return_percentage: number;
		risk_level: 'Low' | 'Medium' | 'High';
		currency: string;
	};

	type ActivityItem = {
		title: string;
		detail: string;
		time: string;
		variant: 'green' | 'yellow' | 'purple' | 'blue';
	};

	let isLoading = $state(true);
	let error = $state('');
	let misGrupos = $state<GroupSummary[]>([]);
	let showNewGroup = $state(false);
	let didInitializeStatusFilter = $state(false);

	let filterRole = $state<FilterRole>('all');
	let filterStatus = $state<FilterStatus>('Active');

	const roleOptions: { val: FilterRole; label: string }[] = [
		{ val: 'all', label: 'Todos' },
		{ val: 'Admin', label: 'Admin' },
		{ val: 'Member', label: 'Miembro' }
	];

	const statusOptions: { val: FilterStatus; label: string; dot: string }[] = [
		{ val: 'all', label: 'Todos', dot: 'bg-foreground' },
		{ val: 'Active', label: 'Activos', dot: 'bg-emerald-500' },
		{ val: 'DebtResolution', label: 'En resolución', dot: 'bg-amber-400' },
		{ val: 'Ended', label: 'Finalizados', dot: 'bg-rose-400' }
	];

	const fallbackActivities: ActivityItem[] = [
		{
			title: 'Juan creó una propuesta',
			detail: 'Retiro parcial desde tesorería compartida',
			time: 'Hace 12 min',
			variant: 'purple'
		},
		{
			title: 'Se aprobó un retiro',
			detail: '3 de 4 miembros votaron a favor',
			time: 'Hace 48 min',
			variant: 'green'
		},
		{
			title: 'Mateo agregó un gasto',
			detail: 'Cena del grupo · split automático',
			time: 'Hoy',
			variant: 'blue'
		},
		{
			title: 'Nueva ronda de aporte',
			detail: 'Pendiente de confirmación',
			time: 'Ayer',
			variant: 'yellow'
		}
	];

	const userName = $derived($authStore.user?.name?.split(' ')[0] ?? 'Mateo');

	const gruposEnriquecidos = $derived(
		misGrupos.map((group, index) => ({
			...group,
			meta: buildMockMeta(group, index)
		}))
	);

	const gruposFiltrados = $derived(
		gruposEnriquecidos.filter((g) => {
			const roleMatch = filterRole === 'all' || g.role.toLowerCase() === filterRole.toLowerCase();
			const statusMatch =
				filterStatus === 'all' || g.status.toLowerCase() === filterStatus.toLowerCase();
			return roleMatch && statusMatch;
		})
	);

	const dashboardMetrics = $derived.by(() => {
		const groups = gruposEnriquecidos;
		const totalTreasury = groups.reduce((total, group) => total + group.meta.treasury, 0);
		const totalBalance = groups.reduce((total, group) => total + group.meta.balance, 0);
		const activeGroups = groups.filter((group) => group.status.toLowerCase() === 'active').length;
		const pendingProposals = groups.reduce((total, group) => total + group.meta.proposals, 0);
		const yieldGenerated = groups.reduce((total, group) => total + group.meta.yieldGenerated, 0);

		return {
			totalTreasury,
			totalBalance,
			activeGroups,
			pendingProposals,
			yieldGenerated
		};
	});

	const recentActivities = $derived.by(() => {
		if (gruposEnriquecidos.length === 0) return fallbackActivities;

		return gruposEnriquecidos.slice(0, 4).map((group, index) => ({
			title:
				index % 3 === 0
					? `Nueva propuesta en ${group.group_name}`
					: index % 3 === 1
						? `Gasto registrado en ${group.group_name}`
						: `Movimiento de tesorería en ${group.group_name}`,
			detail:
				index % 3 === 0
					? `${group.meta.proposals} propuestas activas esperando votos`
					: index % 3 === 1
						? `${group.meta.expenses} expenses conciliados este mes`
						: `${formatMoney(group.meta.treasury, group.meta.currency)} disponibles`,
			time: group.meta.activityLabel,
			variant: (['purple', 'blue', 'green', 'yellow'] as const)[index % 4]
		}));
	});

	const allInvestments = $derived.by(() => {
		return gruposEnriquecidos.flatMap((group) =>
			buildMockInvestments(group.group_id, group.meta.currency)
		);
	});

	const investmentMetrics = $derived.by(() => {
		const totalInvested = allInvestments.reduce((sum, inv) => sum + inv.amount, 0);
		const totalCurrentValue = allInvestments.reduce((sum, inv) => sum + inv.current_value, 0);
		const totalReturn = totalCurrentValue - totalInvested;
		const returnPercentage = totalInvested > 0 ? (totalReturn / totalInvested) * 100 : 0;
		const activeInvestments = allInvestments.filter((inv) => inv.status === 'Active').length;

		return { totalInvested, totalCurrentValue, totalReturn, returnPercentage, activeInvestments };
	});

	function seedFromString(value: string) {
		return value.split('').reduce((total, char) => total + char.charCodeAt(0), 0);
	}

	function buildMockMeta(group: GroupSummary, index: number): GroupMeta {
		const seed = seedFromString(group.group_id + group.group_name);
		const currency = ['USDC', 'USD', 'ARS'][seed % 3];
		const treasury = 850 + ((seed * 137 + index * 503) % 9200);
		const balance = ((seed % 2 === 0 ? 1 : -1) * (120 + ((seed * 17) % 880))) / (index + 1);
		const proposals = group.status.toLowerCase() === 'active' ? 1 + (seed % 4) : seed % 2;
		const health = balance < -250 ? 'debt' : proposals > 3 ? 'warning' : 'healthy';

		return {
			members: 3 + (seed % 8),
			treasury,
			expenses: 4 + (seed % 18),
			proposals,
			activityLabel: ['Ahora', 'Hace 8 min', 'Hoy', 'Ayer'][seed % 4],
			currency,
			balance,
			yieldGenerated: Number((1.8 + (seed % 42) / 10).toFixed(1)),
			icon: ['✈️', '🏠', '🎉', '💸', '🚀', '☕', '🏝️', '🧾'][seed % 8],
			health
		};
	}

	function buildMockInvestments(group_id: string, group_currency: string): InvestmentMock[] {
		const seed = seedFromString(group_id + 'inv');
		const strategies = [
			{ name: 'Fondo Común Lemipay', risk: 'Low' as const, expectedReturn: 3.0 },
			{ name: 'Top 100 ARG', risk: 'Medium' as const, expectedReturn: 7.5 },
			{ name: 'Fondo Común Lemipay', risk: 'Low' as const, expectedReturn: 3.0 },
			{ name: 'Michael Saylor', risk: 'High' as const, expectedReturn: 15.0 },
			{ name: 'Top 100 ARG', risk: 'Medium' as const, expectedReturn: 7.5 }
		];

		if (seed % 3 === 0) return [];

		const count = 1 + ((seed * 7) % 2);

		return Array.from({ length: count }, (_, i) => {
			const strategy = strategies[(seed + i * 3) % strategies.length];
			const amount = 200 + ((seed * 137 + i * 503) % 5000);
			const currentValue =
				Math.round(amount * (1 + (2 + ((seed * i + 17) % 15)) / 100) * 100) / 100;
			const actualReturn = Math.round(((currentValue - amount) / amount) * 10000) / 100;

			return {
				id: `inv-${group_id}-${i}`,
				group_id,
				strategy_name: strategy.name,
				amount,
				current_value: currentValue,
				actual_return: actualReturn,
				status: seed % 7 === 0 ? 'Closed' : 'Active',
				expected_return_percentage: strategy.expectedReturn,
				risk_level: strategy.risk,
				currency: group_currency
			};
		});
	}

	function formatMoney(value: number, currency = 'USD') {
		return `${value < 0 ? '-' : ''}$${Math.abs(value).toLocaleString('en-US', {
			maximumFractionDigits: 0
		})} ${currency}`;
	}

	function getStatusClasses(status: string) {
		if (status.toLowerCase() === 'active')
			return 'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/20 dark:bg-emerald-400/10 dark:text-emerald-300';
		if (status.toLowerCase() === 'debtresolution')
			return 'border-amber-200 bg-amber-50 text-amber-700 dark:border-amber-400/20 dark:bg-amber-400/10 dark:text-amber-300';
		return 'border-rose-200 bg-rose-50 text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300';
	}

	function getStatusDot(status: string) {
		if (status.toLowerCase() === 'active') return 'bg-emerald-500 shadow-emerald-500/30';
		if (status.toLowerCase() === 'debtresolution') return 'bg-amber-400 shadow-amber-400/30';
		return 'bg-rose-400 shadow-rose-400/30';
	}

	function translateStatus(status: string) {
		if (status === 'Active') return 'Activo';
		if (status === 'DebtResolution') return 'En resolución';
		if (status === 'Ended') return 'Finalizado';
		return status;
	}

	function getHealthClasses(health: GroupMeta['health']) {
		if (health === 'healthy') {
			return 'bg-emerald-500 shadow-emerald-500/30';
		}
		if (health === 'warning') {
			return 'bg-amber-400 shadow-amber-400/30';
		}
		return 'bg-rose-500 shadow-rose-500/30';
	}

	function getActivityVariantClasses(variant: ActivityItem['variant']) {
		if (variant === 'green') return 'bg-emerald-500/15 text-emerald-600 dark:text-emerald-300';
		if (variant === 'yellow') return 'bg-amber-500/15 text-amber-600 dark:text-amber-300';
		if (variant === 'purple') return 'bg-violet-500/15 text-violet-600 dark:text-violet-300';
		return 'bg-sky-500/15 text-sky-600 dark:text-sky-300';
	}

	async function load_my_groups() {
		isLoading = true;
		error = '';

		const res = await getMyGroups();

		if (!isSuccess(res)) {
			error = res.message || 'Error al buscar grupos';
			isLoading = false;
			console.error(error);
			return;
		}

		misGrupos = res.body;
		if (!didInitializeStatusFilter) {
			const hasActiveGroups = misGrupos.some((group) => group.status.toLowerCase() === 'active');
			filterStatus = hasActiveGroups ? 'Active' : 'all';
			didInitializeStatusFilter = true;
		}
		isLoading = false;
	}

	onMount(() => {
		load_my_groups();
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
						<!--
                        <div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
                            <p class="text-xs font-medium text-muted-foreground">Total en grupos</p>
                            <p class="mt-2 text-2xl font-semibold">
                                {formatMoney(dashboardMetrics.totalTreasury)}
                            </p>
                        </div>
                        -->
						<div></div>
						<!--
                        <div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
                            <p class="text-xs font-medium text-muted-foreground">Balance total</p>
                            <p
                                class={dashboardMetrics.totalBalance >= 0
                                    ? 'mt-2 text-2xl font-semibold text-emerald-600 dark:text-emerald-300'
                                    : 'mt-2 text-2xl font-semibold text-rose-600 dark:text-rose-300'}
                            >
                                {formatMoney(dashboardMetrics.totalBalance)}
                            </p>
                        </div>
                        -->
						<div
							class=" max-h-30 rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur"
						>
							<p class="text-xs font-medium text-muted-foreground">Grupos activos</p>
							<p class="mt-2 text-2xl font-semibold">{dashboardMetrics.activeGroups}</p>
						</div>
						<!--
                        <div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
                            <p class="text-xs font-medium text-muted-foreground">Yield generado</p>
                            <p class="mt-2 text-2xl font-semibold text-lime-700 dark:text-lime-300">
                                +{dashboardMetrics.yieldGenerated.toFixed(1)}%
                            </p>
                        </div>
                        -->
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
					<div class="flex flex-col gap-4 sm:flex-col sm:justify-between">
						<div>
							<h2 class="mt-1 text-2xl font-semibold tracking-tight">Mis grupos</h2>
						</div>

						<div class="flex flex-wrap items-center gap-2">
							{#each roleOptions as option (option.val)}
								<button
									type="button"
									onclick={() => (filterRole = option.val)}
									class={filterRole === option.val
										? 'rounded-full bg-foreground px-3 py-1.5 text-xs font-semibold text-background'
										: 'rounded-full border border-border px-3 py-1.5 text-xs font-semibold text-muted-foreground transition hover:bg-muted hover:text-foreground'}
								>
									{option.label}
								</button>
							{/each}
							<span class="mx-1 hidden h-5 w-px bg-border sm:block"></span>
							{#each statusOptions as option (option.val)}
								<button
									type="button"
									onclick={() => (filterStatus = option.val)}
									class={filterStatus === option.val
										? 'inline-flex items-center gap-1.5 rounded-full border border-transparent bg-foreground px-3 py-1.5 text-xs font-semibold text-background'
										: 'inline-flex items-center gap-1.5 rounded-full border border-border px-3 py-1.5 text-xs font-semibold text-muted-foreground transition hover:bg-muted hover:text-foreground'}
								>
									<span class={['size-1.5 rounded-full', option.dot]}></span>
									{option.label}
								</button>
							{/each}
						</div>
					</div>

					{#if error}
						<div
							class="mt-5 rounded-2xl border border-rose-200 bg-rose-50 p-4 text-sm font-medium text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
							transition:fade
						>
							{error}
						</div>
					{/if}

					{#if isLoading}
						<div class="mt-6 grid gap-4 md:grid-cols-2">
							{#each { length: 4 }, index}
								<div
									class="rounded-3xl border border-border bg-background p-5"
									in:fade={{ delay: index * 60 }}
								>
									<div class="h-5 w-2/3 animate-pulse rounded bg-muted"></div>
									<div class="mt-3 h-4 w-full animate-pulse rounded bg-muted"></div>
									<div class="mt-2 h-4 w-4/5 animate-pulse rounded bg-muted"></div>
									<div class="mt-6 grid grid-cols-2 gap-2">
										<div class="h-14 animate-pulse rounded-2xl bg-muted"></div>
										<div class="h-14 animate-pulse rounded-2xl bg-muted"></div>
									</div>
								</div>
							{/each}
						</div>
					{:else if gruposFiltrados.length > 0}
						<div class="mt-6 grid gap-4 md:grid-cols-2">
							{#each gruposFiltrados as grupo, index (grupo.group_id)}
								<a
									href={resolve(`/groups/${grupo.group_id}`)}
									class="group relative overflow-hidden rounded-3xl border border-border bg-background p-5 shadow-sm transition hover:-translate-y-1 hover:border-lime-300/80 hover:shadow-2xl hover:shadow-lime-500/10 focus:ring-2 focus:ring-ring focus:outline-none"
									animate:flip={{ duration: 240 }}
									in:fly={{ y: 12, duration: 260, delay: index * 45 }}
								>
									<div
										class="absolute inset-x-0 top-0 h-1 bg-linear-to-r from-lime-300 via-violet-300 to-sky-300 opacity-0 transition group-hover:opacity-100"
									></div>

									<div class="flex items-start justify-between gap-4">
										<div class="flex min-w-0 gap-3">
											<div
												class="flex size-12 shrink-0 items-center justify-center rounded-2xl bg-muted text-2xl shadow-inner"
											>
												{grupo.meta.icon}
											</div>
											<div class="min-w-0">
												<h3 class="truncate text-lg font-semibold">{grupo.group_name}</h3>
												<p class="mt-1 line-clamp-2 text-sm text-muted-foreground">
													{grupo.group_description ||
														'Grupo financiero compartido para coordinar gastos, aportes y decisiones.'}
												</p>
											</div>
										</div>

										<div class="flex shrink-0 flex-col items-end gap-2">
											<span
												class={grupo.role.toLowerCase() === 'admin'
													? 'rounded-full bg-violet-500/15 px-2.5 py-1 text-xs font-semibold text-violet-700 dark:text-violet-300'
													: 'rounded-full bg-muted px-2.5 py-1 text-xs font-semibold text-muted-foreground'}
											>
												{grupo.role}
											</span>
											<span
												class={[
													'inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-semibold',
													getStatusClasses(grupo.status)
												]}
											>
												<span class="size-1.5 rounded-full shadow-lg {getStatusDot(grupo.status)}"
												></span>
												{translateStatus(grupo.status)}
											</span>
										</div>
									</div>

									<div class="mt-6 grid grid-cols-2 gap-3">
										<div class="rounded-2xl bg-muted/60 p-3">
											<div
												class="mb-2 flex items-center gap-2 text-xs font-medium text-muted-foreground"
											>
												<Users class="size-3.5" />
												Miembros
											</div>
											<p class="font-semibold">{grupo.meta.members}</p>
										</div>

										<div class="rounded-2xl bg-muted/60 p-3">
											<div
												class="mb-2 flex items-center gap-2 text-xs font-medium text-muted-foreground"
											>
												<Wallet class="size-3.5" />
												Treasury
											</div>
											<p class="font-semibold">
												{formatMoney(grupo.meta.treasury, grupo.meta.currency)}
											</p>
										</div>
									</div>
									<!--
										<div class="mt-5 grid grid-cols-4 gap-2 text-center text-xs">
											<div class="rounded-2xl border border-border/80 p-2">
												<p class="text-muted-foreground">🧾</p>
												<p class="mt-1 font-semibold">{grupo.meta.expenses}</p>
											</div>
											<div class="rounded-2xl border border-border/80 p-2">
												<p class="text-muted-foreground">🗳</p>
												<p class="mt-1 font-semibold">{grupo.meta.proposals}</p>
											</div>
											<div class="rounded-2xl border border-border/80 p-2">
												<p class="text-muted-foreground">📈</p>
												<p class="mt-1 font-semibold">+{grupo.meta.yieldGenerated}%</p>
											</div>
											<div class="rounded-2xl border border-border/80 p-2">
												<p class="text-muted-foreground">⏱</p>
												<p class="mt-1 truncate font-semibold">{grupo.meta.activityLabel}</p>
											</div>
										</div>
										-->
								</a>
							{/each}
						</div>
					{:else}
						<div
							class="mt-6 rounded-3xl border border-dashed border-border bg-background p-8 text-center"
							transition:scale={{ duration: 220 }}
						>
							<div class="mx-auto flex size-12 items-center justify-center rounded-2xl bg-muted">
								<Wallet class="size-5 text-muted-foreground" />
							</div>
							<h3 class="mt-4 font-semibold">
								{misGrupos.length === 0
									? 'Todavía no tenés grupos'
									: 'No hay grupos con esos filtros'}
							</h3>
							<p class="mx-auto mt-2 max-w-md text-sm text-muted-foreground">
								{misGrupos.length === 0
									? 'Creá tu primera tesorería compartida para coordinar gastos, propuestas y aportes.'
									: 'Probá cambiar el rol o el estado para encontrar otros grupos.'}
							</p>
							<button
								type="button"
								onclick={() => (showNewGroup = true)}
								class="mt-5 inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2 text-sm font-semibold text-background transition hover:bg-foreground/90"
							>
								<Plus class="size-4" />
								Crear grupo
							</button>
						</div>
					{/if}
				</section>
			</div>

			<aside class="space-y-6">
				<!--
					<section
						class="rounded-[2rem] border border-border bg-card p-5 shadow-sm"
						in:fly={{ x: 14, duration: 380 }}
					>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium text-muted-foreground">Actividad reciente</p>
								<h2 class="mt-1 text-xl font-semibold">Sistema vivo</h2>
							</div>
							<div
								class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
							>
								<Activity class="size-4" />
							</div>
						</div>

						<div class="mt-5 space-y-4">
							{#each recentActivities as item, index (item.title)}
								<div class="relative pl-8" in:fly={{ x: 10, duration: 260, delay: index * 70 }}>
									<div
										class="absolute top-1 left-0 flex size-5 items-center justify-center rounded-full border border-border bg-card"
									>
										<span class={['size-2 rounded-full', getActivityVariantClasses(item.variant)]}
										></span>
									</div>
									{#if index !== recentActivities.length - 1}
										<div class="absolute top-7 -bottom-4 left-2.5 w-px bg-border"></div>
									{/if}
									<div
										class="rounded-2xl border border-border/70 bg-background p-3 transition hover:bg-muted/60"
									>
										<div class="flex items-start justify-between gap-3">
											<p class="text-sm font-semibold">{item.title}</p>
											<span class="shrink-0 text-[11px] text-muted-foreground">{item.time}</span>
										</div>
										<p class="mt-1 text-xs text-muted-foreground">{item.detail}</p>
									</div>
								</div>
							{/each}
						</div>
					</section>
-->
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
								<span class="font-semibold">{dashboardMetrics.pendingProposals}</span>
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
						{#each allInvestments as inv (inv.id)}
							<div
								class="rounded-2xl border border-border/70 bg-background p-3 transition hover:bg-muted/60"
							>
								<div class="flex items-start justify-between gap-2">
									<p class="text-sm font-semibold">{inv.strategy_name}</p>
									<span
										class={inv.actual_return != null && inv.actual_return >= 0
											? 'shrink-0 text-xs font-semibold text-emerald-600 dark:text-emerald-300'
											: 'shrink-0 text-xs font-semibold text-rose-600 dark:text-rose-300'}
									>
										{inv.actual_return != null
											? (inv.actual_return >= 0 ? '+' : '') + inv.actual_return.toFixed(1) + '%'
											: '—'}
									</span>
								</div>
								<p class="mt-1 text-xs text-muted-foreground">
									{formatMoney(inv.amount, inv.currency)} → {formatMoney(
										inv.current_value,
										inv.currency
									)}
								</p>
								<div class="mt-2 flex items-center gap-2">
									<span
										class={inv.risk_level === 'Low'
											? 'rounded-full bg-emerald-500/15 px-2 py-0.5 text-[11px] font-medium text-emerald-700 dark:text-emerald-300'
											: inv.risk_level === 'Medium'
												? 'rounded-full bg-amber-500/15 px-2 py-0.5 text-[11px] font-medium text-amber-700 dark:text-amber-300'
												: 'rounded-full bg-rose-500/15 px-2 py-0.5 text-[11px] font-medium text-rose-700 dark:text-rose-300'}
									>
										{inv.risk_level}
									</span>
									<span
										class={inv.status === 'Active'
											? 'rounded-full bg-emerald-500/15 px-2 py-0.5 text-[11px] font-medium text-emerald-700 dark:text-emerald-300'
											: 'rounded-full bg-muted px-2 py-0.5 text-[11px] font-medium text-muted-foreground'}
									>
										{inv.status}
									</span>
								</div>
							</div>
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
