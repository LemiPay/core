<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormField from '$lib/components/ui/FormField.svelte';
	import { X } from 'lucide-svelte';

	import { fundGroupWallet } from '$lib/api/endpoints/groups';
	import { getMyWallets } from '$lib/api/endpoints/wallets';
	import { isSuccess } from '$lib/types/client.types';
	import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';

	interface Props {
		open: boolean;
		group_id: string;
		wallet_id: string;
		currency_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess, wallet_id, currency_id }: Props = $props();

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
		wallets = res.body.flatMap((group) => group.currencies);
	}

	$effect(() => {
		if (open) {
			loadWallets();
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;

		// Le agregamos la validación de !selectedWallet acá
		// Así TypeScript entiende que si pasamos esta línea, la wallet existe.
		if (!formValid || !selectedWallet) return;

		error = '';
		success = '';
		loading = true;

		const response = await fundGroupWallet(group_id, {
			currency_id,
			// Al estar validados arriba, le sacamos el '?' y ya no tira error
			address: selectedWallet.address,
			amount: String(amount).replace(',', '.')
		});

		loading = false;

		if (!isSuccess(response)) {
			error = response.message || 'Error al fondear la wallet del grupo.';
			return;
		}

		success = 'Fondeo realizado exitosamente!';

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
	title="Fondear wallet del grupo"
	description="Transferí fondos desde tu wallet personal a la wallet del grupo."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="fund-group-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="sender-wallet" class="mb-1.5 block text-sm font-medium text-black">
					Wallet de origen
				</label>

				{#if loadingWallets}
					<div class="flex items-center gap-2 py-2">
						<div
							class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
						></div>
						<span class="text-sm text-gray-400">Cargando wallets...</span>
					</div>
				{:else if wallets.length === 0}
					<p class="py-2 text-sm text-gray-400">No tenés wallets disponibles.</p>
				{:else}
					<select
						id="sender-wallet"
						bind:value={selectedWalletId}
						class="w-full rounded-md border px-3 py-2 text-sm text-black transition focus:ring-0 focus:outline-none
                      {attempted && !walletSelected
							? 'border-red-400 focus:border-red-500'
							: selectedWalletId
								? 'border-green-400 focus:border-green-500'
								: 'border-gray-200 focus:border-gray-400'}"
					>
						<option value="" disabled>Elegí una wallet</option>
						{#each wallets as wallet}
							<option value={wallet.wallet_id}>
								{wallet.address.length > 10
									? wallet.address.slice(0, 8) + '...' + wallet.address.slice(-6)
									: wallet.address} — {wallet.ticker} (saldo: ${wallet.balance})
							</option>
						{/each}
					</select>

					{#if attempted && !walletSelected}
						<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
							<X class="h-3.5 w-3.5 shrink-0" />
							Seleccioná una wallet
						</p>
					{:else if selectedWallet}
						<p class="mt-1.5 text-xs text-gray-400">
							Saldo disponible: {selectedWallet.balance}
							{selectedWallet.ticker}
						</p>
					{/if}
				{/if}
			</div>

			<FormField
				id="fund-amount"
				label="Monto"
				minLength={0}
				maxLength={3}
				type="number"
				placeholder="0.00"
				bind:value={amount}
				{attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Fondear"
			type="submit"
			form="fund-group-wallet-form"
			disabled={!formValid}
			{loading}
		/>
	{/snippet}
</Modal>
