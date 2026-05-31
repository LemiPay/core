<script lang="ts">
	import { slide } from 'svelte/transition';
	import {
		HandCoins,
		Calendar,
		Target,
		Users,
		CircleCheckBig,
		ChevronDown,
		Wallet,
		Ban
	} from 'lucide-svelte';

	import type { FundRoundCardProps } from '$lib/components/pages/fundRound/FundRoundCard.types';
	let {
		status,
		expandedFundRoundId,
		selectedContribWalletId,
		recommended,
		myContribution,
		ticker,
		compatibleWallets,
		contribLoading,
		contribError,
		memberCount,
		currentUserId,
		formatAmount,
		shortenAddress,
		statusDisplay,
		onToggleAccordion,
		onContribute,
		onCancelRound
	}: FundRoundCardProps = $props();

	const proposalId = $derived(status.fund_round.proposal.id);

	const proposalStatus = $derived(status.fund_round.proposal.status);

	const target = $derived(Number(status.target_amount));

	const raised = $derived(Number(status.total_contributed));

	const progress = $derived(target > 0 ? Math.min(100, Math.round((raised / target) * 100)) : 0);

	const remaining = $derived(Math.max(0, target - raised));

	const hasContributed = $derived(myContribution > 0);

	const myRemaining = $derived(Math.max(0, recommended - myContribution));

	const canContribute = $derived(
		proposalStatus === 'Approved' && !status.is_completed && !hasContributed
	);

	const isCreator = $derived(
		!!currentUserId && status.fund_round.proposal.created_by === currentUserId
	);

	const canCancel = $derived(isCreator && proposalStatus === 'Approved' && !status.is_completed);

	const isOpen = $derived(expandedFundRoundId === proposalId);
</script>

<div
	class="group rounded-xl border border-gray-200 bg-white transition hover:border-gray-300 hover:shadow-sm"
