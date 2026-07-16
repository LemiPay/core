<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { UserPlus, UserCheck, Search } from 'lucide-svelte';

	import type { NewMemberData } from '$lib/types/endpoints/proposals.types';
	import type { FriendResponse } from '$lib/types/endpoints/friends.types';
	import { createNewMemberProposal } from '$lib/api/endpoints/proposals';
	import { getFriends } from '$lib/api/endpoints/friends';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		group_id: string;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, group_id, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let query = $state('');
	let selectedFriend = $state<FriendResponse | null>(null);
	let friends = $state<FriendResponse[]>([]);
	let friendsLoading = $state(false);
	let friendsLoaded = $state(false);
	let showResults = $state(false);

	const suggestions = $derived.by(() => {
		const q = query.trim().toLowerCase();
		if (q.length < 1 || selectedFriend) return [];
		return friends
			.filter((f) => f.name.toLowerCase().includes(q) || f.email.toLowerCase().includes(q))
			.slice(0, 8);
	});

	const looksLikeEmail = $derived(query.trim().includes('@'));

	const formValid = $derived(
		selectedFriend !== null ||
			(looksLikeEmail && query.trim().length >= 4 && query.trim().length <= 50)
	);

	async function ensureFriends() {
		if (friendsLoading || friendsLoaded) return;
		friendsLoading = true;
		const res = await getFriends();
		if (isSuccess(res)) friends = res.body;
		friendsLoaded = true;
		friendsLoading = false;
	}

	function handleInput(value: string) {
		query = value;
		selectedFriend = null;
		showResults = value.trim().length >= 1;
		void ensureFriends();
	}

	function selectFriend(friend: FriendResponse) {
		selectedFriend = friend;
		query = friend.name;
		showResults = false;
	}

	function clearSelected() {
		selectedFriend = null;
		query = '';
		showResults = false;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		const params: NewMemberData = {
			group_id: group_id,
			email: selectedFriend?.email ?? query.trim()
		};

		await form.submit(() => createNewMemberProposal(params), {
			successMsg: 'Invitación enviada correctamente',
			onSuccess: () => {
				onsuccess?.();
				handleClose();
			}
		});
	}

	function handleClose() {
		query = '';
		selectedFriend = null;
		showResults = false;
		// Allow a fresh friends list next open
		friends = [];
		friendsLoaded = false;
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Invitar nuevo miembro"
	description="Elegí un amigo o escribí un email para enviar la invitación."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
	panelClass="max-w-lg"
>
	<form id="add-member-form" onsubmit={handleSubmit} class="space-y-4">
		{#if selectedFriend}
			<div class="flex items-center gap-3 rounded-lg border border-primary/30 bg-primary/5 p-3">
				<div
					class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
				>
					<UserPlus class="size-5" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="text-sm font-semibold text-foreground">{selectedFriend.name}</p>
					<p class="truncate text-xs text-muted-foreground">{selectedFriend.email}</p>
				</div>
				<span
					class="flex items-center gap-1 rounded-full bg-emerald-50 px-2 py-1 text-xs text-emerald-600 dark:bg-emerald-900/20 dark:text-emerald-400"
				>
					<UserCheck class="size-3" /> Amigo
				</span>
				<button
					type="button"
					onclick={clearSelected}
					aria-label="Quitar selección"
					class="flex size-6 items-center justify-center rounded-full text-muted-foreground transition hover:bg-muted hover:text-foreground"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="size-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>
		{:else}
			<div class="relative">
				<label for="invite-query" class="mb-1.5 block text-sm font-medium text-foreground">
					Amigo o email
				</label>
				<div class="relative">
					<Search class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
					<input
						id="invite-query"
						type="text"
						placeholder="Buscá un amigo o escribí un email..."
						value={query}
						oninput={(e) => handleInput((e.target as HTMLInputElement).value)}
						onfocus={() => {
							void ensureFriends();
							if (query.trim().length >= 1) showResults = true;
						}}
						onblur={() =>
							setTimeout(() => {
								showResults = false;
							}, 200)}
						autocomplete="off"
						class="w-full rounded-md border border-input bg-background py-2 pr-3 pl-10 text-sm text-foreground transition placeholder:text-muted-foreground focus:border-ring focus:ring-0 focus:outline-none"
					/>
				</div>

				{#if showResults && suggestions.length > 0}
					<div
						class="absolute z-20 mt-1 max-h-60 w-full overflow-y-auto rounded-lg border border-border bg-card shadow-lg"
					>
						{#each suggestions as friend (friend.user_id)}
							<button
								type="button"
								onclick={() => selectFriend(friend)}
								class="flex w-full items-center gap-3 px-4 py-3 text-left transition hover:bg-muted/50 focus:bg-muted/50 focus:outline-none"
							>
								<div
									class="flex size-9 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-bold text-muted-foreground"
								>
									{friend.name.slice(0, 2).toUpperCase()}
								</div>
								<div class="min-w-0 flex-1">
									<p class="truncate text-sm font-medium text-foreground">{friend.name}</p>
									<p class="truncate text-xs text-muted-foreground">{friend.email}</p>
								</div>
								<span
									class="flex shrink-0 items-center gap-1 text-xs text-emerald-600 dark:text-emerald-400"
								>
									<UserCheck class="size-3" /> Amigo
								</span>
							</button>
						{/each}
					</div>
				{:else if showResults && query.trim().length >= 1 && suggestions.length === 0 && !looksLikeEmail}
					<div
						class="absolute z-20 mt-1 w-full rounded-lg border border-border bg-card px-4 py-3 text-center text-sm text-muted-foreground shadow-lg"
					>
						No hay amigos que coincidan. Podés escribir un email.
					</div>
				{/if}

				{#if form.attempted && !formValid}
					<p class="mt-1.5 text-xs text-red-500">Elegí un amigo o ingresá un email válido.</p>
				{/if}
			</div>
		{/if}
	</form>

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label={selectedFriend ? `Invitar a ${selectedFriend.name}` : 'Enviar invitación'}
			type="submit"
			form="add-member-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
