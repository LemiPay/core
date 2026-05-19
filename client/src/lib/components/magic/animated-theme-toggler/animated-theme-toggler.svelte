<script lang="ts">
	import { onMount } from 'svelte';
	import { Moon, Sun } from '@lucide/svelte';
	import { cn } from '$lib/utils';

	interface AnimatedThemeTogglerProps {
		class?: string;
		duration?: number;
	}

	let { class: className, duration = 1500, ...props }: AnimatedThemeTogglerProps = $props();

	const THEME_KEY = 'theme';
	const DARK_THEME = 'dark';
	const LIGHT_THEME = 'light';

	let isDark = $state(false);

	function getPreferredTheme(): boolean {
		const savedTheme = localStorage.getItem(THEME_KEY);
		if (savedTheme === DARK_THEME) return true;
		if (savedTheme === LIGHT_THEME) return false;
		return window.matchMedia('(prefers-color-scheme: dark)').matches;
	}

	function applyTheme(nextIsDark: boolean) {
		isDark = nextIsDark;
		document.documentElement.classList.toggle('dark', nextIsDark);
	}

	onMount(() => {
		const updateTheme = () => {
			isDark = document.documentElement.classList.contains('dark');
		};

		applyTheme(getPreferredTheme());

		const observer = new MutationObserver(updateTheme);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		return () => observer.disconnect();
	});

	const toggleTheme = async (event: MouseEvent) => {
		const button = event.currentTarget;
		if (!(button instanceof HTMLButtonElement)) return;

		// Check if View Transition API is supported
		if (!document.startViewTransition) {
			// Fallback for browsers that don't support View Transition API
			const newTheme = !isDark;
			applyTheme(newTheme);
			localStorage.setItem(THEME_KEY, newTheme ? DARK_THEME : LIGHT_THEME);
			return;
		}

		await document.startViewTransition(() => {
			const newTheme = !isDark;
			applyTheme(newTheme);
			localStorage.setItem(THEME_KEY, newTheme ? DARK_THEME : LIGHT_THEME);
		}).ready;

		const { top, left, width, height } = button.getBoundingClientRect();
		const x = left + width / 2;
		const y = top + height / 2;
		const maxRadius = Math.hypot(
			Math.max(left, window.innerWidth - left),
			Math.max(top, window.innerHeight - top)
		);

		document.documentElement.animate(
			{
				clipPath: [`circle(0px at ${x}px ${y}px)`, `circle(${maxRadius}px at ${x}px ${y}px)`]
			},
			{
				duration,
				easing: 'ease-in-out',
				pseudoElement: '::view-transition-new(root)'
			}
		);
	};
</script>

<button
	onclick={toggleTheme}
	class={cn(className) +
		' rounded-full p-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground'}
	{...props}
>
	{#if isDark}
		<Sun size={20} />
	{:else}
		<Moon size={20} />
	{/if}
	<span class="sr-only">Toggle theme</span>
</button>
