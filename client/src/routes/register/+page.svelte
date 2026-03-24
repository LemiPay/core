<script lang="ts">
    import { register } from "$lib/api/auth";
    import { redirect } from '@sveltejs/kit';
    import type {FailedResponse} from "$lib/types/auth.types";
    import {goto} from "$app/navigation";

    let data = $state({
        name: "",
        email: "",
        password: ""
    })

    // true: loading, false: not yet loading, null: end
    let status: boolean|null = $state(false);
    let error = $state("");

    async function create_user() {
        status = true;
        const res = await register(data);

        if (res.status !== 200) {
            const x: FailedResponse = res as FailedResponse;
            const msg = x.message || "An error occurred while registering.";
            error = error + msg;
            status = null;
            return;
        }

        status = null;
        data = {
            name: "",
            email: "",
            password: ""
        }

        // Redirect to /login
        await goto('/login');
    }
</script>

<div class="flex justify-center items-center min-h-screen bg-white p-4">
    <form
            onsubmit={create_user}
            class="flex flex-col w-full max-w-md p-8 border border-gray-200 rounded-lg shadow-sm space-y-6"
    >
        <div class="space-y-2">
            <h2 class="text-2xl font-bold tracking-tight text-black">Create account</h2>
            <p class="text-sm text-gray-500">Enter your details to register on the platform.</p>
        </div>

        {#if status == null && !error}
        <div class="text-green-500 text-sm border-2 font-bold border-b-green-400 bg-green-200 p-4 rounded-2xl">
            User created successfully! Redirecting to login...
        </div>
        {/if}

        {#if status == null && error}
        <div  class="text-red-500 text-sm border-2 font-bold border-red-500 bg-red-200 p-4 rounded-2xl" >
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
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
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
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
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
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
                />
            </div>
        </div>

        <button
                type="submit"
                disabled={status}
                class="w-full bg-black text-white font-medium py-2 px-4 rounded-md hover:bg-gray-800 disabled:bg-gray-400 transition-colors cursor-pointer"
        >
            {status === null ? "Registering..." : "Sign up"}
        </button>
    </form>
</div>