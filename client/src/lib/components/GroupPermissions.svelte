<script lang="ts">
	import { onMount } from 'svelte';
	import { Shield, Loader2 } from 'lucide-svelte';
	import {
		getGroupPermissions,
		addGroupPermission,
		removeGroupPermission
	} from '$lib/api/endpoints/permissions';
	import { isSuccess } from '$lib/types/client.types';

	interface Props {
		groupId: string;
	}

	let { groupId }: Props = $props();

	let loading = $state(true);
	let error = $state('');
	let saveMessage = $state('');

	let memberActionSet = $state<Set<string>>(new Set());

	const ALL_ACTIONS: { action: string; description: string; category: string }[] = [
		{
			action: 'update_group',
			description: 'Actualizar nombre y descripción del grupo',
			category: 'Grupo'
		},
		{
			action: 'enter_debt_resolution',
			description: 'Iniciar resolución de deudas',
			category: 'Grupo'
		},
		{ action: 'invite_member', description: 'Invitar nuevos miembros al grupo', category: 'Grupo' },
		{
			action: 'cancel_proposal',
			description: 'Cancelar propuestas pendientes',
			category: 'Gobernanza'
		},
		{ action: 'create_fund_round', description: 'Crear rondas de fondeo', category: 'Gobernanza' },
		{
			action: 'cancel_fund_round',
			description: 'Cancelar rondas de fondeo',
			category: 'Gobernanza'
		},
		{
			action: 'create_investment',
			description: 'Crear propuestas de inversión',
			category: 'Inversiones'
		}
	];

	const groupedActions = $derived.by(() => {
		const groups: Record<string, typeof ALL_ACTIONS> = {};
		for (const a of ALL_ACTIONS) {
			if (!groups[a.category]) groups[a.category] = [];
			groups[a.category].push(a);
		}
		return Object.entries(groups);
	});

	function getCategoryIcon(category: string): string {
		const icons: Record<string, string> = {
			Grupo: '🏠',
			Gobernanza: '🗳️',
			Inversiones: '📈'
		};
		return icons[category] || '🔘';
	}

	async function load() {
		loading = true;
		error = '';
		try {
			const res = await getGroupPermissions(groupId);
			if (!isSuccess(res)) {
				error = 'No se pudieron cargar los permisos.';
				loading = false;
				return;
			}

			const memberPerms = res.body.roles.find((r) => r.role === 'Member');
			memberActionSet = new Set(memberPerms?.permissions.map((p) => p.action) ?? []);
		} catch (e) {
			console.error(e);
			error = 'Error cargando permisos.';
		} finally {
			loading = false;
		}
	}

	async function toggle(action: string) {
		const current = memberActionSet.has(action);

		// optimistic
		const next = new Set(memberActionSet);
		if (current) {
			next.delete(action);
		} else {
			next.add(action);
		}
		memberActionSet = next;

		try {
			let res;
			if (current) {
				res = await removeGroupPermission(groupId, action);
			} else {
				res = await addGroupPermission(groupId, action);
			}

			if (!isSuccess(res)) {
				throw new Error(res.message || 'Error al guardar');
			}

			saveMessage = 'Guardado';
			setTimeout(() => {
				if (saveMessage === 'Guardado') saveMessage = '';
			}, 1400);
		} catch (e) {
			// revert
			const reverted = new Set(memberActionSet);
			if (current) {
				reverted.add(action);
			} else {
				reverted.delete(action);
			}
			memberActionSet = reverted;
			error = 'No se pudo guardar el cambio. Intenta de nuevo.';
			setTimeout(() => (error = ''), 2500);
			console.error('perm toggle failed', e);
		}
	}

	onMount(() => {
		load();
	});
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-start gap-3">
		<div
			class="mt-0.5 flex h-9 w-9 items-center justify-center rounded-2xl border border-border bg-muted/50 text-muted-foreground"
		>
			<Shield class="h-4 w-4" />
		</div>
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2">
				<h3 class="text-base font-semibold tracking-tight text-foreground">Permisos del grupo</h3>
				<span
					class="rounded-full border border-border bg-muted px-2.5 py-0.5 text-[10px] font-medium text-muted-foreground"
				>
					Miembros
				</span>
			</div>
			<p class="mt-1 text-sm text-muted-foreground">
				Elegí qué acciones pueden realizar los miembros del grupo. Los administradores pueden hacer
				todo. Los cambios se guardan automáticamente.
			</p>
		</div>
	</div>

	{#if loading}
		<div class="rounded-3xl border border-border bg-card p-8">
			<div class="flex flex-col items-center gap-3 text-muted-foreground">
				<Loader2 class="h-5 w-5 animate-spin" />
				<p class="text-sm">Cargando permisos...</p>
			</div>
		</div>
	{:else if error && memberActionSet.size === 0}
		<div
			class="rounded-3xl border border-red-200 bg-red-50 p-5 text-sm text-red-700 dark:border-red-400/30 dark:bg-red-400/10 dark:text-red-300"
		>
			{error}
		</div>
	{:else}
		{#each groupedActions as [category, actions] (category)}
			<div class="overflow-hidden rounded-3xl border border-border bg-card shadow-sm">
				<!-- Category header -->
				<div
					class="border-b border-border bg-muted/30 px-5 py-3 text-xs font-medium tracking-wide text-muted-foreground"
				>
					{getCategoryIcon(category)}
					{category}
				</div>

				<!-- Actions for this category -->
				<div class="divide-y divide-border/60">
					{#each actions as act (act.action)}
						{@const isOn = memberActionSet.has(act.action)}

						<div
							class="grid grid-cols-[1fr_60px] items-center px-5 py-3.5 transition-colors hover:bg-muted/20"
						>
							<div class="pr-4">
								<p class="text-sm leading-tight font-medium text-foreground">
									{act.description}
								</p>
							</div>

							<div class="flex justify-center">
								<button
									type="button"
									class="group relative flex h-5 w-9 cursor-pointer items-center rounded-full border border-border bg-muted p-0.5 transition-all hover:border-border/80 focus:outline-none focus-visible:ring-2 focus-visible:ring-ring/50 active:scale-[0.985] {isOn
										? 'border-emerald-500/90 bg-emerald-500/90'
										: ''}"
									onclick={() => toggle(act.action)}
									aria-pressed={isOn}
									aria-label={act.description}
								>
									<span
										class="block h-3.5 w-3.5 rounded-full bg-white shadow-sm transition-all duration-150 {isOn
											? 'translate-x-4'
											: 'translate-x-0 group-hover:scale-105'}"
									></span>
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/each}

		<!-- Feedback -->
		<div class="flex items-center justify-between px-1 text-xs">
			<p class="text-muted-foreground">Los cambios se aplican de inmediato.</p>

			<div class="min-h-[1rem]">
				{#if saveMessage}
					<span
						class="inline-flex items-center gap-1 rounded-full bg-emerald-500/10 px-2.5 py-0.5 font-medium text-emerald-600 dark:bg-emerald-400/10 dark:text-emerald-400"
					>
						✓ {saveMessage}
					</span>
				{:else if error}
					<span class="text-red-600 dark:text-red-400">{error}</span>
				{/if}
			</div>
		</div>
	{/if}
</div>
