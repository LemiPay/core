<script lang="ts" module>
	type QuickAction = {
		label: string;
		type: 'ask' | 'explain';
		concept?: string;
	};

	const quickActions: QuickAction[] = [
		{ label: '¿Cuánto debo en total?', type: 'ask' },
		{ label: 'Resumí mis grupos', type: 'ask' },
		{ label: '¿Cómo funciona Lemipay?', type: 'explain', concept: 'balances' },
		{ label: '¿Cómo funcionan las propuestas?', type: 'explain', concept: 'governance' }
	];

	const explainTopics: QuickAction[] = [
		{ label: 'Balances y deudas', type: 'explain', concept: 'balances' },
		{ label: 'Grupos y roles', type: 'explain', concept: 'groups' },
		{ label: 'Propuestas y gobernanza', type: 'explain', concept: 'governance' },
		{ label: 'Inversiones', type: 'explain', concept: 'investments' },
		{ label: 'Resolución de deudas', type: 'explain', concept: 'debt_resolution' }
	];
</script>

<script lang="ts">
	import { Bot, MessageCircle, Plus, Trash2, X, SendHorizonal, Sparkles } from 'lucide-svelte';
	import { fade, slide } from 'svelte/transition';
	import { isSuccess } from '$lib/types/client.types';
	import { askAI, explainAI } from '$lib/api/endpoints/ai';
	import ChatMessage from './ChatMessage.svelte';

	type Message = {
		role: 'user' | 'assistant';
		content: string;
	};

	function loadMessages(): Message[] {
		try {
			const stored = localStorage.getItem('lemi-chat');
			return stored ? JSON.parse(stored) : [];
		} catch {
			return [];
		}
	}

	function resetChat() {
		messages = [];
		localStorage.removeItem('lemi-chat');
	}

	let open = $state(false);
	let messages = $state<Message[]>(loadMessages());
	let input = $state('');
	let loading = $state(false);
	let container: HTMLDivElement | undefined = $state();
	let menuBtn: HTMLButtonElement | undefined = $state();
	let showMenu = $state(false);
	let menuStyle = $state('');

	function toggleMenu() {
		if (showMenu) {
			showMenu = false;
			return;
		}
		if (menuBtn) {
			const r = menuBtn.getBoundingClientRect();
			menuStyle = `left:${r.left}px;bottom:${window.innerHeight - r.top + 4}px;`;
		}
		showMenu = true;
	}

	$effect(() => {
		if (messages.length && container) {
			requestAnimationFrame(() => {
				container!.scrollTop = container!.scrollHeight;
			});
		}
	});

	$effect(() => {
		try {
			localStorage.setItem('lemi-chat', JSON.stringify(messages));
		} catch {}
	});

	async function send() {
		const q = input.trim();
		if (!q || loading) return;

		input = '';
		const userMsg: Message = { role: 'user', content: q };
		const history = messages;
		messages = [...messages, userMsg];
		loading = true;

		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 30000);

		try {
			const res = await askAI({ question: q, history }, controller.signal);
			loading = false;

			if (isSuccess(res)) {
				messages = [...messages, { role: 'assistant', content: res.body.answer }];
			} else {
				const errorMsg =
					res.status === 429
						? 'Estamos recibiendo muchas consultas. Esperá unos segundos y probá de nuevo.'
						: 'Disculpá, hubo un error al comunicarme con el servidor. Intentalo de nuevo.';
				messages = [...messages, { role: 'assistant', content: errorMsg }];
			}
		} catch {
			loading = false;
			const msg = controller.signal.aborted
				? 'La conexión tardó demasiado. Intentalo de nuevo.'
				: 'Hubo un error inesperado. Intentalo de nuevo.';
			messages = [...messages, { role: 'assistant', content: msg }];
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

	async function askQuick(action: QuickAction) {
		if (action.type === 'explain') {
			const q = action.label;
			const userMsg: Message = { role: 'user', content: q };
			messages = [...messages, userMsg];
			loading = true;

			const controller = new AbortController();
			const timeout = setTimeout(() => controller.abort(), 30000);

			try {
				const res = await explainAI({ concept: action.concept! }, controller.signal);
				loading = false;
				if (isSuccess(res)) {
					messages = [...messages, { role: 'assistant', content: res.body.explanation }];
				} else {
					messages = [...messages, { role: 'assistant', content: 'Disculpá, hubo un error.' }];
				}
			} catch {
				loading = false;
				messages = [...messages, { role: 'assistant', content: 'La conexión tardó demasiado.' }];
			} finally {
				clearTimeout(timeout);
			}
		} else {
			input = action.label;
			requestAnimationFrame(() => send());
		}
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
			<div class="flex items-center gap-1">
				<button
					type="button"
					onclick={resetChat}
					class="flex h-7 w-7 items-center justify-center rounded-full text-muted-foreground transition hover:bg-destructive/10 hover:text-destructive"
					aria-label="Resetear chat"
				>
					<Trash2 class="size-3.5" />
				</button>
				<button
					type="button"
					class="flex h-7 w-7 items-center justify-center rounded-full text-muted-foreground transition hover:bg-muted hover:text-foreground"
					onclick={() => (open = false)}
					aria-label="Cerrar"
				>
					<X class="size-4" />
				</button>
			</div>
		</div>

		<div
			bind:this={container}
			class="flex-1 space-y-3 overflow-y-auto scroll-smooth p-4"
			onclick={() => (showMenu = false)}
		>
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
								{action.label}
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

		<div class="relative border-t border-border p-3">
			<div
				class="flex items-center gap-1.5 rounded-xl border border-border bg-background px-2 py-2"
			>
				<div class="relative">
					<button
						type="button"
						bind:this={menuBtn}
						class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg text-muted-foreground transition hover:bg-muted hover:text-foreground"
						onclick={toggleMenu}
						aria-label="Temas"
					>
						<Plus class="size-4" />
					</button>
				</div>

				<input
					type="text"
					bind:value={input}
					placeholder="Escribí tu pregunta..."
					class="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
					disabled={loading}
				/>
				<button
					type="button"
					class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg transition {loading ||
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

		{#if showMenu}
			<div
				class="fixed z-[60] w-56 rounded-lg border border-border bg-card py-1 shadow-xl"
				style={menuStyle}
				onclick={() => (showMenu = false)}
				role="menu"
			>
				<div class="flex items-center gap-2 border-b border-border px-3 py-2">
					<Sparkles class="size-3.5 text-primary" />
					<span class="text-xs font-semibold text-foreground">Aprender</span>
				</div>
				{#each explainTopics as topic}
					<button
						type="button"
						class="flex w-full items-center gap-2 px-3 py-2 text-left text-xs font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground"
						onclick={() => askQuick(topic)}
						role="menuitem"
					>
						{topic.label}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}
