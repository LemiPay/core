<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';

	import UserProfileCard from '$lib/components/UserProfileCard.svelte';
	import GroupSummaryCard from '$lib/components/GroupSummaryCard.svelte';

	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import NewGroup from '$lib/components/modals/NewGroup.svelte';

	let isLoading = $state(true);
	let error = $state('');
	let misGrupos = $state<GroupSummary[]>([]);
	let showNewGroup = $state(false);

	async function load_my_groups() {
		isLoading = true;
		error = '';

		const res = await getMyGroups();

		if (!isSuccess(res)) {
			error = res.message || 'Error al buscar grupos';
			isLoading = false;
			console.error(error);
			return;
		}

		misGrupos = res.body;
		isLoading = false;
	}

	load_my_groups();
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 p-6 pt-8">
	<div class="w-full">
		<UserProfileCard />
	</div>
	<div>
		<IconButton variant="primary" ariaLabel="Create group" onclick={() => (showNewGroup = true)}>
			{#snippet icon()}
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<line x1="12" y1="5" x2="12" y2="19" />
					<line x1="5" y1="12" x2="19" y2="12" />
				</svg>
			{/snippet}
		</IconButton>
		<NewGroup open={showNewGroup} onclose={() => (showNewGroup = false)} />
	</div>

	<div class="flex w-full flex-col gap-4">
		<h2 class="text-xl font-bold text-black">Mis Grupos</h2>

		{#each misGrupos as grupo (grupo.group_id)}
			<GroupSummaryCard group={grupo} />
		{/each}

		{#if misGrupos.length === 0 && !isLoading}
			<p class="text-sm text-gray-500">Aún no tienes grupos.</p>
		{/if}
	</div>
</div>
