<script lang="ts" module>
	const quickActions = [
		'¿Cuánto debo en total?',
		'Resumí mis grupos',
		'¿Cómo funciona Lemipay?',
		'¿Qué es debt resolution?'
	];
</script>

<script lang="ts">
	import { Bot, MessageCircle, X, SendHorizonal, Sparkles } from 'lucide-svelte';
	import { fade, slide } from 'svelte/transition';
	import { isSuccess } from '$lib/types/client.types';
	import { askAI } from '$lib/api/endpoints/ai';
	import ChatMessage from './ChatMessage.svelte';

	type Message = {
		role: 'user' | 'assistant';
		content: string;
	};

	let open = $state(false);
	let messages = $state<Message[]>([]);
	let input = $state('');
	let loading = $state(false);
	let container: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (messages.length && container) {
			requestAnimationFrame(() => {
				container!.scrollTop = container!.scrollHeight;
			});
		}
	});

	async function send() {
		const q = input.trim();
		if (!q || loading) return;

		input = '';
		messages = [...messages, { role: 'user', content: q }];
		loading = true;

		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 30000);

		try {
			const res = await askAI({ question: q }, controller.signal);
			loading = false;

			if (isSuccess(res)) {
				messages = [...messages, { role: 'assistant', content: res.body.answer }];
			} else {
				messages = [
					...messages,
					{
						role: 'assistant',
						content: 'Disculpá, hubo un error al comunicarme con el servidor. Intentalo de nuevo.'
					}
				];
			}
		} catch {
			loading = false;
			messages = [
				...messages,
				{
					role: 'assistant',
					content: 'La conexión tardó demasiado. Intentalo de nuevo.'
				}
			];
		} finally {
			clearTimeout(timeout);
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			send();
		}
	}

	function askQuick(question: string) {
		input = question;
		requestAnimationFrame(() => send());
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<div
		class="fixed inset-0 z-40 bg-black/40 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
		onclick={() => (open = false)}
		role="presentation"
	/>
{/if}

<button
	type="button"
	aria-label={open ? 'Cerrar chat' : 'Abrir chat'}
	class="fixed right-5 bottom-5 z-50 flex h-14 w-14 items-center justify-center rounded-full shadow-2xl ring-4 transition hover:scale-105 focus:outline-none focus-visible:ring-2 focus-visible:ring-ring active:scale-95 {open
		? 'bg-muted text-muted-foreground ring-muted/20'
		: 'bg-primary text-primary-foreground ring-lime-400/10'}"
	onclick={() => (open = !open)}
>
	{#if open}
		<X class="size-6" />
	{:else}
		<MessageCircle class="size-6" />
	{/if}
</button>

{#if open}
	<div
		class="fixed right-5 bottom-24 z-50 flex h-[600px] w-[380px] flex-col overflow-hidden rounded-2xl border border-border bg-card shadow-2xl"
		transition:slide={{ duration: 200, axis: 'y' }}
		role="dialog"
		aria-label="Asistente Lemi"
	>
		<div class="flex items-center justify-between border-b border-border px-4 py-3">
			<div class="flex items-center gap-2">
				<Sparkles class="size-4 text-primary" />
				<span class="text-sm font-semibold">Lemi — Asistente</span>
			</div>
			<button
				type="button"
				class="flex h-7 w-7 items-center justify-center rounded-full text-muted-foreground transition hover:bg-muted hover:text-foreground"
				onclick={() => (open = false)}
				aria-label="Cerrar"
			>
				<X class="size-4" />
			</button>
		</div>

		<div bind:this={container} class="flex-1 space-y-3 overflow-y-auto scroll-smooth p-4">
			{#if messages.length === 0}
				<div class="flex h-full flex-col items-center justify-center text-center">
					<Bot class="mt-6 mb-3 size-8 text-muted-foreground" />
					<p class="text-sm font-medium text-foreground">¡Hola! Soy Lemi</p>
					<p class="mt-1 mb-4 max-w-[260px] text-xs text-muted-foreground">
						Preguntame sobre tus grupos, balances, o cómo funciona Lemipay.
					</p>
					<div class="flex flex-wrap justify-center gap-2 px-2">
						{#each quickActions as action}
							<button
								type="button"
								class="rounded-full border border-border bg-background px-3 py-1.5 text-xs font-medium text-muted-foreground transition hover:border-primary hover:text-primary"
								onclick={() => askQuick(action)}
							>
								{action}
							</button>
						{/each}
					</div>
				</div>
			{:else}
				{#each messages as msg (msg)}
					<ChatMessage role={msg.role} content={msg.content} />
				{/each}

				{#if loading}
					<div class="flex items-center gap-2 text-xs text-muted-foreground">
						<div class="flex gap-1">
							<span class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground" />
							<span
								class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground [animation-delay:0.1s]"
							/>
							<span
								class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground [animation-delay:0.2s]"
							/>
						</div>
						<span>Pensando...</span>
					</div>
				{/if}
			{/if}
		</div>

		<div class="border-t border-border p-3">
			<div class="flex items-center gap-2 rounded-xl border border-border bg-background px-3 py-2">
				<input
					type="text"
					bind:value={input}
					placeholder="Escribí tu pregunta..."
					class="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
					disabled={loading}
				/>
				<button
					type="button"
					class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full transition {loading ||
					!input.trim()
						? 'text-muted-foreground'
						: 'bg-primary text-primary-foreground hover:bg-primary/90'}"
					onclick={send}
					disabled={loading || !input.trim()}
					aria-label="Enviar"
				>
					<SendHorizonal class="size-4" />
				</button>
			</div>
		</div>
	</div>
{/if}
