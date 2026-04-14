<script lang="ts">
	import { Trash2, Pencil, Wallet, Coins } from 'lucide-svelte';
	import { page } from '$app/state';

	// Api
	import { getGroup, getGroupMembers, updateGroup, deleteGroup } from '$lib/api/endpoints/groups';

	// Helpers
	import { isSuccess } from '$lib/types/client.types';

	// Types
	import type { Group } from '$lib/types/endpoints/groups.types';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';

	// Components
	import UserIconBadge from '$lib/components/UserIconBadge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import InviteUserToGroup from '$lib/components/modals/InviteUserToGroup.svelte';
	import Confirm from '$lib/components/modals/Confirm.svelte';
	import EditGroup from '$lib/components/modals/EditGroup.svelte';
	import CreateGroupWallet from '$lib/components/modals/CreateGroupWallet.svelte';
	import FundGroupWallet from '$lib/components/modals/FundGroupWallet.svelte';

	let loading = $state(true);
	let loadingMembers = $state(true);
	let groupExists = $state(true);
	let groupData = $state({} as Group);
	let members = $state([] as UserBadge[]);
	const groupId = page.params.group_id as string;

	let showNewMemberModal = $state(false);
	let showDeleteModal = $state(false);
	let showEditModal = $state(false);
	let showCreateWalletModal = $state(false);
	let showFundWalletModal = $state(false);

	let deleteLoading = $state(false);
	let deleteError = $state('');

	async function handleEditGroup(data: { name: string; description: string }) {
		const res = await updateGroup(groupId, data);
		if (!isSuccess(res)) {
			throw new Error(res.message || 'Failed to update group.');
		}
		groupData = res.body;
	}

	async function handleDeleteGroup() {
		deleteLoading = true;
		deleteError = '';
		const res = await deleteGroup(groupId);
		deleteLoading = false;
		if (!isSuccess(res)) {
			deleteError = res.message || 'Failed to delete group.';
			return;
		}
		window.location.href = '/dashboard';
	}

	async function loadGroupData() {
		const res = await getGroup(groupId);
		if (!isSuccess(res)) {
			groupExists = false;
			loading = false;
			/*setTimeout(() => {
				window.location.href = '/dashboard';
			}, 2000);*/
			return;
		}
		groupData = res.body;
		loading = false;
	}
	async function loadMembersData() {
		try {
			const res = await getGroupMembers(groupId);
			if (!isSuccess(res)) {
				members = [];
				return;
			}
			members = res.body;
		} finally {
			loadingMembers = false;
		}
	}

	loadGroupData();
	loadMembersData();
</script>

<svelte:head>
	<title>Lemipay - {groupData.name || 'Group'}</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-64px)] flex-col items-center justify-center p-4">
	{#if loading}
		<div class="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-black"></div>
	{:else if !groupExists}
		<div class="space-y-4 text-center">
			<h1 class="text-2xl font-bold tracking-tight text-black">404 - Group not found</h1>
			<p class="text-sm text-gray-500">The group you are looking for does not exist.</p>
		</div>
	{:else}
		<div
			class="w-full max-w-md space-y-6 rounded-xl border border-gray-200 bg-white p-6 shadow-sm sm:p-8"
		>
			<div class="space-y-2">
				<div class="flex items-start justify-between gap-4">
					<h1 class="text-2xl font-bold tracking-tight text-black">{groupData.name}</h1>

					<div class="flex items-center gap-2">
						{#if groupData.status}
							<span
								class="rounded border border-gray-200 bg-gray-50 px-2.5 py-1 text-xs font-medium text-gray-600"
							>
								{groupData.status}
							</span>
						{/if}
						<button
							onclick={() => (showEditModal = true)}
							class="rounded-md p-1 text-gray-400 transition hover:bg-gray-100 hover:text-gray-600"
							aria-label="Edit group"
						>
							<Pencil class="h-5 w-5" />
						</button>
						<button
							onclick={() => (showDeleteModal = true)}
							class="rounded-md p-1 text-gray-400 transition hover:bg-red-50 hover:text-red-500"
							aria-label="Delete group"
						>
							<Trash2 class="h-5 w-5" />
						</button>
					</div>
				</div>

				{#if groupData.description}
					<p class="text-sm leading-relaxed text-gray-500">{groupData.description}</p>
				{/if}
			</div>

			<hr class="border-gray-100" />

			<div class="space-y-3">
				<h2 class="text-sm font-medium text-black">Members</h2>

				{#if loadingMembers}
					<div
						class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"
					></div>
				{:else if members.length > 0}
					<div class="flex flex-wrap gap-2 pt-1">
						{#each members as member}
							<UserIconBadge user={member} />
						{/each}
					</div>
				{:else}
					<p class="text-sm text-gray-400">No members yet.</p>
				{/if}
			</div>
			<div class="mt-4">
				<Button
					label="Invitar miembro"
					variant="primary"
					onclick={() => (showNewMemberModal = true)}
				>
					{#snippet icon()}
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M16 21v-2a4 4 0 0 0-3-3.87" />
							<path d="M8 21v-2a4 4 0 0 1 3-3.87" />
							<circle cx="12" cy="7" r="4" />
							<line x1="19" y1="8" x2="19" y2="14" />
							<line x1="22" y1="11" x2="16" y2="11" />
						</svg>
					{/snippet}
				</Button>

				<InviteUserToGroup
					group_id={groupData.id}
					open={showNewMemberModal}
					onclose={() => (showNewMemberModal = false)}
					onsuccess={loadMembersData}
				/>
			</div>

			<hr class="border-gray-100" />

			<div class="space-y-3">
				<h2 class="text-sm font-medium text-black">Finanzas</h2>

				<div class="flex flex-col gap-3 sm:flex-row">
					<div class="flex-1">
						<Button
							label="Crear wallet del grupo"
							variant="primary"
							onclick={() => (showCreateWalletModal = true)}
						>
							{#snippet icon()}
								<Wallet class="h-5 w-5" />
							{/snippet}
						</Button>
					</div>
					<div class="flex-1">
						<Button
							label="Fondear con cuenta"
							variant="secondary"
							onclick={() => (showFundWalletModal = true)}
						>
							{#snippet icon()}
								<Coins class="h-5 w-5" />
							{/snippet}
						</Button>
					</div>
				</div>
			</div>

			<CreateGroupWallet
				open={showCreateWalletModal}
				group_id={groupData.id}
				onclose={() => (showCreateWalletModal = false)}
			/>

			<FundGroupWallet
				open={showFundWalletModal}
				group_id={groupData.id}
				onclose={() => (showFundWalletModal = false)}
			/>

			<EditGroup
				open={showEditModal}
				group={groupData}
				onclose={() => (showEditModal = false)}
				onedit={handleEditGroup}
			/>

			<Confirm
				open={showDeleteModal}
				title="Delete group"
				description="This action cannot be undone."
				message="Are you sure you want to delete this group?"
				onclose={() => {
					showDeleteModal = false;
					deleteError = '';
				}}
				onconfirm={handleDeleteGroup}
				loading={deleteLoading}
				error={deleteError}
			/>

			<div class="pt-4">
				<a
					href="/dashboard"
					class="flex w-full items-center justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-black transition hover:bg-gray-50"
				>
					Back to Dashboard
				</a>
			</div>
		</div>
	{/if}
</div>
