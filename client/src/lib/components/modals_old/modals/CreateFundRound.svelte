<script lang="ts">
	import Modal from '$lib/components/modals_old/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormField from '$lib/components/ui/FormField.svelte';
	import { X } from 'lucide-svelte';

	import { createFundRoundProposal } from '$lib/api/endpoints/fund_rounds';
	import { getGroupWallets } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';
	import type { GroupWallet } from '$lib/types/endpoints/groups.types';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	let groupWallets = $state<GroupWallet[]>([]);
	let loadingWallets = $state(false);
	let selectedCurrencyId = $state('');
	let targetAmount = $state('');
	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const currencySelected = $derived(selectedCurrencyId !== '');
	const amountValid = $derived(
		targetAmount != null &&
			targetAmount !== '' &&
			!isNaN(Number(String(targetAmount).replace(',', '.'))) &&
			Number(String(targetAmount).replace(',', '.')) > 0
	);
	const formValid = $derived(currencySelected && amountValid);

	async function loadGroupWallets() {
		loadingWallets = true;
		const res = await getGroupWallets(group_id);
		loadingWallets = false;
		if (!isSuccess(res)) {
			error = 'No se pudieron cargar las billeteras del grupo.';
			return;
		}
		groupWallets = res.body;
	}

	$effect(() => {
		if (open) {
			loadGroupWallets();
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;

		error = '';
		success = '';
		loading = true;

		const response = await createFundRoundProposal({
			group_id,
			currency_id: selectedCurrencyId,
			target_amount: String(targetAmount).replace(',', '.')
		});

		loading = false;

		if (!isSuccess(response)) {
			error = response.message || 'Error al crear la ronda de fondeo.';
			return;
		}

		success = 'Ronda de fondeo creada exitosamente!';

		setTimeout(() => {
			handleClose();
			onsuccess?.();
		}, 1200);
	}

	function handleClose() {
		selectedCurrencyId = '';
		targetAmount = '';
		attempted = false;
		error = '';
		success = '';
		loading = false;
		groupWallets = [];
		onclose();
	}
</script>

<Modal
	{open}
	title="Nueva ronda de fondeo"
	description="Proponé un objetivo de fondeo asociado a una billetera del grupo."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="create-fund-round-form" onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="fund-round-currency" class="mb-1.5 block text-sm font-medium text-black">
					Moneda
				</label>

				{#if loadingWallets}
					<div class="flex items-center gap-2 py-2">
						<div
							class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
						></div>
						<span class="text-sm text-gray-400">Cargando billeteras del grupo...</span>
					</div>
				{:else if groupWallets.length === 0}
					<p class="rounded-md border border-gray-200 bg-gray-50 p-3 text-sm text-gray-500">
						El grupo no tiene billeteras aún. Creá una antes de abrir una ronda de fondeo.
					</p>
				{:else}
					<select
						id="fund-round-currency"
						bind:value={selectedCurrencyId}
						class="w-full rounded-md border px-3 py-2 text-sm text-black transition focus:ring-0 focus:outline-none
							{attempted && !currencySelected
							? 'border-red-400 focus:border-red-500'
							: selectedCurrencyId
								? 'border-green-400 focus:border-green-500'
								: 'border-gray-200 focus:border-gray-400'}"
					>
						<option value="" disabled>Elegí una moneda</option>
						{#each groupWallets as wallet (wallet.id)}
							<option value={wallet.currency_id}>
								{wallet.currency_ticker ?? 'USDC'} (saldo: ${wallet.balance})
							</option>
						{/each}
					</select>

					{#if attempted && !currencySelected}
						<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
							<X class="h-3.5 w-3.5 shrink-0" />
							Seleccioná una moneda
						</p>
					{/if}
				{/if}
			</div>

			<FormField
				id="fund-round-target"
				label="Objetivo"
				minLength={0}
				maxLength={30}
				type="number"
				placeholder="0.00"
				bind:value={targetAmount}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Crear ronda"
			type="submit"
			form="create-fund-round-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
