<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { X } from 'lucide-svelte';

	import { createExpense } from '$lib/api/endpoints/expenses';
	import { getGroupWallets } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';
	import type { GroupWallet } from '$lib/types/endpoints/groups.types';

	interface Props {
		open: boolean;
		group_id: string;
		members: UserBadge[];
		onclose: () => void;
		onsuccess: () => void;
	}

	const { open, group_id, members, onclose, onsuccess }: Props = $props();

	let groupWallets = $state<GroupWallet[]>([]);
	let loadingWallets = $state(false);
	let selectedCurrencyId = $state('');
	let amount = $state('');
	let description = $state('');
	let selectedParticipants = $state<string[]>([]);
	let attempted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const hasMembers = $derived(members.length > 0);
	const currencySelected = $derived(selectedCurrencyId !== '');
	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const formValid = $derived(
		currencySelected && amountValid && selectedParticipants.length > 0 && hasMembers
	);

	$effect(() => {
		if (open) {
			loadGroupWallets();
		}
	});

	$effect(() => {
		if (!open) return;
		if (selectedParticipants.length === 0 && members.length > 0) {
			selectedParticipants = members.map((member) => member.user_id);
		}
	});

	async function loadGroupWallets() {
		loadingWallets = true;
		const res = await getGroupWallets(group_id);
		loadingWallets = false;
		if (!isSuccess(res)) {
			error = 'No se pudieron cargar las billeteras del grupo.';
			return;
		}
		groupWallets = res.body;
	}

	function toggleParticipant(userId: string) {
		if (selectedParticipants.includes(userId)) {
			selectedParticipants = selectedParticipants.filter((id) => id !== userId);
			return;
		}
		selectedParticipants = [...selectedParticipants, userId];
	}

	function handleClose() {
		const shouldReload = success !== '';
		selectedCurrencyId = '';
		amount = '';
		description = '';
		selectedParticipants = [];
		attempted = false;
		loading = false;
		error = '';
		success = '';
		groupWallets = [];
		onclose();
		if (shouldReload) onsuccess();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		attempted = true;
		if (!formValid) return;

		error = '';
		success = '';
		loading = true;

		const result = await createExpense(group_id, {
			currency_id: selectedCurrencyId,
			amount: String(parsedAmount),
			description: description.trim() ? description.trim() : null,
			participants: selectedParticipants.map((user_id) => ({ user_id }))
		});

		loading = false;

		if (!isSuccess(result)) {
			error = result.message || 'No se pudo crear el gasto.';
			return;
		}

		success = 'Gasto creado correctamente';
		setTimeout(() => {
			handleClose();
		}, 1200);
	}
</script>

<Modal
	{open}
	title="Nuevo Gasto"
	description="Crea un gasto y define los participantes."
	onclose={handleClose}
	{error}
	{success}
	{loading}
>
	{#snippet children()}
		<form id="create-expense-form" onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label class="mb-1.5 block text-sm font-medium text-black" for="expense-amount">Monto</label
				>
				<input
					id="expense-amount"
					type="number"
					step="0.01"
					min="0"
					placeholder="Ej. 150.00"
					bind:value={amount}
					class="w-full rounded-md border px-3 py-2 text-sm text-black placeholder-gray-400 transition focus:ring-0 focus:outline-none {attempted &&
					!amountValid
						? 'border-red-400 focus:border-red-500'
						: 'border-gray-200 focus:border-gray-400'}"
				/>
			</div>

			<div>
				<label for="fund-round-currency" class="mb-1.5 block text-sm font-medium text-black">
					Moneda
				</label>

				{#if loadingWallets}
					<div class="flex items-center gap-2 py-2">
						<div
							class="h-4 w-4 animate-spin rounded-full border-2 border-gray-200 border-t-black"
						></div>
						<span class="text-sm text-gray-400">Cargando billeteras del grupo...</span>
					</div>
				{:else if groupWallets.length === 0}
					<p class="rounded-md border border-gray-200 bg-gray-50 p-3 text-sm text-gray-500">
						El grupo no tiene billeteras aún. Creá una antes de crear un gasto.
					</p>
				{:else}
					<select
						id="fund-round-currency"
						bind:value={selectedCurrencyId}
						class="w-full rounded-md border px-3 py-2 text-sm text-black transition focus:ring-0 focus:outline-none
							{attempted && !currencySelected
							? 'border-red-400 focus:border-red-500'
							: selectedCurrencyId
								? 'border-green-400 focus:border-green-500'
								: 'border-gray-200 focus:border-gray-400'}"
					>
						<option value="" disabled>Elegí una moneda</option>
						{#each groupWallets as wallet (wallet.id)}
							<option value={wallet.currency_id}>
								{wallet.currency_ticker ?? 'USDC'} (saldo: ${wallet.balance})
							</option>
						{/each}
					</select>

					{#if attempted && !currencySelected}
						<p class="mt-1.5 flex items-center gap-1 text-xs text-red-500">
							<X class="h-3.5 w-3.5 shrink-0" />
							Seleccioná una moneda
						</p>
					{/if}
				{/if}
			</div>

			<div>
				<label class="mb-1.5 block text-sm font-medium text-black" for="expense-description">
					Descripción (opcional)
				</label>
				<textarea
					id="expense-description"
					rows="3"
					placeholder="Ej. Supermercado"
					bind:value={description}
					maxlength="255"
					class="w-full resize-none rounded-md border border-gray-200 px-3 py-2 text-sm text-black placeholder-gray-400 transition focus:border-gray-400 focus:ring-0 focus:outline-none"
				></textarea>
			</div>

			<div>
				<p class="mb-1.5 text-sm font-medium text-black">Participantes</p>
				{#if hasMembers}
					<div class="max-h-36 space-y-2 overflow-y-auto rounded-md border border-gray-200 p-2">
						{#each members as member (member.user_id)}
							<label class="flex items-center gap-2 rounded px-2 py-1 text-sm hover:bg-gray-50">
								<input
									type="checkbox"
									checked={selectedParticipants.includes(member.user_id)}
									onchange={() => toggleParticipant(member.user_id)}
								/>
								<span>{member.name}</span>
							</label>
						{/each}
					</div>
				{:else}
					<p
						class="rounded-md border border-dashed border-gray-300 px-3 py-2 text-sm text-gray-500"
					>
						No hay miembros para agregar en el gasto.
					</p>
				{/if}
			</div>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />
		<Button
			label="Crear expense"
			type="submit"
			form="create-expense-form"
			disabled={!formValid || groupWallets.length === 0}
			{loading}
		/>
	{/snippet}
</Modal>
