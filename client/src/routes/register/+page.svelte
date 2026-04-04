<script lang="ts">
	import { register } from '$lib/api/auth';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let data = $state({
		name: '',
		email: '',
		password: ''
	});

	let status: boolean | null = $state(false);
	let error = $state('');

	async function create_user() {
		status = true;
		error = '';

		const response = await register(data);

		if (response.status !== 200) {
			error = response.message || 'An error occurred while registering.';
			status = null;
			return;
		}

		status = null;

		data = {
			name: '',
			email: '',
			password: ''
		};

		setTimeout(() => {
			window.location.href = '/login';
		}, 1000);
	}
</script>

<AuthLayout title="Create account" description="Enter your details to register on the platform.">
	<form onsubmit={create_user} class="flex flex-col space-y-6">
		<!-- Success Message -->
		{#if status === null && !error}
			<div
				class="rounded-lg border border-green-300 bg-green-100 p-3 text-sm font-medium text-green-700"
			>
				User created successfully! Redirecting...
			</div>
		{/if}

		<!-- Error Message -->
		{#if status === null && error}
			<div class="rounded-lg border border-red-300 bg-red-100 p-3 text-sm font-medium text-red-700">
				{error}
			</div>
		{/if}

		<!-- Name input -->
		<div class="flex flex-col gap-1.5">
			<label for="name" class="text-sm font-medium">Name</label>
			<input
				id="name"
				bind:value={data.name}
				type="text"
				required
				placeholder="Your full name"
				class="rounded-md border border-gray-300 p-2 focus:ring-2 focus:ring-black focus:outline-none"
			/>
		</div>

		<!-- Email input -->
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

		<!-- Password input -->
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

		<!-- Submit btn -->
		<button
			type="submit"
			disabled={status === true}
			class="w-full rounded-md bg-black px-4 py-2 font-medium text-white transition hover:bg-gray-800 disabled:bg-gray-400"
		>
			{status === true ? 'Registering...' : 'Sign up'}
		</button>
	</form>
</AuthLayout>
