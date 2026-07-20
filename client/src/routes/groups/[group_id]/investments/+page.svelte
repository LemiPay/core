<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import {
		TrendingUp,
		TrendingDown,
		Minus,
		Plus,
		CircleDollarSign,
		Rocket,
		Clock,
		ChevronDown,
		ArrowUpRight,
		Check,
		Info,
		BarChart3,
		Calendar,
		AlertTriangle,
		RefreshCw,
		ArrowLeft,
		List
	} from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import PriceSourceLink from '$lib/components/investments/PriceSourceLink.svelte';
	import { formatAmount, formatDate, formatDateTimeShort } from '$lib/utils/format_utils';
	import { InvestmentsState } from './investments.svelte';
	import type { InvestmentStrategy } from '$lib/types/endpoints/investments.types';
	import { categoryLabels, weightPct } from '$lib/types/endpoints/investments.types';

	const groupId = page.params.group_id as string;
	const investState = new InvestmentsState(groupId);

	let loadingInit = $state(true);
	let activeSection = $state('estrategias');
	let mobileNavOpen = $state(false);

	let readonly = $derived(investState.readonly);

	let showStrategyForm = $state<string | null>(null);
	let selectedAmount = $state('');
	let selectedCurrencyId = $state('');
	let showPastInvestments = $state(false);
	let executingProposal = $state<string | null>(null);
	let ragequitConfirmId = $state<string | null>(null);

	/** null = sin filtro (todas) */
	let filterCategory = $state<string | null>(null);
	let filterRisk = $state<string | null>(null);
	/** null = todas; '1' = solo 1x; 'levered' = x2+; number = exacto */
	let filterLeverage = $state<null | '1' | 'levered' | number>(null);

	const categoryFilters: { value: string | null; label: string }[] = [
		{ value: null, label: 'Todas' },
		{ value: 'simulated', label: 'Simulado' },
		{ value: 'crypto', label: 'Crypto' },
		{ value: 'stocks', label: 'Stocks' },
		{ value: 'mixed', label: 'Mix' },
		{ value: 'rwa', label: 'RWA' }
	];

	const riskFilters: { value: string | null; label: string }[] = [
		{ value: null, label: 'Todos' },
		{ value: 'low', label: 'Bajo' },
		{ value: 'medium', label: 'Medio' },
		{ value: 'high', label: 'Alto' }
	];

	let leverageOptions = $derived.by(() => {
		const set = new Set<number>();
		for (const s of investState.strategies) {
			set.add(s.leverage ?? 1);
		}
		return [...set].sort((a, b) => a - b);
	});

	let filteredStrategies = $derived(
		investState.strategies.filter((s) => {
			const cat = s.category ?? 'simulated';
			if (filterCategory !== null && cat !== filterCategory) return false;
			if (filterRisk !== null && s.risk_level !== filterRisk) return false;
			const lev = s.leverage ?? 1;
			if (filterLeverage === '1' && lev !== 1) return false;
			if (filterLeverage === 'levered' && lev <= 1) return false;
			if (typeof filterLeverage === 'number' && lev !== filterLeverage) return false;
			return true;
		})
	);

	let hasActiveFilters = $derived(
		filterCategory !== null || filterRisk !== null || filterLeverage !== null
	);

	function clearFilters() {
		filterCategory = null;
		filterRisk = null;
		filterLeverage = null;
	}

	function chipClass(active: boolean) {
		return active
			? 'border-foreground bg-foreground text-background'
			: 'border-border bg-card text-muted-foreground hover:border-input hover:text-foreground';
	}

	/** 1 card = full width; max 2 per row when there are more */
	function cardsGridClass(count: number) {
		return count <= 1 ? 'grid grid-cols-1 gap-3' : 'grid grid-cols-1 gap-3 sm:grid-cols-2';
	}

	type NavItem = {
		id: string;
		label: string;
		count?: number;
		tone?: 'default' | 'danger';
	};

	let navItems = $derived.by((): NavItem[] => {
		const items: NavItem[] = [];
		if (investState.maturedInvestments.length > 0) {
			items.push({
				id: 'finalizadas',
				label: 'Finalizadas',
				count: investState.maturedInvestments.length
			});
		}
		if (investState.liquidatedInvestments.length > 0) {
			items.push({
				id: 'liquidadas',
				label: 'Liquidadas',
				count: investState.liquidatedInvestments.length,
				tone: 'danger'
			});
		}
		if (investState.withdrawnInvestments.length > 0) {
			items.push({
				id: 'retiradas',
				label: 'Retiradas',
				count: investState.withdrawnInvestments.length
			});
		}
		if (investState.activeInvestments.length > 0) {
			items.push({
				id: 'activas',
				label: 'Activas',
				count: investState.activeInvestments.length
			});
		}
		if (investState.proposals.length > 0) {
			items.push({
				id: 'propuestas',
				label: 'Propuestas',
				count: investState.proposals.length
			});
		}
		items.push({
			id: 'estrategias',
			label: 'Estrategias',
			count: filteredStrategies.length
		});
		return items;
	});

	function scrollToSection(id: string) {
		const el = document.getElementById(id);
		if (!el) return;
		activeSection = id;
		mobileNavOpen = false;
		el.scrollIntoView({ behavior: 'smooth', block: 'start' });
		history.replaceState(null, '', `#${id}`);
	}

	function navButtonClass(id: string, tone?: 'default' | 'danger') {
		const active = activeSection === id;
		if (active && tone === 'danger') {
			return 'border-rose-500/40 bg-rose-500/10 text-rose-700 dark:text-rose-300';
		}
		if (active) {
			return 'border-foreground/20 bg-foreground text-background';
		}
		if (tone === 'danger') {
			return 'border-transparent text-rose-700/80 hover:bg-rose-500/10 dark:text-rose-300/80';
		}
		return 'border-transparent text-muted-foreground hover:bg-muted hover:text-foreground';
	}

	const categoryBadge: Record<string, string> = {
		simulated:
			'bg-slate-50 border-slate-200 text-slate-700 dark:bg-slate-400/10 dark:border-slate-400/20 dark:text-slate-300',
		crypto:
			'bg-violet-50 border-violet-200 text-violet-700 dark:bg-violet-400/10 dark:border-violet-400/20 dark:text-violet-300',
		stocks:
			'bg-sky-50 border-sky-200 text-sky-700 dark:bg-sky-400/10 dark:border-sky-400/20 dark:text-sky-300',
		mixed:
			'bg-indigo-50 border-indigo-200 text-indigo-700 dark:bg-indigo-400/10 dark:border-indigo-400/20 dark:text-indigo-300',
		rwa: 'bg-teal-50 border-teal-200 text-teal-700 dark:bg-teal-400/10 dark:border-teal-400/20 dark:text-teal-300'
	};

	investState.loadAll().finally(() => (loadingInit = false));

	onMount(() => {
		const hash = page.url.hash.replace('#', '');
		if (hash) {
			requestAnimationFrame(() => scrollToSection(hash));
		}

		const observer = new IntersectionObserver(
			(entries) => {
				const visible = entries
					.filter((e) => e.isIntersecting)
					.sort((a, b) => b.intersectionRatio - a.intersectionRatio);
				if (visible[0]?.target?.id) {
					activeSection = visible[0].target.id;
				}
			},
			{ rootMargin: '-20% 0px -55% 0px', threshold: [0.1, 0.25, 0.5] }
		);

		const t = window.setInterval(() => {
			if (!loadingInit) {
				for (const item of navItems) {
					const el = document.getElementById(item.id);
					if (el) observer.observe(el);
				}
				window.clearInterval(t);
			}
		}, 120);

		return () => {
			window.clearInterval(t);
			observer.disconnect();
		};
	});

	function toggleStrategyForm(strategyId: string) {
		if (showStrategyForm === strategyId) {
			showStrategyForm = null;
		} else {
			showStrategyForm = strategyId;
			selectedAmount = '';
			selectedCurrencyId = '';
			investState.proposeError = '';
		}
	}

	async function handlePropose(strategy: InvestmentStrategy) {
		if (!selectedAmount || !selectedCurrencyId) return;
		const data = {
			amount: String(selectedAmount),
			strategy_id: strategy.id,
			currency_id: selectedCurrencyId
		};
		const ok = await investState.propose(data);
		if (ok) {
			showStrategyForm = null;
			selectedAmount = '';
			selectedCurrencyId = '';
		}
	}

	async function handleExecute(proposalId: string) {
		investState.executeError = '';
		executingProposal = proposalId;
		await investState.execute(proposalId);
		executingProposal = null;
	}

	async function handleWithdraw(investmentId: string) {
		investState.withdrawError = '';
		await investState.withdraw(investmentId);
		ragequitConfirmId = null;
	}

	function ragequitPreview(inv: { current_value: string; ragequit_fee_bps?: number }) {
		const nav = Number(inv.current_value);
		const bps = inv.ragequit_fee_bps ?? 200;
		const fee = (nav * bps) / 10000;
		return { fee, payout: nav - fee, feePct: bps / 100 };
	}

	const riskConfig: Record<string, { color: string; bg: string; label: string }> = {
		low: {
			color: 'text-emerald-700 dark:text-emerald-300',
			bg: 'bg-emerald-50 border-emerald-200 dark:bg-emerald-400/10 dark:border-emerald-400/20',
			label: 'Bajo'
		},
		medium: {
			color: 'text-amber-700 dark:text-amber-300',
			bg: 'bg-amber-50 border-amber-200 dark:bg-amber-400/10 dark:border-amber-400/20',
			label: 'Medio'
		},
		high: {
			color: 'text-rose-700 dark:text-rose-300',
			bg: 'bg-rose-50 border-rose-200 dark:bg-rose-400/10 dark:border-rose-400/20',
			label: 'Alto'
		}
	};
