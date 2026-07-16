<script lang="ts">
	import { ExternalLink } from 'lucide-svelte';
	import {
		providerShortLabel,
		resolvePriceSourceUrl
	} from '$lib/types/endpoints/investments.types';

	let {
		price_provider,
		external_id,
		price_source_url,
		symbol,
		kind,
		size = 'sm'
	}: {
		price_provider?: string;
		external_id?: string;
		price_source_url?: string;
		symbol?: string;
		kind?: string;
		size?: 'sm' | 'md';
	} = $props();

	const url = $derived(
		resolvePriceSourceUrl({ price_provider, external_id, price_source_url, symbol, kind })
	);
	const label = $derived(providerShortLabel(price_provider, kind));
	const iconClass = $derived(size === 'md' ? 'h-3.5 w-3.5' : 'h-3 w-3');
</script>

{#if url}
	<a
		href={url}
		target="_blank"
		rel="noopener noreferrer"
		title="Ver en CoinGecko{symbol ? ` (${symbol})` : ''}"
		aria-label="Abrir CoinGecko para {symbol ?? 'activo'}"
		onclick={(e) => e.stopPropagation()}
		class="inline-flex items-center justify-center rounded p-0.5 text-muted-foreground transition hover:bg-muted hover:text-foreground"
	>
		<ExternalLink class={iconClass} />
	</a>
{/if}
