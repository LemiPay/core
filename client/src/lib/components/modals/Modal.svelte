<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		open: boolean;
		title: string;
		description?: string;
		onclose: () => void;
		children: Snippet;
		footer?: Snippet;
		error?: string;
		success?: string;
		loading?: boolean;
		panelClass?: string;
	}

	const {
		open,
		title,
		description,
		onclose,
		children,
		footer,
		error,
		success,
		loading = false,
		panelClass = ''
	}: Props = $props();

	function handleBackdropClick(e: MouseEvent) {
		if (loading) return;
		if (e.target === e.currentTarget) onclose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (loading) return;
		if (e.key === 'Escape') onclose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop -->
	<div
		role="presentation"
		class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto bg-black/50 p-4 py-6 backdrop-blur-sm sm:items-center sm:py-8"
		onclick={handleBackdropClick}
	>
		<!-- Panel -->
		<div
			role="dialog"
			aria-modal="true"
			aria-labelledby="modal-title"
			aria-busy={loading}
			tabindex="-1"
			class={[
				'my-auto w-full rounded-xl border border-border bg-card p-6 text-card-foreground shadow-xl shadow-black/10 sm:p-8 dark:shadow-black/30',
				panelClass || 'max-w-md'
			]}
		>
			<!-- Header -->
			<div class="mb-6 flex items-start justify-between gap-4">
				<div class="space-y-1">
					<h2 id="modal-title" class="text-xl font-bold tracking-tight text-foreground">{title}</h2>
					{#if description}
						<p class="text-sm text-muted-foreground">{description}</p>
					{/if}
				</div>
				<button
					onclick={onclose}
					disabled={loading}
					class="mt-0.5 rounded-md p-1 text-muted-foreground transition hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40"
					aria-label="Close modal"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>

			<!-- Body / Status / Footer -->
			<div class="relative">
				{#if loading}
					<div
						class="absolute inset-0 z-10 flex items-center justify-center rounded-lg bg-background/70 backdrop-blur-sm"
					>
						<svg class="h-5 w-5 animate-spin text-foreground" viewBox="0 0 24 24" fill="none">
							<circle
								class="opacity-20"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="3"
							/>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 22 6.477 22 12h-4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							/>
						</svg>
					</div>
				{/if}

				<div class={loading ? 'pointer-events-none opacity-40 select-none' : ''}>
					<!-- Body -->
					<div class="space-y-4">
						{@render children()}
					</div>

					<!-- Status message -->
					{#if error}
						<div
							class="mt-4 flex items-center gap-2 rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-sm text-red-600 dark:border-red-400/20 dark:bg-red-400/10 dark:text-red-300"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-4 w-4 shrink-0"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<circle cx="12" cy="12" r="10" />
								<line x1="12" y1="8" x2="12" y2="12" />
								<line x1="12" y1="16" x2="12.01" y2="16" />
							</svg>
							{error}
						</div>
					{:else if success}
						<div
							class="mt-4 flex items-center gap-2 rounded-md border border-green-200 bg-green-50 px-3 py-2.5 text-sm text-green-600 dark:border-green-400/20 dark:bg-green-400/10 dark:text-green-300"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-4 w-4 shrink-0"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
								<polyline points="22 4 12 14.01 9 11.01" />
							</svg>
							{success}
						</div>
					{/if}

					<!-- Footer -->
					{#if footer}
						<div class="mt-6 flex justify-end gap-2">
							{@render footer()}
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
