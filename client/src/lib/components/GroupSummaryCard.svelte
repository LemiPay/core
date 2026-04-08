<script lang="ts">
	import type { GroupSummary } from '$lib/types/endpoints/groups.types';
	let { group }: { group: GroupSummary } = $props();

	// Creamos una variable derivada que calcula el color correcto.
	// Usamos toLowerCase() por las dudas de que el backend mande "active" o "Active".
	let statusClasses = $derived(
		group.status.toLowerCase() === 'active'
			? 'bg-green-50 text-green-700 border border-green-200'
			: group.status.toLowerCase() === 'ended'
				? 'bg-red-50 text-red-600 border border-red-200'
				: 'bg-gray-50 text-gray-600 border border-gray-200'
	);

	let dotClass = $derived(
		group.status.toLowerCase() === 'active'
			? 'bg-green-500'
			: group.status.toLowerCase() === 'ended'
				? 'bg-red-400'
				: 'bg-gray-400'
	);
</script>

<a
	href="/groups/{group.group_id}"
	class="block w-full rounded-md border border-gray-300 bg-white p-4 shadow-sm transition hover:border-gray-400 hover:bg-gray-50 focus:ring-2 focus:ring-black focus:outline-none"
>
	<div class="flex items-start justify-between">
		<div class="flex flex-col gap-1 pr-4">
			<h3 class="text-lg font-semibold text-black">{group.group_name}</h3>
			<p class="line-clamp-2 text-sm text-gray-500">{group.group_description}</p>
		</div>

		<div class="flex flex-col items-end gap-2">
			<span class="rounded bg-black px-2.5 py-0.5 text-xs font-medium text-white">
				{group.role}
			</span>

			<span
				class="flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium {statusClasses}"
			>
				<span class="h-1.5 w-1.5 rounded-full {dotClass}"></span>
				{group.status}
			</span>
		</div>
	</div>
</a>
