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
	class="flex flex-col gap-2 border-b border-border bg-card p-4 text-card-foreground transition-colors last:border-b-0 hover:bg-muted/40"
>
	<p class="text-sm leading-snug text-muted-foreground">
		<span class="font-semibold text-foreground">{senderName}</span> te invitó a unirte al grupo
		<span class="font-semibold text-foreground">{groupName}</span>.
	</p>

	<div class="mt-1 flex gap-2">
		<button
			onclick={onAccept}
			class="flex flex-1 items-center justify-center gap-1 rounded-md bg-muted px-3 py-1.5 text-xs font-medium text-foreground transition-colors hover:bg-accent"
		>
			<Check size={14} strokeWidth={3} /> Aceptar
		</button>
		<button
			onclick={onDecline}
			class="flex flex-1 items-center justify-center gap-1 rounded-md bg-muted px-3 py-1.5 text-xs font-medium text-foreground transition-colors hover:bg-accent"
		>
			<X size={14} strokeWidth={3} /> Rechazar
		</button>
	</div>
</div>
