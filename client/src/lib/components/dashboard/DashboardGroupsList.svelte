<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	import { Plus, Users, Wallet } from 'lucide-svelte';
	import { flip } from 'svelte/animate';
	import { fade, fly, scale } from 'svelte/transition';
	import { resolve } from '$app/paths';

	import { getMyGroups, getGroupMembers } from '$lib/api/endpoints/groups';
	import { getGroupBalances } from '$lib/api/endpoints/core';
	import { isSuccess } from '$lib/types/client.types';
	import { authStore } from '$lib/stores/auth';
	import { formatMoney, parseBalanceValue } from '$lib/utils/format_utils';

	type FilterRole = 'all' | 'Admin' | 'Member';
	type FilterStatus = 'all' | 'Active' | 'DebtResolution' | 'Ended';
	type DebtInfo = {
		balance: number;
		hasDebtors: boolean;
	};

	type GroupTreasury = {
		balance: number;
		currency: string;
	};

	interface Props {
		showHeading?: boolean;
		columns?: 1 | 2;
		onCreateGroup?: () => void;
	}

	let { showHeading = true, columns = 1, onCreateGroup }: Props = $props();

	const gridColsClass = $derived(columns === 2 ? 'md:grid-cols-2' : 'md:grid-cols-1');

	let isLoading = $state(true);
	let loadingBalances = $state(false);
	let error = $state('');
	let misGrupos = $state<GroupSummary[]>([]);
	let didInitializeStatusFilter = $state(false);

	let groupDebtInfo = $state<Record<string, DebtInfo | null>>({});
	let groupMembersCount = $state<Record<string, number>>({});
	let groupTreasury = $state<Record<string, GroupTreasury>>({});
	let loadingMembers = $state(false);
	let loadingTreasury = $state(false);

	let filterRole = $state<FilterRole>('all');
	let filterStatus = $state<FilterStatus>('Active');

	const roleOptions: { val: FilterRole; label: string }[] = [
		{ val: 'all', label: 'Todos' },
		{ val: 'Admin', label: 'Admin' },
		{ val: 'Member', label: 'Miembro' }
	];

	const statusOptions: { val: FilterStatus; label: string; dot: string }[] = [
		{ val: 'all', label: 'Todos', dot: 'bg-foreground' },
		{ val: 'Active', label: 'Activos', dot: 'bg-emerald-500' },
		{ val: 'DebtResolution', label: 'En resolución', dot: 'bg-amber-400' },
		{ val: 'Ended', label: 'Finalizados', dot: 'bg-rose-400' }
	];

	const gruposFiltrados = $derived(
		misGrupos.filter((g) => {
			const roleMatch = filterRole === 'all' || g.role.toLowerCase() === filterRole.toLowerCase();
			const statusMatch =
				filterStatus === 'all' || g.status.toLowerCase() === filterStatus.toLowerCase();
			return roleMatch && statusMatch;
		})
	);

	function groupIcon(group_id: string) {
		const icons = ['✈️', '🏠', '🎉', '💸', '🚀', '☕', '🏝️', '🧾'];
		const seed = group_id.split('').reduce((s, c) => s + c.charCodeAt(0), 0);
		return icons[seed % icons.length];
	}

	function formatBalance(value: number, currency: string) {
		const abs = Math.abs(value);
		const formatted = abs.toLocaleString('en-US', {
			minimumFractionDigits: 2,
			maximumFractionDigits: 2
		});
		return `${value < 0 ? '-' : ''}$${formatted} ${currency}`;
	}

	function getStatusClasses(status: string) {
		if (status.toLowerCase() === 'active')
			return 'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/20 dark:bg-emerald-400/10 dark:text-emerald-300';
		if (status.toLowerCase() === 'debtresolution')
			return 'border-amber-200 bg-amber-50 text-amber-700 dark:border-amber-400/20 dark:bg-amber-400/10 dark:text-amber-300';
		return 'border-rose-200 bg-rose-50 text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300';
	}

	function getStatusDot(status: string) {
		if (status.toLowerCase() === 'active') return 'bg-emerald-500 shadow-emerald-500/30';
		if (status.toLowerCase() === 'debtresolution') return 'bg-amber-400 shadow-amber-400/30';
		return 'bg-rose-400 shadow-rose-400/30';
	}

	function translateStatus(status: string) {
		if (status === 'Active') return 'Activo';
		if (status === 'DebtResolution') return 'En resolución';
		if (status === 'Ended') return 'Finalizado';
		return status;
	}

	function getRoleLabel(role: string) {
		return roleOptions.find((el) => el.val.toLowerCase() === role.toLowerCase())?.label ?? role;
	}

	async function loadGroups() {
		isLoading = true;
		error = '';

		const res = await getMyGroups();

		if (!isSuccess(res)) {
			error = res.message || 'Error al buscar grupos';
			isLoading = false;
			return;
		}

		misGrupos = res.body;
		if (!didInitializeStatusFilter) {
			const hasActiveGroups = misGrupos.some((group) => group.status.toLowerCase() === 'active');
			filterStatus = hasActiveGroups ? 'Active' : 'all';
			didInitializeStatusFilter = true;
		}

		const activeGroups = misGrupos.filter((g) => g.status.toLowerCase() === 'active');
		const debtGroups = misGrupos.filter(
			(g) => g.status === 'DebtResolution' || g.status === 'Ended'
		);

		if (activeGroups.length > 0) {
			loadingMembers = true;
			loadingTreasury = true;

			const memberResults = await Promise.allSettled(
				activeGroups.map((g) => getGroupMembers(g.group_id))
			);
			const newMembersCount: Record<string, number> = {};
			for (let i = 0; i < memberResults.length; i++) {
				const result = memberResults[i];
				const groupId = activeGroups[i].group_id;
				if (result.status === 'fulfilled' && isSuccess(result.value)) {
					newMembersCount[groupId] = result.value.body.length;
				}
			}
			groupMembersCount = newMembersCount;
			loadingMembers = false;

			const treasuryResults = await Promise.allSettled(
				activeGroups.map((g) => getGroupBalances(g.group_id))
			);
			const newTreasury: Record<string, GroupTreasury> = {};
			for (let i = 0; i < treasuryResults.length; i++) {
				const result = treasuryResults[i];
				const groupId = activeGroups[i].group_id;
				if (result.status === 'fulfilled' && isSuccess(result.value)) {
					newTreasury[groupId] = {
						balance: parseBalanceValue(result.value.body.group_balance),
						currency: 'USDC'
					};
				}
			}
			groupTreasury = newTreasury;
			loadingTreasury = false;
		}

		if (debtGroups.length > 0) {
			loadingBalances = true;
			const currentUserId = $authStore.user?.id;
			const newInfo: Record<string, DebtInfo | null> = {};
			for (const g of debtGroups) {
				newInfo[g.group_id] = null;
			}

			const results = await Promise.allSettled(debtGroups.map((g) => getGroupBalances(g.group_id)));

			for (let i = 0; i < results.length; i++) {
				const result = results[i];
				const groupId = debtGroups[i].group_id;
				if (result.status === 'fulfilled' && isSuccess(result.value) && currentUserId) {
					const balances = result.value.body.balances;
					const myEntry = balances.find((b) => b.user_id === currentUserId);
					const myBalance = myEntry ? parseBalanceValue(myEntry.balance) : 0;
					const hasDebtors = balances.some((b) => parseBalanceValue(b.balance) < -0.01);
					newInfo[groupId] = { balance: myBalance, hasDebtors };
				}
			}

			groupDebtInfo = newInfo;
			loadingBalances = false;
		}

		isLoading = false;
	}

	export async function refresh() {
		await loadGroups();
	}

	$effect(() => {
		void loadGroups();
	});
