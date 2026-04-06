<script lang="ts">
	import api from '$lib/api/endpoints/groups';
	import { page } from '$app/state';
	import { isSuccess } from '$lib/types/client.types';
	import type { Group } from '$lib/types/endpoints/groups.types';
	import { CircleUser } from 'lucide-svelte';

	let loading = $state(true);
	let groupExists = $state(true);
	let groupData = $state({} as Group);

	async function loadGroupData() {
		let groupId = page.params.group_id as string;
		const res = await api.getGroup(groupId);
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

	loadGroupData();
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>
<div>
	{#if !loading}
		{#if !groupExists}
			<h1>404 group not found</h1>
		{:else}
			<h1>{groupData.name}</h1>
			<CircleUser />
		{/if}
	{/if}
</div>
