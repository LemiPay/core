<script lang="ts">
	import {
		ArrowLeft,
		ArrowDownToLine,
		Copy,
		Plus,
		Send,
		Wallet,
		ArrowUpRight,
		Shield,
		Clock,
		CheckCircle,
		Mail,
		ReceiptText
	} from 'lucide-svelte';
	import type { User } from '$lib/types/endpoints/auth.types';
	import { me } from '$lib/api/auth';
	import { isSuccess } from '$lib/types/client.types';
	import type { WalletInfo } from '$lib/types/endpoints/user_wallet.types';
	import { getAllMyWallets } from '$lib/api/endpoints/user_wallet';

	// Importar Modales
	import FaucetModal from '$lib/components/modals/user/FaucetModal.svelte';
	import TransferModal from '$lib/components/modals/user/TransferModal.svelte';
	import CreateWalletModal from '$lib/components/modals/user/CreateWalletModal.svelte';
	import { shortenAddress, copyToClipboard } from '$lib/utils/address_utils';
	import { resolve } from '$app/paths';
	import { fly, fade, scale } from 'svelte/transition';
	import UserTransactionHistory from '$lib/components/UserTransactionHistory.svelte';
	import type { Transaction } from '$lib/types/endpoints/transactions.types';
	import { listUserTransactions } from '$lib/api/endpoints/transactions';

	// --- ESTADOS DE DATOS ---
	let loadingUserInfo = $state(true);
	let user = $state({} as User);
	let openTxHistory = $state(false);
	let copiedAddress = $state<string | null>(null);

	let loadingWalletsInfo = $state(true);
	let walletsArray = $state([] as WalletInfo[]);

	let loadingTransactions = $state(true);
	let transactionsArray = $state([] as Transaction[]);

	// --- ESTADOS DE MODALES ---
	let faucetTarget = $state<{ wallet_id: string; ticker: string } | null>(null);
	let transferTarget = $state<{ sender_wallet_id: string; ticker: string } | null>(null);
	let openCreateWalletModal = $state(false);

	// --- ESTADO DERIVADO ---
	let totalBalance = $derived(
		walletsArray.reduce((acc, group) => {
			const groupSum = group.currencies.reduce((sum, curr) => sum + Number(curr.balance || 0), 0);
			return acc + groupSum;
		}, 0)
	);

	// --- CARGA DE DATOS ---
	async function loadUserProfile() {
		const result = await me();
		if (isSuccess(result)) {
			user = result.body;
		}
		loadingUserInfo = false;
	}

	async function loadWallets() {
		const result = await getAllMyWallets();
		if (!isSuccess(result)) {
			loadingWalletsInfo = false;
			return;
		}
		walletsArray = result.body;
		loadingWalletsInfo = false;
	}

	async function loadTransactions() {
		loadingTransactions = true;
		const result = await listUserTransactions();
		if (isSuccess(result)) transactionsArray = result.body.reverse();
		loadingTransactions = false;
	}

	async function handleCopy(address: string) {
		await copyToClipboard(address);
		copiedAddress = address;
		setTimeout(() => (copiedAddress = null), 2000);
	}

	function goBack() {
		if (typeof history !== 'undefined' && history.length > 1) {
			history.back();
		} else {
			window.location.href = '/dashboard';
		}
	}

	const totalWallets = $derived(walletsArray.length);

	const userInitials = $derived(
		user.name
			? user.name
					.split(' ')
					.slice(0, 2)
					.map((n) => n[0])
					.join('')
					.toUpperCase()
			: '?'
	);

	// --- INIT ---
	loadUserProfile();
	loadWallets();
	loadTransactions();
</script>

<svelte:head>
	<title>Lemipay – Perfil de {user.name ?? '...'}</title>
</svelte:head>

<FaucetModal
	open={faucetTarget !== null}
	wallet_id={faucetTarget?.wallet_id ?? ''}
	ticker={faucetTarget?.ticker ?? ''}
	onclose={() => (faucetTarget = null)}
	onsuccess={() => loadWallets()}
/>

<TransferModal
	open={transferTarget !== null}
	sender_wallet_id={transferTarget?.sender_wallet_id ?? ''}
	ticker={transferTarget?.ticker ?? ''}
	onclose={() => (transferTarget = null)}
	onsuccess={() => loadWallets()}
/>

<CreateWalletModal
	open={openCreateWalletModal}
	onclose={() => (openCreateWalletModal = false)}
	onsuccess={() => loadWallets()}
/>

