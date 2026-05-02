export function formatAmount(value: number): string {
	return value.toFixed(2);
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

export function formatExpenseDate(value: string): string {
	return new Date(value).toLocaleDateString('es-AR', {
		day: '2-digit',
		month: '2-digit',
		year: 'numeric'
	});
}

export function parseBalanceValue(v: string | number): number {
	if (typeof v === 'number') return Number.isFinite(v) ? v : 0;
	const n = Number(v);
	return Number.isFinite(n) ? n : 0;
}

export function formatTxType(t: string): string {
	const map: Record<string, string> = {
		Deposit: 'Depósito',
		Withdraw: 'Retiro',
		Expense: 'Gasto',
		Investment: 'Inversión'
	};
	return map[t] ?? t;
}

export function formatDateTimeShort(iso: string): string {
	const d = new Date(iso);
	if (Number.isNaN(d.getTime())) return iso;
	return d.toLocaleString('es-AR', { dateStyle: 'short', timeStyle: 'short' });
}
