<script lang="ts">
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import { generateRandomAddress } from '$lib/utils/address_utils';
	import { Sparkles } from 'lucide-svelte';

	interface Props {
		id?: string;
		value: string;
		attempted?: boolean;
	}

	let { id = 'wallet-address', value = $bindable(''), attempted = false }: Props = $props();
</script>

<div class="relative">
	<FormField
		{id}
		label="Dirección de wallet"
		type="text"
		placeholder="0x..."
		minLength={3}
		maxLength={100}
		bind:value
		{attempted}
	/>

	<button
		type="button"
		onclick={() => (value = generateRandomAddress())}
		class="absolute top-0 right-0 flex items-center gap-1.5 rounded-md px-2 py-0.5 text-[11px] font-medium text-gray-400 transition hover:bg-gray-100 hover:text-black"
	>
		<Sparkles class="h-3 w-3" />
		Generar aleatoria
	</button>
</div>
