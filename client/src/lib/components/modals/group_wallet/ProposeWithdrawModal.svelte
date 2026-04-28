<script lang="ts">
	import Modal from '../Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import UserWalletSelectField from '$lib/components/input_fields/UserWalletSelectField.svelte';

	import { proposeWithdraw } from '$lib/api/endpoints/transactions';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		group_id: string;
		currency_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, currency_id, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let selectedAddress = $state(''); // Acá se va a guardar la address directo
	let amount = $state('');

	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const formValid = $derived(
		selectedAddress !== '' && Number.isFinite(parsedAmount) && parsedAmount > 0
	);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		await form.submit(
			() =>
				proposeWithdraw(
					{
						currency_id,
						user_address: selectedAddress,
						amount: String(parsedAmount)
					},
					group_id
				),
			{
				successMsg: 'Propuesta de retiro creada exitosamente!',
				onSuccess: () => {
					onsuccess?.();
					handleClose();
				}
			}
		);
	}

	function handleClose() {
		selectedAddress = '';
		amount = '';
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Proponer Retiro"
	description="Creá una propuesta para retirar fondos de la wallet del grupo hacia tu cuenta personal."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="propose-withdraw-form" onsubmit={handleSubmit} class="space-y-4">
			<NumberField
				id="withdraw-amount"
				label="Monto a retirar"
				min={0.01}
				placeholder="0.00"
				bind:value={amount}
				attempted={form.attempted}
			/>

			<UserWalletSelectField
				label="Wallet de destino (Tu cuenta)"
				{currency_id}
				returnType="address"
				bind:value={selectedAddress}
				attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Proponer"
			type="submit"
			form="propose-withdraw-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
