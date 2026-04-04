<script lang="ts">
	// Mock data para el MVP.
	// Más adelante puedes reemplazar esto con los props o el estado global de tu app.
	const user = {
		name: 'Pepe',
		wallet: '0xF5a...3b21',
		balance: '0.00',
		currency: 'USDC'
	};
</script>

<div
	class="flex w-full max-w-sm items-center justify-between rounded-md border border-gray-300 bg-white p-3 shadow-sm"
>
	<div class="flex items-center gap-3">
		<div
			class="flex h-10 w-10 items-center justify-center rounded-full border border-gray-300 bg-gray-50 text-black"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
				<circle cx="12" cy="7" r="4"></circle>
			</svg>
		</div>

		<div class="flex flex-col">
			<span class="text-sm font-medium text-black">{user.name}</span>
			<span class="text-xs text-gray-500">{user.wallet}</span>
		</div>
	</div>

	<div class="text-sm font-semibold text-black">
		${user.balance} <span class="text-xs font-normal text-gray-500">{user.currency}</span>
	</div>
</div>
