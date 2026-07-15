<script lang="ts">
	import Modal from '$lib/components/modals/Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { UserPlus, Check, Users, UserCheck, Search } from 'lucide-svelte';

	import type { NewMemberData } from '$lib/types/endpoints/proposals.types';
	import type { UserSearchResult } from '$lib/types/endpoints/friends.types';
	import { createNewMemberProposal } from '$lib/api/endpoints/proposals';
	import { searchUsers } from '$lib/api/endpoints/friends';
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

	let email = $state('');
	let searchQuery = $state('');
	let selectedUser = $state<UserSearchResult | null>(null);
	let searchResults = $state<UserSearchResult[]>([]);
	let searchLoading = $state(false);
	let showResults = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	const formValid = $derived(
		selectedUser !== null || (email.trim().length >= 4 && email.trim().length <= 30)
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

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!formValid) return;

		const params: NewMemberData = {
			group_id: group_id,
			email: selectedUser?.email ?? email.trim()
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
	title="Invitar nuevo miembro"
	description="Buscá por nombre o email, o ingresá un email manualmente."
	onclose={handleClose}
	error={form.error}
	success={form.success}
	loading={form.loading}
	panelClass="max-w-lg"
>
	{#snippet children()}
		<form id="add-member-form" onsubmit={handleSubmit} class="space-y-4">
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
					<div class="relative">
						<Search class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
						<input
							id="user-search"
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
								{#each searchResults as result}
									<button
										type="button"
										onclick={() => selectUser(result)}
										class="flex w-full items-center gap-3 px-4 py-3 text-left transition hover:bg-muted/50 focus:bg-muted/50 focus:outline-none"
										class:border-t={result.is_friend && result !== searchResults[0]}
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
					id="member-email"
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
	{/snippet}

	{#snippet footer()}
		<Button label="Cancelar" variant="secondary" onclick={handleClose} />

		<Button
			label={selectedUser ? `Invitar a ${selectedUser.name}` : 'Enviar Invitación'}
			type="submit"
			form="add-member-form"
			disabled={!formValid}
			loading={form.loading}
		/>
	{/snippet}
</Modal>
