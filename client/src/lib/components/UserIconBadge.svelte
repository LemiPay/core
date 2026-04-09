<script lang="ts">
	import { CircleUser } from 'lucide-svelte';
	import type { UserBadge } from '$lib/types/endpoints/auth.types';

	let { user }: { user: UserBadge } = $props();

	const isAdmin = $derived(user.role === 'Admin');
</script>

<a href="/users/{user.user_id}">
	<div class="group relative flex w-max cursor-pointer justify-center">
		<CircleUser
			class="h-6 w-6 text-gray-700 transition-colors hover:text-black"
			strokeWidth={isAdmin ? 2.5 : 2}
		/>

		<div
			class="pointer-events-none invisible absolute top-full z-50 mt-2 rounded-md bg-[#222327] px-3 py-1.5 text-sm font-medium whitespace-nowrap text-white opacity-0 shadow-sm transition-all duration-200 group-hover:visible group-hover:opacity-100"
		>
			{user.name}

			{#if isAdmin}
				<span class="ml-1 text-xs font-medium text-gray-400">(Admin)</span>
			{/if}
		</div>
	</div>
</a>
