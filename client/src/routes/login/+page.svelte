<script lang="ts">
    import { login } from "$lib/api/auth";
    import type {SuccessResponse} from "$lib/types/auth.types";



    let data = $state({
        email: "",
        password: ""
    })
    let status: boolean|null = $state(false);
    let error = $state("");


    async function login_user() {
        status = true;
        const response = await login(data);
        if(response.status !== 200){
            error=response.message;
            status=null;
            return
        }else{
            localStorage.setItem("token", (response as SuccessResponse<{token: string}>).body.token);
            status=null;
            data = {
                email: "",
                password: ""
            }
        }
        setTimeout(()=>{
            window.location.href = '/';
        }, 1000);
    }
</script>

<div class="flex justify-center items-center min-h-screen bg-white p-4">
    <form
            onsubmit={login_user}
            class="flex flex-col w-full max-w-md p-8 border border-gray-200 rounded-lg shadow-sm space-y-6"
    >
        <div class="space-y-2">
            <h2 class="text-2xl font-bold tracking-tight text-black">Log in to your account</h2>
            <p class="text-sm text-gray-500">Enter your details to access the platform.</p>
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
        {#if status != null || error}
        <button
                type="submit"
                disabled={status}
                class="w-full bg-black text-white font-medium py-2 px-4 rounded-md hover:bg-gray-800 disabled:bg-gray-400 transition-colors cursor-pointer"
        >
            {status ? "Logging in..." : "Log in"}
        </button>
        {/if}
    </form>
</div>