<script lang="ts">
	import { page } from '$app/state';
	import {
		TrendingUp,
		TrendingDown,
		ArrowLeft,
		BarChart3,
		Calendar,
		Clock,
		Check,
		Info,
		Rocket
	} from 'lucide-svelte';
	import PriceSourceLink from '$lib/components/investments/PriceSourceLink.svelte';
	import { formatAmount, formatDate } from '$lib/utils/format_utils';
	import { categoryLabels, providerShortLabel } from '$lib/types/endpoints/investments.types';
	import { InvestmentDetailState } from './investment_detail.svelte';

	const groupId = page.params.group_id as string;
	const investmentId = page.params.investment_id as string;
	const detailState = new InvestmentDetailState(groupId, investmentId);

	let loadingInit = $state(true);
	detailState.loadAll().finally(() => (loadingInit = false));

	/** Prices can be large (BTC) or tiny (DOGE) — adaptive decimals. */
	function formatPrice(n: number): string {
		if (!Number.isFinite(n)) return '—';
		const abs = Math.abs(n);
		if (abs >= 1000) return n.toLocaleString('en-US', { maximumFractionDigits: 2 });
		if (abs >= 1) return n.toLocaleString('en-US', { maximumFractionDigits: 4 });
		return n.toLocaleString('en-US', { maximumFractionDigits: 8 });
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
	function makeChartCoords(values: number[], baseline: number) {
		const W = 600,
			H = 200,
			padX = 8,
			padY = 12;
		const maxAbove = Math.max(...values.map((v) => v - baseline), 0) || 1;
		const maxBelow = Math.max(...values.map((v) => baseline - v), 0);
		const totalRange = maxAbove + maxBelow || 1;
		const baselineY = padY + (maxAbove / totalRange) * (H - padY * 2);
		const toX = (i: number) => padX + (i / (values.length - 1)) * (W - padX * 2);
		const toY = (v: number) => padY + ((maxAbove - (v - baseline)) / totalRange) * (H - padY * 2);
		return { W, H, padX, padY, baselineY, toX, toY };
	}
</script>

<svelte:head>
	<title>Lemipay - {detailState.investment?.strategy_name ?? 'Inversión'}</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4 pt-16">
	{#if loadingInit}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-border border-t-foreground"
		></div>
	{:else if detailState.detailError}
		<div class="mt-20 space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-foreground">Error</h1>
			<p class="text-sm text-muted-foreground">{detailState.detailError}</p>
			<a
				href={`/groups/${groupId}/investments`}
				class="inline-flex items-center gap-1.5 text-sm font-medium text-muted-foreground transition hover:text-foreground"
			>
				<ArrowLeft class="h-4 w-4" /> Volver a inversiones
			</a>
		</div>
	{:else if detailState.investment}
		{@const inv = detailState.investment}
		{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
		{@const currency = detailState.getTicker(inv.currency_id)}

		{@const cat = inv.category ?? 'simulated'}
		{@const isMtm = inv.valuation_mode === 'mark_to_market'}
		<div class="w-full max-w-4xl pt-8 pb-6">
			<div class="flex flex-wrap items-center gap-2">
				<h1 class="text-2xl font-bold tracking-tight text-foreground">{inv.strategy_name}</h1>
				<span class="rounded-full border px-2.5 py-0.5 text-xs font-medium {risk.bg} {risk.color}">
					{risk.label}
				</span>
				<span
					class="rounded-full border px-2.5 py-0.5 text-xs font-medium {categoryBadge[cat] ??
						categoryBadge.simulated}"
				>
					{categoryLabels[cat] ?? cat}
				</span>
			</div>
			{#if isMtm}
				<p class="mt-2 text-sm text-muted-foreground">
					Portfolio paper mark-to-market
					{#if (inv.leverage ?? 1) > 1}
						· <span class="font-semibold text-rose-700 dark:text-rose-300"
							>leverage x{inv.leverage}</span
						>
					{/if}
				</p>
			{/if}
			{#if inv.status === 'liquidated'}
				<p
					class="mt-2 rounded-lg border border-rose-200 bg-rose-50/60 px-3 py-2 text-sm text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
				>
					Posición liquidada: el margen fue quemado y no se puede retirar.
				</p>
			{/if}
		</div>

		<div class="w-full max-w-4xl space-y-8 pb-16">
			<div class="grid gap-4 sm:grid-cols-3">
				<div class="rounded-xl border border-border bg-card p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Margen
					</p>
					<p class="text-2xl font-bold text-foreground">
						${formatAmount(detailState.investedAmount)}
						{currency}
					</p>
					{#if (inv.leverage ?? 1) > 1 && inv.entry_exposure}
						<p class="mt-1 text-xs text-muted-foreground">
							Exposición entry: ${formatAmount(Number(inv.entry_exposure))} (x{inv.leverage})
						</p>
					{/if}
				</div>

				<div class="rounded-xl border border-border bg-card p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Equity actual
					</p>
					<p
						class="flex items-center gap-1.5 text-2xl font-bold {detailState.isUp
							? 'text-emerald-700 dark:text-emerald-300'
							: 'text-rose-700 dark:text-rose-300'}"
					>
						${formatAmount(detailState.currentValue)}
						{currency}
						<span class="text-sm font-medium">
							{detailState.isUp ? '+' : ''}{detailState.pctChange.toFixed(2)}%
						</span>
						{#if detailState.isUp}
							<TrendingUp class="h-5 w-5" />
						{:else}
							<TrendingDown class="h-5 w-5" />
						{/if}
					</p>
				</div>

				<div class="rounded-xl border border-border bg-card p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Estado
					</p>
					<p class="flex items-center gap-1.5 text-2xl font-bold text-foreground">
						{#if inv.status === 'active'}
							<Rocket class="h-5 w-5 text-blue-600 dark:text-blue-400" />
							<span class="text-base font-medium">Activa</span>
						{:else if inv.status === 'matured'}
							<Check class="h-5 w-5 text-emerald-600 dark:text-emerald-400" />
							<span class="text-base font-medium text-emerald-700 dark:text-emerald-300"
								>Finalizada</span
							>
						{:else if inv.status === 'liquidated'}
							<span class="text-base font-medium text-rose-700 dark:text-rose-300">Liquidada</span>
						{:else}
							<Clock class="h-5 w-5 text-muted-foreground" />
							<span class="text-base font-medium text-muted-foreground">Retirada</span>
						{/if}
					</p>
				</div>
			</div>

			<div class="grid gap-4 sm:grid-cols-2">
				<div class="rounded-xl border border-border bg-card p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						{isMtm ? 'Valuación' : 'Retorno esperado'}
					</p>
					{#if isMtm}
						<p class="text-lg font-bold text-foreground">Mark-to-market</p>
					{:else}
						<p
							class="flex items-center gap-1.5 text-lg font-bold text-emerald-700 dark:text-emerald-300"
						>
							+{inv.expected_return_percentage}%
						</p>
					{/if}
				</div>
				<div class="rounded-xl border border-border bg-card p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Fecha inicio
					</p>
					<p class="flex items-center gap-1.5 text-lg font-bold text-foreground">
						<Calendar class="h-4 w-4 text-muted-foreground" />
						{formatDate(inv.started_at)}
					</p>
				</div>
			</div>

			{#if inv.holdings && inv.holdings.length > 0}
				<section class="space-y-3">
					<div class="flex flex-wrap items-center justify-between gap-2">
						<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
							<BarChart3 class="h-4 w-4 text-muted-foreground" />
							Holdings
						</h2>
					</div>
					<div class="overflow-hidden overflow-x-auto rounded-xl border border-border bg-card">
						<table class="w-full min-w-[640px] text-sm">
							<thead
								class="border-b border-border bg-muted/40 text-left text-xs text-muted-foreground"
							>
								<tr>
									<th class="px-4 py-2 font-medium">Asset</th>
									<th class="px-4 py-2 font-medium">Fuente</th>
									<th class="px-4 py-2 font-medium">Units</th>
									<th class="px-4 py-2 font-medium">Px entry</th>
									<th class="px-4 py-2 font-medium">Px actual</th>
									<th class="px-4 py-2 font-medium">Cost basis</th>
									<th class="px-4 py-2 font-medium">Valor actual</th>
									<th class="px-4 py-2 font-medium">PnL</th>
								</tr>
							</thead>
							<tbody class="divide-y divide-border">
								{#each inv.holdings as h}
									{@const units = Number(h.units)}
									{@const cost = Number(h.cost_basis_usd)}
									{@const entryPx = (() => {
										const raw = h.entry_price_usd;
										if (raw !== undefined && raw !== null && raw !== '') {
											const n = Number(raw);
											if (Number.isFinite(n) && n > 0) return n;
										}
										return units > 0 ? cost / units : 0;
									})()}
									{@const currentPx = (() => {
										const raw = h.current_price_usd;
										if (raw !== undefined && raw !== null && raw !== '') {
											const n = Number(raw);
											if (Number.isFinite(n) && n > 0) return n;
										}
										// Fallback: pro-rate portfolio NAV by entry weight (approx.)
										const w = h.weight_bps_at_entry ?? 0;
										const nav = Number(inv.current_value);
										if (w > 0 && units > 0 && Number.isFinite(nav) && nav > 0) {
											return (nav * w) / 10000 / units;
										}
										return entryPx > 0 ? entryPx : null;
									})()}
									{@const currentVal = currentPx != null && units > 0 ? currentPx * units : null}
									{@const pnl = currentVal != null ? currentVal - cost : null}
									{@const pnlPct = pnl != null && cost > 0 ? (pnl / cost) * 100 : null}
									<tr>
										<td class="px-4 py-2.5">
											<span class="inline-flex items-center gap-1.5">
												<span class="font-medium text-foreground">{h.symbol}</span>
												<PriceSourceLink
													price_provider={h.price_provider}
													external_id={h.external_id}
													price_source_url={h.price_source_url}
													symbol={h.symbol}
													kind={h.kind}
													size="md"
												/>
											</span>
											<span class="ml-1 text-xs text-muted-foreground">{h.name}</span>
										</td>
										<td class="px-4 py-2.5 text-xs text-muted-foreground">
											{providerShortLabel(h.price_provider, h.kind)}
										</td>
										<td class="px-4 py-2.5 font-mono text-xs text-foreground">
											{Number(h.units).toPrecision(6)}
										</td>
										<td class="px-4 py-2.5 font-mono text-xs text-muted-foreground">
											${formatPrice(entryPx)}
										</td>
										<td class="px-4 py-2.5 font-mono text-xs text-foreground">
											{#if currentPx != null}
												${formatPrice(currentPx)}
											{:else}
												<span class="text-muted-foreground">—</span>
											{/if}
										</td>
										<td class="px-4 py-2.5 text-muted-foreground">
											${formatAmount(cost)}
										</td>
										<td class="px-4 py-2.5 font-medium text-foreground">
											{#if currentVal != null}
												${formatAmount(currentVal)}
											{:else}
												<span class="text-muted-foreground">—</span>
											{/if}
										</td>
										<td class="px-4 py-2.5 text-xs font-medium">
											{#if pnl != null && pnlPct != null}
												<span
													class={pnl >= 0
														? 'text-emerald-700 dark:text-emerald-300'
														: 'text-rose-700 dark:text-rose-300'}
												>
													{pnl >= 0 ? '+' : ''}${formatAmount(pnl)}
													<span class="opacity-80">({pnl >= 0 ? '+' : ''}{pnlPct.toFixed(2)}%)</span
													>
												</span>
											{:else}
												<span class="text-muted-foreground">—</span>
											{/if}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</section>
			{/if}

			{#if detailState.chartData.length > 1}
				{@const values = detailState.chartData.map((d) => d.value)}
				{@const baseline = detailState.investedAmount}
				{@const { W, H, padX, padY, baselineY, toX, toY } = makeChartCoords(values, baseline)}
				{@const points = detailState.chartData.map((d, i) => ({
					x: toX(i),
					y: toY(d.value),
					value: d.value,
					label: d.label
				}))}
				{@const linePath = points.map((p, i) => `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`).join(' ')}
				{@const areaPath = `${linePath} L ${points[points.length - 1].x} ${baselineY} L ${points[0].x} ${baselineY} Z`}

				<section class="space-y-4">
					<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
						<BarChart3 class="h-4 w-4 text-muted-foreground" />
						Evolución del valor
					</h2>

					<div class="rounded-xl border border-border bg-card p-6">
						<svg
							viewBox="0 0 {W} {H}"
							class="w-full"
							style="height: 200px"
							preserveAspectRatio="none"
						>
							<defs>
								<linearGradient id="areaGradient" x1="0" y1="0" x2="0" y2="1">
									<stop offset="0%" stop-color="#10b981" stop-opacity="0.25" />
									<stop offset="100%" stop-color="#10b981" stop-opacity="0.03" />
								</linearGradient>
								<linearGradient
									id="lineGradient"
									x1="0"
									y1="0"
									x2="0"
									y2={H}
									gradientUnits="userSpaceOnUse"
								>
									<stop offset="{(baselineY / H) * 100}%" stop-color="#10b981" />
									<stop offset="{(baselineY / H) * 100}%" stop-color="#fb7185" />
								</linearGradient>
								<clipPath id="aboveBaseline">
									<rect x="0" y="0" width={W} height={baselineY} />
								</clipPath>
								<clipPath id="belowBaseline">
									<rect x="0" y={baselineY} width={W} height={H - baselineY} />
								</clipPath>
							</defs>

							<!-- Área sobre baseline (ganancia) -->
							<path d={areaPath} fill="url(#areaGradient)" clip-path="url(#aboveBaseline)" />
							<!-- Área bajo baseline (pérdida) -->
							<path
								d={areaPath}
								fill="#fb7185"
								fill-opacity="0.15"
								clip-path="url(#belowBaseline)"
							/>

							<!-- Línea baseline -->
							<line
								x1={padX}
								y1={baselineY}
								x2={W - padX}
								y2={baselineY}
								stroke="#d1d5db"
								stroke-width="1"
								stroke-dasharray="4 3"
							/>

							<!-- Línea del valor -->
							<path
								d={linePath}
								fill="none"
								stroke="url(#lineGradient)"
								stroke-width="2"
								stroke-linejoin="round"
								stroke-linecap="round"
							/>
						</svg>

						<div class="mt-3 flex items-center justify-between border-t border-border pt-3">
							<p class="text-xs text-muted-foreground">
								Base: <span class="font-medium text-foreground">${formatAmount(baseline)}</span>
							</p>
							<p class="text-xs text-muted-foreground">
								Actual: <span class="font-medium text-foreground"
									>${formatAmount(detailState.currentValue)}</span
								>
							</p>
						</div>
					</div>
				</section>
			{/if}

			<section class="space-y-3">
				<h2 class="flex items-center gap-2 text-sm font-medium text-foreground">
					<Info class="h-4 w-4 text-muted-foreground" />
					Detalles
				</h2>
				<div class="divide-y divide-border overflow-hidden rounded-xl border border-border bg-card">
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-muted-foreground">Estrategia</span>
						<span class="font-medium text-foreground">{inv.strategy_name}</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-muted-foreground">Riesgo</span>
						<span class="font-medium {risk.color}">{risk.label}</span>
					</div>
					{#if !isMtm}
						<div class="flex items-center justify-between px-4 py-3 text-sm">
							<span class="text-muted-foreground">Retorno esperado</span>
							<span class="font-medium text-emerald-700 dark:text-emerald-300"
								>+{inv.expected_return_percentage}%</span
							>
						</div>
					{/if}
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-muted-foreground">Moneda</span>
						<span class="font-medium text-foreground">{currency}</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-muted-foreground">Iniciada</span>
						<span class="text-foreground">{formatDate(inv.started_at)}</span>
					</div>
					{#if inv.exit_kind}
						<div class="flex items-center justify-between px-4 py-3 text-sm">
							<span class="text-muted-foreground">Tipo de salida</span>
							<span class="font-medium text-foreground">
								{inv.exit_kind === 'ragequit' ? 'Ragequit' : 'Madurez'}
							</span>
						</div>
					{/if}
					{#if inv.fee_amount && Number(inv.fee_amount) > 0}
						<div class="flex items-center justify-between px-4 py-3 text-sm">
							<span class="text-muted-foreground">Fee ragequit (quemado)</span>
							<span class="font-medium text-rose-700 dark:text-rose-300">
								${formatAmount(Number(inv.fee_amount))}
								{currency}
							</span>
						</div>
					{/if}
					{#if inv.actual_return != null}
						<div class="flex items-center justify-between px-4 py-3 text-sm">
							<span class="text-muted-foreground">Retorno generado</span>
							<span
								class="font-medium {detailState.actualReturn >= 0
									? 'text-emerald-700 dark:text-emerald-300'
									: 'text-rose-700 dark:text-rose-300'}"
							>
								{detailState.actualReturn >= 0 ? '+' : ''}${formatAmount(detailState.actualReturn)}
								{currency}
							</span>
						</div>
					{/if}
				</div>
			</section>
		</div>

		<div class="w-full max-w-4xl pb-10">
			<a
				href={`/groups/${groupId}/investments`}
				class="text-sm font-medium text-muted-foreground transition hover:text-foreground hover:underline"
			>
				← Volver a inversiones
			</a>
		</div>
	{/if}
</div>
