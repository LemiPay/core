import type { Config } from '@wagmi/core';
import { getBalance, readContract } from '@wagmi/core';
import { formatUnits, pad } from 'viem';
import { sepolia } from '@reown/appkit/networks';
import { DISPLAY_DECIMALS, formatAmount, truncateToDecimals } from '$lib/utils/format_utils';

const MIN_ETH_FOR_GAS = 0.0001;

export const SEPOLIA_USDC = {
	address: '0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238' as const,
	decimals: 6
};

const erc20BalanceOfAbi = [
	{
		type: 'function',
		name: 'balanceOf',
		inputs: [{ type: 'address', name: 'account' }],
		outputs: [{ type: 'uint256' }],
		stateMutability: 'view'
	}
] as const;

export type OnChainWalletBalances = {
	usdc: string;
	eth: string;
};

export async function fetchSepoliaWalletBalances(
	config: Config,
	address: string
): Promise<OnChainWalletBalances> {
	const chainId = sepolia.id;
	const account = address as `0x${string}`;

	const [usdcBalance, ethBalance] = await Promise.all([
		readContract(config, {
			address: SEPOLIA_USDC.address,
			abi: erc20BalanceOfAbi,
			functionName: 'balanceOf',
			args: [account],
			chainId
		}),
		getBalance(config, { address: account, chainId })
	]);

	return {
		usdc: formatTokenBalance(usdcBalance, SEPOLIA_USDC.decimals),
		eth: formatTokenBalance(ethBalance.value, 18)
	};
}
const BPS_DENOMINATOR = 10_000n;
export const DEFAULT_FUND_FEE_BPS = 10n;

/** Typical gas units when on-chain estimation is unavailable. */
export const FALLBACK_APPROVE_GAS_UNITS = 65_000n;
export const FALLBACK_FUND_GAS_UNITS = 200_000n;
const GAS_ESTIMATE_BUFFER_NUMERATOR = 120n;
const GAS_ESTIMATE_BUFFER_DENOMINATOR = 100n;

export type FundGasEstimate = {
	approveGasUnits: bigint;
	fundGasUnits: bigint;
	approveGasWei: bigint;
	fundGasWei: bigint;
	totalGasWei: bigint;
	isApproximate: boolean;
};

export function walletAddressToBytes32(address: string): `0x${string}` {
	const normalized = address.startsWith('0x') ? address : `0x${address}`;
	return pad(normalized as `0x${string}`, { size: 32 });
}

export function calculateFundGasEstimate(
	gasPrice: bigint,
	needsApproval: boolean,
	options?: { approveGasUnits?: bigint; fundGasUnits?: bigint; isApproximate?: boolean }
): FundGasEstimate {
	const approveGasUnits = needsApproval
		? (options?.approveGasUnits ?? FALLBACK_APPROVE_GAS_UNITS)
		: 0n;
	const fundGasUnits = options?.fundGasUnits ?? FALLBACK_FUND_GAS_UNITS;
	const gasUnits = approveGasUnits + fundGasUnits;
	const totalGasWei =
		(gasUnits * gasPrice * GAS_ESTIMATE_BUFFER_NUMERATOR) / GAS_ESTIMATE_BUFFER_DENOMINATOR;

	return {
		approveGasUnits,
		fundGasUnits,
		approveGasWei: approveGasUnits * gasPrice,
		fundGasWei: fundGasUnits * gasPrice,
		totalGasWei,
		isApproximate: options?.isApproximate ?? true
	};
}

export type FundBreakdown = {
	grossUnits: bigint;
	feeUnits: bigint;
	netUnits: bigint;
	feeBps: bigint;
	feePercentLabel: string;
};

export function calculateFundBreakdown(grossUnits: bigint, feeBps: bigint): FundBreakdown {
	const feeUnits = (grossUnits * feeBps) / BPS_DENOMINATOR;
	const netUnits = grossUnits - feeUnits;
	const feePercent = Number(feeBps) / 100;
	return {
		grossUnits,
		feeUnits,
		netUnits,
		feeBps,
		feePercentLabel: `${feePercent.toFixed(feePercent % 1 === 0 ? 0 : 1)}%`
	};
}

export function formatEthGasCost(gasWei: bigint): string {
	const eth = Number(formatUnits(gasWei, 18));
	if (!Number.isFinite(eth) || eth === 0) return formatAmount(0);
	if (eth < 0.0001) return '<0.0001';
	return truncateToDecimals(eth, DISPLAY_DECIMALS).toFixed(DISPLAY_DECIMALS);
}

export function formatTokenBalance(value: bigint, decimals: number): string {
	const formatted = formatUnits(value, decimals);
	const n = Number(formatted);
	if (!Number.isFinite(n)) return formatAmount(0);
	return truncateToDecimals(n, DISPLAY_DECIMALS).toFixed(DISPLAY_DECIMALS);
}

