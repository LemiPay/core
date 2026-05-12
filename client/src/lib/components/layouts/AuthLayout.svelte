<script lang="ts">
	const { title, description, children } = $props();
</script>

<div class="flex min-h-screen items-center justify-center bg-background p-4 text-foreground">
	<div
		class="w-full max-w-md rounded-xl border border-border bg-card p-8 text-card-foreground shadow-sm shadow-black/5 dark:shadow-none"
	>
		<div class="mb-6 space-y-2">
			<h2 class="text-2xl font-bold tracking-tight text-foreground">{title}</h2>
			<p class="text-sm text-muted-foreground">{description}</p>
		</div>

		{@render children()}
	</div>
</div>
