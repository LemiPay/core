<script lang="ts">
	import { getGroup, getGroupMembers } from '$lib/api/endpoints/groups';
	import { page } from '$app/state';
	import { isSuccess } from '$lib/types/client.types';
	import type { Group } from '$lib/types/endpoints/groups.types';
	import UserIconBadge from '$lib/components/UserIconBadge.svelte';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';

	let loading = $state(true);
	let loadingMembers = $state(true);
	let groupExists = $state(true);
	let groupData = $state({} as Group);
	let members = $state([] as UserBadge[]);
	const groupId = page.params.group_id as string;

	async function loadGroupData() {
		const res = await getGroup(groupId);
		if (!isSuccess(res)) {
			groupExists = false;
			loading = false;
			setTimeout(() => {
				window.location.href = '/dashboard';
			}, 2000);
			return;
		}
		groupData = res.body;
		loading = false;
	}
	async function loadMembersData() {
		const res = await getGroupMembers(groupId);
		if (!isSuccess(res)) {
			return;
		}
		members = res.body;
		loadingMembers = false;
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

					{#if groupData.status}
						<span
							class="rounded border border-gray-200 bg-gray-50 px-2.5 py-1 text-xs font-medium text-gray-600"
						>
							{groupData.status}
						</span>
					{/if}
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
