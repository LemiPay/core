<script lang="ts">
	import { Bell } from 'lucide-svelte';
	import GroupInvite from './GroupInvite.svelte';
	import type { Action } from 'svelte/action';

	interface NotificationData {
		senderName: string;
		groupName: string;
	}

	interface Notification {
		id: string;
		type: string;
		read: boolean;
		data: NotificationData;
	}

	interface Props {
		notifications?: Notification[];
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

	let { notifications: initialNotifications = [] }: Props = $props();

	let notifications = $state(initialNotifications);
	let isOpen = $state(false);

	let unreadCount = $derived(notifications.filter((n) => !n.read).length);

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function closeDropdown() {
		isOpen = false;
	}

	function handleAccept(id: string) {
		console.log('Aceptaste la invitación:', id);
		notifications = notifications.filter((n) => n.id !== id);
	}

	function handleDecline(id: string) {
		console.log('Rechazaste la invitación:', id);
		notifications = notifications.filter((n) => n.id !== id);
	}
</script>

<div class="relative inline-block" use:clickOutside={closeDropdown}>
	<button
		onclick={toggleDropdown}
		class="relative rounded-full bg-slate-100 p-2 text-slate-600 transition-colors hover:bg-slate-200 hover:text-slate-900 focus:ring-2 focus:ring-blue-500 focus:outline-none"
	>
		<Bell size={20} />

		{#if unreadCount > 0}
			<span
				class="absolute top-0 right-0 flex h-[18px] min-w-[18px] items-center justify-center rounded-full border-2 border-white bg-red-500 px-1 text-[10px] font-bold text-white"
			>
				{unreadCount}
			</span>
		{/if}
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-2 w-80 origin-top-right overflow-hidden rounded-xl bg-white shadow-lg ring-1 ring-black/5 focus:outline-none"
		>
			<div class="flex items-center justify-between border-b bg-slate-50 px-4 py-3">
				<h3 class="text-sm font-semibold text-slate-800">Notificaciones</h3>
				{#if unreadCount > 0}
					<button class="text-xs text-blue-600 hover:text-blue-800 hover:underline">
						Marcar leídas
					</button>
				{/if}
			</div>

			<div class="max-h-96 overflow-y-auto">
				{#if notifications.length === 0}
					<div class="flex flex-col items-center gap-2 p-6 text-center text-sm text-slate-500">
						<Bell size={24} class="text-slate-300" />
						No tienes notificaciones nuevas.
					</div>
				{:else}
					{#each notifications as notif (notif.id)}
						<div class={notif.read ? 'opacity-60' : 'bg-blue-50/20'}>
							{#if notif.type === 'group_invite'}
								<GroupInvite
									senderName={notif.data.senderName}
									groupName={notif.data.groupName}
									onAccept={() => handleAccept(notif.id)}
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
