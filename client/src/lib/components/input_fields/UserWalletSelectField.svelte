<script lang="ts">
	import { getMyWallets } from '$lib/api/endpoints/wallets';
	import { isSuccess } from '$lib/types/client.types';
	import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';
	import { X, ChevronDown } from 'lucide-svelte';

	interface Props {
		id?: string;
		label?: string;
		currency_id?: string; // Opcional: si lo pasás, filtra solo wallets compatibles
		value: string;
		returnType?: 'wallet_id' | 'address'; // Elegí qué dato querés bindear
		attempted?: boolean;
	}

	let {
		id = 'user-wallet-select',
		label = 'Wallet de destino',
		currency_id = '',
		value = $bindable(''),
		returnType = 'address',
		attempted = false
	}: Props = $props();

	let wallets = $state<WalletCurrency[]>([]);
	let loading = $state(false);
	let error = $state('');
	let touched = $state(false);

	const showFeedback = $derived(touched || attempted);
	const isValid = $derived(value !== '');

	async function loadWallets() {
		loading = true;
		error = '';
		const res = await getMyWallets();
		loading = false;

		if (!isSuccess(res)) {
			error = 'No se pudieron cargar tus wallets.';
			return;
		}

		// Aplanamos las monedas de todos los grupos del usuario
		let allCurrencies = res.body.flatMap((group) => group.currencies);

		// Filtramos si nos pasaron un currency_id específico
		if (currency_id) {
			wallets = allCurrencies.filter((w) => w.currency_id === currency_id);
		} else {
			wallets = allCurrencies;
		}
	}

	// Volver a cargar/filtrar si cambia el currency_id
	$effect(() => {
		loadWallets();
	});

	function shortenAddress(address: string) {
		if (!address || address.length <= 14) return address;
		return address.slice(0, 8) + '...' + address.slice(-6);
	}
</script>

<div class="w-full">
	<label for={id} class="mb-1.5 block text-sm font-medium text-black">
		{label}
	</label>

	<div class="relative">
		{#if loading}
			<div class="absolute inset-y-0 left-3 flex items-center">
				<div
					class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
				></div>
			</div>
		{/if}

		<select
			{id}
			bind:value
			onblur={() => (touched = true)}
			disabled={loading || wallets.length === 0}
			class="w-full appearance-none rounded-md border px-3 py-2 text-sm text-black transition focus:ring-0 focus:outline-none disabled:bg-gray-50 disabled:text-gray-400
             {loading ? 'pl-9' : 'pl-3'}
             {showFeedback
				? isValid
					? 'border-green-400 focus:border-green-500'
					: 'border-red-400 focus:border-red-500'
				: 'border-gray-200 focus:border-gray-400'}"
		>
			<option value="" disabled selected>
				{loading
					? 'Buscando wallets compatibles...'
					: wallets.length === 0
						? 'Sin wallets compatibles'
						: 'Elegí una wallet de destino'}
			</option>

			{#each wallets as wallet}
				<option value={returnType === 'wallet_id' ? wallet.wallet_id : wallet.address}>
					{shortenAddress(wallet.address)} — {wallet.ticker}
					{#if wallet.balance}
						(Saldo: ${wallet.balance})
					{/if}
				</option>
			{/each}
		</select>

		<div class="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2 text-gray-400">
			<ChevronDown class="h-4 w-4" />
		</div>
	</div>

	{#if showFeedback && !isValid && wallets.length > 0}
		<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
			<X class="h-3.5 w-3.5 shrink-0" />
			Seleccioná una wallet de destino
		</p>
	{:else if wallets.length === 0 && !loading && !error}
		<p class="mt-1.5 text-xs text-gray-500">
			{currency_id
				? 'No tenés wallets compatibles con la moneda de este grupo.'
				: 'No tenés wallets configuradas.'}
		</p>
	{:else if error}
		<p class="mt-1.5 text-xs text-red-500">{error}</p>
	{:else}
		<div class="mt-1.5 h-[18px]"></div>
	{/if}
</div>
