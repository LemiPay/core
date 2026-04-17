<script lang="ts">
	import { page } from '$app/state';
	import { FileQuestion, AlertCircle, ArrowLeft } from 'lucide-svelte';
	const isNotFound = page.status === 404;
	const isInternalError = page.status === 500;
</script>

<svelte:head>
	<title>{page.status} — Lemipay</title>
</svelte:head>

<div class="flex min-h-[80vh] items-center justify-center px-4 font-sans">
	<div
		class="w-full max-w-md rounded-2xl border border-gray-200 bg-white p-8 text-center shadow-sm"
	>
		<div
			class="mx-auto mb-6 flex h-14 w-14 items-center justify-center rounded-full border border-gray-100 bg-gray-50"
		>
			{#if isNotFound}
				<FileQuestion class="h-7 w-7 text-black" strokeWidth={1.5} />
			{:else}
				<AlertCircle class="h-7 w-7 text-black" strokeWidth={1.5} />
			{/if}
		</div>

		<h1 class="mb-2 text-xl font-bold text-black">
			{#if isNotFound}
				Página no encontrada
			{:else if isInternalError}
				Error DB
			{:else}
				Algo salió mal
			{/if}
		</h1>

		<p class="text-sm text-gray-500">
			{#if isNotFound}
				La ruta a la que intentás acceder no existe o fue movida.
			{:else if isInternalError}
				La base de datos debe estar apagada o rota
			{:else}
				{page.error?.message ?? 'Ocurrió un error inesperado al procesar tu solicitud.'}
			{/if}
		</p>

		<div class="my-16 flex items-center justify-center">
			<span
				class="inline-block rounded-full border border-gray-200 bg-gray-50 px-6 py-2 font-mono text-base text-gray-600"
			>
				{page.status}
			</span>
		</div>

		<a
			href="/"
			class="flex w-full items-center justify-center gap-2 rounded-lg bg-black px-4 py-3 text-sm font-medium text-white transition-colors hover:bg-gray-800"
		>
			<ArrowLeft class="h-4 w-4" />
			Volver al inicio
		</a>
	</div>
</div>
