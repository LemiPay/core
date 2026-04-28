<script lang="ts">
    import { getGroupWallets } from '$lib/api/endpoints/groups';
    import { isSuccess } from '$lib/types/client.types';
    import type { GroupWallet } from '$lib/types/endpoints/groups.types';
    import { X, ChevronDown } from 'lucide-svelte';

    interface Props {
        group_id: string;
        id?: string;
        label?: string;
        value: string; // El ID que queremos capturar (wallet_id o currency_id)
        returnType?: 'wallet_id' | 'currency_id'; // Para decidir qué valor bindiar
        attempted?: boolean;
    }

    let {
        group_id,
        id = 'group-wallet-select',
        label = 'Seleccionar Billetera',
        value = $bindable(''),
        returnType = 'currency_id',
        attempted = false
    }: Props = $props();

    let wallets = $state<GroupWallet[]>([]);
    let loading = $state(false);
    let error = $state('');
    let touched = $state(false);

    const showFeedback = $derived(touched || attempted);
    const isValid = $derived(value !== '');

    async function loadWallets() {
        if (!group_id) return;
        loading = true;
        error = '';
        const res = await getGroupWallets(group_id);
        loading = false;

        if (!isSuccess(res)) {
            error = 'No se pudieron cargar las billeteras.';
            return;
        }
        wallets = res.body;
    }

    // Recargar si cambia el group_id o cuando se monta
    $effect(() => {
        loadWallets();
    });
</script>

<div class="w-full">
    <label for={id} class="mb-1.5 block text-sm font-medium text-black">
        {label}
    </label>

    <div class="relative">
        {#if loading}
            <div class="absolute inset-y-0 left-3 flex items-center">
                <div class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
            </div>
        {/if}

        <select
                {id}
                bind:value
                onblur={() => (touched = true)}
                disabled={loading || wallets.length === 0}
                class="w-full appearance-none rounded-md border px-3 py-2 text-sm text-black transition focus:outline-none focus:ring-0 disabled:bg-gray-50 disabled:text-gray-400
             {loading ? 'pl-9' : 'pl-3'}
             {showFeedback
                ? isValid
                   ? 'border-green-400 focus:border-green-500'
                   : 'border-red-400 focus:border-red-500'
                : 'border-gray-200 focus:border-gray-400'}"
        >
            <option value="" disabled selected>
                {loading ? 'Cargando...' : wallets.length === 0 ? 'Sin billeteras disponibles' : 'Elegí una billetera'}
            </option>

            {#each wallets as wallet (wallet.id)}
                <option value={returnType === 'wallet_id' ? wallet.id : wallet.currency_id}>
                    {wallet.currency_ticker ?? 'USDC'} — Saldo: ${wallet.balance}
                </option>
            {/each}
        </select>

        <div class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">
            <ChevronDown class="h-4 w-4" />
        </div>
    </div>

    {#if showFeedback && !isValid && wallets.length > 0}
        <p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
            <X class="h-3.5 w-3.5 shrink-0" />
            Seleccioná una billetera para continuar
        </p>
    {:else if wallets.length === 0 && !loading && !error}
        <p class="mt-1.5 text-xs text-gray-500">
            El grupo no tiene billeteras creadas.
        </p>
    {:else if error}
        <p class="mt-1.5 text-xs text-red-500">{error}</p>
    {:else}
        <div class="mt-1.5 h-[18px]"></div>
    {/if}
</div>