<UserTransactionHistory
	open={openTxHistory}
	onclose={() => (openTxHistory = false)}
	onsuccess={() => (openTxHistory = false)}
	{transactionsArray}
	{loadingTransactions}
/>

<div class="min-h-screen bg-background text-foreground">
	<!-- Ambient background blobs matching dashboard -->
	<div
		class="pointer-events-none fixed inset-0 -z-10 bg-[radial-gradient(circle_at_top_left,rgba(163,230,53,0.18),transparent_32%),radial-gradient(circle_at_90%_10%,rgba(168,85,247,0.14),transparent_28%)]"
	></div>

	<div class="mx-auto w-full max-w-3xl px-4 pt-28 pb-16 sm:px-6">
		<!-- Barrita -->
		<div class="flex w-full justify-between">
			<!-- Back button -->
			<div in:fly={{ x: -20, duration: 600 }}>
				<button
					onclick={goBack}
					class="mb-6 inline-flex items-center gap-2 rounded-full border border-border bg-card/80 px-3.5 py-1.5 text-xs font-medium text-muted-foreground backdrop-blur transition hover:border-border/80 hover:text-foreground"
				>
					<ArrowLeft class="size-3.5" />
					Volver
				</button>
			</div>

			<!-- Back button -->
			<div in:fly={{ x: 20, duration: 600 }}>
				<button
					onclick={() => (openTxHistory = true)}
					class="mb-6 inline-flex items-center gap-2 rounded-full border border-border bg-card/80 px-3.5 py-1.5 text-xs font-medium text-muted-foreground backdrop-blur transition hover:border-border/80 hover:text-foreground"
				>
					<ReceiptText class="size-3.5" />
					Historial de transacciones
				</button>
			</div>
		</div>

		<!-- Hero profile card -->
		<section
			class="relative overflow-hidden rounded-[2rem] border border-border/80 bg-card shadow-sm shadow-black/5 dark:shadow-none"
			in:fly={{ y: 14, duration: 420 }}
		>
			<!-- Decorative blobs inside card -->
			<div
				class="absolute top-0 right-0 h-52 w-52 translate-x-16 -translate-y-16 rounded-full bg-lime-300/30 blur-3xl dark:bg-lime-400/10"
			></div>
			<div
				class="absolute bottom-0 left-24 h-40 w-40 translate-y-20 rounded-full bg-violet-400/20 blur-3xl dark:bg-violet-500/10"
			></div>

			<div class="relative p-6 sm:p-8">
				<div class="flex flex-col gap-6 sm:flex-row sm:items-start sm:justify-between">
					<!-- Avatar + info -->
					<div class="flex items-center gap-5">
						<!-- Avatar ring -->
						<div class="relative shrink-0">
							<div
								class="flex size-20 items-center justify-center rounded-3xl bg-linear-to-br from-lime-300 via-emerald-200 to-violet-300 text-2xl font-semibold text-lime-900 shadow-lg shadow-lime-500/20 dark:from-lime-400/40 dark:via-emerald-400/20 dark:to-violet-500/30 dark:text-lime-200"
							>
								{#if loadingUserInfo}
									<span class="animate-pulse">…</span>
								{:else}
									{userInitials}
								{/if}
							</div>
							<!-- Online indicator -->
							<span
								class="absolute -right-1 -bottom-1 flex size-5 items-center justify-center rounded-full border-2 border-card bg-emerald-500 shadow shadow-emerald-500/30"
							>
								<span class="size-2 rounded-full bg-white"></span>
							</span>
						</div>

						<div class="flex items-center gap-5">
							<div>
								<h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">
									{user.name}
								</h1>

								<div class="mt-2 flex items-center gap-1 text-muted-foreground">
									<Mail class="size-3.5" />
									<p>{user.email}</p>
								</div>
								<!-- <span
										class="rounded-full bg-violet-500/15 px-3 py-1 text-xs font-semibold text-violet-700 dark:text-violet-300"
									>
										Builder
									</span>

									<span
										class="rounded-full bg-lime-500/15 px-3 py-1 text-xs font-semibold text-lime-700 dark:text-lime-300"
									>
										Early Member
									</span> -->
							</div>
						</div>
					</div>

					<!-- Quick stat pills -->
					<div class="flex flex-wrap gap-2 sm:flex-col sm:items-end">
						<div
							class="inline-flex items-center gap-1.5 rounded-2xl border border-border/80 bg-background/70 px-3 py-1.5 text-xs font-medium backdrop-blur"
						>
							<Wallet class="size-3.5 text-muted-foreground" />
							<span class="text-muted-foreground">Wallets:</span>
							<span class="font-semibold">{totalWallets}</span>
						</div>

						<div
							class="inline-flex items-center gap-1.5 rounded-2xl border border-emerald-300/60 bg-emerald-50/80 px-3 py-1.5 text-xs font-medium dark:border-emerald-400/20 dark:bg-emerald-400/10"
						>
							<CheckCircle class="size-3.5 text-emerald-600 dark:text-emerald-400" />
							<span class="text-emerald-700 dark:text-emerald-300">Verificado</span>
						</div>
					</div>
				</div>

				<!-- Summary metric cards -->
				<div class="mt-6 grid grid-cols-3 gap-3">
					<div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
						<p class="text-xs font-medium text-muted-foreground">Balance total</p>
						<p class="mt-1.5 text-xl font-semibold">
							${totalBalance.toLocaleString('en-US', { maximumFractionDigits: 2 })}
						</p>
					</div>

					<div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
						<p class="text-xs font-medium text-muted-foreground">Wallets activas</p>
						<p class="mt-1.5 text-xl font-semibold">{totalWallets}</p>
					</div>

					<div class="rounded-3xl border border-border/80 bg-background/70 p-4 backdrop-blur">
						<p class="text-xs font-medium text-muted-foreground">Último acceso</p>
						<p class="mt-1.5 flex items-center gap-1 text-sm font-semibold">
							<Clock class="size-3.5 text-muted-foreground" />
							Ahora
						</p>
					</div>
				</div>
			</div>
		</section>

		<!-- Wallets section -->
		<section class="mt-6 space-y-4" in:fly={{ y: 14, duration: 450, delay: 80 }}>
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm font-medium text-muted-foreground">Workspace</p>
					<h2 class="mt-0.5 text-2xl font-semibold tracking-tight">Mis billeteras</h2>
				</div>
				<button
					onclick={() => (openCreateWalletModal = true)}
					class="inline-flex items-center gap-2 rounded-2xl border border-border bg-card px-4 py-2 text-sm font-semibold transition hover:-translate-y-0.5 hover:border-lime-300 hover:shadow-lg hover:shadow-lime-500/10"
				>
					<Plus class="size-4" />
					Nueva dirección
				</button>
			</div>

			{#if loadingWalletsInfo}
				<!-- Skeleton loaders -->
				<div class="space-y-4">
					{#each { length: 2 }, i}
						<div
							class="rounded-[2rem] border border-border bg-card p-5"
							in:fade={{ delay: i * 60 }}
						>
							<div class="h-5 w-48 animate-pulse rounded-lg bg-muted"></div>
							<div class="mt-4 grid grid-cols-2 gap-3">
								<div class="h-16 animate-pulse rounded-2xl bg-muted"></div>
								<div class="h-16 animate-pulse rounded-2xl bg-muted"></div>
							</div>
						</div>
					{/each}
				</div>
			{:else if walletsArray.length === 0}
				<div
					class="rounded-[2rem] border border-dashed border-border bg-card p-10 text-center"
					transition:scale={{ duration: 220 }}
				>
					<div class="mx-auto flex size-14 items-center justify-center rounded-2xl bg-muted">
						<Wallet class="size-6 text-muted-foreground" />
					</div>
					<h3 class="mt-4 font-semibold">Ninguna billetera creada</h3>
					<p class="mx-auto mt-2 max-w-sm text-sm text-muted-foreground">
						Creá tu primera dirección para recibir y enviar tokens.
					</p>
					<button
						onclick={() => (openCreateWalletModal = true)}
						class="mt-5 inline-flex items-center gap-2 rounded-2xl bg-foreground px-4 py-2 text-sm font-semibold text-background transition hover:bg-foreground/90"
					>
						<Plus class="size-4" />
						Crear billetera
					</button>
				</div>
			{:else}
				{#each walletsArray as group, wi (group.address)}
					<div
						class="overflow-hidden rounded-[2rem] border border-border/80 bg-card shadow-sm shadow-black/5 dark:shadow-none"
						in:fly={{ y: 10, duration: 300, delay: wi * 60 }}
					>
						<!-- Wallet header -->
						<div
							class="flex items-center justify-between border-b border-border/60 bg-linear-to-r from-muted/40 to-muted/10 px-5 py-3.5"
						>
							<div class="flex items-center gap-2.5">
								<div
									class="flex size-8 items-center justify-center rounded-xl bg-muted text-muted-foreground"
								>
									<Wallet class="size-4" />
								</div>
								<div>
									<p class="font-mono text-sm font-semibold">
										{shortenAddress(group.address)}
									</p>
									<p class="text-[11px] text-muted-foreground">
										{group.currencies.length} token{group.currencies.length !== 1 ? 's' : ''}
									</p>
								</div>
							</div>

							<button
								onclick={() => handleCopy(group.address)}
								class={[
									'inline-flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium transition',
									copiedAddress === group.address
										? 'border-emerald-300/60 bg-emerald-50 text-emerald-700 dark:border-emerald-400/20 dark:bg-emerald-400/10 dark:text-emerald-300'
										: 'border-border bg-background/70 text-muted-foreground hover:border-border/80 hover:text-foreground'
								]}
							>
								{#if copiedAddress === group.address}
									<CheckCircle class="size-3.5" />
									Copiado
								{:else}
									<Copy class="size-3.5" />
									Copiar
								{/if}
							</button>
						</div>

						<!-- Token rows -->
						<div class="divide-y divide-border/40 px-5">
							{#each group.currencies as currency, ci (currency.ticker)}
								<div
									class="flex items-center justify-between py-4"
									in:fly={{ x: 8, duration: 220, delay: ci * 40 }}
								>
									<div class="flex items-center gap-3">
										<!-- Token avatar -->
										<div
											class={[
												'flex size-10 items-center justify-center rounded-2xl text-xs font-bold',
												currency.ticker === 'USDC'
													? 'bg-sky-100 text-sky-700 dark:bg-sky-400/10 dark:text-sky-300'
													: currency.ticker === 'ETH'
														? 'bg-violet-100 text-violet-700 dark:bg-violet-400/10 dark:text-violet-300'
														: 'bg-lime-100 text-lime-700 dark:bg-lime-400/10 dark:text-lime-300'
											]}
										>
											{currency.ticker.slice(0, 3)}
										</div>
										<div>
											<p class="font-semibold">{currency.ticker}</p>
											<p class="text-xs text-muted-foreground">
												{Number(currency.balance).toLocaleString('en-US', {
													maximumFractionDigits: 6
												})} disponibles
											</p>
										</div>
									</div>

									<div class="flex items-center gap-2">
										<span class="mr-2 text-right text-lg font-semibold tabular-nums">
											{Number(currency.balance).toLocaleString('en-US', {
												maximumFractionDigits: 4
											})}
										</span>
										<button
											onclick={() =>
												(faucetTarget = {
													wallet_id: currency.wallet_id,
													ticker: currency.ticker
												})}
											class="inline-flex items-center gap-1.5 rounded-xl border border-border bg-background/70 px-3 py-1.5 text-xs font-semibold text-muted-foreground transition hover:border-border/80 hover:text-foreground"
										>
											<ArrowDownToLine class="size-3.5" />
											Recibir
										</button>
										<button
											onclick={() =>
												(transferTarget = {
													sender_wallet_id: currency.wallet_id,
													ticker: currency.ticker
												})}
											class="inline-flex items-center gap-1.5 rounded-xl bg-foreground px-3 py-1.5 text-xs font-semibold text-background shadow-sm transition hover:bg-foreground/85"
										>
											<Send class="size-3.5" />
											Enviar
										</button>
									</div>
								</div>
							{/each}
						</div>

						<!-- Wallet footer -->
						<div class="border-t border-border/40 bg-muted/20 px-5 py-3">
							<button
								class="flex items-center gap-1.5 text-xs font-medium text-muted-foreground transition hover:text-foreground"
							>
								<Plus class="size-3.5" />
								Agregar token
							</button>
						</div>
					</div>
				{/each}
			{/if}
		</section>

		<!-- Account settings teaser -->
		<section
			class="mt-6 rounded-[2rem] border border-border/80 bg-card p-5 shadow-sm"
			in:fly={{ y: 14, duration: 480, delay: 160 }}
		>
			<div class="flex items-center gap-3">
				<div
					class="flex size-10 items-center justify-center rounded-2xl bg-violet-400/15 text-violet-700 dark:text-violet-300"
				>
					<Shield class="size-4" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="font-semibold">Seguridad y cuenta</p>
					<p class="text-sm text-muted-foreground">2FA · contraseña · sesiones activas</p>
				</div>
				<a
					href={resolve('/profile/me')}
					class="inline-flex items-center gap-1 text-sm font-semibold text-muted-foreground transition hover:text-foreground"
				>
					Configurar
					<ArrowUpRight class="size-4" />
				</a>
			</div>
		</section>
	</div>
</div>
