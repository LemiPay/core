<script lang="ts">
	import { login } from '$lib/api/auth';
	import type { SuccessResponse } from '$lib/types/auth.types';

	let data = $state({
		email: '',
		password: ''
	});
	let status: boolean | null = $state(false);
	let error = $state('');

	async function login_user() {
		status = true;
		const response = await login(data);
		if (response.status !== 200) {
			error = response.message;
			status = null;
			return;
		} else {
			localStorage.setItem('token', (response as SuccessResponse<{ token: string }>).body.token);
			status = null;
			data = {
				email: '',
				password: ''
			};
		}
		setTimeout(() => {
			window.location.href = '/';
		}, 1000);
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-white p-4">
	<form
		onsubmit={login_user}
		class="flex w-full max-w-md flex-col space-y-6 rounded-lg border border-gray-200 p-8 shadow-sm"
	>
		<div class="space-y-2">
			<h2 class="text-2xl font-bold tracking-tight text-black">Log in to your account</h2>
			<p class="text-sm text-gray-500">Enter your details to access the platform.</p>
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
		{#if status != null || error}
			<button
				type="submit"
				disabled={status}
				class="w-full cursor-pointer rounded-md bg-black px-4 py-2 font-medium text-white transition-colors hover:bg-gray-800 disabled:bg-gray-400"
			>
				{status ? 'Logging in...' : 'Log in'}
			</button>
		{/if}
	</form>
</div>