</script>

<div class="flex flex-col gap-4 sm:flex-col sm:justify-between">
	{#if showHeading}
		<div>
			<h2 class="mt-1 text-2xl font-semibold tracking-tight">Mis grupos</h2>
		</div>
	{/if}

	<div class="flex flex-wrap items-center gap-2">
		{#each roleOptions as option (option.val)}
			<button
				type="button"
				onclick={() => (filterRole = option.val)}
				class={filterRole === option.val
					? 'rounded-full bg-foreground px-3 py-1.5 text-xs font-semibold text-background'
					: 'rounded-full border border-border px-3 py-1.5 text-xs font-semibold text-muted-foreground transition hover:bg-muted hover:text-foreground'}
			>
				{option.label}
			</button>
		{/each}
		<span class="mx-1 hidden h-5 w-px bg-border sm:block"></span>
		{#each statusOptions as option (option.val)}
			<button
				type="button"
				onclick={() => (filterStatus = option.val)}
				class={filterStatus === option.val
					? 'inline-flex items-center gap-1.5 rounded-full border border-transparent bg-foreground px-3 py-1.5 text-xs font-semibold text-background'
					: 'inline-flex items-center gap-1.5 rounded-full border border-border px-3 py-1.5 text-xs font-semibold text-muted-foreground transition hover:bg-muted hover:text-foreground'}
			>
				<span class={['size-1.5 rounded-full', option.dot]}></span>
				{option.label}
			</button>
		{/each}
	</div>
</div>

{#if error}
	<div
		class="mt-5 rounded-2xl border border-rose-200 bg-rose-50 p-4 text-sm font-medium text-rose-700 dark:border-rose-400/20 dark:bg-rose-400/10 dark:text-rose-300"
		transition:fade
	>
		{error}
	</div>
{/if}

{#if isLoading}
	<div class="mt-6 grid gap-4 md:grid-cols-2">
		{#each { length: 4 }, index}
			<div
				class="rounded-3xl border border-border bg-background p-5"
				in:fade={{ delay: index * 60 }}
			>
				<div class="h-5 w-2/3 animate-pulse rounded bg-muted"></div>
				<div class="mt-3 h-4 w-full animate-pulse rounded bg-muted"></div>
				<div class="mt-2 h-4 w-4/5 animate-pulse rounded bg-muted"></div>
				<div class="mt-6 grid grid-cols-2 gap-2">
					<div class="h-14 animate-pulse rounded-2xl bg-muted"></div>
					<div class="h-14 animate-pulse rounded-2xl bg-muted"></div>
				</div>
			</div>
		{/each}
	</div>
{:else if gruposFiltrados.length > 0}
	<div class={['mt-6 grid gap-4', gridColsClass]}>
		{#each gruposFiltrados as grupo, index (grupo.group_id)}
			<a
				href={resolve(`/groups/${grupo.group_id}`)}
				class="group relative overflow-hidden rounded-3xl border border-border bg-background p-5 shadow-sm transition hover:-translate-y-1 hover:border-lime-300/80 hover:shadow-2xl hover:shadow-lime-500/10 focus:ring-2 focus:ring-ring focus:outline-none"
				animate:flip={{ duration: 240 }}
				in:fly={{ y: 12, duration: 260, delay: index * 45 }}
			>
				<div
					class="absolute inset-x-0 top-0 h-1 bg-linear-to-r from-lime-300 via-violet-300 to-sky-300 opacity-0 transition group-hover:opacity-100"
				></div>

				<div class="flex items-start justify-between gap-4">
					<div class="flex min-w-0 gap-3">
						<div
							class="flex size-12 shrink-0 items-center justify-center rounded-2xl bg-muted text-2xl shadow-inner"
						>
							{groupIcon(grupo.group_id)}
						</div>
						<div class="min-w-0">
							<h3 class="truncate text-lg font-semibold">{grupo.group_name}</h3>
							<p class="mt-1 line-clamp-2 text-sm text-muted-foreground">
								{grupo.group_description ||
									'Grupo financiero compartido para coordinar gastos, aportes y decisiones.'}
							</p>
						</div>
					</div>

					<div class="flex shrink-0 flex-col items-end gap-2">
						<span
							class={grupo.role.toLowerCase() === 'admin'
								? 'rounded-full bg-violet-500/15 px-2.5 py-1 text-xs font-semibold text-violet-700 dark:text-violet-300'
								: 'rounded-full bg-muted px-2.5 py-1 text-xs font-semibold text-muted-foreground'}
						>
							{getRoleLabel(grupo.role)}
						</span>
						<span
							class={[
								'inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-semibold',
								getStatusClasses(grupo.status)
							]}
						>
							<span class="size-1.5 rounded-full shadow-lg {getStatusDot(grupo.status)}"></span>
							{translateStatus(grupo.status)}
						</span>
					</div>
				</div>

				{#if grupo.status === 'DebtResolution' || grupo.status === 'Ended'}
					<div class="mt-6 rounded-2xl bg-muted/60 p-3">
						<div class="mb-2 flex items-center gap-2 text-xs font-medium text-muted-foreground">
							<Wallet class="size-3.5" />
							Deuda
						</div>
						{#if loadingBalances}
							<p class="h-5 w-24 animate-pulse rounded bg-muted"></p>
						{:else}
							{@const info = groupDebtInfo[grupo.group_id]}
							{#if info === null || info === undefined}
								<p class="font-semibold text-muted-foreground">No disponible</p>
							{:else if info.balance < -0.01}
								<p class="font-semibold text-amber-600">
									Debes {formatBalance(Math.abs(info.balance), 'USDC')}
								</p>
							{:else if info.balance > 0.01 && info.hasDebtors}
								<p class="font-semibold text-emerald-600">
									Te deben {formatBalance(info.balance, 'USDC')}
								</p>
							{:else if info.balance > 0.01}
								<p class="font-semibold text-emerald-600">
									Podés retirar {formatBalance(info.balance, 'USDC')}
								</p>
							{:else}
								<p class="font-semibold text-muted-foreground">Saldado</p>
							{/if}
						{/if}
					</div>
				{:else}
					<div class="mt-6 grid grid-cols-2 gap-3">
						<div class="rounded-2xl bg-muted/60 p-3">
							<div class="mb-2 flex items-center gap-2 text-xs font-medium text-muted-foreground">
								<Users class="size-3.5" />
								Miembros
							</div>
							{#if loadingMembers}
								<p class="h-5 w-16 animate-pulse rounded bg-muted"></p>
							{:else}
								<p class="font-semibold">{groupMembersCount[grupo.group_id] ?? '-'}</p>
							{/if}
						</div>

						<div class="rounded-2xl bg-muted/60 p-3">
							<div class="mb-2 flex items-center gap-2 text-xs font-medium text-muted-foreground">
								<Wallet class="size-3.5" />
								Treasury
							</div>
							{#if loadingTreasury}
								<p class="h-5 w-24 animate-pulse rounded bg-muted"></p>
							{:else}
								{@const treasury = groupTreasury[grupo.group_id]}
								{#if treasury}
									<p class="font-semibold">
										{formatMoney(treasury.balance, treasury.currency)}
									</p>
								{:else}
									<p class="font-semibold text-muted-foreground">No disponible</p>
								{/if}
							{/if}
						</div>
					</div>
				{/if}
			</a>
		{/each}
	</div>
{:else}
	<div
		class="mt-6 rounded-3xl border border-dashed border-border bg-background p-8 text-center"
		transition:scale={{ duration: 220 }}
	>
		<div class="mx-auto flex size-12 items-center justify-center rounded-2xl bg-muted">
			<Wallet class="size-5 text-muted-foreground" />
		</div>
		<h3 class="mt-4 font-semibold">
			{misGrupos.length === 0 ? 'Todavía no tenés grupos' : 'No hay grupos con esos filtros'}
		</h3>
		<p class="mx-auto mt-2 max-w-md text-sm text-muted-foreground">
			{misGrupos.length === 0
				? 'Creá tu primera tesorería compartida para coordinar gastos, propuestas y aportes.'
				: 'Probá cambiar el rol o el estado para encontrar otros grupos.'}
		</p>
		{#if onCreateGroup}
			<button
				type="button"
				onclick={onCreateGroup}
				class="mt-5 inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2 text-sm font-semibold text-background transition hover:bg-foreground/90"
			>
				<Plus class="size-4" />
				Crear grupo
			</button>
		{/if}
	</div>
{/if}
