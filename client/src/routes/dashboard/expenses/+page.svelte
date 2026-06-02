<script>
	import DashboardLayout from '../DashboardLayout.svelte';
	import DiaTextReveal from '$lib/components/magic/dia-text-reveal/dia-text-reveal.svelte';
</script>

<DashboardLayout>
	<div class="flex min-h-40 items-center justify-center">
		<DiaTextReveal
			duration={5}
			text="Coming Soon..."
			class="font-figtree pb-1 text-6xl font-semibold tracking-normal sm:text-8xl"
		/>
	</div>
</DashboardLayout>