</script>

<svelte:head>
	<title>Lemipay - {investState.groupData.name || 'Group'} - Inversiones</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4 pt-16">
	{#if loadingInit}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-border border-t-foreground"
		></div>
	{:else}
		<div class="w-full max-w-6xl pt-8 pb-6">
			<a
				href={`/groups/${groupId}`}
				class="mb-4 inline-flex items-center gap-1.5 rounded-md px-1 py-1.5 text-sm font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
			>
				<ArrowLeft class="h-4 w-4 shrink-0" aria-hidden="true" />
				Volver al grupo
			</a>
			<div class="flex items-center gap-3">
				<h1 class="text-2xl font-bold tracking-tight text-foreground">Inversiones</h1>
				<button
					type="button"
					onclick={() => investState.loadAll()}
					class="rounded-md p-2 text-muted-foreground transition hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
					title="Recargar"
					aria-label="Recargar inversiones"
				>
					<RefreshCw class="h-4 w-4" aria-hidden="true" />
				</button>
			</div>
			<p class="mt-2 max-w-2xl text-sm text-muted-foreground">
				Portfolios paper con precios de mercado (crypto / stocks estilo Ondo / mix). No se adquieren
				tokens on-chain. Las estrategias simuladas usan retorno sintético.
			</p>
		</div>

		<!-- Mobile section picker -->
		<div class="sticky top-16 z-20 mb-4 w-full max-w-6xl lg:hidden">
			<div class="rounded-xl border border-border bg-card/95 p-2 shadow-sm backdrop-blur">
				<button
					type="button"
					onclick={() => (mobileNavOpen = !mobileNavOpen)}
					class="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2 text-sm font-medium text-foreground"
					aria-expanded={mobileNavOpen}
					aria-controls="invest-section-nav"
				>
					<span class="inline-flex items-center gap-2">
						<List class="h-4 w-4 text-muted-foreground" aria-hidden="true" />
						Ir a sección
					</span>
					<ChevronDown
						class="h-4 w-4 text-muted-foreground transition {mobileNavOpen ? 'rotate-180' : ''}"
						aria-hidden="true"
					/>
				</button>
				{#if mobileNavOpen}
					<nav
						id="invest-section-nav"
						aria-label="Secciones de inversiones"
						class="mt-1 flex flex-col gap-0.5 border-t border-border pt-2"
					>
						{#each navItems as item}
							<button
								type="button"
								onclick={() => scrollToSection(item.id)}
								class="flex items-center justify-between rounded-lg border px-3 py-2 text-left text-sm font-medium transition {navButtonClass(
									item.id,
									item.tone
								)}"
							>
								<span>{item.label}</span>
								{#if item.count != null}
									<span class="text-[11px] opacity-80">{item.count}</span>
								{/if}
							</button>
						{/each}
					</nav>
				{/if}
			</div>
		</div>

		<div class="flex w-full max-w-6xl items-start gap-8 pb-16">
			<aside
				class="sticky top-24 hidden w-52 shrink-0 lg:block"
				aria-label="Navegación de secciones"
			>
				<div class="rounded-2xl border border-border bg-card p-3 shadow-sm">
					<p
						class="mb-2 px-2 text-[10px] font-semibold tracking-wider text-muted-foreground uppercase"
					>
						En esta página
					</p>
					<nav class="flex flex-col gap-0.5">
						{#each navItems as item}
							<button
								type="button"
								onclick={() => scrollToSection(item.id)}
								class="group flex items-center justify-between gap-2 rounded-xl border px-3 py-2.5 text-left text-sm font-medium transition {navButtonClass(
									item.id,
									item.tone
								)}"
							>
								<span class="truncate">{item.label}</span>
								{#if item.count != null}
									<span
										class="rounded-full bg-black/5 px-1.5 py-0.5 text-[10px] font-semibold tabular-nums dark:bg-white/10"
									>
										{item.count}
									</span>
								{/if}
							</button>
						{/each}
					</nav>
				</div>
			</aside>

			<div class="min-w-0 flex-1 space-y-10">
				{#if investState.maturedInvestments.length > 0}
					<section id="finalizadas" class="scroll-mt-28 space-y-4">
						<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
							<Check class="h-4 w-4 text-emerald-600 dark:text-emerald-400" />
							Inversiones finalizadas
						</h2>

						<div class={cardsGridClass(investState.maturedInvestments.length)}>
							{#each investState.maturedInvestments as inv}
								{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
								{@const invested = Number(inv.amount)}
								{@const actualReturn = Number(inv.actual_return ?? '0')}
								{@const totalReturn = invested + actualReturn}
								{@const pctReturn =
									invested > 0 ? ((actualReturn / invested) * 100).toFixed(2) : '0'}
								{@const returnPositive = actualReturn >= 0}
								<div
									class="rounded-xl border border-emerald-200 bg-card p-5 transition hover:shadow-sm dark:border-emerald-400/20"
								>
									<div class="mb-3 flex items-start justify-between gap-2">
										<div class="min-w-0 space-y-0.5">
											<a href={`/groups/${groupId}/investments/${inv.id}`} class="block">
												<p class="truncate text-sm font-medium text-foreground hover:underline">
													{inv.strategy_name}
												</p>
												<p class="text-xs text-muted-foreground">
													Vencida {formatDate(inv.updated_at)}
												</p>
											</a>
										</div>
										<span
											class="shrink-0 rounded-full border px-2.5 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
										>
											{risk.label}
										</span>
									</div>

									<div class="mb-1 grid grid-cols-2 gap-3">
										<div>
											<p
												class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase"
											>
												Invertido
											</p>
											<p class="text-sm font-semibold text-foreground">
												${formatAmount(invested)}
												{investState.getTicker(inv.currency_id)}
											</p>
										</div>
										<div>
											<p
												class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase"
											>
												Retorno
											</p>
											<p
												class="text-sm font-semibold {returnPositive
													? 'text-emerald-700 dark:text-emerald-300'
													: 'text-rose-700 dark:text-rose-300'}"
											>
												{returnPositive ? '+' : ''}${formatAmount(actualReturn)}
												{investState.getTicker(inv.currency_id)}
											</p>
										</div>
									</div>

									<div class="mb-4 flex items-center justify-between rounded-lg bg-muted px-3 py-2">
										<span class="text-xs text-muted-foreground">Total a retirar</span>
										<span class="text-sm font-bold text-foreground">
											${formatAmount(totalReturn)}
											{investState.getTicker(inv.currency_id)}
											<span
												class="text-xs font-medium {returnPositive
													? 'text-emerald-600 dark:text-emerald-300'
													: 'text-rose-600 dark:text-rose-300'}"
											>
												({returnPositive ? '+' : ''}{pctReturn}%)
											</span>
										</span>
									</div>

									{#if investState.withdrawError}
										<div
											class="mb-3 flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
										>
											<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
											<span>{investState.withdrawError}</span>
										</div>
									{/if}

									{#if !readonly}
										<Button
											label={investState.withdrawing ? 'Retirando...' : 'Retirar al grupo'}
											onclick={() => handleWithdraw(inv.id)}
											disabled={investState.withdrawing}
											loading={investState.withdrawing}
											fullWidth={true}
										>
											{#snippet icon()}<ArrowUpRight class="h-4 w-4" />{/snippet}
										</Button>
									{/if}
								</div>
							{/each}
						</div>
					</section>
				{/if}

				{#if investState.liquidatedInvestments.length > 0}
					<section id="liquidadas" class="scroll-mt-28 space-y-4">
						<h2
							class="flex items-center gap-2 text-sm font-medium text-rose-700 dark:text-rose-300"
						>
							<AlertTriangle class="h-4 w-4" />
							Liquidadas ({investState.liquidatedInvestments.length})
						</h2>
						<div class="space-y-2">
							{#each investState.liquidatedInvestments as inv}
								<a
									href={`/groups/${groupId}/investments/${inv.id}`}
									class="group flex items-center justify-between rounded-lg border border-rose-200 bg-rose-50/40 px-4 py-3 dark:border-rose-400/20 dark:bg-rose-400/10"
								>
									<div class="space-y-0.5">
										<p class="text-sm font-medium text-foreground group-hover:underline">
											{inv.strategy_name}
										</p>
										<p class="text-xs text-rose-700 dark:text-rose-300">
											Margen quemado · ${formatAmount(Number(inv.amount))}
											{investState.getTicker(inv.currency_id)}
											{#if (inv.leverage ?? 1) > 1}
												· x{inv.leverage}
											{/if}
											· {formatDate(inv.updated_at)}
										</p>
									</div>
									<span
										class="rounded-full border border-rose-200 bg-rose-50 px-2 py-0.5 text-[10px] font-semibold text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
									>
										Liquidada
									</span>
								</a>
							{/each}
						</div>
					</section>
				{/if}

				{#if investState.withdrawnInvestments.length > 0}
					<section id="retiradas" class="scroll-mt-28 space-y-4">
						<button
							onclick={() => (showPastInvestments = !showPastInvestments)}
							class="flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-foreground"
						>
							<Clock class="h-4 w-4" />
							Retiradas ({investState.withdrawnInvestments.length})
							<ChevronDown
								class="h-3.5 w-3.5 transition {showPastInvestments ? 'rotate-180' : ''}"
							/>
						</button>

						{#if showPastInvestments}
							<div class="space-y-2">
								{#each investState.withdrawnInvestments as inv}
									{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
									{@const exitLabel =
										inv.exit_kind === 'ragequit' ? 'Ragequit' : 'Retirada al madurar'}
									<a
										href={`/groups/${groupId}/investments/${inv.id}`}
										class="group flex items-center justify-between rounded-lg border border-border bg-card px-4 py-3 transition hover:border-border hover:shadow-sm"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted text-muted-foreground"
											>
												<Minus class="h-4 w-4" />
											</div>
											<div class="space-y-0.5">
												<p class="text-sm font-medium text-foreground group-hover:underline">
													{inv.strategy_name}
												</p>
												<p class="text-xs text-muted-foreground">
													${formatAmount(Number(inv.amount))}
													{investState.getTicker(inv.currency_id)}
													· {exitLabel}
													{formatDate(inv.updated_at)}
													{#if inv.exit_kind === 'ragequit' && inv.fee_amount}
														· fee ${formatAmount(Number(inv.fee_amount))}
													{/if}
												</p>
											</div>
										</div>
										<span
											class="rounded-full border px-2 py-0.5 text-[10px] font-medium {risk.bg} {risk.color}"
										>
											{risk.label}
										</span>
									</a>
								{/each}
							</div>
						{/if}
					</section>
				{/if}

				{#if investState.activeInvestments.length > 0}
					<section id="activas" class="scroll-mt-28 space-y-4">
						<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
							<TrendingUp class="h-4 w-4 text-emerald-600 dark:text-emerald-400" />
							Inversiones activas
							<span
								class="rounded-full bg-muted px-2 py-0.5 text-[11px] font-semibold text-muted-foreground"
							>
								{investState.activeInvestments.length}
							</span>
						</h2>
						<div class={cardsGridClass(investState.activeInvestments.length)}>
							{#each investState.activeInvestments as inv}
								{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
								{@const invested = Number(inv.amount)}
								{@const current = Number(inv.current_value)}
								{@const pctChange =
									invested > 0 ? (((current - invested) / invested) * 100).toFixed(2) : '0'}
								{@const isUp = current >= invested}
								{@const cat = inv.category ?? 'simulated'}
								{@const rq = ragequitPreview(inv)}
								<div
									class="rounded-xl border border-border bg-card p-5 transition hover:border-border hover:shadow-sm"
								>
									<a href={`/groups/${groupId}/investments/${inv.id}`} class="group block">
										<div class="mb-3 flex items-start justify-between gap-2">
											<div class="min-w-0 space-y-0.5">
												<p
													class="truncate text-sm font-medium text-foreground group-hover:underline"
												>
													{inv.strategy_name}
												</p>
												<p class="flex items-center gap-1.5 text-xs text-muted-foreground">
													<Calendar class="h-3 w-3" />
													Iniciada {formatDate(inv.started_at)}
												</p>
											</div>
											<div class="flex shrink-0 flex-col items-end gap-1">
												<span
													class="rounded-full border px-2.5 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
												>
													{risk.label}
												</span>
												<span
													class="rounded-full border px-2 py-0.5 text-[10px] font-medium {categoryBadge[
														cat
													] ?? categoryBadge.simulated}"
												>
													{categoryLabels[cat] ?? cat}
												</span>
											</div>
										</div>

										<div class="grid grid-cols-2 gap-3">
											<div>
												<p
													class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase"
												>
													Invertido
												</p>
												<p class="text-sm font-semibold text-foreground">
													${formatAmount(invested)}
													{investState.getTicker(inv.currency_id)}
												</p>
											</div>
											<div>
												<p
													class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase"
												>
													Valor actual
												</p>
												<p
													class="flex items-center gap-1 text-sm font-semibold {isUp
														? 'text-emerald-700 dark:text-emerald-300'
														: 'text-rose-700 dark:text-rose-300'}"
												>
													${formatAmount(current)}
													{investState.getTicker(inv.currency_id)}
													<span class="text-[11px] font-medium">
														{isUp ? '+' : ''}{pctChange}%
													</span>
													{#if isUp}
														<TrendingUp class="h-3.5 w-3.5" />
													{:else}
														<TrendingDown class="h-3.5 w-3.5" />
													{/if}
												</p>
											</div>
										</div>

										<div class="mt-3 flex items-center gap-1 text-xs text-muted-foreground">
											<BarChart3 class="h-3 w-3" />
											<span>Ver detalle →</span>
										</div>
									</a>

									{#if !readonly}
										<div class="mt-4 border-t border-border pt-3">
											{#if ragequitConfirmId === inv.id}
												<div class="space-y-2">
													<p class="text-xs text-muted-foreground">
														Ragequit: se cierra la posición al NAV actual con fee del {rq.feePct}%
														(se quema). Payout neto ≈ ${formatAmount(rq.payout)}
														{investState.getTicker(inv.currency_id)}.
													</p>
													{#if investState.withdrawError}
														<div
															class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-2 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
														>
															<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
															<span>{investState.withdrawError}</span>
														</div>
													{/if}
													<div class="flex gap-2">
														<button
															onclick={() => (ragequitConfirmId = null)}
															class="px-3 py-2 text-xs text-muted-foreground hover:text-foreground"
														>
															Cancelar
														</button>
														<Button
															label={investState.withdrawing ? 'Saliendo...' : 'Confirmar ragequit'}
															onclick={() => handleWithdraw(inv.id)}
															disabled={investState.withdrawing}
															loading={investState.withdrawing}
														/>
													</div>
												</div>
											{:else}
												<button
													onclick={() => {
														investState.withdrawError = '';
														ragequitConfirmId = inv.id;
													}}
													class="text-xs font-medium text-rose-700 hover:underline dark:text-rose-300"
												>
													Ragequit (early exit −{rq.feePct}%)
												</button>
											{/if}
										</div>
									{/if}
								</div>
							{/each}
						</div>
					</section>
				{/if}

				{#if investState.proposals.length > 0}
					<section id="propuestas" class="scroll-mt-28 space-y-4">
						<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
							<Check class="h-4 w-4 text-amber-600 dark:text-amber-400" />
							Propuestas aprobadas
							<span
								class="rounded-full bg-muted px-2 py-0.5 text-[11px] font-semibold text-muted-foreground"
							>
								{investState.proposals.length}
							</span>
						</h2>
						<div class={cardsGridClass(investState.proposals.length)}>
							{#each investState.proposals as proposal}
								<div
									class="rounded-xl border border-amber-200 bg-amber-50/60 p-5 transition hover:shadow-sm dark:border-amber-400/20 dark:bg-amber-400/10"
								>
									<div class="mb-4 flex items-start justify-between gap-3">
										<div class="space-y-1">
											<p class="text-sm font-medium text-foreground">{proposal.strategy_name}</p>
											<p class="text-xs text-muted-foreground">
												Monto: ${formatAmount(Number(proposal.amount))}
												{investState.getTicker(proposal.currency_id)}
											</p>
										</div>
									</div>

									{#if investState.executeError}
										<div
											class="mb-3 flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
										>
											<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
											<span>{investState.executeError}</span>
										</div>
									{/if}

									{#if !readonly}
										<Button
											label={executingProposal === proposal.proposal_id
												? 'Ejecutando...'
												: 'Ejecutar inversión'}
											onclick={() => handleExecute(proposal.proposal_id)}
											disabled={executingProposal === proposal.proposal_id}
											loading={executingProposal === proposal.proposal_id}
										>
											{#snippet icon()}<Rocket class="h-4 w-4" />{/snippet}
										</Button>
									{/if}
								</div>
							{/each}
						</div>
					</section>
				{/if}

				<section id="estrategias" class="scroll-mt-28 space-y-4">
					<div class="flex flex-wrap items-center justify-between gap-2">
						<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
							<CircleDollarSign class="h-4 w-4 text-muted-foreground" />
							Estrategias disponibles
							<span
								class="rounded-full bg-muted px-2 py-0.5 text-[11px] font-semibold text-muted-foreground"
							>
								{filteredStrategies.length}
							</span>
						</h2>
						{#if hasActiveFilters}
							<button
								type="button"
								onclick={clearFilters}
								class="text-xs font-medium text-muted-foreground underline-offset-2 hover:text-foreground hover:underline"
							>
								Limpiar filtros
							</button>
						{/if}
					</div>

					<div class="space-y-3 rounded-xl border border-border bg-card/50 p-4">
						<div class="space-y-1.5">
							<p class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
								Tipo
							</p>
							<div class="flex flex-wrap gap-1.5">
								{#each categoryFilters as f}
									<button
										type="button"
										onclick={() => (filterCategory = f.value)}
										class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
											filterCategory === f.value
										)}"
									>
										{f.label}
									</button>
								{/each}
							</div>
						</div>

						<div class="space-y-1.5">
							<p class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
								Riesgo
							</p>
							<div class="flex flex-wrap gap-1.5">
								{#each riskFilters as f}
									<button
										type="button"
										onclick={() => (filterRisk = f.value)}
										class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
											filterRisk === f.value
										)}"
									>
										{f.label}
									</button>
								{/each}
							</div>
						</div>

						<div class="space-y-1.5">
							<p class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
								Leverage
							</p>
							<div class="flex flex-wrap gap-1.5">
								<button
									type="button"
									onclick={() => (filterLeverage = null)}
									class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
										filterLeverage === null
									)}"
								>
									Todos
								</button>
								<button
									type="button"
									onclick={() => (filterLeverage = '1')}
									class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
										filterLeverage === '1'
									)}"
								>
									1x
								</button>
								<button
									type="button"
									onclick={() => (filterLeverage = 'levered')}
									class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
										filterLeverage === 'levered'
									)}"
								>
									Con leverage
								</button>
								{#each leverageOptions.filter((l) => l > 1) as lev}
									<button
										type="button"
										onclick={() => (filterLeverage = lev)}
										class="rounded-full border px-2.5 py-1 text-xs font-medium transition {chipClass(
											filterLeverage === lev
										)}"
									>
										x{lev}
									</button>
								{/each}
							</div>
						</div>
					</div>

					{#if investState.strategyError}
						<div
							class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
						>
							<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500 dark:text-rose-300" />
							<span>{investState.strategyError}</span>
						</div>
					{/if}

					{#if filteredStrategies.length === 0 && !investState.strategyError}
						<div class="rounded-xl border border-dashed border-border p-6 text-center">
							<p class="text-sm text-muted-foreground">No hay estrategias con estos filtros.</p>
							{#if hasActiveFilters}
								<button
									type="button"
									onclick={clearFilters}
									class="mt-2 text-xs font-medium text-foreground underline-offset-2 hover:underline"
								>
									Limpiar filtros
								</button>
							{/if}
						</div>
					{/if}

					<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
						{#each filteredStrategies as strategy}
							{@const risk = riskConfig[strategy.risk_level] ?? riskConfig.low}
							{@const isFormOpen = showStrategyForm === strategy.id}
							{@const cat = strategy.category ?? 'simulated'}
							{@const isMtm = strategy.valuation_mode === 'mark_to_market'}
							{@const lev = strategy.leverage ?? 1}
							<div class="rounded-xl border border-border bg-card p-5 transition hover:shadow-sm">
								<div class="mb-3 flex items-start justify-between gap-2">
									<div class="min-w-0 space-y-1">
										<p class="text-sm font-medium text-foreground">{strategy.name}</p>
										<p class="text-xs leading-relaxed text-muted-foreground">
											{strategy.description}
										</p>
									</div>
									<div class="flex shrink-0 flex-col items-end gap-1">
										<span
											class="rounded-full border px-2 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
										>
											{risk.label}
										</span>
										{#if lev > 1}
											<span
												class="rounded-full border border-rose-200 bg-rose-50 px-2 py-0.5 text-[10px] font-semibold text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
											>
												x{lev}
											</span>
										{/if}
										<span
											class="rounded-full border px-2 py-0.5 text-[10px] font-medium {categoryBadge[
												cat
											] ?? categoryBadge.simulated}"
										>
											{categoryLabels[cat] ?? cat}
										</span>
									</div>
								</div>

								{#if isMtm && strategy.allocations?.length}
									<div class="mb-3 flex flex-wrap gap-1">
										{#each strategy.allocations as alloc}
											<span
												class="inline-flex items-center gap-0.5 rounded-md bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground"
											>
												{alloc.symbol}
												{weightPct(alloc.weight_bps)}%
												<PriceSourceLink
													price_provider={alloc.price_provider}
													external_id={alloc.external_id}
													price_source_url={alloc.price_source_url}
													symbol={alloc.symbol}
													kind={alloc.kind}
												/>
											</span>
										{/each}
									</div>
								{/if}

								<div class="mb-4 space-y-2">
									{#if isMtm}
										<div class="flex items-center justify-between text-xs">
											<span class="text-muted-foreground">Valuación</span>
											<span class="font-semibold text-foreground">Mark-to-market</span>
										</div>
										{#if lev > 1}
											<div class="flex items-center justify-between text-xs">
												<span class="text-muted-foreground">Leverage</span>
												<span class="font-semibold text-rose-700 dark:text-rose-300">x{lev}</span>
											</div>
										{/if}
										<div class="flex items-center justify-between text-xs">
											<span class="text-muted-foreground">Ragequit fee</span>
											<span class="font-medium text-foreground"
												>{(strategy.ragequit_fee_bps / 100).toFixed(1)}%</span
											>
										</div>
									{:else}
										<div class="flex items-center justify-between text-xs">
											<span class="text-muted-foreground">Retorno esperado</span>
											<span class="font-semibold text-emerald-700 dark:text-emerald-300">
												+{strategy.expected_return_percentage}%
											</span>
										</div>
									{/if}
									<div class="flex items-center justify-between text-xs">
										<span class="text-muted-foreground">Duración</span>
										<span class="font-medium text-foreground">{strategy.duration_days} días</span>
									</div>
								</div>

								{#if !readonly}
									{#if isFormOpen}
										<div class="space-y-3 border-t border-border pt-4">
											{#if investState.proposeError}
												<div
													class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
												>
													<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
													<span>{investState.proposeError}</span>
												</div>
											{/if}

											<div>
												<label
													for="amount-input"
													class="mb-1 block text-xs font-medium text-muted-foreground">Monto</label
												>
												<input
													id="amount-input"
													type="number"
													step="0.01"
													min="0"
													bind:value={selectedAmount}
													placeholder="Ej: 100"
													class="w-full rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground focus:border-foreground focus:outline-none"
												/>
											</div>

											<div>
												<label
													for="currency-select"
													class="mb-1 block text-xs font-medium text-muted-foreground">Moneda</label
												>
												{#if investState.walletCurrencies.length > 0}
													<select
														id="currency-select"
														bind:value={selectedCurrencyId}
														class="w-full rounded-md border border-border bg-background px-3 py-2.5 text-sm text-foreground"
													>
														<option value="" disabled>Seleccionar moneda...</option>
														{#each investState.walletCurrencies as wc}
															<option value={wc.currency_id}>
																{wc.ticker}
															</option>
														{/each}
													</select>
												{:else if investState.walletsError}
													<div
														class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
													>
														<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
														<span>{investState.walletsError}</span>
													</div>
												{:else}
													<p class="text-xs text-muted-foreground">
														No hay wallets en el grupo. Creá una wallet primero.
													</p>
												{/if}
											</div>

											<div class="flex gap-2">
												<button
													onclick={() => toggleStrategyForm(strategy.id)}
													class="px-3 py-2 text-xs text-muted-foreground hover:text-foreground"
												>
													Cancelar
												</button>
												<Button
													label={investState.proposing ? 'Creando...' : 'Proponer'}
													onclick={() => handlePropose(strategy)}
													disabled={!selectedAmount || !selectedCurrencyId || investState.proposing}
													loading={investState.proposing}
												>
													{#snippet icon()}<Rocket class="h-4 w-4" />{/snippet}
												</Button>
											</div>
										</div>
									{:else}
										<Button
											label="Invertir"
											variant="secondary"
											fullWidth={true}
											onclick={() => toggleStrategyForm(strategy.id)}
										>
											{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
										</Button>
									{/if}
								{/if}
							</div>
						{/each}
					</div>
				</section>

				{#if investState.activeInvestments.length === 0 && investState.maturedInvestments.length === 0 && investState.withdrawnInvestments.length === 0 && investState.liquidatedInvestments.length === 0 && !investState.investmentError}
					<section class="space-y-4">
						<div class="rounded-xl border border-dashed border-border p-8 text-center">
							<TrendingUp class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
							<p class="text-sm font-medium text-foreground">Sin inversiones aún</p>
							<p class="text-sm text-muted-foreground">
								Elegí una estrategia más abajo para empezar a invertir.
							</p>
						</div>
					</section>
				{/if}

				{#if investState.investmentError}
					<div
						class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
					>
						<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
						<span>{investState.investmentError}</span>
					</div>
				{/if}
			</div>
		</div>

		<div class="w-full max-w-6xl pb-10">
			<a
				href={`/groups/${groupId}`}
				class="text-sm font-medium text-muted-foreground transition hover:text-foreground hover:underline"
			>
				← Volver al grupo
			</a>
		</div>
	{/if}
</div>
