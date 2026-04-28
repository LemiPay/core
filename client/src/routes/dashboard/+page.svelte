<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	import { Plus } from 'lucide-svelte';

	import UserProfileCard from '$lib/components/UserProfileCard.svelte';
	import GroupSummaryCard from '$lib/components/GroupSummaryCard.svelte';

	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { isSuccess } from '$lib/types/client.types';
	import FAB from '$lib/components/ui/FAB.svelte';
	import NewGroup from '$lib/components/modals_old/modals/NewGroup.svelte';

	let isLoading = $state(true);
	let error = $state('');
	let misGrupos = $state<GroupSummary[]>([]);
	let showNewGroup = $state(false);
	let didInitializeStatusFilter = $state(false);

	let filterRole = $state<'all' | 'Admin' | 'Member'>('all');
	let filterStatus = $state<'all' | 'Active' | 'Ended'>('Active');

	const gruposFiltrados = $derived(
		misGrupos.filter((g) => {
			const roleMatch = filterRole === 'all' || g.role.toLowerCase() === filterRole.toLowerCase();
			const statusMatch =
				filterStatus === 'all' || g.status.toLowerCase() === filterStatus.toLowerCase();
			return roleMatch && statusMatch;
		})
	);

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
		if (!didInitializeStatusFilter) {
			const hasActiveGroups = misGrupos.some((group) => group.status.toLowerCase() === 'active');
			filterStatus = hasActiveGroups ? 'Active' : 'all';
			didInitializeStatusFilter = true;
		}
		isLoading = false;
	}

	load_my_groups();
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 p-6 pt-8">
	<div class="w-full">
		<a href="/profile/me">
			<UserProfileCard />
		</a>
	</div>
	<NewGroup open={showNewGroup} onclose={() => (showNewGroup = false)} />

	<FAB ariaLabel="Create group" onclick={() => (showNewGroup = true)}>
		{#snippet icon()}
			<Plus />
		{/snippet}
	</FAB>

	<div class="flex w-full flex-col gap-4">
		<h2 class="text-xl font-bold text-black">Mis Grupos</h2>

		<!-- Filtros -->
		<div class="flex flex-wrap items-center gap-x-6 gap-y-3">
			<div class="flex items-center gap-2">
				<span class="text-xs font-medium text-gray-500">Rol</span>
				<div class="flex gap-1">
					{#each [['all', 'Todos'], ['Admin', 'Admin'], ['Member', 'Miembro']] as [val, label] (val)}
						<button
							onclick={() => (filterRole = val as typeof filterRole)}
							class={filterRole === val
								? 'rounded-full bg-black px-3 py-1 text-xs font-medium text-white'
								: 'rounded-full border border-gray-200 px-3 py-1 text-xs font-medium text-gray-500 transition hover:border-gray-400 hover:text-black'}
						>
							{label}
						</button>
					{/each}
				</div>
			</div>

			<div class="flex items-center gap-2">
				<span class="text-xs font-medium text-gray-500">Estado</span>
				<div class="flex gap-1">
					{#each [{ val: 'all', label: 'Todos', dot: '', active: 'bg-black text-white border-transparent', inactive: 'border-gray-200 text-gray-500 hover:bg-gray-100 hover:border-gray-400 hover:text-black' }, { val: 'Active', label: 'Activo', dot: 'bg-green-500', active: 'text-green-800 bg-green-200 border-green-600', inactive: 'border-green-200 text-green-600 hover:bg-green-100 hover:border-green-400 hover:text-green-800' }, { val: 'Ended', label: 'Finalizado', dot: 'bg-red-400', active: 'bg-red-200 border-red-400 text-red-700', inactive: 'border-red-200 text-red-400 hover:bg-red-100 hover:border-red-400 hover:text-red-700' }] as opt (opt.val)}
						<button
							onclick={() => (filterStatus = opt.val as typeof filterStatus)}
							class="flex items-center gap-1.5 rounded-full border px-3 py-1 text-xs font-medium transition {filterStatus ===
							opt.val
								? opt.active
								: opt.inactive}"
						>
							{#if opt.dot}
								<span class="h-1.5 w-1.5 rounded-full {opt.dot}"></span>
							{/if}
							{opt.label}
						</button>
					{/each}
				</div>
			</div>
		</div>

		{#each gruposFiltrados as grupo (grupo.group_id)}
			<GroupSummaryCard group={grupo} />
		{/each}

		{#if gruposFiltrados.length === 0 && !isLoading}
			<p class="text-sm text-gray-500">
				{misGrupos.length === 0
					? 'Aún no tienes grupos.'
					: 'No hay grupos que coincidan con los filtros.'}
			</p>
		{/if}
	</div>
</div>
