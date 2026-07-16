<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { UserPlus, UserCheck, Search, Mail, Users } from 'lucide-svelte';

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

	const q = $derived(query.trim().toLowerCase());
	const looksLikeEmail = $derived(query.trim().includes('@') && query.trim().length >= 4);

	const filteredFriends = $derived.by(() => {
		if (!q) return friends;
		return friends.filter(
			(f) => f.name.toLowerCase().includes(q) || f.email.toLowerCase().includes(q)
		);
	});

	const formValid = $derived(
		selectedFriend !== null ||
			(looksLikeEmail && query.trim().length >= 4 && query.trim().length <= 50)
	);

	// Load friends whenever the modal opens; cancel if it closes mid-fetch
	$effect(() => {
		if (!open) return;

		let cancelled = false;
		friendsLoading = true;

		void getFriends().then((res) => {
			if (cancelled) return;
			if (isSuccess(res)) friends = res.body;
			friendsLoading = false;
		});

		return () => {
			cancelled = true;
		};
	});

	function handleInput(value: string) {
		query = value;
		selectedFriend = null;
	}

	function selectFriend(friend: FriendResponse) {
		selectedFriend = friend;
		query = '';
	}

	function clearSelected() {
		selectedFriend = null;
		query = '';
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
		friends = [];
		friendsLoading = false;
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Invitar nuevo miembro"
	description="Elegí un amigo de la lista o buscá por nombre o email."
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
			<!-- Search -->
			<div>
				<label for="invite-query" class="mb-1.5 block text-sm font-medium text-foreground">
					Buscar
				</label>
				<div class="relative">
					<Search class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
					<input
						id="invite-query"
						type="text"
						placeholder="Nombre de un amigo o email..."
						value={query}
						oninput={(e) => handleInput((e.target as HTMLInputElement).value)}
						autocomplete="off"
						class="w-full rounded-md border border-input bg-background py-2 pr-3 pl-10 text-sm text-foreground transition placeholder:text-muted-foreground focus:border-ring focus:ring-0 focus:outline-none"
					/>
				</div>
			</div>

			<!-- Email invite hint -->
			{#if looksLikeEmail}
				<div
					class="flex items-center gap-3 rounded-lg border border-dashed border-primary/40 bg-primary/5 px-3 py-3"
				>
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
					>
						<Mail class="size-5" />
					</div>
					<div class="min-w-0 flex-1">
						<p class="text-sm font-semibold text-foreground">Invitar por email</p>
						<p class="truncate text-xs text-muted-foreground">
							Se enviará la invitación a {query.trim()}
						</p>
					</div>
				</div>
			{/if}

			<!-- Friends list always visible -->
			<div>
				<div class="mb-2 flex items-center gap-2">
					<Users class="size-3.5 text-muted-foreground" />
					<p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						Tus amigos
						{#if !friendsLoading}
							({filteredFriends.length}{q && friends.length !== filteredFriends.length
								? ` de ${friends.length}`
								: ''})
						{/if}
					</p>
				</div>

				<div
					class="max-h-56 divide-y divide-border overflow-y-auto rounded-lg border border-border bg-muted/20"
				>
					{#if friendsLoading}
						<div class="flex items-center justify-center gap-2 py-8 text-sm text-muted-foreground">
							<svg class="size-4 animate-spin" viewBox="0 0 24 24" fill="none" aria-hidden="true">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								/>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
								/>
							</svg>
							Cargando amigos...
						</div>
					{:else if friends.length === 0}
						<div class="px-4 py-8 text-center text-sm text-muted-foreground">
							Todavía no tenés amigos. Podés invitar escribiendo un email arriba.
						</div>
					{:else if filteredFriends.length === 0}
						<div class="px-4 py-8 text-center text-sm text-muted-foreground">
							{#if looksLikeEmail}
								Ningún amigo con ese email. Podés enviar la invitación igual.
							{:else}
								No hay amigos que coincidan con “{query.trim()}”.
							{/if}
						</div>
					{:else}
						{#each filteredFriends as friend (friend.user_id)}
							<button
								type="button"
								onclick={() => selectFriend(friend)}
								class="flex w-full items-center gap-3 px-3 py-2.5 text-left transition hover:bg-muted/60 focus:bg-muted/60 focus:outline-none"
							>
								<div
									class="flex size-9 shrink-0 items-center justify-center rounded-full bg-linear-to-br from-violet-200 to-lime-200 text-xs font-bold text-violet-800 dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
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
									<UserCheck class="size-3" />
								</span>
							</button>
						{/each}
					{/if}
				</div>
			</div>

			{#if form.attempted && !formValid}
				<p class="text-xs text-red-500">Elegí un amigo de la lista o ingresá un email válido.</p>
			{/if}
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
