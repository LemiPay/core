<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import NumberField from '$lib/components/input_fields/NumberField.svelte';
	import UserWalletSelectField from '$lib/components/input_fields/UserWalletSelectField.svelte';

	import { fundGroupWallet } from '$lib/api/endpoints/groups';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';

	interface Props {
		open: boolean;
		group_id: string;
		wallet_id: string; // Viene del padre aunque no se usa directo en el endpoint
		currency_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess, wallet_id, currency_id }: Props = $props();

	const form = new ModalState();

	let amount = $state('');
	let senderAddress = $state(''); // Acá guardamos directamente la address

	const parsedAmount = $derived(Number(String(amount).replace(',', '.')));
	const amountValid = $derived(Number.isFinite(parsedAmount) && parsedAmount > 0);
	const formValid = $derived(senderAddress !== '' && amountValid);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();

		if (!formValid) return;

		await form.submit(
			() =>
				fundGroupWallet(group_id, {
					currency_id,
					address: senderAddress,
					amount: String(parsedAmount)
				}),
			{
				successMsg: '¡Fondeo realizado exitosamente!',
				onSuccess: () => {
					onsuccess?.();
					handleClose();
				}
			}
		);
	}

	function handleClose() {
		amount = '';
		senderAddress = '';
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Fondear wallet del grupo"
	description="Transferí fondos desde tu wallet personal a la wallet del grupo."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
>
	{#snippet children()}
		<form id="fund-group-wallet-form" onsubmit={handleSubmit} class="space-y-4">
			<UserWalletSelectField
				id="sender-wallet"
				label="Wallet de origen"
				{currency_id}
				returnType="address"
				bind:value={senderAddress}
				attempted={form.attempted}
			/>

			<NumberField
				id="fund-amount"
				label="Monto"
				min={0.01}
				placeholder="0.00"
				bind:value={amount}
				attempted={form.attempted}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label="Fondear"
			type="submit"
			form="fund-group-wallet-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
