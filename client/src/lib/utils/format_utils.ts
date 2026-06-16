export function parseBalanceValue(v: string | number): number {
	if (typeof v === 'number') return Number.isFinite(v) ? v : 0;
	const n = Number(v);
	return Number.isFinite(n) ? n : 0;
}

export const DISPLAY_DECIMALS = 4;

/** Truncates toward zero — never rounds up (e.g. 9.99999 → 9.9999 at 4 dp). */
export function truncateToDecimals(value: number, decimals: number = DISPLAY_DECIMALS): number {
	if (!Number.isFinite(value)) return 0;
	const factor = 10 ** decimals;
	return Math.trunc(value * factor) / factor;
}

export function formatAmount(value: string | number, decimals: number = DISPLAY_DECIMALS): string {
	return truncateToDecimals(parseBalanceValue(value), decimals).toFixed(decimals);
}

export function formatMoney(value: string | number, currency = 'USD'): string {
	const n = parseBalanceValue(value);
	const sign = n < 0 ? '-' : '';
	return `${sign}$${formatAmount(Math.abs(n))} ${currency}`;
}

export function getInitials(name: string): string {
	if (!name) return '?';
	return (
		name
			.trim()
			.split(/\s+/)
			.slice(0, 2)
			.map((p) => p[0]?.toUpperCase() ?? '')
			.join('') || '?'
	);
}

export function formatDate(value: string): string {
	return new Date(value).toLocaleDateString('es-AR', {
		day: '2-digit',
		month: '2-digit',
		year: 'numeric'
	});
}

export function formatTxType(t: string): string {
	const key = t.toLowerCase();
	const map: Record<string, string> = {
		deposit: 'Depósito',
		withdraw: 'Retiro',
		expense: 'Gasto',
		investment: 'Inversión',
		settlement_payment: 'Pago de deuda',
		claim: 'Retiro final',
		fund: 'Fondeo'
	};
	return map[key] ?? t;
}

export function formatDateTimeShort(iso: string): string {
	const d = new Date(iso);
	if (Number.isNaN(d.getTime())) return iso;
	return d.toLocaleString('es-AR', { dateStyle: 'short', timeStyle: 'short' });
}
