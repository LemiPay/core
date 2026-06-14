<script lang="ts">
	import { Bell, X } from 'lucide-svelte';
	import GroupInvite from './GroupInvite.svelte';
	import type { Action } from 'svelte/action';
	import { getReceivedProposals, respondToReceivedProposal } from '$lib/api/endpoints/proposals';
	import { getMyGroups } from '$lib/api/endpoints/groups';
	import { getGroupFundRoundProposals } from '$lib/api/endpoints/fund_rounds';
	import { getAllWithdrawProposals } from '$lib/api/endpoints/transactions';
	import { listApprovedProposals } from '$lib/api/endpoints/investments';
	import { isSuccess } from '$lib/types/client.types';
	import { authStore } from '$lib/stores/auth';
	import type { ReceivedNewMemberProposalExpanded } from '$lib/types/endpoints/proposals.types';
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	import type { FundRoundProposalExpanded } from '$lib/types/endpoints/fund_rounds.types';
	import type { WithdrawProposalExpanded } from '$lib/types/endpoints/transactions.types';

	type NotificationData = {
		groupId: string;
		senderName?: string;
		groupName: string;
		kind?: string;
		status?: string;
		amount?: string;
	};

	type Notification = {
		id: string;
		type: string; // 'group_invite' | 'proposal'
		data: NotificationData;
	};

	// Client-side dismissed notifications persisted in localStorage (per user)
	// Stored as [{id, dismissedAt}] to allow auto-cleanup of old ones
	let dismissedItems = $state<Array<{ id: string; dismissedAt: number }>>([]);

	const dismissedIds = $derived(new Set(dismissedItems.map((d) => d.id)));

	function getDismissKey() {
		const userId = $authStore?.user?.id ?? 'anonymous';
		return `lemipay_dismissed_notifs_${userId}`;
	}

	function loadDismissedItems() {
		try {
			const key = getDismissKey();
			const stored = localStorage.getItem(key);
			if (stored) {
				let items: Array<{ id: string; dismissedAt: number }> = JSON.parse(stored);
				const now = Date.now();
				const maxAge = 30 * 24 * 60 * 60 * 1000; // 30 days
				items = items.filter((item) => now - item.dismissedAt < maxAge);
				dismissedItems = items;
				if (items.length !== JSON.parse(stored).length) {
					saveDismissedItems();
				}
			}
		} catch (e) {
			dismissedItems = [];
		}
	}

	function saveDismissedItems() {
		try {
			const key = getDismissKey();
			localStorage.setItem(key, JSON.stringify(dismissedItems));
		} catch (e) {}
	}

	function dismissNotification(id: string) {
		if (!dismissedIds.has(id)) {
			dismissedItems = [...dismissedItems, { id, dismissedAt: Date.now() }];
			saveDismissedItems();
			notifications = notifications.filter((n) => n.id !== id);
		}
	}

	function clearAll() {
		const now = Date.now();
		const toAdd = notifications
			.filter((n) => !dismissedIds.has(n.id))
			.map((n) => ({ id: n.id, dismissedAt: now }));
		dismissedItems = [...dismissedItems, ...toAdd];
		saveDismissedItems();
		notifications = [];
	}

	loadDismissedItems();

	const clickOutside: Action<HTMLElement, () => void> = (node, callback) => {
		const handleClick = (event: MouseEvent) => {
			if (node && !node.contains(event.target as Node) && !event.defaultPrevented) {
				callback();
			}
		};

		document.addEventListener('click', handleClick, true);

		return {
			update(newCallback) {
				callback = newCallback;
			},
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	};

	let notifications = $state<Notification[]>([]);
	let isOpen = $state(false);

	async function loadNotifications() {
		loadDismissedItems();
		try {
			const response = await getReceivedProposals();

			if (!isSuccess(response)) {
				return;
			}

			const proposals = response.body;

			let notifs: Notification[] = proposals.map((p: ReceivedNewMemberProposalExpanded) => {
				return {
					id: p.proposal?.id || p.new_member_proposal?.proposal_id,
					type: 'group_invite',
					data: {
						groupId: p.proposal.group_id,
						senderName: p.sender_name,
						groupName: p.group_name
					}
				};
			});

			// Load additional proposal notifications (pending/executed for other kinds) from user's groups
			try {
				const groupsRes = await getMyGroups();
				if (isSuccess(groupsRes)) {
					const groups: GroupSummary[] = groupsRes.body;
					for (const g of groups.slice(0, 3)) {
						// limit to avoid too many calls
						// Fund rounds (pending or recently acted)
						const frRes = await getGroupFundRoundProposals(g.group_id);
						if (isSuccess(frRes)) {
							for (const fr of (frRes.body as any[])
								.filter(
									(f: any) =>
										(f.fund_round?.proposal?.status || f.proposal?.status) === 'Pending' ||
										(f.fund_round?.proposal?.status || f.proposal?.status) === 'Executed'
								)
								.slice(0, 2)) {
								const frData = fr.fund_round || fr;
								notifs.push({
									id: frData.proposal?.id || fr.proposal?.id,
									type: 'proposal',
									data: {
										groupId: g.group_id,
										groupName: g.group_name,
										kind: 'Ronda de fondeo',
										status: frData.proposal?.status || fr.proposal?.status
									}
								});
							}
						}
						// Withdraw proposals
						const wRes = await getAllWithdrawProposals(g.group_id);
						if (isSuccess(wRes)) {
							for (const w of wRes.body
								.filter(
									(wp: WithdrawProposalExpanded) =>
										wp.proposal.status === 'Pending' || wp.proposal.status === 'Executed'
								)
								.slice(0, 2)) {
								notifs.push({
									id: w.proposal.id,
									type: 'proposal',
									data: {
										groupId: g.group_id,
										groupName: g.group_name,
										kind: 'Retiro',
										status: w.proposal.status,
										amount: w.amount || w.withdraw_proposal?.amount
									}
								});
							}
						}

						// Investment proposals (approved ones ready for execution or recent)
						try {
							const invRes = await listApprovedProposals(g.group_id);
							if (isSuccess(invRes)) {
								for (const inv of (invRes.body as any[]).slice(0, 2)) {
									notifs.push({
										id: inv.proposal_id || inv.proposal?.id,
										type: 'proposal',
										data: {
											groupId: g.group_id,
											groupName: g.group_name,
											kind: 'Inversión',
											status: 'Approved' // or Executed if we track
										}
									});
								}
							}
						} catch {}
					}
				}
			} catch (e) {
				// non fatal, the main received are enough
			}

			// Filter dismissed (client-side, persisted in localStorage)
			notifs = notifs.filter((n) => !dismissedIds.has(n.id));

			// Deduplicate by id (in case a proposal appears in multiple lists)
			const seen = new Set<string>();
			notifications = notifs.filter((n) => {
				if (seen.has(n.id)) return false;
				seen.add(n.id);
				return true;
			});
		} catch (error) {
			console.error('Error catcheado:', error);
		}
	}

	// Initial load (for badge count etc.). Will refresh on open too.
	loadNotifications();

	function toggleDropdown() {
		isOpen = !isOpen;
		if (isOpen) {
			// Refresh notifications every time the bell is opened
			void loadNotifications();
		}
	}

	function closeDropdown() {
		isOpen = false;
	}

	async function handleAccept(proposalId: string, groupId: string) {
		await respondToReceivedProposal(true, proposalId);
		closeDropdown();
		window.location.href = `/groups/${groupId}`;
	}

	async function handleDecline(proposalId: string) {
		// 1. Lo borramos localmente del estado para que desaparezca al instante (Optimistic update)
		//notifications = notifications.filter((n) => n.id !== proposalId);

		await respondToReceivedProposal(false, proposalId);
		await loadNotifications();
	}
</script>

<div class="relative inline-block" use:clickOutside={closeDropdown}>
	<button
		onclick={toggleDropdown}
		class="relative rounded-full p-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
	>
		<Bell size={20} />

		{#if notifications.length > 0}
			<span
				class="absolute top-0 right-0 flex h-4.5 min-w-4.5 items-center justify-center rounded-full border-2 border-background bg-primary px-1 text-[10px] font-bold text-primary-foreground"
			>
				{notifications.length}
			</span>
		{/if}
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-[7px] w-80 origin-top-right overflow-hidden rounded-xl bg-background shadow-lg ring-1 ring-border focus:outline-none"
		>
			<div class="border-b border-border bg-muted/30 px-4 py-3">
				<h3 class="text-sm font-semibold text-foreground">Notificaciones</h3>
			</div>

			<div class="max-h-96 overflow-y-auto">
				{#if notifications.length === 0}
					<div
						class="flex flex-col items-center gap-2 p-6 text-center text-sm text-muted-foreground"
					>
						<Bell size={24} class="opacity-50" />
						No tienes notificaciones nuevas.
					</div>
				{:else}
					{#each notifications as notif (notif.id)}
						<div class="bg-primary/5 transition-colors hover:bg-muted/50">
							{#if notif.type === 'group_invite'}
								<GroupInvite
									senderName={notif.data.senderName ?? 'Alguien'}
									groupName={notif.data.groupName}
									onAccept={() => handleAccept(notif.id, notif.data.groupId)}
									onDecline={() => handleDecline(notif.id)}
								/>
							{:else if notif.type === 'proposal'}
								<div class="relative px-4 py-3 pr-8 text-sm">
									<button
										class="absolute top-1 right-1 rounded p-0.5 text-muted-foreground transition hover:bg-muted hover:text-foreground"
										onclick={() => dismissNotification(notif.id)}
										title="Ocultar notificación"
										aria-label="Ocultar notificación"
									>
										<X size={12} />
									</button>
									<div class="font-medium text-foreground">
										{notif.data.kind || 'Propuesta'} · {notif.data.status || 'Actualizada'}
									</div>
									<div class="text-xs text-muted-foreground">
										{notif.data.groupName}
										{#if notif.data.amount}
											· ${notif.data.amount}{/if}
									</div>
									<div class="mt-1 text-[11px] text-muted-foreground">
										{notif.data.status === 'Executed' ? 'Ejecutada' : 'Pendiente de acción'}
									</div>
								</div>
							{/if}
						</div>
					{/each}

					{#if notifications.length > 0}
						<div class="border-t border-border px-4 py-2 text-center">
							<button
								onclick={clearAll}
								class="text-xs text-muted-foreground transition hover:text-foreground hover:underline"
							>
								Limpiar todas
							</button>
						</div>
					{/if}
				{/if}
			</div>
		</div>
	{/if}
</div>
