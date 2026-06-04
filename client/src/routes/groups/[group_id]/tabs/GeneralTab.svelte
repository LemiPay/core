<script lang="ts">
	import { resolve } from '$app/paths';
	import { Plus, Users, Scale, ArrowRight, Info } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount, getInitials } from '$lib/utils/format_utils';
	import type { GroupState } from '../group.svelte';
	import { shortenAddress } from '$lib/utils/address_utils';

	let { groupState, onInviteClick, onGoToBalances } = $props<{
		groupState: GroupState;
		onInviteClick: () => void;
		onGoToBalances: () => void;
	}>();
</script>

<div class="animate-in space-y-8 duration-300 fade-in slide-in-from-bottom-2">
	<div class="space-y-4">
		<div class="flex items-center justify-between gap-3">
			<div class="flex items-center gap-2">
				<h2 class="text-sm font-medium text-foreground">Miembros</h2>
				{#if !groupState.loadingMembers && groupState.members.length > 0}
					<span
						class="rounded-full bg-muted px-2 py-0.5 text-[11px] font-semibold text-muted-foreground"
					>
						{groupState.members.length}
					</span>
				{/if}
			</div>
			<Button label="Invitar" variant="secondary" onclick={onInviteClick}>
				{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
			</Button>
		</div>

		{#if groupState.loadingMembers}
			<div
				class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-foreground"
			></div>
		{:else if groupState.members.length > 0}
			<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
				{#each groupState.members as member (member.user_id)}
					{@const isAdmin = member.role === 'Admin'}
					{@const initials = getInitials(member.name)}
					<a
						href={resolve('/users/[user_id]', { user_id: member.user_id })}
						class="group flex items-center gap-3 rounded-xl border border-border bg-card px-3 py-2.5 text-card-foreground transition hover:border-input hover:bg-accent hover:shadow-sm hover:shadow-black/5 dark:hover:shadow-black/20"
					>
						<div
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isAdmin
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-border bg-muted text-muted-foreground'} transition group-hover:border-foreground"
						>
							{member.name.slice(0, 2) === '0x' ? '0x' : initials}
						</div>
						<div class="min-w-0 flex-1 space-y-0.5">
							<p class="truncate text-sm font-medium text-foreground group-hover:underline">
								{member.name && member.name.slice(0, 2) === '0x'
									? shortenAddress(member.name)
									: member.name}
							</p>
							<p class="text-[11px] text-muted-foreground">{isAdmin ? 'Admin' : 'Miembro'}</p>
						</div>
					</a>
				{/each}

				<button
					type="button"
					onclick={onInviteClick}
					class="group flex items-center gap-3 rounded-xl border border-dashed border-border bg-card px-3 py-2.5 text-left text-card-foreground transition hover:border-foreground hover:bg-accent"
				>
					<div
						class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-dashed border-border bg-background text-muted-foreground transition group-hover:border-foreground group-hover:text-foreground"
					>
						<Plus class="h-4 w-4" />
					</div>
					<div class="min-w-0 space-y-0.5">
						<p
							class="truncate text-sm font-medium text-muted-foreground group-hover:text-foreground"
						>
							Invitar miembro
						</p>
						<p class="text-[11px] text-muted-foreground">Sumá a alguien al grupo</p>
					</div>
				</button>
			</div>
		{:else}
			<div
				class="rounded-xl border border-dashed border-border bg-card p-8 text-center text-card-foreground"
			>
				<Users class="mx-auto mb-3 h-8 w-8 text-muted-foreground" />
				<p class="text-sm font-medium text-foreground">Sin miembros aún</p>
				<p class="mb-4 text-sm text-muted-foreground">
					Invitá a alguien para empezar a mover plata en grupo.
				</p>
				<Button label="Invitar primer miembro" variant="secondary" onclick={onInviteClick}>
					{#snippet icon()}<Plus class="h-4 w-4" />{/snippet}
				</Button>
			</div>
		{/if}
	</div>

	{#if groupState.coreBalancesError && !groupState.loadingCoreBalances}
		<div
			class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
		>
			<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500 dark:text-rose-300" />
			<span>{groupState.coreBalancesError}</span>
		</div>
	{/if}

	{#if !groupState.loadingMembers && !groupState.loadingCoreBalances && groupState.memberBalances.length > 0 && !groupState.coreBalancesError}
		{@const topMovers = groupState.sortedMemberBalances.slice(0, 5)}

		<div class="space-y-5">
			<div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
				<div class="space-y-1">
					<h2 class="text-sm font-medium text-foreground">Balance del grupo</h2>
					<p class="text-xs text-muted-foreground">
						Resumen de cuánto debe o tiene a favor cada miembro.
					</p>
				</div>
			</div>

			<div
				class="flex items-center justify-between gap-4 rounded-xl border border-border bg-card p-5 text-card-foreground transition hover:border-input hover:shadow-sm hover:shadow-black/5 dark:hover:shadow-black/20"
			>
				<div class="space-y-1">
					<p class="text-[11px] font-medium tracking-wider text-muted-foreground uppercase">
						Billeteras del grupo
					</p>
					<p class="flex items-baseline gap-2">
						<span class="text-3xl font-bold tracking-tight text-foreground">
							${formatAmount(groupState.groupWalletsBalance)}
						</span>
					</p>
					<p class="text-xs text-muted-foreground">Suma de balances en las wallets del grupo</p>
					<p class="text-xs text-muted-foreground">
						Balance según movimientos (core):
						<span class="font-medium text-foreground"
							>${formatAmount(groupState.coreGroupBalance)}</span
						>
					</p>
				</div>
				<div
					class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-border bg-muted text-muted-foreground"
				>
					<Scale class="h-5 w-5" />
				</div>
			</div>

			<div class="space-y-3">
				<div class="flex items-center justify-between">
					<h3 class="text-xs font-medium tracking-wider text-muted-foreground uppercase">
						{groupState.memberBalances.length > 5 ? 'Top movimientos' : 'Detalle por miembro'}
					</h3>
					<span class="text-[11px] text-muted-foreground">Ordenado por balance</span>
				</div>

				<div
					class="divide-y divide-border overflow-hidden rounded-xl border border-border bg-card text-card-foreground"
				>
					{#each topMovers as mb (mb.user.user_id)}
						{@const isCredit = mb.balance > 0.01}
						{@const isDebt = mb.balance < -0.01}
						{@const pct = Math.min(
							100,
							(Math.abs(mb.balance) / groupState.maxAbsoluteBalance) * 100
						)}
						<a
							href={resolve('/users/[user_id]', { user_id: mb.user.user_id })}
							class="flex items-center gap-4 px-4 py-3 transition hover:bg-accent"
						>
							<div
								class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isCredit
									? 'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/20 dark:bg-emerald-400/10 dark:text-emerald-300'
									: isDebt
										? 'border-rose-200 bg-rose-50 text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300'
										: 'border-border bg-muted text-muted-foreground'}"
							>
								{getInitials(mb.user.name)}
							</div>

							<div class="min-w-0 flex-1 space-y-1.5">
								<div class="flex items-center gap-2">
									<span class="truncate text-sm font-medium text-foreground">{mb.user.name}</span>
									{#if mb.user.role === 'Admin'}
										<span
											class="rounded border border-border bg-muted px-1.5 py-0.5 text-[9px] font-semibold tracking-wider text-muted-foreground uppercase"
											>Admin</span
										>
									{/if}
								</div>

								<div class="relative h-1.5 w-full rounded-full bg-muted">
									<div class="absolute top-0 bottom-0 left-1/2 w-px bg-border"></div>
									{#if isCredit}
										<div
											class="absolute top-0 bottom-0 left-1/2 rounded-r-full bg-gradient-to-r from-emerald-400 to-emerald-600 transition-all duration-700"
											style="width: {pct / 2}%"
										></div>
									{:else if isDebt}
										<div
											class="absolute top-0 right-1/2 bottom-0 rounded-l-full bg-gradient-to-l from-rose-400 to-rose-600 transition-all duration-700"
											style="width: {pct / 2}%"
										></div>
									{/if}
								</div>
							</div>

							<div class="shrink-0 text-right">
								<p
									class="text-sm font-semibold tabular-nums {isCredit
										? 'text-emerald-700 dark:text-emerald-300'
										: isDebt
											? 'text-rose-700 dark:text-rose-300'
											: 'text-muted-foreground'}"
								>
									{isCredit ? '+' : isDebt ? '-' : ''}${formatAmount(Math.abs(mb.balance))}
								</p>
								<p
									class="text-[10px] font-medium tracking-wider uppercase {isCredit
										? 'text-emerald-600/70 dark:text-emerald-300/80'
										: isDebt
											? 'text-rose-600/70 dark:text-rose-300/80'
											: 'text-muted-foreground'}"
								>
									{isCredit ? 'a favor' : isDebt ? 'debe' : 'saldado'}
								</p>
							</div>
						</a>
					{/each}
				</div>
			</div>

			<button
				type="button"
				onclick={onGoToBalances}
				class="group flex w-full items-center justify-between rounded-xl border border-border bg-card px-4 py-3 text-left text-card-foreground transition hover:border-input hover:bg-accent hover:shadow-sm hover:shadow-black/5 dark:hover:shadow-black/20"
			>
				<div class="min-w-0 space-y-0.5">
					<p class="flex items-center gap-2 text-sm font-medium text-foreground">
						<Scale class="h-4 w-4 text-muted-foreground" />
						Ver detalle de deudas
					</p>
					<p class="text-[11px] text-muted-foreground">
						{#if groupState.settlements.length > 0}
							{groupState.settlements.length}
							{groupState.settlements.length === 1
								? 'transferencia sugerida'
								: 'transferencias sugeridas'} para saldar todo
						{:else}
							Nadie le debe nada a nadie. ¡Todo saldado!
						{/if}
					</p>
				</div>
				<ArrowRight
					class="h-4 w-4 shrink-0 text-muted-foreground transition group-hover:translate-x-0.5 group-hover:text-foreground"
				/>
			</button>
		</div>
	{/if}
</div>
