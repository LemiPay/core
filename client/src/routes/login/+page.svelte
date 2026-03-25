<script>
    import { login } from "$lib/api/auth";
    

    let email = "";
    let password = "";
    let isLoading = false;

    async function login_user() {
        isLoading = true;
        try {
            const response = await login({ email, password });
            localStorage.setItem("token", response.token);
            alert("User Logged In!");
            email = password = "";
            //redirecciono
            window.location.href = "/";
        } catch (error) {
            console.error(error);
            alert("Error logging in");
        } finally {
            isLoading = false;
        }
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

        <div class="space-y-4">
            <div class="flex flex-col gap-1.5">
                <label for="email" class="text-sm font-medium">Email</label>
                <input
                        id="email"
                        bind:value={email}
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
                        bind:value={password}
                        type="password"
                        placeholder="••••••••"
                        required
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
                />
            </div>
        </div>

        <button
                type="submit"
                disabled={isLoading}
                class="w-full bg-black text-white font-medium py-2 px-4 rounded-md hover:bg-gray-800 disabled:bg-gray-400 transition-colors cursor-pointer"
        >
            {isLoading ? "Logging in..." : "Log in"}
        </button>
    </form>
</div>