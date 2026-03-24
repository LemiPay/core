<script>
    import { register } from "$lib/api/auth";

    let name = "";
    let email = "";
    let password = "";
    let isLoading = false;

    async function create_user() {
        isLoading = true;
        try {
            await register({ name, email, password });
            alert("User created!");
            // Limpiar campos tras éxito
            name = email = password = "";
        } catch (error) {
            console.error(error);
            alert("Error al registrar usuario");
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="flex justify-center items-center min-h-screen bg-white p-4">
    <form
            onsubmit={create_user}
            class="flex flex-col w-full max-w-md p-8 border border-gray-200 rounded-lg shadow-sm space-y-6"
    >
        <div class="space-y-2">
            <h2 class="text-2xl font-bold tracking-tight text-black">Crear cuenta</h2>
            <p class="text-sm text-gray-500">Ingresa tus datos para registrarte en la plataforma.</p>
        </div>

        <div class="space-y-4">
            <div class="flex flex-col gap-1.5">
                <label for="name" class="text-sm font-medium">Nombre</label>
                <input
                        id="name"
                        bind:value={name}
                        type="text"
                        placeholder="Tu nombre completo"
                        required
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="email" class="text-sm font-medium">Email</label>
                <input
                        id="email"
                        bind:value={email}
                        type="email"
                        placeholder="nombre@ejemplo.com"
                        required
                        class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black transition-all"
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="password" class="text-sm font-medium">Contraseña</label>
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
            {isLoading ? "Registrando..." : "Registrarse"}
        </button>
    </form>
</div>