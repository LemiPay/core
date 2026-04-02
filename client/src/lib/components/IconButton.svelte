<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'primary' | 'secondary' | 'danger' | 'ghost';
	type Size = 'sm' | 'md' | 'lg';
	type ButtonType = 'button' | 'submit' | 'reset';
	type Rounded = 'md' | 'full';

	interface Props {
		icon: Snippet;
		ariaLabel: string;
		onclick?: () => void;
		variant?: Variant;
		size?: Size;
		type?: ButtonType;
		rounded?: Rounded;
		disabled?: boolean;
		loading?: boolean;
	}

	let {
		icon,
		ariaLabel,
		onclick,
		variant = 'ghost',
		size = 'md',
		type = 'button',
		rounded = 'md',
		disabled = false,
		loading = false
	}: Props = $props();

	const variantClasses: Record<Variant, string> = {
		primary: 'bg-black text-white border border-transparent hover:bg-gray-800 active:bg-gray-900',
		secondary: 'bg-white text-black border border-gray-300 hover:bg-gray-50 active:bg-gray-100',
		danger:
			'bg-white text-red-500 border border-red-200 hover:bg-red-50 hover:border-red-400 active:bg-red-100',
		ghost:
			'bg-transparent text-gray-500 border border-transparent hover:bg-gray-100 hover:text-black active:bg-gray-200'
	};

	const sizeClasses: Record<Size, string> = {
		sm: 'h-7 w-7',
		md: 'h-9 w-9',
		lg: 'h-11 w-11'
	};

	const iconSizeClasses: Record<Size, string> = {
		sm: 'h-3.5 w-3.5',
		md: 'h-4 w-4',
		lg: 'h-5 w-5'
	};

	const classes = $derived(
		[
			'inline-flex items-center justify-center transition cursor-pointer',
			'focus:outline-none focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2',
			'disabled:pointer-events-none disabled:opacity-40',
			rounded === 'full' ? 'rounded-full' : 'rounded-md',
			variantClasses[variant],
			sizeClasses[size]
		]
			.filter(Boolean)
			.join(' ')
	);
</script>

<button
	{type}
	disabled={disabled || loading}
	aria-label={ariaLabel}
	aria-busy={loading}
	class={classes}
	onclick={!disabled && !loading ? onclick : undefined}
>
	{#if loading}
		<svg
			class="{iconSizeClasses[size]} animate-spin"
			viewBox="0 0 24 24"
			fill="none"
			aria-hidden="true"
		>
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
			<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z" />
		</svg>
	{:else}
		<span class="{iconSizeClasses[size]} flex items-center justify-center" aria-hidden="true">
			{@render icon()}
		</span>
	{/if}
</button>
