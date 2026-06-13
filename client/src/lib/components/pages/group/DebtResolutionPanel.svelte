<script lang="ts">
	import { X, CircleCheckBig, CircleAlert } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import UserWalletSelectField from '$lib/components/input_fields/UserWalletSelectField.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';

	interface DebtInfo {
		creditorName: string;
		creditorId: string;
		amount: string;
	}

	interface CreditInfo {
		debtorName: string;
		debtorId: string;
		amount: string;
	}

	let {
		debts = [],
		credits = [],
		loading = false,
		error = '',
		currentUserBalance = 0,
		currencyId = '',
		paying = false,
		payError = '',
		onClose,
		onPaySettlement = async (_debtIndex: number, _address: string, _currencyId: string) => false
	} = $props<{
		debts: DebtInfo[];
		credits: CreditInfo[];
		loading?: boolean;
		error?: string;
		currentUserBalance?: number;
		currencyId?: string;
		paying?: boolean;
		payError?: string;
		onClose: () => void;
		onPaySettlement: (debtIndex: number, address: string, currencyId: string) => Promise<boolean>;
	}>();

	let isDebtor = $derived(currentUserBalance < -0.01);
	let isCreditor = $derived(currentUserBalance > 0.01);
	let isSettled = $derived(!isDebtor && !isCreditor);

	let selectedDebts = $state<Set<number>>(new Set());
	let initialized = $state(false);
	let senderAddress = $state('');

	$effect(() => {
		if (debts.length > 0 && !initialized) {
			selectedDebts = new Set(debts.map((_d: DebtInfo, i: number) => i));
			initialized = true;
		}
	});

	let allSelected = $derived(selectedDebts.size === debts.length);
	let selectedCount = $derived(selectedDebts.size);

	function toggleDebt(index: number) {
		const next = new Set(selectedDebts);
		if (next.has(index)) next.delete(index);
		else next.add(index);
		selectedDebts = next;
	}

	function displayName(name: string): string {
		return name.startsWith('0x') ? shortenAddress(name) : name;
	}

	async function handleSettle() {
		for (const i of selectedDebts) {
			const ok = await onPaySettlement(i, senderAddress, currencyId);
			if (!ok) break;
		}
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 backdrop-blur-sm">
	<div
		class="w-full max-w-md rounded-xl border border-border bg-card p-8 text-card-foreground shadow-xl shadow-black/10 dark:shadow-black/30"
	>
		<div class="mb-6 flex items-start justify-between gap-4">
			<div class="space-y-1">
				<h2 class="text-xl font-bold tracking-tight text-foreground">
					{#if loading}
						Liquidaciones
					{:else if isDebtor}
						Saldá tus deudas
					{:else if isCreditor}
						Deudas a tu favor
					{:else}
						Deudas saldadas
					{/if}
				</h2>
				<p class="text-sm text-muted-foreground">
					{#if loading}
						Cargando...
					{:else if isDebtor}
						Seleccioná cuáles querés saldar
					{:else if isCreditor}
						{credits.length === 1
							? 'Una persona te debe plata'
							: credits.length + ' personas te deben plata'}
					{:else}
						No tenés deudas activas en este grupo
					{/if}
				</p>
			</div>
			<button
				onclick={onClose}
				class="mt-0.5 rounded-md p-1 text-muted-foreground transition hover:bg-muted hover:text-foreground"
				aria-label="Cerrar"
			>
				<X class="h-5 w-5" />
			</button>
		</div>

		<div class="space-y-4">
			{#if loading}
				<div class="flex justify-center py-8">
					<div
						class="h-6 w-6 animate-spin rounded-full border-2 border-border border-t-foreground"
					></div>
				</div>
			{:else if error}
				<div
					class="rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
				>
					{error}
				</div>
			{:else if isDebtor}
				<div class="space-y-2">
					{#each debts as debt, i (debt.creditorId)}
						<label
							class="flex cursor-pointer items-center gap-3 rounded-lg border border-border bg-muted/30 px-4 py-3 transition hover:border-input hover:bg-accent"
						>
							<input
								type="checkbox"
								checked={selectedDebts.has(i)}
								onchange={() => toggleDebt(i)}
								disabled={paying}
								class="h-4 w-4 accent-foreground"
							/>
							<span class="flex-1 text-sm font-medium text-foreground">
								{displayName(debt.creditorName)}
							</span>
							<span class="text-sm font-semibold text-foreground tabular-nums">
								${Number(debt.amount).toFixed(2)}
							</span>
						</label>
					{/each}
				</div>

				<UserWalletSelectField
					id="settlement-sender-wallet"
					label="Wallet de origen"
					currency_id={currencyId}
					returnType="address"
					bind:value={senderAddress}
				/>
			{:else if isCreditor}
				<div class="space-y-2">
					{#each credits as credit (credit.debtorId)}
						<div
							class="flex items-center justify-between rounded-lg border border-border bg-muted/30 px-4 py-3"
						>
							<span class="text-sm font-medium text-foreground">
								{displayName(credit.debtorName)} te debe
							</span>
							<span class="text-sm font-semibold text-foreground tabular-nums">
								${Number(credit.amount).toFixed(2)}
							</span>
						</div>
					{/each}
				</div>

				<div
					class="flex items-start gap-2 rounded-lg border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-800 dark:border-amber-400/20 dark:bg-amber-400/10 dark:text-amber-300"
				>
					<CircleAlert class="mt-0.5 h-4 w-4 shrink-0" />
					<span
						>Tu plata está segura en el grupo. Cuando se paguen todas las deudas la vas a poder
						retirar.</span
					>
				</div>
			{:else}
				<div class="flex flex-col items-center gap-3 py-6 text-center">
					<CircleCheckBig class="h-10 w-10 text-emerald-600 dark:text-emerald-400" />
					<p class="text-sm font-medium text-foreground">Todo saldado</p>
					<p class="text-sm text-muted-foreground">No tenés deudas activas en este grupo.</p>
				</div>
			{/if}
		</div>

		<div class="mt-6 flex flex-col gap-3">
			{#if payError}
				<div
					class="rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
				>
					{payError}
				</div>
			{/if}
			<div class="flex items-center justify-between gap-2">
				<Button label="Ver grupo" variant="ghost" onclick={onClose} />

				{#if isDebtor && debts.length > 0}
					<Button
						label={paying ? 'Pagando...' : allSelected ? 'Saldar todo' : 'Saldar ' + selectedCount}
						onclick={handleSettle}
						disabled={selectedCount === 0 || paying || !senderAddress}
					/>
				{/if}
			</div>
		</div>
	</div>
</div>
