<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import GroupWalletSelectField from '$lib/components/input_fields/GroupWalletSelectField.svelte';

	import { createFundRoundProposal } from '$lib/api/endpoints/fund_rounds';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let selectedCurrencyId = $state('');
	let targetAmount = $state('');

	const parsedAmount = $derived(Number(String(targetAmount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const currencySelected = $derived(selectedCurrencyId !== '');

	const formValid = $derived(currencySelected && amountValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(
			() =>
				createFundRoundProposal({
					group_id,
					currency_id: selectedCurrencyId,
					target_amount: String(parsedAmount)
				}),
			{
				successMsg: '¡Ronda de fondeo creada exitosamente!',
				onSuccess: () => {
					onsuccess?.();
					handleClose();
				}
			}
		);
	}

	function handleClose() {
		selectedCurrencyId = '';
		targetAmount = '';
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Nueva ronda de fondeo"
	description="Proponé un objetivo de fondeo asociado a una billetera del grupo."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="create-fund-round-form" onsubmit={handleSubmit} class="space-y-4">
			<GroupWalletSelectField
				{group_id}
				label="Moneda"
				returnType="currency_id"
				bind:value={selectedCurrencyId}
				attempted={form.attempted}
			/>

			<NumberField
				id="fund-round-target"
				label="Objetivo"
				min={0.01}
				placeholder="Ej. 1000.00"
				bind:value={targetAmount}
				attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Crear ronda"
			type="submit"
			form="create-fund-round-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