export function parseGasEstimateWarning(
	err: unknown,
	context?: { ticker?: string; needsApproval?: boolean }
): string {
	const error = err as {
		shortMessage?: string;
		message?: string;
		cause?: { shortMessage?: string; message?: string };
	};

	const parts = [
		error?.shortMessage,
		error?.message,
		error?.cause?.shortMessage,
		error?.cause?.message
	]
		.filter(Boolean)
		.join(' ')
		.toLowerCase();

	if (context?.needsApproval) {
		return `Todavía no aprobaste el gasto de ${context.ticker ?? 'USDC'} en el vault. El gas del fondeo es aproximado hasta ese primer paso.`;
	}

	if (parts.includes('paused') || parts.includes('enforcepause')) {
		return 'El vault está pausado en Sepolia. No se puede simular el fondeo.';
	}

	if (parts.includes('tokendisabled') || parts.includes('not supported')) {
		return `${context?.ticker ?? 'Este token'} no está habilitado en el vault de Sepolia.`;
	}

	if (
		parts.includes('insufficient allowance') ||
		parts.includes('safeerc20failedoperation') ||
		parts.includes('transfer amount exceeds balance')
	) {
		const ticker = context?.ticker ?? 'USDC';
		return `La simulación falló: el vault no puede debitar ${ticker} (falta saldo o aprobación).`;
	}

	if (parts.includes('insufficient funds')) {
		return 'No tenés ETH suficiente en Sepolia para simular el gas de la transacción.';
	}

	return 'No se pudo simular la transacción on-chain. El gas mostrado es aproximado.';
}

function collectBlockchainErrorText(err: unknown): string {
	const seen = new Set<unknown>();
	const chunks: string[] = [];

	const visit = (value: unknown, depth = 0) => {
		if (!value || depth > 6 || seen.has(value)) return;
		seen.add(value);

		if (typeof value === 'string') {
			chunks.push(value);
			return;
		}

		if (typeof value !== 'object') return;

		const error = value as {
			shortMessage?: string;
			message?: string;
			details?: string;
			name?: string;
			cause?: unknown;
			data?: { message?: string };
		};

		for (const part of [error.shortMessage, error.message, error.details, error.data?.message]) {
			if (part) chunks.push(part);
		}

		if (error.cause) visit(error.cause, depth + 1);
	};

	visit(err);
	return chunks.join(' ');
}

export function parseBlockchainTxError(
	err: unknown,
	context?: { ticker?: string; functionName?: string }
): string {
	const error = err as {
		shortMessage?: string;
		message?: string;
		name?: string;
		cause?: { shortMessage?: string; message?: string; name?: string };
	};

	const fullText = collectBlockchainErrorText(err);
	const parts = fullText.toLowerCase();

	if (
		parts.includes('user rejected') ||
		parts.includes('user denied') ||
		error?.name === 'UserRejectedRequestError'
	) {
		return 'Transacción cancelada.';
	}

	if (
		parts.includes('insufficient funds') ||
		parts.includes('insufficient balance for transfer') ||
		parts.includes('exceeds balance')
	) {
		if (context?.functionName === 'approve') {
			return 'No tenés ETH en Sepolia para pagar el gas. Conseguí ETH de prueba en un faucet de Sepolia.';
		}
		if (context?.ticker) {
			return `No tenés suficiente ${context.ticker} en tu wallet conectada para este monto.`;
		}
		return 'Saldo insuficiente para completar la transacción.';
	}

	if (parts.includes('insufficient allowance')) {
		return 'No se pudo autorizar el gasto del token. Intentá de nuevo.';
	}

	if (parts.includes('connector not connected') || parts.includes('wallet_not_ready')) {
		return 'La wallet no está lista para firmar. Si usás Google/Reown, cerrá sesión en el perfil y volvé a conectar.';
	}

	if (
		parts.includes('cannot convert eip155') ||
		(parts.includes('chain:') && parts.includes('undefined'))
	) {
		return 'La wallet embebida no pudo enviar la transacción en Sepolia. Recargá la página, volvé a iniciar sesión con Google y probá de nuevo.';
	}

	if (parts.includes('unknown error occurred while executing the contract function')) {
		const detailsMatch = fullText.match(/details:\s*([^\n]+)/i);
		const details = detailsMatch?.[1]?.trim();
		if (details && !details.toLowerCase().includes('unknown')) {
			return details.endsWith('.') ? details : `${details}.`;
		}

		const fn = context?.functionName ?? 'la transacción';
		if (fn === 'approve') {
			return 'No se pudo aprobar el gasto en Sepolia. Si tu saldo es correcto, recargá la página y volvé a conectar con Google.';
		}
		if (fn === 'fund' && context?.ticker) {
			return `No se pudo fondear en Sepolia. Verificá que tengas suficiente ${context.ticker} y ETH para el gas.`;
		}
		return `No se pudo completar ${fn} en Sepolia. Recargá la página y volvé a conectar tu wallet.`;
	}

	const shortMessage =
		error?.shortMessage || error?.cause?.shortMessage || error?.message || error?.cause?.message;

	if (shortMessage && !shortMessage.toLowerCase().includes('unknown error occurred')) {
		return shortMessage;
	}

	return 'Transacción rechazada o fallida.';
}

export function hasEnoughEthForGas(ethBalanceWei: bigint, requiredGasWei?: bigint): boolean {
	if (requiredGasWei !== undefined && requiredGasWei > 0n) {
		return ethBalanceWei >= requiredGasWei;
	}
	const minWei = BigInt(Math.floor(MIN_ETH_FOR_GAS * 1e18));
	return ethBalanceWei >= minWei;
}
