<script lang="ts">
	import { X, HandCoins, CheckCircle2, Clock, ChevronRight, Loader2 } from 'lucide-svelte';
	import { fade, slide } from 'svelte/transition';
	import { authStore } from '$lib/stores/auth'; // Ajustá la ruta a tu store

	import Button from '$lib/components/ui/Button.svelte';
	import {
		getAllWithdrawProposals,
		executeWithdrawProposal
	} from '$lib/api/endpoints/transactions';
	import { getMyWallets } from '$lib/api/endpoints/wallets';
	import { isSuccess } from '$lib/types/client.types';

	import type { WithdrawProposalExpanded } from '$lib/types/endpoints/transactions.types';
	import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';
	import { shortenAddress } from '$lib/utils/address_utils';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	// Estados
	let loading = $state(true);
	let actionLoading = $state(false);
	let proposals = $state<WithdrawProposalExpanded[]>([]);
	let wallets = $state<WalletCurrency[]>([]);
	let error = $state('');

	// Estado para el acordeón de ejecución
	let executingProposalId = $state<string | null>(null);
	let selectedWalletId = $state<string>('');

	// Derivados
	const currentUserId = $derived($authStore.user?.id);
	const selectedWallet = $derived(wallets.find((w) => w.wallet_id === selectedWalletId));

	// Funciones de carga
	async function loadData() {
		loading = true;
		error = '';

		// Cargamos propuestas y wallets en paralelo para que sea más rápido
		const [proposalsRes, walletsRes] = await Promise.all([
			getAllWithdrawProposals(group_id),
			getMyWallets()
		]);

		if (isSuccess(proposalsRes)) {
			proposals = proposalsRes.body;
		} else {
			error = 'No se pudieron cargar las propuestas.';
		}

		if (isSuccess(walletsRes)) {
			wallets = walletsRes.body.flatMap((group) => group.currencies);
		}

		loading = false;
	}

	$effect(() => {
		if (open) {
			loadData();
			// Reseteamos el estado visual al abrir
			executingProposalId = null;
			selectedWalletId = '';
		}
	});

	// Lógica del acordeón
	function toggleExecution(proposalId: string) {
		if (executingProposalId === proposalId) {
			executingProposalId = null;
			selectedWalletId = '';
		} else {
			executingProposalId = proposalId;
			selectedWalletId = '';
		}
	}

	// Ejecutar
	async function handleExecute(proposal: WithdrawProposalExpanded) {
		if (!selectedWallet) return;

		actionLoading = true;
		error = '';

		const request = {
			currency_id: proposal.withdraw_proposal.currency_id,
			proposal_id: proposal.proposal.id,
			address: selectedWallet.address
		};

		const res = await executeWithdrawProposal(group_id, request);

		actionLoading = false;

		if (!isSuccess(res)) {
			error = res.message || 'Error al ejecutar la propuesta.';
			return;
		}

		// Si fue un éxito, cerramos el acordeón y recargamos la lista
		executingProposalId = null;
		selectedWalletId = '';
		await loadData();
		onsuccess?.();
	}
</script>

