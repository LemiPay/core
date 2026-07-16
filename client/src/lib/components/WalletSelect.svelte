<script lang="ts">
	import { X } from 'lucide-svelte';

	interface Wallet {
		id: string;
		label: string;
	}

	interface Props {
		id: string;
		label: string;
		wallets: Wallet[];
		loading: boolean;
		emptyMessage: string;
		loadingMessage?: string;
		attempted: boolean;
		value: string;
		onchange?: (value: string) => void;
	}

	const {
		id,
		label,
		wallets,
		loading,
		emptyMessage,
		loadingMessage = 'Cargando billeteras...',
		attempted,
		value,
		onchange
	}: Props = $props();

	const selected = $derived(value !== '');
</script>

<div>
	<label for={id} class="mb-1.5 block text-sm font-medium text-foreground">
		{label}
	</label>

	{#if loading}
		<div class="flex items-center gap-2 py-2">
			<div
				class="h-4 w-4 animate-spin rounded-full border-2 border-border border-t-foreground"
			></div>
			<span class="text-sm text-muted-foreground">{loadingMessage}</span>
		</div>
	{:else if wallets.length === 0}
		<p class="rounded-md border border-border bg-muted/50 p-3 text-sm text-muted-foreground">
			{emptyMessage}
		</p>
	{:else}
		<select
			{id}
			{value}
			onchange={(e) => onchange?.((e.target as HTMLSelectElement).value)}
			class="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground transition focus:ring-0 focus:outline-none
				{attempted && !selected
				? 'border-red-400 focus:border-red-500 dark:border-red-400/50'
				: selected
					? 'border-green-400 focus:border-green-500 dark:border-green-400/50'
					: 'border-border focus:border-foreground'}"
		>
			<option value="" disabled>Elegí una opción</option>
			{#each wallets as wallet (wallet.id)}
				<option value={wallet.id}>{wallet.label}</option>
			{/each}
		</select>

		{#if attempted && !selected}
			<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500 dark:text-red-300">
				<X class="h-3.5 w-3.5 shrink-0" />
				Seleccioná una opción
			</p>
		{/if}
	{/if}
</div>
