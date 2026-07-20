<script lang="ts">
	import Modal from '../Modal.svelte';
	import FormField from '$lib/components/input_fields/FormField.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { Stepper, type StepItem } from '$lib/components/ui/stepper';
	import { Search, UserCheck, Mail, Users, X } from 'lucide-svelte';

	import type { NewGroupData } from '$lib/types/endpoints/groups.types';
	import type { FriendResponse } from '$lib/types/endpoints/friends.types';
	import { createGroup } from '$lib/api/endpoints/groups';
	import { createNewMemberProposal } from '$lib/api/endpoints/proposals';
	import { getFriends } from '$lib/api/endpoints/friends';
	import { ModalState } from '$lib/utils/modal_state.svelte.js';
	import { isSuccess } from '$lib/types/client.types';

	const wizardSteps: StepItem[] = [
		{ title: 'Grupo', description: 'Nombre y descripción' },
		{ title: 'Invitados', description: 'Amigos o emails' }
	];

	interface Props {
		open: boolean;
		onclose: () => void;
	}

	type InviteTarget =
		| { kind: 'friend'; user_id: string; name: string; email: string }
		| { kind: 'email'; email: string };

	const { open, onclose }: Props = $props();

	const form = new ModalState();

	let step = $state<1 | 2>(1);
	let name = $state('');
	let description = $state('');
	let createdGroupId = $state<string | null>(null);

	let query = $state('');
	let friends = $state<FriendResponse[]>([]);
	let friendsLoading = $state(false);
	let selected = $state<InviteTarget[]>([]);
	let inviteError = $state('');

	const nameValid = $derived(name.trim().length >= 4 && name.trim().length <= 30);
	const descValid = $derived(description.trim().length >= 8 && description.trim().length <= 30);
	const step1Valid = $derived(nameValid && descValid);

	const q = $derived(query.trim().toLowerCase());
	const looksLikeEmail = $derived(query.trim().includes('@') && query.trim().length >= 4);

	const filteredFriends = $derived.by(() => {
		if (!q) return friends;
		return friends.filter(
			(f) => f.name.toLowerCase().includes(q) || f.email.toLowerCase().includes(q)
		);
	});

	const selectedEmails = $derived(new Set(selected.map((s) => s.email.toLowerCase())));

	const selectedFriendIds = $derived(
		new Set(
			selected
				.filter((s): s is Extract<InviteTarget, { kind: 'friend' }> => s.kind === 'friend')
				.map((s) => s.user_id)
		)
	);

	// Load friends when entering step 2
	$effect(() => {
		if (!open || step !== 2) return;

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

	function isFriendSelected(userId: string) {
		return selectedFriendIds.has(userId);
	}

	function toggleFriend(friend: FriendResponse) {
		if (isFriendSelected(friend.user_id)) {
			selected = selected.filter((s) => !(s.kind === 'friend' && s.user_id === friend.user_id));
			return;
		}
		// Avoid duplicate by email
		selected = [
			...selected.filter((s) => s.email.toLowerCase() !== friend.email.toLowerCase()),
			{
				kind: 'friend',
				user_id: friend.user_id,
				name: friend.name,
				email: friend.email
			}
		];
	}

	function addEmailFromQuery() {
		const email = query.trim();
		if (!looksLikeEmail) return;
		if (selectedEmails.has(email.toLowerCase())) {
			query = '';
			return;
		}
		selected = [...selected, { kind: 'email', email }];
		query = '';
	}

	function removeSelected(index: number) {
		selected = selected.filter((_, i) => i !== index);
	}

	async function handleStep1(e: SubmitEvent) {
		e.preventDefault();
		form.setAttempted();
		if (!step1Valid) return;

		form.error = '';
		form.success = '';
		form.loading = true;

		const params: NewGroupData = {
			name: name.trim(),
			description: description.trim()
		};

		const result = await createGroup(params);
		form.loading = false;

		if (!isSuccess(result)) {
			form.error = result.message || 'No se pudo crear el grupo.';
			return;
		}

		createdGroupId = result.body.id;
		step = 2;
	}

	async function sendInvitesAndGo() {
		if (!createdGroupId) return;

		form.loading = true;
		inviteError = '';

		const results = await Promise.all(
			selected.map((target) =>
				createNewMemberProposal({
					group_id: createdGroupId!,
					email: target.email
				})
			)
		);

		const failures = results.filter((r) => !isSuccess(r));
		if (failures.length > 0 && failures.length === results.length) {
			form.loading = false;
			inviteError =
				failures[0] && 'message' in failures[0]
					? failures[0].message || 'No se pudieron enviar las invitaciones.'
					: 'No se pudieron enviar las invitaciones.';
			return;
		}

		// Even with partial failures, go to the group
		window.location.href = `/groups/${createdGroupId}`;
	}

	function skipInvitesAndGo() {
		if (!createdGroupId) return;
		window.location.href = `/groups/${createdGroupId}`;
	}

	function handleClose() {
		// If group was already created, still take the user there
		if (createdGroupId) {
			window.location.href = `/groups/${createdGroupId}`;
			return;
		}
		resetAll();
		onclose();
	}

	function resetAll() {
		step = 1;
		name = '';
		description = '';
		createdGroupId = null;
		query = '';
		friends = [];
		friendsLoading = false;
		selected = [];
		inviteError = '';
		form.reset();
	}
</script>

<Modal
	{open}
	title="Nuevo grupo"
	description={step === 1
		? 'Creá un grupo para empezar a dividir gastos con otros.'
		: 'Elegí amigos o agregá emails. Podés omitir este paso.'}
	onclose={handleClose}
	error={step === 1 ? form.error : inviteError || form.error}
	success={step === 1 ? form.success : ''}
	loading={form.loading}
	panelClass="max-w-lg"
>
	<div class="mb-6">
		<Stepper steps={wizardSteps} current={step} />
	</div>

	{#if step === 1}
		<form id="new-group-form" onsubmit={handleStep1} class="space-y-4">
			<FormField
				id="group-name"
				label="Nombre"
				type="text"
				placeholder="Ej. Viaje a Roma"
				minLength={4}
				maxLength={30}
				bind:value={name}
				attempted={form.attempted}
			/>
			<FormField
				id="group-description"
				label="Descripción"
				type="textarea"
				placeholder="¿Para qué es este grupo?"
				minLength={8}
				maxLength={30}
				rows={3}
				bind:value={description}
				attempted={form.attempted}
			/>
		</form>
	{:else}
		<div class="space-y-4">
			<!-- Selected chips -->
			{#if selected.length > 0}
				<div class="flex flex-wrap gap-2">
					{#each selected as target, i (target.kind === 'friend' ? target.user_id : target.email)}
						<span
							class="inline-flex items-center gap-1.5 rounded-full border border-primary/30 bg-primary/10 py-1 pr-1 pl-2.5 text-xs font-medium text-foreground"
						>
							{#if target.kind === 'friend'}
								{target.name}
							{:else}
								<Mail class="size-3" />
								{target.email}
							{/if}
							<button
								type="button"
								onclick={() => removeSelected(i)}
								aria-label="Quitar"
								class="flex size-5 items-center justify-center rounded-full text-muted-foreground transition hover:bg-muted hover:text-foreground"
							>
								<X class="size-3" />
							</button>
						</span>
					{/each}
				</div>
			{/if}

			<!-- Search -->
			<div>
				<label
					for="new-group-invite-query"
					class="mb-1.5 block text-sm font-medium text-foreground"
				>
					Buscar
				</label>
				<div class="relative">
					<Search class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
					<input
						id="new-group-invite-query"
						type="text"
						placeholder="Nombre de un amigo o email..."
						value={query}
						oninput={(e) => (query = (e.target as HTMLInputElement).value)}
						onkeydown={(e) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								if (looksLikeEmail) addEmailFromQuery();
							}
						}}
						autocomplete="off"
						class="w-full rounded-md border border-input bg-background py-2 pr-3 pl-10 text-sm text-foreground transition placeholder:text-muted-foreground focus:border-ring focus:ring-0 focus:outline-none"
					/>
				</div>
			</div>

			{#if looksLikeEmail && !selectedEmails.has(query.trim().toLowerCase())}
				<button
					type="button"
					onclick={addEmailFromQuery}
					class="flex w-full items-center gap-3 rounded-lg border border-dashed border-primary/40 bg-primary/5 px-3 py-3 text-left transition hover:bg-primary/10"
				>
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
					>
						<Mail class="size-5" />
					</div>
					<div class="min-w-0 flex-1">
						<p class="text-sm font-semibold text-foreground">Agregar email</p>
						<p class="truncate text-xs text-muted-foreground">{query.trim()}</p>
					</div>
				</button>
			{/if}

			<!-- Friends multi-select -->
			<div>
				<div class="mb-2 flex items-center gap-2">
					<Users class="size-3.5 text-muted-foreground" />
					<p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						Tus amigos
						{#if !friendsLoading}
							({filteredFriends.length})
						{/if}
					</p>
				</div>

				<div
					class="max-h-52 divide-y divide-border overflow-y-auto rounded-lg border border-border bg-muted/20"
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
							Todavía no tenés amigos. Podés agregar emails arriba.
						</div>
					{:else if filteredFriends.length === 0}
						<div class="px-4 py-8 text-center text-sm text-muted-foreground">
							No hay amigos que coincidan.
						</div>
					{:else}
						{#each filteredFriends as friend (friend.user_id)}
							{@const checked = isFriendSelected(friend.user_id)}
							<button
								type="button"
								onclick={() => toggleFriend(friend)}
								class="flex w-full items-center gap-3 px-3 py-2.5 text-left transition hover:bg-muted/60 focus:bg-muted/60 focus:outline-none {checked
									? 'bg-primary/5'
									: ''}"
							>
								<div
									class="flex size-5 shrink-0 items-center justify-center rounded border transition"
									class:border-primary={checked}
									class:bg-primary={checked}
									class:border-border={!checked}
									class:bg-background={!checked}
								>
									{#if checked}
										<svg
											class="size-3 text-primary-foreground"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="3"
										>
											<polyline points="20 6 9 17 4 12" />
										</svg>
									{/if}
								</div>
								<div
									class="flex size-9 shrink-0 items-center justify-center rounded-full bg-linear-to-br from-violet-200 to-lime-200 text-xs font-bold text-violet-800 dark:from-violet-400/30 dark:to-lime-400/20 dark:text-violet-200"
								>
									{friend.name.slice(0, 2).toUpperCase()}
								</div>
								<div class="min-w-0 flex-1">
									<p class="truncate text-sm font-medium text-foreground">{friend.name}</p>
									<p class="truncate text-xs text-muted-foreground">{friend.email}</p>
								</div>
								{#if checked}
									<UserCheck class="size-4 shrink-0 text-primary" />
								{/if}
							</button>
						{/each}
					{/if}
				</div>
			</div>
		</div>
	{/if}

	{#snippet footer()}
		{#if step === 1}
			<Button label="Cancelar" variant="secondary" onclick={handleClose} />
			<Button
				label="Continuar"
				type="submit"
				form="new-group-form"
				disabled={!step1Valid}
				loading={form.loading}
			/>
		{:else}
			<Button
				label="Omitir"
				variant="secondary"
				onclick={skipInvitesAndGo}
				disabled={form.loading}
			/>
			<Button
				label={selected.length > 0 ? `Invitar (${selected.length}) y abrir` : 'Abrir grupo'}
				onclick={() => {
					if (selected.length > 0) void sendInvitesAndGo();
					else skipInvitesAndGo();
				}}
				loading={form.loading}
			/>
		{/if}
	{/snippet}
</Modal>
