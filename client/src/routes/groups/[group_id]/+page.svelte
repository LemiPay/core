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
	<title>Lemipay - Home</title>
</svelte:head>
<div>
	{#if !loading}
		<div class="flex min-h-screen flex-col items-center justify-center gap-4 bg-gray-50">
			{#if !groupExists}
				<h1 class="text-3xl font-bold text-gray-800">404 group not found</h1>
			{:else}
				<h1 class="text-3xl font-bold text-gray-800">{groupData.name}</h1>
				{#if !loadingMembers}
					{#each members as member}
						<UserIconBadge user={member} />
					{/each}
				{/if}
			{/if}
		</div>
	{/if}
</div>
