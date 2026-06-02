<script lang="ts" module>
	export type SidebarItem = {
		label: string;
		icon: typeof LayoutDashboard;
		link: string;
		active: boolean;
	};
</script>

<script lang="ts">
	import AsideMenu from './AsideMenu.svelte';
	import { page } from '$app/state';

	import { Activity, LayoutDashboard, Landmark, ReceiptText, Settings, Users } from 'lucide-svelte';

	const baseSidebarItems: {
		label: string;
		icon: typeof LayoutDashboard;
		link: string;
	}[] = [
		{ label: 'Dashboard', icon: LayoutDashboard, link: '/dashboard' },
		{ label: 'Grupos', icon: Users, link: '/dashboard/groups' },
		{ label: 'Expenses', icon: ReceiptText, link: '/dashboard/expenses' },
		{ label: 'Governance', icon: Activity, link: '/dashboard/governance' },
		{ label: 'Treasury', icon: Landmark, link: '/dashboard/treasury' },
		{ label: 'Settings', icon: Settings, link: '/dashboard/settings' }
	];

	let sidebarItems = $derived(
		baseSidebarItems.map((item) => ({
			...item,
			active: page.url.pathname === item.link
		}))
	);

	let { children } = $props();
</script>

<div class="min-h-screen bg-background text-foreground">
	<div
		class="pointer-events-none fixed inset-0 -z-10 bg-[radial-gradient(circle_at_top_left,rgba(163,230,53,0.18),transparent_32%),radial-gradient(circle_at_90%_10%,rgba(168,85,247,0.14),transparent_28%)]"
	></div>

	<div
		class="mx-auto grid w-full max-w-7xl gap-6 px-4 pt-28 pb-10 sm:px-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:px-8"
	>
		<AsideMenu {sidebarItems} />

		{@render children()}
	</div>
</div>
