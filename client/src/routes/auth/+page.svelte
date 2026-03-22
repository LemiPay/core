<script>
    import { register } from "$lib/api/auth";

    let name="";
    let email="";
    let password="";

    async function create_user(){
        try {
            await register({ name, email, password })
            alert("User created!");
        } catch (error) {
            console.error(error)
        }
    }

</script>

<form onsubmit={() => create_user()} class="flex flex-col max-w-2xl">
    <input bind:value={name} type="text" placeholder="Nombre" required />
    <input bind:value={email} type="email" placeholder="Email" required />
    <input bind:value={password} type="password" placeholder="Password" />

    <button type="submit">Sign Up</button>
</form>