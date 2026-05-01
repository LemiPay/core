<script lang="ts">
	import { Plus, Users, Scale, ArrowRight, Info } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatAmount, getInitials } from '$lib/utils/format_utils';
	import type { GroupState } from '../group.svelte';

	let { groupState, onInviteClick, onGoToBalances } = $props<{
		groupState: GroupState;
		onInviteClick: () => void;
		onGoToBalances: () => void;
	}>();
</script>

<div class="animate-in fade-in slide-in-from-bottom-2 space-y-8 duration-300">
	<div class="space-y-4">
		<div class="flex items-center justify-between gap-3">
			<div class="flex items-center gap-2">
				<h2 class="text-sm font-medium text-black">Miembros</h2>
				{#if !groupState.loadingMembers && groupState.members.length > 0}
					<span
						class="rounded-full bg-gray-100 px-2 py-0.5 text-[11px] font-semibold text-gray-600"
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
			<div class="h-5 w-5 animate-spin rounded-full border-2 border-gray-200 border-t-black"></div>
		{:else if groupState.members.length > 0}
			<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
				{#each groupState.members as member (member.user_id)}
					{@const isAdmin = member.role === 'Admin'}
					{@const initials = getInitials(member.name)}
					<a
						href={`/users/${member.user_id}`}
						class="group flex items-center gap-3 rounded-xl border border-gray-200 bg-white px-3 py-2.5 transition hover:border-gray-300 hover:shadow-sm"
					>
						<div
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isAdmin
								? 'border-gray-900 bg-gray-900 text-white'
								: 'border-gray-200 bg-gray-50 text-gray-700'} transition group-hover:border-black"
						>
							{initials}
						</div>
						<div class="min-w-0 flex-1 space-y-0.5">
							<p class="truncate text-sm font-medium text-black group-hover:underline">
								{member.name}
							</p>
							<p class="text-[11px] text-gray-400">{isAdmin ? 'Admin' : 'Miembro'}</p>
						</div>
					</a>
				{/each}

				<button
					type="button"
					onclick={onInviteClick}
					class="group flex items-center gap-3 rounded-xl border border-dashed border-gray-300 bg-white px-3 py-2.5 text-left transition hover:border-black hover:bg-gray-50"
				>
					<div
						class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-dashed border-gray-300 bg-white text-gray-400 transition group-hover:border-black group-hover:text-black"
					>
						<Plus class="h-4 w-4" />
					</div>
					<div class="min-w-0 space-y-0.5">
						<p class="truncate text-sm font-medium text-gray-500 group-hover:text-black">
							Invitar miembro
						</p>
						<p class="text-[11px] text-gray-400">Sumá a alguien al grupo</p>
					</div>
				</button>
			</div>
		{:else}
			<div class="rounded-xl border border-dashed border-gray-300 bg-white p-8 text-center">
				<Users class="mx-auto mb-3 h-8 w-8 text-gray-400" />
				<p class="text-sm font-medium text-black">Sin miembros aún</p>
				<p class="mb-4 text-sm text-gray-500">
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
			class="flex items-start gap-2 rounded-lg border border-rose-200 bg-rose-50/60 p-3 text-xs text-rose-800"
		>
			<Info class="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-500" />
			<span>{groupState.coreBalancesError}</span>
		</div>
	{/if}

	{#if !groupState.loadingMembers && !groupState.loadingCoreBalances && groupState.memberBalances.length > 0 && !groupState.coreBalancesError}
		{@const topMovers = groupState.sortedMemberBalances.slice(0, 5)}

		<div class="space-y-5">
			<div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
				<div class="space-y-1">
					<h2 class="text-sm font-medium text-black">Balance del grupo</h2>
					<p class="text-xs text-gray-500">Resumen de cuánto debe o tiene a favor cada miembro.</p>
				</div>
			</div>

			<div
				class="flex items-center justify-between gap-4 rounded-xl border border-gray-200 bg-white p-5 transition hover:border-gray-300 hover:shadow-sm"
			>
				<div class="space-y-1">
					<p class="text-[11px] font-medium tracking-wider text-gray-400 uppercase">
						Billeteras del grupo
					</p>
					<p class="flex items-baseline gap-2">
						<span class="text-3xl font-bold tracking-tight text-black">
							${formatAmount(groupState.groupWalletsBalance)}
						</span>
					</p>
					<p class="text-xs text-gray-500">Suma de balances en las wallets del grupo</p>
					<p class="text-xs text-gray-500">
						Balance según movimientos (core):
						<span class="font-medium text-black">${formatAmount(groupState.coreGroupBalance)}</span>
					</p>
				</div>
				<div
					class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-600"
				>
					<Scale class="h-5 w-5" />
				</div>
			</div>

			<div class="space-y-3">
				<div class="flex items-center justify-between">
					<h3 class="text-xs font-medium tracking-wider text-gray-500 uppercase">
						{groupState.memberBalances.length > 5 ? 'Top movimientos' : 'Detalle por miembro'}
					</h3>
					<span class="text-[11px] text-gray-400">Ordenado por balance</span>
				</div>

				<div
					class="divide-y divide-gray-100 overflow-hidden rounded-xl border border-gray-200 bg-white"
				>
					{#each topMovers as mb (mb.user.user_id)}
						{@const isCredit = mb.balance > 0.01}
						{@const isDebt = mb.balance < -0.01}
						{@const pct = Math.min(
							100,
							(Math.abs(mb.balance) / groupState.maxAbsoluteBalance) * 100
						)}
						<a
							href={`/users/${mb.user.user_id}`}
							class="flex items-center gap-4 px-4 py-3 transition hover:bg-gray-50"
						>
							<div
								class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-xs font-semibold {isCredit
									? 'border-emerald-200 bg-emerald-50 text-emerald-700'
									: isDebt
										? 'border-rose-200 bg-rose-50 text-rose-700'
										: 'border-gray-200 bg-gray-50 text-gray-500'}"
							>
								{getInitials(mb.user.name)}
							</div>

							<div class="min-w-0 flex-1 space-y-1.5">
								<div class="flex items-center gap-2">
									<span class="truncate text-sm font-medium text-black">{mb.user.name}</span>
									{#if mb.user.role === 'Admin'}
										<span
											class="rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 text-[9px] font-semibold tracking-wider text-gray-500 uppercase"
											>Admin</span
										>
									{/if}
								</div>

								<div class="relative h-1.5 w-full rounded-full bg-gray-100">
									<div class="absolute top-0 bottom-0 left-1/2 w-px bg-gray-300"></div>
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
										? 'text-emerald-700'
										: isDebt
											? 'text-rose-700'
											: 'text-gray-500'}"
								>
									{isCredit ? '+' : isDebt ? '-' : ''}${formatAmount(Math.abs(mb.balance))}
								</p>
								<p
									class="text-[10px] font-medium tracking-wider uppercase {isCredit
										? 'text-emerald-600/70'
										: isDebt
											? 'text-rose-600/70'
											: 'text-gray-400'}"
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
				class="group flex w-full items-center justify-between rounded-xl border border-gray-200 bg-white px-4 py-3 text-left transition hover:border-gray-300 hover:shadow-sm"
			>
				<div class="min-w-0 space-y-0.5">
					<p class="flex items-center gap-2 text-sm font-medium text-black">
						<Scale class="h-4 w-4 text-gray-500" />
						Ver detalle de deudas
					</p>
					<p class="text-[11px] text-gray-500">
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
					class="h-4 w-4 shrink-0 text-gray-400 transition group-hover:translate-x-0.5 group-hover:text-black"
				/>
			</button>
		</div>
	{/if}
</div>
