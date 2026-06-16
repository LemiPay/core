<script lang="ts">
	import { Inbox, Landmark, TrendingUp, UserPlus, Wallet } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import DashboardLayout from '../DashboardLayout.svelte';
	import GovernanceProposalList, {
		type GovernanceProposalItem
	} from '$lib/components/GovernanceProposalList.svelte';
	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { getGroupFundRoundProposals } from '$lib/api/endpoints/fund_rounds';
	import { getAllWithdrawProposals } from '$lib/api/endpoints/transactions';
	import { listApprovedProposals } from '$lib/api/endpoints/investments';
	import { getGroupNewMemberProposals } from '$lib/api/endpoints/proposals';
	import { isSuccess } from '$lib/types/client.types';
	import type { ProposalKind } from '$lib/utils/proposal_labels';

	type TypeFilter = ProposalKind;

	let loading = $state(true);
	let error = $state('');
	let filter = $state<TypeFilter>('all');
	let proposals = $state<GovernanceProposalItem[]>([]);

	const filterOptions: {
		value: TypeFilter;
		label: string;
		icon: typeof Inbox;
	}[] = [
		{ value: 'all', label: 'Todas', icon: Inbox },
		{ value: 'new_member', label: 'Nuevo miembro', icon: UserPlus },
		{ value: 'withdraw', label: 'Retiro', icon: Wallet },
		{ value: 'fund_round', label: 'Ronda de fondos', icon: Landmark },
		{ value: 'investment', label: 'Inversión', icon: TrendingUp }
	];

	const filteredProposals = $derived(
		filter === 'all' ? proposals : proposals.filter((p) => p.kind === filter)
	);

	async function loadProposals() {
		loading = true;
		error = '';

		const groupsRes = await getMyGroups();
		if (!isSuccess(groupsRes)) {
			error = 'No se pudieron cargar los grupos.';
			loading = false;
			return;
		}

		const groups = groupsRes.body;

		if (groups.length === 0) {
			proposals = [];
			loading = false;
			return;
		}

		const results = await Promise.all(
			groups.map(async (group) => {
				const [newMemberRes, withdrawRes, fundRoundRes, investmentRes] = await Promise.all([
					getGroupNewMemberProposals(group.group_id),
					getAllWithdrawProposals(group.group_id),
					getGroupFundRoundProposals(group.group_id),
					listApprovedProposals(group.group_id)
				]);

				const items: GovernanceProposalItem[] = [];

				if (isSuccess(newMemberRes)) {
					for (const p of newMemberRes.body) {
						items.push({
							id: p.proposal.id,
							groupId: group.group_id,
							groupName: group.group_name,
							kind: 'new_member',
							status: p.proposal.status,
							createdAt: p.proposal.created_at
						});
					}
				}

				if (isSuccess(withdrawRes)) {
					for (const p of withdrawRes.body) {
						items.push({
							id: p.proposal.id,
							groupId: group.group_id,
							groupName: group.group_name,
							kind: 'withdraw',
							status: p.proposal.status,
							createdAt: p.proposal.created_at
						});
					}
				}

				if (isSuccess(fundRoundRes)) {
					for (const p of fundRoundRes.body) {
						items.push({
							id: p.proposal.id,
							groupId: group.group_id,
							groupName: group.group_name,
							kind: 'fund_round',
							status: p.proposal.status,
							createdAt: p.proposal.created_at
						});
					}
				}

				if (isSuccess(investmentRes)) {
					for (const p of investmentRes.body) {
						items.push({
							id: p.proposal_id,
							groupId: group.group_id,
							groupName: group.group_name,
							kind: 'investment',
							status: p.status,
							createdAt: p.created_at
						});
					}
				}

				return items;
			})
		);

		proposals = results
			.flat()
			.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());

		loading = false;
	}

	$effect(() => {
		void loadProposals();
	});
</script>

<svelte:head>
	<title>Lemipay – Gobernanza</title>
</svelte:head>

<DashboardLayout>
	<section class="space-y-6">
		<div
			class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
			in:fly={{ y: 12, duration: 360 }}
		>
			<div>
				<p class="text-sm font-medium text-muted-foreground">Tus grupos</p>
				<h1 class="mt-1 text-3xl font-semibold tracking-tight">Gobernanza</h1>
				<p class="mt-2 max-w-xl text-sm text-muted-foreground">
					Todas las propuestas de los grupos a los que pertenecés, con su tipo y estado.
				</p>
			</div>
			<div
				class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
			>
				<Landmark class="size-4" />
			</div>
		</div>

		<div class="flex flex-wrap gap-2">
			{#each filterOptions as option (option.value)}
				{@const Icon = option.icon}
				<button
					type="button"
					onclick={() => (filter = option.value)}
					class={filter === option.value
						? 'inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2.5 text-sm font-semibold text-background shadow-sm'
						: 'inline-flex items-center gap-2 rounded-2xl border border-border px-4 py-2.5 text-sm font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground'}
				>
					<Icon class="size-4" />
					{option.label}
				</button>
			{/each}
		</div>

		<section class="rounded-[2rem] border border-border bg-card p-5 shadow-sm sm:p-6">
			{#if loading}
				<div class="space-y-3">
					{#each Array(4) as _, i (i)}
						<div class="h-20 animate-pulse rounded-3xl bg-muted/50"></div>
					{/each}
				</div>
			{:else if error}
				<p class="text-sm text-destructive">{error}</p>
			{:else}
				<GovernanceProposalList
					proposals={filteredProposals}
					emptyMessage={filter === 'all'
						? 'No hay propuestas en tus grupos.'
						: 'No hay propuestas de este tipo.'}
				/>
			{/if}
		</section>
	</section>
</DashboardLayout>
