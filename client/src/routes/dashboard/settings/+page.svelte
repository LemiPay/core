<script lang="ts">
	import { Settings } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import DashboardLayout from '../DashboardLayout.svelte';
	import NotificationPreferences from '$lib/components/NotificationPreferences.svelte';
</script>

<svelte:head>
	<title>Lemipay – Configuración</title>
</svelte:head>

<DashboardLayout>
	<section class="space-y-6">
		<div
			class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
			in:fly={{ y: 12, duration: 360 }}
		>
			<div>
				<p class="text-sm font-medium text-muted-foreground">Tu cuenta</p>
				<h1 class="mt-1 text-3xl font-semibold tracking-tight">Configuración</h1>
				<p class="mt-2 max-w-xl text-sm text-muted-foreground">
					Elegí qué notificaciones recibir y por qué medio. Los cambios se guardan automáticamente.
				</p>
			</div>
			<div
				class="flex size-10 items-center justify-center rounded-2xl bg-lime-400/15 text-lime-700 dark:text-lime-300"
			>
				<Settings class="size-4" />
			</div>
		</div>

		<section class="rounded-[2rem] border border-border bg-card p-5 shadow-sm sm:p-6">
			<NotificationPreferences embedded />
		</section>
	</section>
</DashboardLayout>
