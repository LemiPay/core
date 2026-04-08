<script lang="ts">
	import { page } from '$app/state';
	import NotificationDropdown from '$lib/components/NotificationDropdown.svelte';

	interface NavLink {
		label: string;
		href: string;
	}

	const links: NavLink[] = [
		{ label: 'Home', href: '/' },
		{ label: 'Dashboard', href: '/dashboard' }
	];
</script>

<nav class="w-full border-b border-gray-200 bg-white">
	<div class="mx-auto flex max-w-5xl items-center justify-between px-6 py-3">
		<a href="/" class="flex items-center">
			<img src="/logo.png" alt="Lemipay" class="h-8 w-auto" />

			<span class="ml-2 text-sm font-bold tracking-tight text-black"> LemiPay </span>
		</a>

		<div class="flex items-center gap-6">
			<div class="flex items-center gap-6">
				{#each links as link}
					{@const isActive = page.url.pathname === link.href}
					<a
						href={link.href}
						class={[
							'text-sm font-medium transition-colors',
							isActive
								? 'border-b-2 border-black pb-0.5 text-black'
								: 'text-gray-500 hover:text-black'
						].join(' ')}
					>
						{link.label}
					</a>
				{/each}
			</div>

			<div class="h-4 w-px bg-gray-300"></div>

			<NotificationDropdown />
		</div>
	</div>
</nav>
