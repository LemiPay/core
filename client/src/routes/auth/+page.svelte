<script>
    let name="";
    let email='';
    let password='';

    async function register(){
        const respuesta = await fetch('http://localhost:3000/auth/register', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name, email, password })
        });

        if (respuesta.ok) {
            alert("¡Usuario creado!");
        }

    }

</script>

<input bind:value={name} placeholder="Nombre" />
<input bind:value={email} placeholder="Email" />
<input bind:value={password} placeholder="Password" />
<button onclick={() => register()}>Registrarse</button>