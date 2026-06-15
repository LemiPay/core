<script lang="ts">
	import {
		BellRing,
		CheckCheck,
		FileText,
		Loader2,
		ReceiptText,
		Sparkles,
		TrendingUp,
		UserPlus,
		Wallet,
		X
	} from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import type { NotificationRecord } from '$lib/types/endpoints/notifications.types';
	import {
		formatRelativeTime,
		getEventIconContainerClasses,
		getEventLabel,
		getEventVariant,
		getNotificationDetail,
		getNotificationTitle
	} from '$lib/utils/notificationLabels';

	interface Props {
		notifications: NotificationRecord[];
		markRead?: (id: string) => void | Promise<void>;
		showMarkAll?: boolean;
		markAllRead?: () => void | Promise<void>;
		emptyMessage?: string;
		compact?: boolean;
	}

	let {
		notifications,
		markRead,
		showMarkAll = false,
		markAllRead,
		emptyMessage = 'No hay notificaciones para mostrar.',
		compact = false
	}: Props = $props();

	let markingIds = $state<Set<string>>(new Set());

	const hasUnread = $derived(notifications.some((n) => !n.read));
	const unreadCount = $derived(notifications.filter((n) => !n.read).length);

	function getEventIcon(eventName: string) {
		if (eventName === 'withdraw_proposal_created' || eventName === 'proposal_executed')
			return FileText;
		if (eventName === 'investment_created' || eventName === 'investment_matured') return TrendingUp;
		if (eventName === 'expense_created') return ReceiptText;
		if (eventName === 'fund_round_created') return Wallet;
		if (eventName === 'new_member_added') return UserPlus;
		if (eventName === 'welcome' || eventName === 'login_alert') return Sparkles;
		return BellRing;
	}

	async function handleMarkRead(id: string) {
		if (!markRead || markingIds.has(id)) return;
		markingIds = new Set([...markingIds, id]);
		try {
			await markRead(id);
		} finally {
			const next = new Set(markingIds);
			next.delete(id);
			markingIds = next;
		}
	}
</script>

{#if showMarkAll && hasUnread && markAllRead}
	<div
		class="mb-5 flex items-center justify-between gap-3 rounded-2xl border border-border/70 bg-muted/30 px-4 py-3"
	>
		<p class="text-sm text-muted-foreground">
			<span class="font-semibold text-foreground">{unreadCount}</span>
			{unreadCount === 1 ? 'sin leer' : 'sin leer'}
		</p>
		<button
			type="button"
			onclick={() => markAllRead()}
			class="inline-flex items-center gap-2 rounded-2xl bg-foreground px-3.5 py-2 text-xs font-semibold text-background transition hover:bg-foreground/90"
		>
			<CheckCheck class="size-3.5" />
			Marcar todas
		</button>
	</div>
{/if}

{#if notifications.length === 0}
	<div
		class="flex flex-col items-center gap-3 rounded-3xl border border-dashed border-border/80 bg-muted/20 px-6 py-10 text-center"
	>
		<div
			class="flex size-12 items-center justify-center rounded-2xl bg-muted text-muted-foreground"
		>
			<BellRing class="size-5 opacity-60" />
		</div>
		<p class="text-sm text-muted-foreground">{emptyMessage}</p>
	</div>
{:else}
	<div class={compact ? 'space-y-2.5' : 'space-y-3'}>
		{#each notifications as notification, index (notification.id)}
			{@const variant = getEventVariant(notification.event_name)}
			{@const Icon = getEventIcon(notification.event_name)}
			{@const isMarking = markingIds.has(notification.id)}
			<div
				class="group relative"
				in:fly={{ y: 8, duration: 220, delay: Math.min(index * 40, 200) }}
			>
				<div
					class={[
						'flex items-start gap-3 rounded-3xl border p-3.5 transition duration-200',
						compact ? 'p-3' : 'p-4',
						!notification.read
							? 'border-lime-300/50 bg-linear-to-r from-lime-400/8 via-card to-card shadow-sm shadow-lime-500/5 hover:border-lime-300/80 hover:shadow-md hover:shadow-lime-500/10'
							: 'border-border/70 bg-card/60 hover:border-border hover:bg-muted/40'
					]}
				>
					<div
						class={[
							'flex shrink-0 items-center justify-center rounded-2xl',
							compact ? 'size-9' : 'size-11',
							getEventIconContainerClasses(variant)
						]}
					>
						<Icon class={compact ? 'size-4' : 'size-5'} />
					</div>

					<div class="min-w-0 flex-1">
						<div class="flex items-start justify-between gap-2">
							<div class="min-w-0">
								<div class="flex items-center gap-2">
									{#if !notification.read}
										<span
											class="size-2 shrink-0 rounded-full bg-lime-500 shadow-sm shadow-lime-500/50"
										></span>
									{/if}
									<p class="truncate text-sm font-semibold text-foreground">
										{getNotificationTitle(notification.event_name, notification.group_name)}
									</p>
								</div>
								<p class="mt-0.5 truncate text-xs text-muted-foreground">
									{getNotificationDetail(notification.event_name, notification.group_name)}
								</p>
							</div>
							<span class="shrink-0 text-[11px] font-medium text-muted-foreground">
								{formatRelativeTime(notification.created_at)}
							</span>
						</div>

						{#if !compact}
							<p class="mt-2 text-[11px] tracking-wide text-muted-foreground/80 uppercase">
								{getEventLabel(notification.event_name)}
							</p>
						{/if}
					</div>

					{#if !notification.read && markRead}
						<button
							type="button"
							class="flex size-8 shrink-0 items-center justify-center rounded-xl border border-border/80 bg-background/80 text-muted-foreground transition hover:border-rose-300 hover:bg-rose-400/10 hover:text-rose-700 dark:hover:text-rose-300"
							onclick={(e) => {
								e.stopPropagation();
								void handleMarkRead(notification.id);
							}}
							title="Marcar como leída"
							aria-label="Marcar como leída"
							disabled={isMarking}
						>
							{#if isMarking}
								<Loader2 class="size-3.5 animate-spin" />
							{:else}
								<X class="size-3.5" />
							{/if}
						</button>
					{/if}
				</div>
			</div>
		{/each}
	</div>
{/if}
