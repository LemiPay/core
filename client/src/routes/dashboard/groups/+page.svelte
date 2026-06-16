<script lang="ts">
	import { Plus, Users } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import DashboardLayout from '../DashboardLayout.svelte';
	import DashboardGroupsList from '$lib/components/dashboard/DashboardGroupsList.svelte';
	import NewGroup from '$lib/components/modals/group/NewGroup.svelte';

	let showNewGroup = $state(false);
</script>

<svelte:head>
	<title>Lemipay – Grupos</title>
</svelte:head>

<NewGroup onclose={() => (showNewGroup = false)} open={showNewGroup} />

<DashboardLayout>
	<section class="space-y-6">
		<div
			class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
			in:fly={{ y: 12, duration: 360 }}
		>
			<div>
				<p class="text-sm font-medium text-muted-foreground">Tus tesorerías</p>
				<h1 class="mt-1 text-3xl font-semibold tracking-tight">Grupos</h1>
				<p class="mt-2 max-w-xl text-sm text-muted-foreground">
					Todos los grupos a los que pertenecés, con su rol, estado y resumen financiero.
				</p>
			</div>
			<div
				class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
			>
				<Users class="size-4" />
			</div>
		</div>

		<section class="rounded-[2rem] border border-border bg-card p-5 shadow-sm sm:p-6">
			<DashboardGroupsList
				showHeading={false}
				columns={2}
				onCreateGroup={() => (showNewGroup = true)}
			/>
		</section>
	</section>

	<div class="group fixed right-5 bottom-5 z-40 flex items-center gap-3">
		<div
			class="pointer-events-none hidden translate-x-2 rounded-2xl border border-border bg-card px-3 py-2 text-sm font-semibold opacity-0 shadow-xl transition group-hover:translate-x-0 group-hover:opacity-100 sm:block"
		>
			Crear grupo
		</div>
		<button
			aria-label="Crear grupo"
			class="flex h-14 w-14 items-center justify-center rounded-full bg-foreground text-background shadow-2xl ring-4 shadow-lime-500/20 ring-lime-400/10 transition hover:scale-105 hover:shadow-lime-500/30 focus:ring-2 focus:ring-ring focus:outline-none active:scale-95"
			onclick={() => (showNewGroup = true)}
			type="button"
		>
			<Plus class="size-6 transition group-hover:rotate-90" />
		</button>
	</div>
</DashboardLayout>
