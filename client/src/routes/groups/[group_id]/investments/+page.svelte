<script lang="ts">
	import { page } from '$app/state';
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
		RefreshCw
	} from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount, formatDate, formatDateTimeShort } from '$lib/utils/format_utils';
	import { InvestmentsState } from './investments.svelte';
	import type { InvestmentStrategy } from '$lib/types/endpoints/investments.types';

	const groupId = page.params.group_id as string;
	const investState = new InvestmentsState(groupId);

	let loadingInit = $state(true);

	let showStrategyForm = $state<string | null>(null);
	let selectedAmount = $state('');
	let selectedCurrencyId = $state('');
	let showPastInvestments = $state(false);
	let showProposalExecute = $state<string | null>(null);

	let pendingProposal = $state<{
		proposal_id: string;
		amount: string;
		strategy_name: string;
		currency_id: string;
	} | null>(null);

	investState.loadAll().finally(() => (loadingInit = false));

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
		const proposal = await investState.propose(data);
		if (proposal) {
			pendingProposal = {
				proposal_id: proposal.proposal_id,
				amount: proposal.amount,
				strategy_name: strategy.name,
				currency_id: proposal.currency_id
			};
			showStrategyForm = null;
			selectedAmount = '';
			selectedCurrencyId = '';
		}
	}

	async function handleExecute(proposalId: string) {
		investState.executeError = '';
		showProposalExecute = proposalId;
		const ok = await investState.execute(proposalId);
		if (ok) {
			pendingProposal = null;
			showProposalExecute = null;
		}
	}

	async function handleWithdraw(investmentId: string) {
		investState.withdrawError = '';
		await investState.withdraw(investmentId);
	}

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
	<title>Lemipay - {investState.groupData.name || 'Group'} - Inversiones</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center px-4 pt-16">
	{#if loadingInit}
		<div
			class="mt-20 h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-black"
		></div>
	{:else}
		<div class="w-full max-w-4xl pt-8 pb-6">
			<div class="flex items-center gap-3">
				<h1 class="text-2xl font-bold tracking-tight text-black">Inversiones</h1>
				<button
					onclick={() => investState.loadAll()}
					class="rounded-md p-2 text-gray-400 transition hover:bg-gray-100 hover:text-gray-700"
					title="Recargar"
				>
					<RefreshCw class="h-4 w-4" />
				</button>
			</div>
		</div>

		<div class="w-full max-w-4xl space-y-10 pb-16">
			{#if investState.activeInvestments.length > 0}
				<section class="space-y-4">
					<h2 class="flex items-center gap-2 text-sm font-medium text-black">
						<TrendingUp class="h-4 w-4 text-emerald-600" />
						Inversiones activas
						<span
							class="rounded-full bg-gray-100 px-2 py-0.5 text-[11px] font-semibold text-gray-600"
						>
							{investState.activeInvestments.length}
						</span>
					</h2>
					<div class="grid gap-3 sm:grid-cols-2">
						{#each investState.activeInvestments as inv}
							{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
							{@const invested = Number(inv.amount)}
							{@const current = Number(inv.current_value)}
							{@const pctChange =
								invested > 0 ? (((current - invested) / invested) * 100).toFixed(2) : '0'}
							{@const isUp = current >= invested}
							<a
								href={`/groups/${groupId}/investments/${inv.id}`}
								class="group block rounded-xl border border-gray-200 bg-white p-5 transition hover:border-gray-300 hover:shadow-sm"
							>
								<div class="mb-3 flex items-start justify-between gap-2">
									<div class="min-w-0 space-y-0.5">
										<p class="truncate text-sm font-medium text-black group-hover:underline">
											{inv.strategy_name}
										</p>
										<p class="flex items-center gap-1.5 text-xs text-gray-500">
											<Calendar class="h-3 w-3" />
											Iniciada {formatDate(inv.started_at)}
										</p>
									</div>
									<span
										class="shrink-0 rounded-full border px-2.5 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
									>
										{risk.label}
									</span>
								</div>

								<div class="grid grid-cols-2 gap-3">
									<div>
										<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
											Invertido
										</p>
										<p class="text-sm font-semibold text-black">
											${formatAmount(invested)}
											{investState.getTicker(inv.currency_id)}
										</p>
									</div>
									<div>
										<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
											Valor actual
										</p>
										<p
											class="flex items-center gap-1 text-sm font-semibold {isUp
												? 'text-emerald-700'
												: 'text-rose-700'}"
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

								<div class="mt-3 flex items-center gap-1 text-xs text-gray-400">
									<BarChart3 class="h-3 w-3" />
									<span>Ver detalle →</span>
								</div>
							</a>
						{/each}
					</div>
				</section>
			{/if}

			{#if pendingProposal}
				{@const proposal = pendingProposal}
				<section class="space-y-4">
					<h2 class="flex items-center gap-2 text-sm font-medium text-black">
						<Check class="h-4 w-4 text-amber-600" />
						Propuesta aprobada
					</h2>
					<div
						class="rounded-xl border border-amber-200 bg-amber-50/60 p-5 transition hover:shadow-sm"
					>
						<div class="mb-4 flex items-start justify-between gap-3">
							<div class="space-y-1">
								<p class="text-sm font-medium text-black">{proposal.strategy_name}</p>
								<p class="text-xs text-gray-600">
									Monto: ${formatAmount(Number(proposal.amount))}
									{investState.getTicker(proposal.currency_id)}
								</p>
							</div>
						</div>

						{#if investState.executeError}
							<div
								class="mb-3 flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
							>
								<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
								<span>{investState.executeError}</span>
							</div>
						{/if}

						<Button
							label={showProposalExecute === proposal.proposal_id
								? 'Ejecutando...'
								: 'Ejecutar inversión'}
							onclick={() => handleExecute(proposal.proposal_id)}
							disabled={showProposalExecute === proposal.proposal_id}
							loading={showProposalExecute === proposal.proposal_id}
						>
							{#snippet icon()}<Rocket class="h-4 w-4" />{/snippet}
						</Button>
					</div>
				</section>
			{/if}

			<section class="space-y-4">
				<h2 class="flex items-center gap-2 text-sm font-medium text-black">
					<CircleDollarSign class="h-4 w-4 text-gray-600" />
					Estrategias disponibles
				</h2>

				{#if investState.strategyError}
					<div
						class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
					>
						<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
						<span>{investState.strategyError}</span>
					</div>
				{/if}

				<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{#each investState.strategies as strategy}
						{@const risk = riskConfig[strategy.risk_level] ?? riskConfig.low}
						{@const isFormOpen = showStrategyForm === strategy.id}
						<div class="rounded-xl border border-gray-200 bg-white p-5 transition hover:shadow-sm">
							<div class="mb-3 flex items-start justify-between gap-2">
								<div class="min-w-0 space-y-1">
									<p class="text-sm font-medium text-black">{strategy.name}</p>
									<p class="text-xs leading-relaxed text-gray-500">{strategy.description}</p>
								</div>
								<span
									class="shrink-0 rounded-full border px-2 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
								>
									{risk.label}
								</span>
							</div>

							<div class="mb-4 space-y-2">
								<div class="flex items-center justify-between text-xs">
									<span class="text-gray-500">Retorno esperado</span>
									<span class="font-semibold text-emerald-700">
										+{strategy.expected_return_percentage}%
									</span>
								</div>
								<div class="flex items-center justify-between text-xs">
									<span class="text-gray-500">Duración</span>
									<span class="font-medium text-black">{strategy.duration_days} días</span>
								</div>
							</div>

							{#if isFormOpen}
								<div class="space-y-3 border-t border-gray-100 pt-4">
									{#if investState.proposeError}
										<div
											class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
										>
											<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
											<span>{investState.proposeError}</span>
										</div>
									{/if}

									<div>
										<label for="amount-input" class="mb-1 block text-xs font-medium text-gray-600"
											>Monto</label
										>
										<input
											id="amount-input"
											type="number"
											step="0.01"
											min="0"
											bind:value={selectedAmount}
											placeholder="Ej: 100"
											class="w-full rounded-md border border-gray-200 px-3 py-2 text-sm focus:border-black focus:outline-none"
										/>
									</div>

									<div>
										<label
											for="currency-select"
											class="mb-1 block text-xs font-medium text-gray-600">Moneda</label
										>
										{#if investState.walletCurrencies.length > 0}
											<select
												id="currency-select"
												bind:value={selectedCurrencyId}
												class="w-full rounded-md border border-gray-200 px-3 py-2.5 text-sm"
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
												class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
											>
												<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
												<span>{investState.walletsError}</span>
											</div>
										{:else}
											<p class="text-xs text-gray-400">
												No hay wallets en el grupo. Creá una wallet primero.
											</p>
										{/if}
									</div>

									<div class="flex gap-2">
										<button
											onclick={() => toggleStrategyForm(strategy.id)}
											class="px-3 py-2 text-xs text-gray-500 hover:text-black"
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
						</div>
					{/each}
				</div>
			</section>

			{#if investState.maturedInvestments.length > 0}
				<section class="space-y-4">
					<h2 class="flex items-center gap-2 text-sm font-medium text-black">
						<Check class="h-4 w-4 text-emerald-600" />
						Inversiones finalizadas
					</h2>

					<div class="grid gap-3 sm:grid-cols-2">
						{#each investState.maturedInvestments as inv}
							{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
							{@const invested = Number(inv.amount)}
							{@const actualReturn = Number(inv.actual_return ?? '0')}
							{@const totalReturn = invested + actualReturn}
							{@const pctReturn = invested > 0 ? ((actualReturn / invested) * 100).toFixed(2) : '0'}
							{@const withdrawId = `withdraw-${inv.id}`}
							<div
								class="rounded-xl border border-emerald-200 bg-white p-5 transition hover:shadow-sm"
							>
								<div class="mb-3 flex items-start justify-between gap-2">
									<div class="min-w-0 space-y-0.5">
										<p class="truncate text-sm font-medium text-black">{inv.strategy_name}</p>
										<p class="text-xs text-gray-500">
											Vencida {formatDate(inv.updated_at)}
										</p>
									</div>
									<span
										class="shrink-0 rounded-full border px-2.5 py-0.5 text-[11px] font-medium {risk.bg} {risk.color}"
									>
										{risk.label}
									</span>
								</div>

								<div class="mb-1 grid grid-cols-2 gap-3">
									<div>
										<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
											Invertido
										</p>
										<p class="text-sm font-semibold text-black">
											${formatAmount(invested)}
											{investState.getTicker(inv.currency_id)}
										</p>
									</div>
									<div>
										<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
											Retorno
										</p>
										<p class="text-sm font-semibold text-emerald-700">
											+${formatAmount(actualReturn)}
											{investState.getTicker(inv.currency_id)}
										</p>
									</div>
								</div>

								<div class="mb-4 flex items-center justify-between rounded-lg bg-gray-50 px-3 py-2">
									<span class="text-xs text-gray-500">Total a retirar</span>
									<span class="text-sm font-bold text-black">
										${formatAmount(totalReturn)}
										{investState.getTicker(inv.currency_id)}
										<span class="text-xs font-medium text-emerald-600">(+{pctReturn}%)</span>
									</span>
								</div>

								{#if investState.withdrawError}
									<div
										class="mb-3 flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
									>
										<AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
										<span>{investState.withdrawError}</span>
									</div>
								{/if}

								<Button
									label={investState.withdrawing ? 'Retirando...' : 'Retirar al grupo'}
									onclick={() => handleWithdraw(inv.id)}
									disabled={investState.withdrawing}
									loading={investState.withdrawing}
									fullWidth={true}
								>
									{#snippet icon()}<ArrowUpRight class="h-4 w-4" />{/snippet}
								</Button>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			{#if investState.withdrawnInvestments.length > 0}
				<section class="space-y-4">
					<button
						onclick={() => (showPastInvestments = !showPastInvestments)}
						class="flex items-center gap-2 text-sm font-medium text-gray-500 hover:text-black"
					>
						<Clock class="h-4 w-4" />
						Retiradas ({investState.withdrawnInvestments.length})
						<ChevronDown class="h-3.5 w-3.5 transition {showPastInvestments ? 'rotate-180' : ''}" />
					</button>

					{#if showPastInvestments}
						<div class="space-y-2">
							{#each investState.withdrawnInvestments as inv}
								{@const risk = riskConfig[inv.risk_level] ?? riskConfig.low}
								<a
									href={`/groups/${groupId}/investments/${inv.id}`}
									class="group flex items-center justify-between rounded-lg border border-gray-200 bg-white px-4 py-3 transition hover:border-gray-300 hover:shadow-sm"
								>
									<div class="flex items-center gap-3">
										<div
											class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-500"
										>
											<Minus class="h-4 w-4" />
										</div>
										<div class="space-y-0.5">
											<p class="text-sm font-medium text-black group-hover:underline">
												{inv.strategy_name}
											</p>
											<p class="text-xs text-gray-400">
												${formatAmount(Number(inv.amount))}
												{investState.getTicker(inv.currency_id)}
												· Retirada {formatDate(inv.updated_at)}
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

			{#if investState.activeInvestments.length === 0 && investState.maturedInvestments.length === 0 && investState.withdrawnInvestments.length === 0 && !investState.investmentError}
				<section class="space-y-4">
					<div class="rounded-xl border border-dashed border-gray-300 p-8 text-center">
						<TrendingUp class="mx-auto mb-3 h-8 w-8 text-gray-400" />
						<p class="text-sm font-medium text-black">Sin inversiones aún</p>
						<p class="text-sm text-gray-500">
							Elegí una estrategia de arriba para empezar a invertir.
						</p>
					</div>
				</section>
			{/if}

			{#if investState.investmentError}
				<div
					class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
				>
					<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
					<span>{investState.investmentError}</span>
				</div>
			{/if}
		</div>

		<div class="w-full max-w-4xl pb-10">
			<a
				href={`/groups/${groupId}`}
				class="text-sm font-medium text-gray-400 transition hover:text-black hover:underline"
			>
				← Volver al grupo
			</a>
		</div>
	{/if}
</div>
