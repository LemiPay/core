<script lang="ts">
	import api from '$lib/api/endpoints/groups';
	import { page } from '$app/state';
	import { isSuccess } from '$lib/types/client.types';
	import type { Group } from '$lib/types/endpoints/groups.types';
	import Button from '$lib/components/ui/Button.svelte';
	import InviteUserToGroup from '$lib/components/modals/InviteUserToGroup.svelte';

	let loading = $state(true);
	let groupExists = $state(true);
	let groupData = $state({} as Group);

	let showNewMemberModal = $state(false);

	async function loadGroupData() {
		const groupId = page.params.group_id as string;
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
				/>
			</div>
		{/if}
	{/if}
</div>
