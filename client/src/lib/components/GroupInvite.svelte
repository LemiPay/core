<script lang="ts">
	import { Check, X } from 'lucide-svelte';

	// Definimos la interfaz de las props para TypeScript
	interface Props {
		senderName: string;
		groupName: string;
		onAccept: () => void;
		onDecline: () => void;
	}

	// Usamos la rune $props()
	let { senderName, groupName, onAccept, onDecline }: Props = $props();
</script>

<div
	class="flex flex-col gap-2 border-b bg-white p-4 transition-colors last:border-b-0 hover:bg-slate-50"
>
	<p class="text-sm leading-snug text-slate-700">
		<span class="font-semibold text-slate-900">{senderName}</span> te invitó a unirte al grupo
		<span class="font-semibold text-slate-900">{groupName}</span>.
	</p>

	<div class="mt-1 flex gap-2">
		<button
			onclick={onAccept}
			class="flex flex-1 items-center justify-center gap-1 rounded-md bg-slate-100 px-3 py-1.5 text-xs font-medium text-slate-700 transition-colors hover:bg-slate-200"
		>
			<Check size={14} strokeWidth={3} /> Aceptar
		</button>
		<button
			onclick={onDecline}
			class="flex flex-1 items-center justify-center gap-1 rounded-md bg-slate-100 px-3 py-1.5 text-xs font-medium text-slate-700 transition-colors hover:bg-slate-200"
		>
			<X size={14} strokeWidth={3} /> Rechazar
		</button>
	</div>
</div>
