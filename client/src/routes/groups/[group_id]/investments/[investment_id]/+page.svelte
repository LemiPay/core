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
	import { formatAmount, formatDate } from '$lib/utils/format_utils';
	import { InvestmentDetailState } from './investment_detail.svelte';

	const groupId = page.params.group_id as string;
	const investmentId = page.params.investment_id as string;
	const detailState = new InvestmentDetailState(groupId, investmentId);

	let loadingInit = $state(true);
	detailState.loadAll().finally(() => (loadingInit = false));

	const riskConfig: Record<string, { color: string; bg: string; label: string }> = {
		low: { color: 'text-emerald-700', bg: 'bg-emerald-50 border-emerald-200', label: 'Bajo' },
		medium: {
			color: 'text-amber-700',
			bg: 'bg-amber-50 border-amber-200',
			label: 'Medio'
		},
		high: { color: 'text-rose-700', bg: 'bg-rose-50 border-rose-200', label: 'Alto' }
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

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4">
	{#if loadingInit}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-black"
		></div>
	{:else if detailState.detailError}
		<div class="mt-20 space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-black">Error</h1>
			<p class="text-sm text-gray-500">{detailState.detailError}</p>
			<a
				href={`/groups/${groupId}/investments`}
				class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-400 transition hover:text-black"
			>
				<ArrowLeft class="h-4 w-4" /> Volver a inversiones
			</a>
		</div>
	{:else if detailState.investment}
		{@const inv = detailState.investment}
		{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
		{@const currency = detailState.getTicker(inv.currency_id)}

		<div class="w-full max-w-4xl pt-8 pb-6">
			<a
				href={`/groups/${groupId}/investments`}
				class="mb-4 inline-flex items-center gap-1.5 text-sm font-medium text-gray-400 transition hover:text-black"
			>
				<ArrowLeft class="h-4 w-4" /> Inversiones
			</a>
			<div class="flex items-center gap-3">
				<h1 class="text-2xl font-bold tracking-tight text-black">{inv.strategy_name}</h1>
				<span class="rounded-full border px-2.5 py-0.5 text-xs font-medium {risk.bg} {risk.color}">
					{risk.label}
				</span>
			</div>
		</div>

		<div class="w-full max-w-4xl space-y-8 pb-16">
			<div class="grid gap-4 sm:grid-cols-3">
				<div class="rounded-xl border border-gray-200 bg-white p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-gray-400 uppercase">
						Invertido
					</p>
					<p class="text-2xl font-bold text-black">
						${formatAmount(detailState.investedAmount)}
						{currency}
					</p>
				</div>

				<div class="rounded-xl border border-gray-200 bg-white p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-gray-400 uppercase">
						Valor actual
					</p>
					<p
						class="flex items-center gap-1.5 text-2xl font-bold {detailState.isUp
							? 'text-emerald-700'
							: 'text-rose-700'}"
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

				<div class="rounded-xl border border-gray-200 bg-white p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-gray-400 uppercase">Estado</p>
					<p class="flex items-center gap-1.5 text-2xl font-bold text-black">
						{#if inv.status === 'active'}
							<Rocket class="h-5 w-5 text-blue-600" />
							<span class="text-base font-medium">Activa</span>
						{:else if inv.status === 'matured'}
							<Check class="h-5 w-5 text-emerald-600" />
							<span class="text-base font-medium text-emerald-700">Finalizada</span>
						{:else}
							<Clock class="h-5 w-5 text-gray-600" />
							<span class="text-base font-medium text-gray-600">Retirada</span>
						{/if}
					</p>
				</div>
			</div>

			<div class="grid gap-4 sm:grid-cols-2">
				<div class="rounded-xl border border-gray-200 bg-white p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-gray-400 uppercase">
						Retorno esperado
					</p>
					<p class="flex items-center gap-1.5 text-lg font-bold text-emerald-700">
						+{inv.expected_return_percentage}%
					</p>
				</div>
				<div class="rounded-xl border border-gray-200 bg-white p-5">
					<p class="mb-1 text-[11px] font-medium tracking-wider text-gray-400 uppercase">
						Fecha inicio
					</p>
					<p class="flex items-center gap-1.5 text-lg font-bold text-black">
						<Calendar class="h-4 w-4 text-gray-500" />
						{formatDate(inv.started_at)}
					</p>
				</div>
			</div>

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
					<h2 class="flex items-center gap-2 text-sm font-medium text-black">
						<BarChart3 class="h-4 w-4 text-gray-600" />
						Evolución del valor
					</h2>

					<div class="rounded-xl border border-gray-200 bg-white p-6">
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

						<div class="mt-3 flex items-center justify-between border-t border-gray-100 pt-3">
							<p class="text-xs text-gray-500">
								Base: <span class="font-medium text-black">${formatAmount(baseline)}</span>
							</p>
							<p class="text-xs text-gray-500">
								Actual: <span class="font-medium text-black"
									>${formatAmount(detailState.currentValue)}</span
								>
							</p>
						</div>
					</div>
				</section>
			{/if}

			<section class="space-y-3">
				<h2 class="flex items-center gap-2 text-sm font-medium text-black">
					<Info class="h-4 w-4 text-gray-600" />
					Detalles
				</h2>
				<div
					class="divide-y divide-gray-100 overflow-hidden rounded-xl border border-gray-200 bg-white"
				>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-gray-500">Estrategia</span>
						<span class="font-medium text-black">{inv.strategy_name}</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-gray-500">Riesgo</span>
						<span class="font-medium {risk.color}">{risk.label}</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-gray-500">Retorno esperado</span>
						<span class="font-medium text-emerald-700">+{inv.expected_return_percentage}%</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-gray-500">Moneda</span>
						<span class="font-medium text-black">{currency}</span>
					</div>
					<div class="flex items-center justify-between px-4 py-3 text-sm">
						<span class="text-gray-500">Iniciada</span>
						<span class="text-black">{formatDate(inv.started_at)}</span>
					</div>
					{#if inv.actual_return}
						<div class="flex items-center justify-between px-4 py-3 text-sm">
							<span class="text-gray-500">Retorno generado</span>
							<span class="font-medium text-emerald-700">
								+${formatAmount(detailState.actualReturn)}
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
				class="text-sm font-medium text-gray-400 transition hover:text-black hover:underline"
			>
				← Volver a inversiones
			</a>
		</div>
	{/if}
</div>
