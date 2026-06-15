export function getWithdrawAppOrigin(): string {
	if (typeof window !== 'undefined') return window.location.origin;
	return 'https://lemipay.app';
}

export function buildWithdrawSignMessage(params: {
	walletId: string;
	amount: string;
	address: string;
	uri?: string;
}): string {
	const uri = params.uri ?? getWithdrawAppOrigin();
	return [
		'lemipay.app quiere autorizar un retiro:',
		'',
		`Wallet: ${params.walletId}`,
		`Monto: ${params.amount}`,
		`Address: ${params.address}`,
		'',
		`URI: ${uri}`
	].join('\n');
}