>
	<div class="space-y-4 p-5">
		<div class="flex items-start justify-between gap-3">
			<div class="flex items-start gap-3">
				<div
					class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-700"
				>
					<HandCoins class="h-5 w-5" />
				</div>

				<div class="space-y-1">
					<div class="flex items-baseline gap-1.5">
						<span class="text-2xl font-bold tracking-tight text-black">
							${target}
						</span>

						<span
							class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
						>
							{ticker}
						</span>
					</div>
					<p class="flex items-center gap-1 text-[11px] font-medium tracking-wide text-gray-400">
						<Calendar class="h-3 w-3" />

						{new Date(status.fund_round.proposal.created_at).toLocaleDateString('es-AR', {
							day: '2-digit',
							month: 'short',
							year: 'numeric'
						})}
					</p>
				</div>
			</div>

			<div class="flex shrink-0 items-center gap-1.5">
				<span
					class={`rounded-full border px-2.5 py-1 text-xs font-medium ${statusDisplay.classes}`}
				>
					{statusDisplay.label}
				</span>

				{#if canCancel}
					<div class="group/cancel relative flex">
						<button
							type="button"
							aria-label="Cancelar ronda"
							onclick={() => onCancelRound(proposalId)}
							class="flex h-7 w-7 items-center justify-center rounded-full border border-gray-200 bg-white text-gray-500 transition hover:border-red-200 hover:bg-red-50 hover:text-red-600 active:scale-95"
						>
							<Ban class="h-3.5 w-3.5" />
						</button>

						<span
							class="pointer-events-none invisible absolute top-full right-0 z-50 mt-2 rounded-md bg-[#222327] px-2.5 py-1 text-xs font-medium whitespace-nowrap text-white opacity-0 shadow-sm transition-all duration-200 group-hover/cancel:visible group-hover/cancel:opacity-100"
						>
							Cancelar ronda
						</span>
					</div>
				{/if}
			</div>
		</div>

		<div class="space-y-2">
			<div class="h-2 w-full overflow-hidden rounded-full bg-gray-100">
				<div
					class="h-full rounded-full bg-linear-to-r from-gray-800 to-black transition-all duration-700"
					style="width: {progress}%"
				></div>
			</div>

			<div class="flex items-center justify-between text-xs">
				<span class="font-medium text-gray-700">
					${formatAmount(raised)}
					<span class="text-gray-400">/ ${formatAmount(target)} {ticker}</span>
				</span>
				<span class="text-gray-500">
					<span class="font-medium text-gray-700">{progress}%</span>
					{#if remaining > 0}
						— faltan ${formatAmount(remaining)} {ticker}
					{:else}
						— completado
					{/if}
				</span>
			</div>
		</div>

		{#if proposalStatus === 'Approved'}
			<div
				class="flex flex-col gap-3 rounded-lg border border-gray-100 bg-gray-50/70 px-3 py-2.5 text-xs sm:flex-row sm:items-center sm:justify-between"
			>
				<div class="flex items-start gap-2">
					<Target class="mt-0.5 h-3.5 w-3.5 shrink-0 text-gray-400" />

					<div class="space-y-0.5">
						<p class="text-gray-500">
							Te toca aportar
							<span class="font-semibold text-black">${formatAmount(recommended)} {ticker}</span>
						</p>
						{#if memberCount > 0}
							<p class="flex items-center gap-1 text-[11px] text-gray-400">
								<Users class="h-3 w-3" />
								${formatAmount(target)} entre {memberCount}
								{memberCount === 1 ? 'miembro' : 'miembros'}
							</p>
						{/if}
					</div>
				</div>

				<div class="flex items-center gap-4 self-stretch sm:self-auto">
					{#if hasContributed}
						<span
							class="inline-flex flex-1 items-center justify-center gap-1 rounded-md border border-green-200 bg-green-50 px-2 py-1 font-medium text-green-700 sm:flex-none"
						>
							<CircleCheckBig class="h-3 w-3" />
							Aportaste ${formatAmount(myContribution)}
							{ticker}
						</span>
					{:else}
						<span class="flex-1 text-right font-medium text-gray-700 sm:flex-none sm:text-left">
							Te falta ${formatAmount(myRemaining)}
							{ticker}
						</span>
					{/if}

					{#if canContribute}
						<button
							onclick={() => onToggleAccordion(proposalId)}
							class="flex shrink-0 items-center gap-1.5 rounded-md bg-black px-3 py-1.5 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95"
						>
							Aportar
							<ChevronDown
								class={[
									'h-3.5 w-3.5 transition-transform duration-200',
									isOpen ? 'rotate-180' : ''
								].join(' ')}
							/>
						</button>
					{/if}
				</div>
			</div>
		{/if}
	</div>

	{#if isOpen && canContribute}
		<div
			transition:slide={{ duration: 220 }}
			class="space-y-4 overflow-hidden rounded-b-xl border-t border-gray-100 bg-gray-50/60 px-5 py-4"
		>
			<div class="space-y-1">
				<p class="text-[11px] font-medium tracking-wider text-gray-500 uppercase">
					Aportar a esta ronda
				</p>
				<p class="text-xs text-gray-500">Elegí desde qué wallet personal salen los fondos.</p>
			</div>

			{#if compatibleWallets.length === 0}
				<div
					class="flex items-start gap-2 rounded-md border border-gray-200 bg-white p-3 text-xs text-gray-500"
				>
					<Wallet class="mt-0.5 h-3.5 w-3.5 shrink-0 text-gray-400" />
					<span>No tenés wallets compatibles con la moneda de esta ronda.</span>
				</div>
			{:else}
				<div class="space-y-3">
					<select
						bind:value={selectedContribWalletId}
						class="w-full rounded-md border border-gray-200 bg-white px-3 py-2.5 text-sm text-black transition outline-none focus:border-black focus:ring-1 focus:ring-black"
					>
						<option value="" disabled>Seleccionar wallet personal...</option>
						{#each compatibleWallets as wallet (wallet.wallet_id)}
							<option value={wallet.wallet_id}>
								{shortenAddress(wallet.address)} — ${wallet.balance}
								{wallet.ticker}
							</option>
						{/each}
					</select>

					<div
						class="flex items-center justify-between rounded-md border border-gray-200 bg-white px-3 py-2.5 text-sm"
					>
						<span class="text-gray-500">Monto a aportar</span>
						<span class="flex items-baseline gap-1.5">
							<span class="font-semibold text-black">
								${formatAmount(recommended)}
							</span>

							<span
								class="rounded bg-black px-1.5 py-0.5 text-[10px] font-bold tracking-wider text-white uppercase"
							>
								{ticker}
							</span>
						</span>
					</div>

					{#if contribError}
						<p class="text-xs text-red-500">{contribError}</p>
					{/if}

					<div class="flex items-center justify-end gap-2">
						<button
							onclick={() => onToggleAccordion(proposalId)}
							class="rounded-md px-3.5 py-2 text-xs font-medium text-gray-500 transition hover:text-black"
							disabled={contribLoading}
						>
							Cancelar
						</button>
						<button
							onclick={() => onContribute(status)}
							class="rounded-md bg-black px-4 py-2 text-xs font-medium text-white transition hover:bg-gray-800 active:scale-95 disabled:opacity-40"
							disabled={!selectedContribWalletId || contribLoading}
						>
							{contribLoading
								? 'Enviando...'
								: `Confirmar aporte de $${formatAmount(recommended)} ${ticker}`}
						</button>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
