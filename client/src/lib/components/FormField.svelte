<script lang="ts">
	interface Props {
		id: string;
		label: string;
		type?: string;
		placeholder?: string;
		minLength: number;
		maxLength: number;
		value: string;
		rows?: number;
		attempted?: boolean;
	}

	let {
		id,
		label,
		type = 'text',
		placeholder = '',
		minLength,
		maxLength,
		value = $bindable(''),
		rows = 3,
		attempted = false
	}: Props = $props();

	let touched = $state(false);

	const showFeedback = $derived(touched || attempted);
	const len = $derived(value.trim().length);
	const isValid = $derived(len >= minLength && len <= maxLength);

	const message = $derived(
		len === 0
			? `${label} is required`
			: len < minLength
				? `${minLength - len} more character${minLength - len === 1 ? '' : 's'} needed`
				: len > maxLength
					? `${len - maxLength} character${len - maxLength === 1 ? '' : 's'} over the limit`
					: 'Looks good!'
	);
</script>

<div>
	<label for={id} class="mb-1.5 block text-sm font-medium text-black">{label}</label>

	{#if type === 'textarea'}
		<textarea
			{id}
			bind:value
			{placeholder}
			{rows}
			onblur={() => (touched = true)}
			class="w-full resize-none rounded-md border px-3 py-2 text-sm text-black placeholder-gray-400 transition focus:outline-none focus:ring-0
				{showFeedback
				? isValid
					? 'border-green-400 focus:border-green-500'
					: 'border-red-400 focus:border-red-500'
				: 'border-gray-200 focus:border-gray-400'}"
		></textarea>
	{:else}
		<input
			{id}
			{type}
			bind:value
			{placeholder}
			onblur={() => (touched = true)}
			class="w-full rounded-md border px-3 py-2 text-sm text-black placeholder-gray-400 transition focus:outline-none focus:ring-0
				{showFeedback
				? isValid
					? 'border-green-400 focus:border-green-500'
					: 'border-red-400 focus:border-red-500'
				: 'border-gray-200 focus:border-gray-400'}"
		/>
	{/if}

	{#if showFeedback}
		<div
			class="mt-1.5 flex items-center justify-between text-xs
				{isValid ? 'text-green-600' : 'text-red-500'}"
		>
			<span class="flex items-center gap-1">
				{#if isValid}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-3.5 w-3.5 shrink-0"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<polyline points="20 6 9 17 4 12" />
					</svg>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-3.5 w-3.5 shrink-0"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				{/if}
				{message}
			</span>
			<span class="{len > maxLength ? 'text-red-500' : 'text-gray-400'} tabular-nums">
				{len}/{maxLength}
			</span>
		</div>
	{/if}
</div>