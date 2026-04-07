<script lang="ts">
	import { Bell } from 'lucide-svelte';
	import GroupInvite from './GroupInvite.svelte';
	import type { Action } from 'svelte/action';
	import { getReceivedProposals, respondToReceivedProposal } from '$lib/api/endpoints/proposals';
	import { isSuccess } from '$lib/types/client.types';
	import type { ReceivedNewMemberProposalExpanded } from '$lib/types/endpoints/proposals.types';
	import { invalidate } from '$app/navigation';

	interface NotificationData {
		groupId: string;
		senderName: string;
		groupName: string;
	}

	interface Notification {
		id: string;
		type: string;
		data: NotificationData;
	}

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
		try {
			const response = await getReceivedProposals();

			if (!isSuccess(response)) {
				return;
			}

			const proposals = response.body;

			notifications = proposals.map((p: ReceivedNewMemberProposalExpanded) => {
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
		} catch (error) {
			console.error('Error catcheado:', error);
		}
	}

	loadNotifications();

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function closeDropdown() {
		isOpen = false;
	}

	function handleAccept(proposalId: string, groupId: string) {
		respondToReceivedProposal(true, proposalId);
		closeDropdown();
		window.location.href = `/groups/${groupId}`;
	}

	function handleDecline(proposalId: string) {
		respondToReceivedProposal(false, proposalId);
		loadNotifications();
		closeDropdown();
	}
</script>

<div class="relative inline-block" use:clickOutside={closeDropdown}>
	<button
		onclick={toggleDropdown}
		class="relative rounded-full bg-slate-100 p-2 text-slate-600 transition-colors hover:bg-slate-200 hover:text-slate-900"
	>
		<Bell size={20} />

		{#if notifications.length > 0}
			<span
				class="absolute top-0 right-0 flex h-4.5 min-w-4.5 items-center justify-center rounded-full border-2 border-white bg-red-500 px-1 text-[10px] font-bold text-white"
			>
				{notifications.length}
			</span>
		{/if}
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-2 w-80 origin-top-right overflow-hidden rounded-xl bg-white shadow-lg ring-1 ring-black/5 focus:outline-none"
		>
			<div class="border-b bg-slate-50 px-4 py-3">
				<h3 class="text-sm font-semibold text-slate-800">Notificaciones</h3>
			</div>

			<div class="max-h-96 overflow-y-auto">
				{#if notifications.length === 0}
					<div class="flex flex-col items-center gap-2 p-6 text-center text-sm text-slate-500">
						<Bell size={24} class="text-slate-300" />
						No tienes notificaciones nuevas.
					</div>
				{:else}
					{#each notifications as notif (notif.id)}
						<div class="bg-blue-50/20">
							{#if notif.type === 'group_invite'}
								<GroupInvite
									senderName={notif.data.senderName}
									groupName={notif.data.groupName}
									onAccept={() => handleAccept(notif.id, notif.data.groupId)}
									onDecline={() => handleDecline(notif.id)}
								/>
							{/if}
						</div>
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</div>
