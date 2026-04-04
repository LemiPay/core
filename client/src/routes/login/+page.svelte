<script lang="ts">
	import api from '$lib/api/auth';
	import { authStore } from '$lib/stores/auth';
	import { isSuccess } from '$lib/types/client.types';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let data = $state({
		email: '',
		password: ''
	});

	// false: idle | true: loading | null: end
	let status: boolean | null = $state(false);
	let error = $state('');

	async function login_user() {
	    error = ''
		status = true;
		
		const response = await api.login(data);

		if (!isSuccess(response)) {
			error = response.message || 'Invalid credentials.';
			status = null;
			return;
		}

		authStore.login(response.body.token);
		status = null;
		
		data = {
			email: '',
			password: ''
		};

		setTimeout(() => {
			window.location.href = '/';
		}, 1000);
	}
</script>

<AuthLayout title="Log in to your account" description="Enter your details to access the platform.">
	<form onsubmit={login_user} onchange={() => status = false} class="flex flex-col space-y-6">
		<!-- Success Message -->
		{#if status === null && !error}
			<div
				class="rounded-lg border border-green-300 bg-green-100 p-3 text-sm font-medium text-green-700"
			>
				Login successful! Redirecting...
			</div>
		{/if}

		<!-- Error Message -->
		{#if status === null && error}
			<div class="rounded-lg border border-red-300 bg-red-100 p-3 text-sm font-medium text-red-700">
				{error}
			</div>
		{/if}

		<div class="space-y-4">
			<!-- Email -->
			<div class="flex flex-col gap-1.5">
				<label for="email" class="text-sm font-medium">Email</label>
				<input
					id="email"
					bind:value={data.email}
					type="email"
					required
					placeholder="name@example.com"
					class="rounded-md border border-gray-300 p-2 focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>

			<!-- Password -->
			<div class="flex flex-col gap-1.5">
				<label for="password" class="text-sm font-medium">Password</label>
				<input
					id="password"
					bind:value={data.password}
					type="password"
					required
					placeholder="••••••••"
					class="rounded-md border border-gray-300 p-2 focus:ring-2 focus:ring-black focus:outline-none"
				/>
			</div>
		</div>

		<button
			type="submit"
			disabled={status === true}
			class="w-full rounded-md bg-black px-4 py-2 font-medium text-white transition hover:bg-gray-800 disabled:bg-gray-400"
		>
			{status === true ? 'Logging in...' : 'Log in'}
		</button>
	</form>
</AuthLayout>
