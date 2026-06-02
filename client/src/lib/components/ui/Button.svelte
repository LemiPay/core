<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'primary' | 'secondary' | 'danger' | 'ghost';
	type Size = 'sm' | 'md' | 'lg';
	type ButtonType = 'button' | 'submit' | 'reset';

	interface Props {
		label: string;
		icon?: Snippet;
		onclick?: () => void;
		class?: string;
		variant?: Variant;
		size?: Size;
		type?: ButtonType;
		disabled?: boolean;
		loading?: boolean;
		fullWidth?: boolean;
		form?: string;
	}

	let {
		label,
		icon,
		onclick,
		class: customClass = '',
		variant = 'primary',
		size = 'md',
		type = 'button',
		disabled = false,
		loading = false,
		fullWidth = false,
		form = undefined
	}: Props = $props();

	const variantClasses: Record<Variant, string> = {
		primary:
			'bg-primary text-primary-foreground border border-transparent hover:bg-primary/90 active:bg-primary/80',
		secondary:
			'bg-background text-foreground border border-input hover:bg-accent hover:text-accent-foreground active:bg-accent/80',
		danger:
			'bg-background text-red-600 border border-red-200 hover:bg-red-50 hover:border-red-400 active:bg-red-100 dark:text-red-300 dark:border-red-400/20 dark:hover:bg-red-400/10 dark:hover:border-red-400/40 dark:active:bg-red-400/15',
		ghost:
			'bg-transparent text-muted-foreground border border-transparent hover:bg-accent hover:text-accent-foreground active:bg-accent/80'
	};

	const sizeClasses: Record<Size, string> = {
		sm: 'px-3 py-1.5 text-xs',
		md: 'px-4 py-2 text-sm',
		lg: 'px-5 py-2.5 text-base'
	};

	const classes = $derived(
		[
			'inline-flex items-center justify-center gap-2 rounded-md font-medium transition',
			'focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background',
			'disabled:pointer-events-none disabled:opacity-40 cursor-pointer',
			variantClasses[variant],
			sizeClasses[size],
			fullWidth ? 'w-full' : ''
		]
			.filter(Boolean)
			.join(' ')
	);
</script>

<button
	{type}
	{form}
	disabled={disabled || loading}
	aria-busy={loading}
	class={classes + ' ' + customClass}
	onclick={!disabled && !loading ? onclick : undefined}
>
	{#if loading}
		<svg class="h-3.5 w-3.5 animate-spin" viewBox="0 0 24 24" fill="none" aria-hidden="true">
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
			<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z" />
		</svg>
	{:else if icon}
		<span class="h-4 w-4" aria-hidden="true">
			{@render icon()}
		</span>
	{/if}
	{label}
</button>
