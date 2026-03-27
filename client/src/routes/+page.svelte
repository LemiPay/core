<script lang="ts">
	import { isAuthenticated } from '$lib/stores/store';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

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
			href="/login"
			class="w-full rounded-md bg-black px-4 py-2 text-center font-medium text-white transition hover:bg-gray-800"
		>
			Log in
		</a>

		<a
			href="/register"
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
</AuthLayout>
