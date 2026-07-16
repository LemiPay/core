<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { UserPlus, UserCheck, Search, Users } from 'lucide-svelte';

	import type { UserSearchResult } from '$lib/types/endpoints/friends.types';
	import { searchUsers, sendFriendRequest } from '$lib/api/endpoints/friends';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsuccess?: () => void;
	}

	const { open, onclose, onsuccess }: Props = $props();

	const form = new ModalState();

	let email = $state('');
	let searchQuery = $state('');
	let selectedUser = $state<UserSearchResult | null>(null);
	let searchResults = $state<UserSearchResult[]>([]);
	let searchLoading = $state(false);
	let showResults = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	const formValid = $derived(
		selectedUser !== null
			? !selectedUser.is_friend
			: email.trim().length >= 4 && email.trim().length <= 30
	);

	function handleSearchInput(value: string) {
		searchQuery = value;
		selectedUser = null;
		if (value.trim().length < 2) {
			searchResults = [];
			showResults = false;
			return;
		}
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(async () => {
			searchLoading = true;
			showResults = true;
			const res = await searchUsers(value.trim());
			if (isSuccess(res)) {
				searchResults = res.body;
			}
			searchLoading = false;
		}, 300);
	}

	function selectUser(user: UserSearchResult) {
		selectedUser = user;
		email = user.email;
		searchQuery = '';
		searchResults = [];
		showResults = false;
	}

	function clearSelected() {
		selectedUser = null;
		email = '';
	}

	async function resolveUserId(): Promise<string | null> {
		if (selectedUser) {
			if (selectedUser.is_friend) {
				form.error = 'Ya son amigos.';
				return null;
			}
			return selectedUser.user_id;
		}

		const query = email.trim();
		const res = await searchUsers(query);
		if (!isSuccess(res)) {
			form.error = res.message || 'No se pudo buscar el usuario.';
			return null;
		}

		const match = res.body.find((u) => u.email.toLowerCase() === query.toLowerCase());
		if (!match) {
			form.error = 'No se encontró un usuario con ese email.';
			return null;
		}
		if (match.is_friend) {
			form.error = 'Ya son amigos.';
			return null;
		}

		return match.user_id;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		form.error = '';
		form.success = '';
		form.loading = true;

		const userId = await resolveUserId();
		if (!userId) {
			form.loading = false;
			return;
		}

		form.loading = false;
		await form.submit(() => sendFriendRequest(userId), {
			successMsg: 'Solicitud enviada',
			onSuccess: () => {
				onsuccess?.();
				handleClose();
			}
		});
	}

	function handleClose() {
		email = '';
		searchQuery = '';
		selectedUser = null;
		searchResults = [];
		showResults = false;
		clearTimeout(debounceTimer);
		form.reset();
		onclose();
	}
</script>

<Modal
	{open}
	title="Agregar amigo"
	description="Buscá por nombre o email, o ingresá el email del usuario."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
	panelClass="max-w-lg"
>
	<form id="add-friend-form" onsubmit={handleSubmit} class="space-y-4">
		{#if selectedUser}
			<div class="flex items-center gap-3 rounded-lg border border-primary/30 bg-primary/5 p-3">
				<div
					class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
				>
					<UserPlus class="size-5" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="text-sm font-semibold text-foreground">{selectedUser.name}</p>
					<p class="truncate text-xs text-muted-foreground">{selectedUser.email}</p>
				</div>
				{#if selectedUser.is_friend}
					<span
						class="flex items-center gap-1 rounded-full bg-emerald-50 px-2 py-1 text-xs text-emerald-600 dark:bg-emerald-900/20 dark:text-emerald-400"
					>
						<UserCheck class="size-3" /> Amigo
					</span>
				{/if}
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
			{#if selectedUser.is_friend}
				<p class="text-xs text-muted-foreground">Ya tenés a este usuario como amigo.</p>
			{/if}
		{:else}
			<div class="relative">
				<div class="relative">
					<Search class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
					<input
						id="friend-search"
						type="text"
						placeholder="Buscar por nombre o email..."
						value={searchQuery}
						oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
						onfocus={() => {
							if (searchResults.length > 0) showResults = true;
						}}
						onblur={() =>
							setTimeout(() => {
								showResults = false;
							}, 200)}
						class="w-full rounded-md border border-input bg-background py-2 pr-3 pl-10 text-sm text-foreground transition placeholder:text-muted-foreground focus:border-ring focus:ring-0 focus:outline-none"
					/>
				</div>

				{#if showResults && searchQuery.trim().length >= 2}
					<div
						class="absolute z-20 mt-1 max-h-60 w-full overflow-y-auto rounded-lg border border-border bg-card shadow-lg"
					>
						{#if searchLoading}
							<div class="flex items-center justify-center py-4">
								<svg
									class="size-5 animate-spin text-muted-foreground"
									viewBox="0 0 24 24"
									fill="none"
								>
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
							</div>
						{:else if searchResults.length === 0}
							<div class="px-4 py-3 text-center text-sm text-muted-foreground">
								No se encontraron usuarios
							</div>
						{:else}
							{#each searchResults as result (result.user_id)}
								<button
									type="button"
									onclick={() => selectUser(result)}
									class="flex w-full items-center gap-3 px-4 py-3 text-left transition hover:bg-muted/50 focus:bg-muted/50 focus:outline-none"
								>
									<div
										class="flex size-9 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-bold text-muted-foreground"
									>
										{result.name.slice(0, 2).toUpperCase()}
									</div>
									<div class="min-w-0 flex-1">
										<p class="truncate text-sm font-medium text-foreground">{result.name}</p>
										<p class="truncate text-xs text-muted-foreground">{result.email}</p>
									</div>
									{#if result.is_friend}
										<span
											class="flex shrink-0 items-center gap-1 text-xs text-emerald-600 dark:text-emerald-400"
										>
											<Users class="size-3" /> Amigo
										</span>
									{/if}
								</button>
							{/each}
						{/if}
					</div>
				{/if}
			</div>

			<div class="flex items-center gap-2">
				<div class="flex-1 border-t border-border"></div>
				<span class="text-xs text-muted-foreground">o ingresá un email</span>
				<div class="flex-1 border-t border-border"></div>
			</div>

			<FormField
				id="friend-email"
				label="Email"
				type="email"
				placeholder="e.g. joe@doe.com"
				minLength={4}
				maxLength={30}
				bind:value={email}
				attempted={form.attempted}
			/>
		{/if}
	</form>

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label={selectedUser ? `Agregar a ${selectedUser.name}` : 'Enviar solicitud'}
			type="submit"
			form="add-friend-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
