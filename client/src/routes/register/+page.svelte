<script lang="ts">
	import { register } from '$lib/api/auth';
	import { redirect } from '@sveltejs/kit';
	import type { FailedResponse } from '$lib/types/auth.types';
	import { goto } from '$app/navigation';

	let data = $state({
		name: '',
		email: '',
		password: ''
	});

	// true: loading, false: not yet loading, null: end
	let status: boolean | null = $state(false);
	let error = $state('');

	async function create_user() {
		status = true;
		const res = await register(data);

		if (res.status !== 200) {
			const x: FailedResponse = res as FailedResponse;
			const msg = x.message || 'An error occurred while registering.';
			error = error + msg;
			status = null;
			return;
		}

		status = null;
		data = {
			name: '',
			email: '',
			password: ''
		};

		// Redirect to /login
		await goto('/login');
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-white p-4">
	<form
		onsubmit={create_user}
		class="flex w-full max-w-md flex-col space-y-6 rounded-lg border border-gray-200 p-8 shadow-sm"
	>
		<div class="space-y-2">
			<h2 class="text-2xl font-bold tracking-tight text-black">Create account</h2>
			<p class="text-sm text-gray-500">Enter your details to register on the platform.</p>
		</div>

		{#if status == null && !error}
			<div
				class="rounded-2xl border-2 border-b-green-400 bg-green-200 p-4 text-sm font-bold text-green-500"
			>
				User created successfully! Redirecting to login...
			</div>
		{/if}

		{#if status == null && error}
			<div
				class="rounded-2xl border-2 border-red-500 bg-red-200 p-4 text-sm font-bold text-red-500"
			>
				{error}
			</div>
		{/if}

		<div class="space-y-4">
			<div class="flex flex-col gap-1.5">
				<label for="name" class="text-sm font-medium">Name</label>
				<input
					id="name"
					bind:value={data.name}
					type="text"
					placeholder="Your full name"
					required
					class="rounded-md border border-gray-300 p-2 transition-all focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="email" class="text-sm font-medium">Email</label>
				<input
					id="email"
					bind:value={data.email}
					type="email"
					placeholder="name@example.com"
					required
					class="rounded-md border border-gray-300 p-2 transition-all focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="password" class="text-sm font-medium">Password</label>
				<input
					id="password"
					bind:value={data.password}
					type="password"
					placeholder="••••••••"
					required
					class="rounded-md border border-gray-300 p-2 transition-all focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>
		</div>

		<button
			type="submit"
			disabled={status}
			class="w-full cursor-pointer rounded-md bg-black px-4 py-2 font-medium text-white transition-colors hover:bg-gray-800 disabled:bg-gray-400"
		>
			{status === null ? 'Registering...' : 'Sign up'}
		</button>
	</form>
</div>
