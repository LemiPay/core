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
				{@const minVal = Math.min(...values)}
				{@const maxVal = Math.max(...values)}
				{@const range = maxVal - minVal || 1}
				{@const baseline = detailState.investedAmount}

				<section class="space-y-4">
					<h2 class="flex items-center gap-2 text-sm font-medium text-black">
						<BarChart3 class="h-4 w-4 text-gray-600" />
						Evolución del valor
					</h2>

					<div class="rounded-xl border border-gray-200 bg-white p-6">
						<div class="flex items-end justify-between gap-0.5" style="height: 200px">
							{#each detailState.chartData as point}
								{@const pct = ((point.value - minVal) / range) * 100}
								{@const isAboveBaseline = point.value >= baseline}
								<div class="flex h-full flex-1 flex-col items-center justify-end">
									<div
										class="w-full rounded-t-sm transition-all duration-500 {isAboveBaseline
											? 'bg-emerald-500'
											: 'bg-rose-400'}"
										style="height: {Math.max(pct, 2)}%"
										title="{point.label}: ${formatAmount(point.value)}"
									></div>
								</div>
							{/each}
						</div>

						<div class="mt-4 flex items-center justify-between border-t border-gray-100 pt-3">
							<div class="space-y-0.5 text-xs text-gray-500">
								<p class="font-medium text-black">${formatAmount(minVal)}</p>
							</div>
							<div class="flex gap-4 text-[10px] text-gray-400">
								<span class="flex items-center gap-1">
									<span class="h-2 w-2 rounded-sm bg-emerald-500"></span> Sobre la inversión
								</span>
								<span class="flex items-center gap-1">
									<span class="h-2 w-2 rounded-sm bg-rose-400"></span> Bajo la inversión
								</span>
							</div>
							<div class="space-y-0.5 text-right text-xs text-gray-500">
								<p class="font-medium text-black">${formatAmount(maxVal)}</p>
							</div>
						</div>
					</div>

					<div class="flex flex-wrap gap-2">
						{#each detailState.chartData.slice(-7) as point}
							<div class="rounded-lg border border-gray-200 bg-white px-3 py-2 text-center text-xs">
								<p class="text-gray-400">{point.label}</p>
								<p
									class="font-semibold tabular-nums {point.value >= baseline
										? 'text-emerald-700'
										: 'text-rose-700'}"
								>
									${formatAmount(point.value)}
								</p>
							</div>
						{/each}
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
						<span class="text-gray-500">ID de inversión</span>
						<span class="font-mono text-xs text-black">{inv.id}</span>
					</div>
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
