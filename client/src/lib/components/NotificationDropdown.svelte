<script lang="ts">
	import { Bell } from 'lucide-svelte';
	import GroupInvite from './GroupInvite.svelte';
	import type { Action } from 'svelte/action';
	import { getReceivedProposals, respondToReceivedProposal } from '$lib/api/endpoints/proposals';
	import { isSuccess } from '$lib/types/client.types';
	import type { ReceivedNewMemberProposalExpanded } from '$lib/types/endpoints/proposals.types';

	type GroupInviteNotification = {
		id: string;
		groupId: string;
		senderName: string;
		groupName: string;
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

	let notifications = $state<GroupInviteNotification[]>([]);
	let isOpen = $state(false);

	async function loadNotifications() {
		try {
			const response = await getReceivedProposals();

			if (!isSuccess(response)) {
				return;
			}

			notifications = response.body.map((p: ReceivedNewMemberProposalExpanded) => ({
				id: p.proposal?.id || p.new_member_proposal?.proposal_id,
				groupId: p.proposal.group_id,
				senderName: p.sender_name,
				groupName: p.group_name
			}));
		} catch (error) {
			console.error('Error loading group invites:', error);
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

	async function handleAccept(proposalId: string, groupId: string) {
		await respondToReceivedProposal(true, proposalId);
		closeDropdown();
		window.location.href = `/groups/${groupId}`;
	}

	async function handleDecline(proposalId: string) {
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
				<h3 class="text-sm font-semibold text-foreground">Invitaciones a grupos</h3>
			</div>

			<div class="max-h-96 overflow-y-auto">
				{#if notifications.length === 0}
					<div
						class="flex flex-col items-center gap-2 p-6 text-center text-sm text-muted-foreground"
					>
						<Bell size={24} class="opacity-50" />
						No tienes invitaciones pendientes.
					</div>
				{:else}
					{#each notifications as notif (notif.id)}
						<div class="bg-primary/5 transition-colors hover:bg-muted/50">
							<GroupInvite
								senderName={notif.senderName ?? 'Alguien'}
								groupName={notif.groupName}
								onAccept={() => handleAccept(notif.id, notif.groupId)}
								onDecline={() => handleDecline(notif.id)}
							/>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</div>
