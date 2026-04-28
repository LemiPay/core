<script lang="ts">
	import Modal from '$lib/components/modals_old/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import { X } from 'lucide-svelte';

	import { proposeWithdraw } from '$lib/api/endpoints/transactions';
	import { getMyWallets } from '$lib/api/endpoints/wallets';
	import { isSuccess } from '$lib/types/client.types';
	import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';

	interface Props {
		open: boolean;
		group_id: string;
		currency_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, currency_id, onclose, onsuccess }: Props = $props();

	let wallets = $state<WalletCurrency[]>([]);
	let loadingWallets = $state(false);
	let selectedWalletId = $state('');
	let amount = $state('');
	let attempted = $state(false);
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	const selectedWallet = $derived(wallets.find((w) => w.wallet_id === selectedWalletId));

	const amountValid = $derived(
		amount != null &&
			amount !== '' &&
			!isNaN(Number(String(amount).replace(',', '.'))) &&
			Number(String(amount).replace(',', '.')) > 0
	);

	const walletSelected = $derived(selectedWalletId !== '');
	const formValid = $derived(walletSelected && amountValid);

	async function loadWallets() {
		loadingWallets = true;
		const res = await getMyWallets();
		loadingWallets = false;
		if (!isSuccess(res)) {
			error = 'No se pudieron cargar tus wallets.';
			return;
		}
		wallets = res.body
			.flatMap((group) => group.currencies)
			.filter((w) => w.currency_id === currency_id);
	}

	$effect(() => {
		if (open) {
			loadWallets();
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;

		if (!formValid || !selectedWallet) return;

		error = '';
		success = '';
		loading = true;

		const request = {
			currency_id,
			user_address: selectedWallet.address,
			amount: String(amount).replace(',', '.')
		};

		const response = await proposeWithdraw(request, group_id);

		loading = false;

		if (!isSuccess(response)) {
			error = response.message || 'Error al proponer el retiro.';
			return;
		}

		success = 'Propuesta de retiro creada exitosamente!';

		setTimeout(() => {
			handleClose();
			onsuccess?.();
		}, 1200);
	}

	function handleClose() {
		selectedWalletId = '';
		amount = '';
		attempted = false;
		error = '';
		success = '';
		loading = false;
		wallets = [];
		onclose();
	}
</script>

<Modal
	{open}
	title="Proponer Retiro"
	description="Creá una propuesta para retirar fondos de la wallet del grupo hacia tu cuenta personal."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="propose-withdraw-form" onsubmit={handleSubmit} class="space-y-4">
			<FormField
				id="withdraw-amount"
				label="Monto a retirar"
				minLength={0}
				maxLength={3}
				type="number"
				placeholder="0.00"
				bind:value={amount}
				{attempted}
			/>

			<div>
				<label for="destination-wallet" class="mb-1.5 block text-sm font-medium text-black">
					Wallet de destino (Tu cuenta)
				</label>

				{#if loadingWallets}
					<div class="flex items-center gap-2 py-2">
						<div
							class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
						></div>
						<span class="text-sm text-gray-400">Buscando wallets compatibles...</span>
					</div>
				{:else if wallets.length === 0}
					<p class="rounded-md border border-gray-200 bg-gray-50 p-3 text-sm text-gray-500">
						No tenés wallets compatibles con la moneda de este grupo.
					</p>
				{:else}
					<select
						id="destination-wallet"
						bind:value={selectedWalletId}
						class="w-full rounded-md border px-3 py-2 text-sm text-black transition focus:ring-0 focus:outline-none
                      {attempted && !walletSelected
							? 'border-red-400 focus:border-red-500'
							: selectedWalletId
								? 'border-green-400 focus:border-green-500'
								: 'border-gray-200 focus:border-gray-400'}"
					>
						<option value="" disabled>Elegí una wallet de destino</option>
						{#each wallets as wallet}
							<option value={wallet.wallet_id}>
								{wallet.address.length > 10
									? wallet.address.slice(0, 8) + '...' + wallet.address.slice(-6)
									: wallet.address} — {wallet.ticker}
							</option>
						{/each}
					</select>

					{#if attempted && !walletSelected}
						<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
							<X class="h-3.5 w-3.5 shrink-0" />
							Seleccioná una wallet de destino
						</p>
					{/if}
				{/if}
			</div>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Proponer"
			type="submit"
			form="propose-withdraw-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
