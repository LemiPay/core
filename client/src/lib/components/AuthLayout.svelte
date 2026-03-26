<script lang="ts">
	const { title, description, children } = $props();
</script>

<div class="flex min-h-screen items-center justify-center bg-white p-4">
	<div class="w-full max-w-md rounded-xl border border-gray-200 p-8 shadow-sm">
		<div class="mb-6 space-y-2">
			<h2 class="text-2xl font-bold tracking-tight text-black">{title}</h2>
			<p class="text-sm text-gray-500">{description}</p>
		</div>

		{@render children()}
	</div>
</div>
