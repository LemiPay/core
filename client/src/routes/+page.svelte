<script>
	import { isAuthenticated } from '$lib/stores/store';
	import { goto, invalidateAll } from '$app/navigation';

	async function logoutUser() {
		localStorage.removeItem('token');
		window.location.href = '/';
	}

	const me = 'dsada0d9sa-d8asd89sa-d3nnd3-da9d9sa';
	// Aca se puede pegarle a /auth/me y devulve el ID de las credentials logeadas.
</script>

<svelte:head>
	<title>Lemipay - Home</title>
</svelte:head>

<div class="container">
	<h1>Lemipay</h1>
	<p>Manage your group expenses in a decentralized and transparent way.</p>

	<div class="button-group">
		<a href="/login" class="btn btn-primary">Log in</a>
		<a href="/register" class="btn btn-secondary">Create account</a>
		{#if $isAuthenticated}
			<button onclick={logoutUser} class="btn btn-logout">Log out</button>
		{/if}
	</div>

	{#if $isAuthenticated}
		<div class="status-badge">
			<p>You are currently logged in as {me}</p>
		</div>
	{/if}
</div>

<style>
	* {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}
	:global(body) {
		background-color: #ffffff;
		height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.container {
		width: 100%;
		max-width: 400px;
		padding: 2.5rem;
		border: 1px solid #f0f0f0;
		border-radius: 12px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
		text-align: left;
	}

	h1 {
		font-size: 1.8rem;
		font-weight: 800;
		margin-bottom: 0.5rem;
		color: #000;
	}

	/* Estilo para el párrafo descriptivo principal */
	.container > p {
		font-size: 0.95rem;
		color: #666;
		margin-bottom: 2rem;
		line-height: 1.4;
	}

	.button-group {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		/* Espacio inferior para cuando aparezca el badge */
		margin-bottom: 2rem;
	}

	.btn {
		display: block;
		width: 100%;
		padding: 0.85rem;
		text-align: center;
		text-decoration: none;
		font-weight: 600;
		border-radius: 8px;
		transition: opacity 0.2s;
	}

	.btn-primary {
		background-color: #000;
		color: #fff;
	}

	.btn-secondary {
		background-color: #fff;
		color: #000;
		border: 1px solid #e0e0e0;
	}

	.btn:hover {
		opacity: 0.8;
	}

	/* --- NUEVOS ESTILOS PARA EL MENSAJE DE ESTADO --- */
	.status-badge {
		background-color: #f9f9f9; /* Un gris muy sutil de fondo */
		border: 1px solid #eee; /* Borde suave */
		border-radius: 8px;
		padding: 1rem;
		text-align: center;
	}

	.status-badge p {
		font-size: 0.85rem;
		color: #000; /* Texto en negro para que resalte */
		margin: 0; /* Resetear márgenes por defecto del p */
		font-weight: 500;
	}
	.btn-logout {
		background-color: #fff;
		color: #e63946; /* Un rojo elegante, no muy chillón */
		border: 1px solid #ffccd5; /* Borde sutil rojizo */
		cursor: pointer;
		width: 100%;
		display: block;
	}

	.btn-logout:hover {
		background-color: #fff5f5; /* Cambio de fondo muy leve al pasar el mouse */
		border-color: #e63946;
		opacity: 1; /* Sobrescribimos el opacity 0.8 genérico para mejor feedback */
	}
</style>
