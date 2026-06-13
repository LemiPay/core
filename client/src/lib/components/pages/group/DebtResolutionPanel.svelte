<script lang="ts">
	import { X, CircleCheckBig, CircleAlert, TriangleAlert } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import UserWalletSelectField from '$lib/components/input_fields/UserWalletSelectField.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';

	interface CreditInfo {
		debtorName: string;
		debtorId: string;
		amount: string;
	}

	let {
		debtAmount = '0',
		credits = [],
		loading = false,
		error = '',
		currentUserBalance = 0,
		claimableAmount = '0',
		hasDebtors = false,
		currencyId = '',
		paying = false,
		payError = '',
		claiming = false,
		claimError = '',
		onClose,
		onPaySettlement = async (_amount: string, _address: string, _currencyId: string) => false,
		onClaim = async (_address: string, _currencyId: string, _amount: string) => false
	} = $props<{
		debtAmount?: string;
		credits: CreditInfo[];
		loading?: boolean;
		error?: string;
		currentUserBalance?: number;
		claimableAmount?: string;
		hasDebtors?: boolean;
		currencyId?: string;
		paying?: boolean;
		payError?: string;
		claiming?: boolean;
		claimError?: string;
		onClose: () => void;
		onPaySettlement: (amount: string, address: string, currencyId: string) => Promise<boolean>;
		onClaim: (address: string, currencyId: string, amount: string) => Promise<boolean>;
	}>();

	let isDebtor = $derived(currentUserBalance < -0.01);
	let isCreditor = $derived(currentUserBalance > 0.01);
	let isSettled = $derived(!isDebtor && !isCreditor);

	let totalDebt = $derived(Math.abs(currentUserBalance));

	let senderAddress = $state('');
	let claimAddress = $state('');
	let payAmount = $state('');

	let payAmountValid = $derived.by(() => {
		if (!payAmount) return false;
		const n = Number(payAmount);
		return Number.isFinite(n) && n > 0 && n <= totalDebt;
	});

	function displayName(name: string): string {
		return name.startsWith('0x') ? shortenAddress(name) : name;
	}

	async function handleSettleAll() {
		await onPaySettlement(debtAmount, senderAddress, currencyId);
	}

	async function handleSettlePartial() {
		await onPaySettlement(payAmount, senderAddress, currencyId);
	}

	async function handleClaim() {
		await onClaim(claimAddress, currencyId, claimableAmount);
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
						Elegí cuánto querés pagar
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
				<div
					class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
				>
					<TriangleAlert class="mt-0.5 h-4 w-4 shrink-0" />
					<div>
						<span class="font-medium">Debés ${totalDebt.toFixed(2)} al grupo</span>
					</div>
				</div>

				<div>
					<label for="pay-amount" class="mb-1.5 block text-sm font-medium text-foreground"
						>Monto a pagar</label
					>
					<input
						id="pay-amount"
						type="text"
						inputmode="decimal"
						placeholder={totalDebt.toFixed(2)}
						bind:value={payAmount}
						disabled={paying}
						class="w-full rounded-lg border border-border bg-background px-4 py-2.5 text-sm text-foreground placeholder-muted-foreground transition focus:border-foreground focus:outline-none disabled:opacity-50"
					/>
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

				{#if hasDebtors}
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
					<div
						class="flex items-start gap-2 rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-800 dark:border-emerald-400/20 dark:bg-emerald-400/10 dark:text-emerald-300"
					>
						<CircleCheckBig class="mt-0.5 h-4 w-4 shrink-0" />
						<span class="font-medium">Podés retirar ${currentUserBalance.toFixed(2)}</span>
					</div>

					<UserWalletSelectField
						id="settlement-claim-wallet"
						label="Wallet de destino"
						currency_id={currencyId}
						returnType="address"
						bind:value={claimAddress}
					/>
				{/if}
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
			{#if claimError}
				<div
					class="rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
				>
					{claimError}
				</div>
			{/if}
			<div class="flex items-center justify-between gap-2">
				<Button label="Ver grupo" variant="ghost" onclick={onClose} />

				{#if isDebtor}
					<div class="flex items-center gap-2">
						<Button
							label={paying ? 'Pagando...' : 'Pagar'}
							onclick={handleSettlePartial}
							disabled={!payAmountValid || paying || !senderAddress}
						/>
						<Button
							label={paying ? 'Pagando...' : 'Pagar todo'}
							onclick={handleSettleAll}
							disabled={paying || !senderAddress}
						/>
					</div>
				{/if}

				{#if isCreditor && !hasDebtors}
					<Button
						label={claiming ? 'Retirando...' : 'Retirar todo'}
						onclick={handleClaim}
						disabled={claiming || !claimAddress}
					/>
				{/if}
			</div>
		</div>
	</div>
</div>
