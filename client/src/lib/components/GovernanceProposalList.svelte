<script lang="ts">
	import { FileText, Landmark, TrendingUp, UserPlus, Wallet } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import {
		getGovernanceStatusDisplay,
		getProposalKindDisplay,
		type ProposalKind
	} from '$lib/utils/proposal_labels';
	import { formatRelativeTime } from '$lib/utils/notificationLabels';

	export type GovernanceProposalItem = {
		id: string;
		groupId: string;
		groupName: string;
		kind: Exclude<ProposalKind, 'all'>;
		status: string;
		createdAt: string;
	};

	interface Props {
		proposals: GovernanceProposalItem[];
		emptyMessage?: string;
	}

	let { proposals, emptyMessage = 'No hay propuestas para mostrar.' }: Props = $props();

	function getKindIcon(kind: GovernanceProposalItem['kind']) {
		if (kind === 'new_member') return UserPlus;
		if (kind === 'withdraw') return Wallet;
		if (kind === 'fund_round') return Landmark;
		if (kind === 'investment') return TrendingUp;
		return FileText;
	}

	function getKindIconClasses(kind: GovernanceProposalItem['kind']): string {
		if (kind === 'new_member') return 'bg-violet-400/15 text-violet-700 dark:text-violet-300';
		if (kind === 'withdraw') return 'bg-orange-400/15 text-orange-700 dark:text-orange-300';
		if (kind === 'fund_round') return 'bg-sky-400/15 text-sky-700 dark:text-sky-300';
		if (kind === 'investment') return 'bg-emerald-400/15 text-emerald-700 dark:text-emerald-300';
		return 'bg-muted text-muted-foreground';
	}
</script>

{#if proposals.length === 0}
	<div
		class="flex flex-col items-center gap-3 rounded-3xl border border-dashed border-border/80 bg-muted/20 px-6 py-10 text-center"
	>
		<div
			class="flex size-12 items-center justify-center rounded-2xl bg-muted text-muted-foreground"
		>
			<Landmark class="size-5 opacity-60" />
		</div>
		<p class="text-sm text-muted-foreground">{emptyMessage}</p>
	</div>
{:else}
	<div class="space-y-3">
		{#each proposals as proposal, index (proposal.id)}
			{@const kindDisplay = getProposalKindDisplay(proposal.kind)}
			{@const statusDisplay = getGovernanceStatusDisplay(proposal.status)}
			{@const Icon = getKindIcon(proposal.kind)}
			<div
				class="group relative"
				in:fly={{ y: 8, duration: 220, delay: Math.min(index * 40, 200) }}
			>
				<div
					class="flex items-start gap-3 rounded-3xl border border-border/70 bg-card/60 p-4 transition duration-200 hover:border-border hover:bg-muted/40"
				>
					<div
						class={[
							'flex size-11 shrink-0 items-center justify-center rounded-2xl',
							getKindIconClasses(proposal.kind)
						]}
					>
						<Icon class="size-5" />
					</div>

					<div class="min-w-0 flex-1">
						<div class="flex items-start justify-between gap-2">
							<div class="min-w-0">
								<p class="truncate text-sm font-semibold text-foreground">
									{proposal.groupName}
								</p>
								<p class="mt-0.5 truncate text-xs text-muted-foreground">
									Propuesta #{proposal.id.slice(0, 8)}
								</p>
							</div>
							<span class="shrink-0 text-[11px] font-medium text-muted-foreground">
								{formatRelativeTime(proposal.createdAt)}
							</span>
						</div>

						<div class="mt-3 flex flex-wrap items-center gap-2">
							<span
								class={[
									'inline-flex items-center rounded-full border px-2.5 py-1 text-[11px] font-semibold',
									kindDisplay.classes
								]}
							>
								{kindDisplay.label}
							</span>
							<span
								class={[
									'inline-flex items-center rounded-full border px-2.5 py-1 text-[11px] font-semibold',
									statusDisplay.classes
								]}
							>
								{statusDisplay.label}
							</span>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>
{/if}
