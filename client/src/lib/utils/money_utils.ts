import { formatAmount } from './format_utils';

/** @deprecated Use formatAmount from format_utils instead */
export function roundBalance(balance: string | number): string {
	return formatAmount(balance);
}
