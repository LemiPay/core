export function roundBalance(balance: string, decimals: number = 5): string {
	console.log(balance);
	return parseFloat(balance).toFixed(decimals);
}
