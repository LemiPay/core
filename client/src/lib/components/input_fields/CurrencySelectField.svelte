<script lang="ts">
	import { isSuccess } from '$lib/types/client.types';
	import { getAllCurrencies } from '$lib/api/endpoints/currency';
	import type { Currency } from '$lib/types/endpoints/currency.types';
	import usdcLogo from '$lib/assets/USDC_logo.png';

	interface Props {
		id?: string;
		label?: string;
		value: string;
		attempted?: boolean;
	}

	let {
		id = 'currency-select',
		label = 'Moneda',
		value = $bindable(''),
		attempted = false
	}: Props = $props();

	let touched = $state(false);
	let loading = $state(true);
	let error = $state('');
	let currencies = $state<Currency[]>([]);

	const showFeedback = $derived(touched || attempted);
	const isEmpty = $derived(value === '' || value == null);
	const isValid = $derived(!isEmpty);

	// Diccionario para mapear el ticker con su logo correspondiente
	const currencyLogos: Record<string, string> = {
		USDC: usdcLogo
		// En el futuro podés agregar más: 'ETH': ethLogo, 'BTC': btcLogo, etc.
	};

	// Reactividad: buscamos el logo de la moneda que el usuario seleccionó
	const selectedLogo = $derived(currencyLogos[value]);

	async function fetchCurrencies() {
		loading = true;
		error = '';
		const res = await getAllCurrencies();
		loading = false;

		if (!isSuccess(res)) {
			error = res.message || 'Error al cargar monedas';
			return;
		}
		currencies = res.body;
	}

	$effect(() => {
		fetchCurrencies();
	});
</script>

<div>
	<label for={id} class="mb-1.5 block text-sm font-medium text-black">{label}</label>

	<div class="relative">
		{#if selectedLogo}
			<img
				src={selectedLogo}
				alt="Logo de {value}"
				class="pointer-events-none absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 rounded-full shadow-xs"
			/>
		{/if}

		<select
			{id}
			bind:value
			onblur={() => (touched = true)}
			disabled={loading || error !== ''}
			class="w-full appearance-none rounded-md border py-2 pr-10 text-sm text-black transition focus:ring-0 focus:outline-none disabled:bg-gray-50 disabled:text-gray-400
             {selectedLogo ? 'pl-10' : 'pl-3'}
             {showFeedback
				? isValid
					? 'border-green-400 focus:border-green-500'
					: 'border-red-400 focus:border-red-500'
				: 'border-gray-200 focus:border-gray-400'}"
		>
			<option value="" disabled selected>
				{loading ? 'Cargando monedas...' : error ? 'Error al cargar' : 'Elegí una moneda'}
			</option>

			{#each currencies as currency}
				<option value={currency.currency_ticker}>
					{currency.currency_ticker}
				</option>
			{/each}
		</select>

		<div class="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2 text-gray-400">
			{#if loading}
				<div
					class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
				></div>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"><path d="m6 9 6 6 6-6" /></svg
				>
			{/if}
		</div>
	</div>

	{#if showFeedback}
		<div
			class="mt-1.5 flex items-center gap-1 text-xs {isValid ? 'text-green-600' : 'text-red-500'}"
		>
			{#if isValid}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-3.5 w-3.5 shrink-0"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"><polyline points="20 6 9 17 4 12" /></svg
				>
				¡Se ve bien!
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-3.5 w-3.5 shrink-0"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
					><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
				>
				Seleccioná una moneda
			{/if}
		</div>
	{:else if error}
		<div class="mt-1.5 text-xs text-red-500">{error}</div>
	{:else}
		<div class="mt-1.5 h-[18px]"></div>
	{/if}
</div>
