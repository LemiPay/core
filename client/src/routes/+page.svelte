<script lang="ts">
	import { browser } from '$app/environment';
	import { isAuthenticated } from '$lib/stores/store';
	import AuthLayout from '$lib/components/AuthLayout.svelte';
	import { walletAddress, wagmiConfig } from '$lib/stores/appkit';
	//import { web3AuthState } from '$lib/stores/web3Auth';
	//import { runWeb3AuthFlow } from '$lib/wallet/web3AuthFlow';
	import { resolve } from '$app/paths';

	$effect(() => {
		if (!browser) return;
		const addr = $walletAddress;
		const cfg = wagmiConfig;
		if (!addr || !cfg) {
			//web3AuthState.set({ status: 'idle' });
			return;
		}
		const ac = new AbortController();
		//void runWeb3AuthFlow(addr, cfg, ac.signal);
		return () => ac.abort();
	});

	async function logoutUser() {
		localStorage.removeItem('token');
		window.location.href = '/';
	}

	const me = 'dsada0d9sa-d8asd89sa-d3nnd3-da9d9sa';
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<AuthLayout
	title="Lemipay"
	description="Manage your group expenses in a decentralized and transparent way."
>
	<div class="mb-6 flex flex-col gap-3">
		<a
			href={resolve('/login')}
			class="w-full rounded-md bg-black px-4 py-2 text-center font-medium text-white transition hover:bg-gray-800"
		>
			Log in
		</a>

		<a
			href={resolve('/register')}
			class="w-full rounded-md border border-gray-300 bg-white px-4 py-2 text-center font-medium text-black transition hover:bg-gray-50"
		>
			Create account
		</a>

		{#if $isAuthenticated}
			<button
				onclick={logoutUser}
				class="w-full rounded-md border border-red-200 bg-white px-4 py-2 font-medium text-red-500 transition hover:border-red-400 hover:bg-red-50"
			>
				Log out
			</button>
		{/if}
	</div>

	{#if $isAuthenticated}
		<div class="rounded-lg border border-gray-200 bg-gray-50 p-3 text-center">
			<p class="text-sm font-medium text-black">
				You are currently logged in as {me}
			</p>
		</div>
	{/if}

	<p class="max-w-xs overflow-hidden text-sm text-wrap text-ellipsis text-gray-500">
		{$walletAddress ?? 'Not connected'}
	</p>

	<!--
	{#if $web3AuthState.status === 'signing'}
		<p class="text-sm text-amber-700">Confirm the signature in your wallet…</p>
	{:else if $web3AuthState.status === 'posting'}
		<p class="text-sm text-amber-700">Talking to the server…</p>
	{:else if $web3AuthState.status === 'success'}
		<p class="text-sm font-medium text-green-800">
			{$web3AuthState.isNew
				? 'New wallet — welcome.'
				: 'Welcome back — this wallet was already registered.'}
		</p>
	{:else if $web3AuthState.status === 'error'}
		<p class="text-sm text-red-600">{$web3AuthState.message}</p>
	{/if}
	-->

	<div class="rounded-lg border border-gray-200 bg-gray-50 p-3 text-center">
		<appkit-button class="mx-auto"></appkit-button>
	</div>
</AuthLayout>