{#if open}
	<div
		role="presentation"
		tabindex="-1"
		class="fixed inset-0 z-40 bg-black/20 backdrop-blur-sm transition-opacity"
		transition:fade={{ duration: 200 }}
		onclick={onclose}
		onkeydown={(e) => e.key === 'Escape' && onclose()}
	></div>

	<div
		class="fixed inset-y-0 right-0 z-50 flex w-full max-w-md flex-col border-l border-gray-200 bg-white shadow-2xl"
		transition:slide={{ axis: 'x', duration: 300 }}
	>
		<div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
			<h2 class="text-lg font-bold text-black">Propuestas de Retiro</h2>
			<button
				onclick={onclose}
				class="rounded-md p-1.5 text-gray-400 transition hover:bg-gray-100 hover:text-black"
			>
				<X class="h-5 w-5" />
			</button>
		</div>

		<div class="flex-1 overflow-y-auto p-6">
			{#if error}
				<div class="mb-4 rounded-md bg-red-50 p-3 text-sm text-red-600">
					{error}
				</div>
			{/if}

			{#if loading}
				<div class="flex items-center justify-center py-10">
					<Loader2 class="h-6 w-6 animate-spin text-gray-400" />
				</div>
			{:else if proposals.length === 0}
				<div class="flex flex-col items-center justify-center py-12 text-center">
					<HandCoins class="mb-3 h-8 w-8 text-gray-300" />
					<p class="text-sm font-medium text-black">Sin propuestas</p>
					<p class="text-sm text-gray-500">No hay retiros pendientes en este grupo.</p>
				</div>
			{:else}
				<div class="space-y-4">
					{#each proposals as p}
						{@const isOwner = p.proposal.created_by === currentUserId}
						{@const isApproved = p.proposal.status === 'Approved'}
						{@const isExecuted = p.proposal.status === 'Executed'}
						{@const compatibleWallets = wallets.filter(
							(w) => w.currency_id === p.withdraw_proposal.currency_id
						)}

						<div
							class="overflow-hidden rounded-lg border border-gray-200 bg-white transition-all hover:border-gray-300"
						>
							<div class="p-4">
								<div class="flex items-start justify-between">
									<div class="space-y-1">
										<div class="flex items-center gap-2">
											<span class="text-lg font-bold text-black">${p.withdraw_proposal.amount}</span
											>
										</div>
										<div class="text-xs text-gray-500 capitalize">
											{new Date(p.proposal.created_at).toLocaleDateString('es-AR', {
												day: '2-digit',
												month: 'short',
												year: 'numeric'
											})}
										</div>
									</div>

									{#if isExecuted}
										<span
											class="inline-flex items-center gap-1 rounded bg-black px-2 py-1 text-xs font-medium text-white"
										>
											<CheckCircle2 class="h-3 w-3" /> Ejecutado
										</span>
									{:else if isApproved}
										<span
											class="inline-flex items-center gap-1 rounded border border-green-200 bg-green-50 px-2 py-1 text-xs font-medium text-green-700"
										>
											Aprobado
										</span>
									{:else}
										<span
											class="inline-flex items-center gap-1 rounded border border-gray-200 bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600"
										>
											<Clock class="h-3 w-3" /> Pendiente
										</span>
									{/if}
								</div>

								{#if isOwner && isApproved && !isExecuted}
									<div class="mt-4 border-t border-gray-100 pt-3">
										<button
											onclick={() => toggleExecution(p.proposal.id)}
											class="flex w-full items-center justify-between text-sm font-medium transition-colors {executingProposalId ===
											p.proposal.id
												? 'text-black'
												: 'text-gray-500 hover:text-black'}"
										>
											<span>Ejecutar retiro</span>
											<ChevronRight
												class="h-4 w-4 transition-transform {executingProposalId === p.proposal.id
													? 'rotate-90'
													: ''}"
											/>
										</button>
									</div>
								{/if}
							</div>

							{#if executingProposalId === p.proposal.id}
								<div
									class="border-t border-gray-100 bg-gray-50 p-4"
									transition:slide={{ duration: 200 }}
								>
									<label
										for="wallet-select-{p.proposal.id}"
										class="mb-2 block text-xs font-medium text-gray-600"
									>
										Seleccioná tu wallet de destino
									</label>

									{#if compatibleWallets.length === 0}
										<p class="text-xs text-red-500">
											No tenés wallets compatibles con esta moneda.
										</p>
									{:else}
										<select
											id="wallet-select-{p.proposal.id}"
											bind:value={selectedWalletId}
											class="mb-3 w-full rounded-md border border-gray-200 bg-white px-3 py-2 text-sm text-black focus:border-black focus:ring-0 focus:outline-none"
										>
											<option value="" disabled>Elegí una wallet</option>
											{#each compatibleWallets as w}
												<option value={w.wallet_id}>
													{shortenAddress(w.address) + ' - ' + w.ticker}
												</option>
											{/each}
										</select>

										<div class="flex justify-end">
											<Button
												label="Confirmar Ejecución"
												variant="primary"
												disabled={!selectedWalletId || actionLoading}
												loading={actionLoading}
												onclick={() => handleExecute(p)}
											/>
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
{/if}
