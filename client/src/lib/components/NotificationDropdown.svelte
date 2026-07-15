<script lang="ts">
	import { Bell, UserPlus, Check, X } from 'lucide-svelte';
	import GroupInvite from './GroupInvite.svelte';
	import type { Action } from 'svelte/action';
	import { getReceivedProposals, respondToReceivedProposal } from '$lib/api/endpoints/proposals';
	import { getReceivedRequests, respondToFriendRequest } from '$lib/api/endpoints/friends';
	import { isSuccess } from '$lib/types/client.types';
	import type { ReceivedNewMemberProposalExpanded } from '$lib/types/endpoints/proposals.types';
	import type { FriendResponse } from '$lib/types/endpoints/friends.types';

	type GroupInviteNotification = {
		id: string;
		groupId: string;
		senderName: string;
		groupName: string;
	};

	type FriendRequestNotification = {
		userId: string;
		name: string;
	};

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

	let groupInvites = $state<GroupInviteNotification[]>([]);
	let friendRequests = $state<FriendRequestNotification[]>([]);
	let isOpen = $state(false);

	const totalCount = $derived(groupInvites.length + friendRequests.length);

	async function loadNotifications() {
		try {
			const [groupRes, friendRes] = await Promise.all([
				getReceivedProposals(),
				getReceivedRequests()
			]);

			if (isSuccess(groupRes)) {
				groupInvites = groupRes.body.map((p: ReceivedNewMemberProposalExpanded) => ({
					id: p.proposal?.id || p.new_member_proposal?.proposal_id,
					groupId: p.proposal.group_id,
					senderName: p.sender_name,
					groupName: p.group_name
				}));
			}

			if (isSuccess(friendRes)) {
				friendRequests = friendRes.body.map((f: FriendResponse) => ({
					userId: f.user_id,
					name: f.name
				}));
			}
		} catch (error) {
			console.error('Error loading notifications:', error);
		}
	}

	loadNotifications();

	function toggleDropdown() {
		isOpen = !isOpen;
		if (isOpen) {
			void loadNotifications();
		}
	}

	function closeDropdown() {
		isOpen = false;
	}

	async function handleAcceptGroup(proposalId: string, groupId: string) {
		await respondToReceivedProposal(true, proposalId);
		closeDropdown();
		window.location.href = `/groups/${groupId}`;
	}

	async function handleDeclineGroup(proposalId: string) {
		await respondToReceivedProposal(false, proposalId);
		await loadNotifications();
	}

	async function handleAcceptFriend(userId: string) {
		await respondToFriendRequest(userId, 'accept');
		await loadNotifications();
	}

	async function handleDeclineFriend(userId: string) {
		await respondToFriendRequest(userId, 'reject');
		await loadNotifications();
	}
</script>

<div class="relative inline-block" use:clickOutside={closeDropdown}>
	<button
		onclick={toggleDropdown}
		class="relative rounded-full p-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
	>
		<Bell size={20} />

		{#if totalCount > 0}
			<span
				class="absolute top-0 right-0 flex h-4.5 min-w-4.5 items-center justify-center rounded-full border-2 border-background bg-primary px-1 text-[10px] font-bold text-primary-foreground"
			>
				{totalCount}
			</span>
		{/if}
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-[7px] w-80 origin-top-right overflow-hidden rounded-xl bg-background shadow-lg ring-1 ring-border focus:outline-none"
		>
			{#if groupInvites.length > 0}
				<div class="border-b border-border bg-muted/30 px-4 py-3">
					<h3 class="text-sm font-semibold text-foreground">Invitaciones a grupos</h3>
				</div>
				<div class="max-h-64 overflow-y-auto">
					{#each groupInvites as notif (notif.id)}
						<div class="bg-primary/5 transition-colors hover:bg-muted/50">
							<GroupInvite
								senderName={notif.senderName ?? 'Alguien'}
								groupName={notif.groupName}
								onAccept={() => handleAcceptGroup(notif.id, notif.groupId)}
								onDecline={() => handleDeclineGroup(notif.id)}
							/>
						</div>
					{/each}
				</div>
			{/if}

			{#if friendRequests.length > 0}
				<div class="border-b border-border bg-muted/30 px-4 py-3">
					<h3 class="text-sm font-semibold text-foreground">Solicitudes de amistad</h3>
				</div>
				<div class="max-h-64 overflow-y-auto">
					{#each friendRequests as req (req.userId)}
						<div
							class="flex items-center gap-3 border-b border-border bg-primary/5 p-4 transition-colors last:border-b-0 hover:bg-muted/50"
						>
							<div
								class="flex size-9 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-bold text-muted-foreground"
							>
								{req.name.slice(0, 2).toUpperCase()}
							</div>
							<div class="min-w-0 flex-1">
								<p class="text-sm text-foreground">
									<span class="font-semibold">{req.name}</span> te envió una solicitud de amistad.
								</p>
							</div>
							<div class="flex shrink-0 gap-1">
								<button
									onclick={() => handleAcceptFriend(req.userId)}
									class="flex size-8 items-center justify-center rounded-full bg-emerald-100 text-emerald-600 transition hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 dark:hover:bg-emerald-900/50"
									title="Aceptar"
								>
									<Check size={14} strokeWidth={3} />
								</button>
								<button
									onclick={() => handleDeclineFriend(req.userId)}
									class="flex size-8 items-center justify-center rounded-full bg-red-100 text-red-500 transition hover:bg-red-200 dark:bg-red-900/30 dark:text-red-400 dark:hover:bg-red-900/50"
									title="Rechazar"
								>
									<X size={14} strokeWidth={3} />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			{#if totalCount === 0}
				<div class="flex flex-col items-center gap-2 p-6 text-center text-sm text-muted-foreground">
					<Bell size={24} class="opacity-50" />
					No tienes notificaciones pendientes.
				</div>
			{/if}
		</div>
	{/if}
</div>
