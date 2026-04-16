export function generateRandomAddress(): string {
	const chars = '0123456789abcdef';
	let randomHex = '';
	for (let i = 0; i < 40; i++) {
		randomHex += chars[Math.floor(Math.random() * chars.length)];
	}
	return '0x' + randomHex;
}
export function shortenAddress(address: string) {
	if (address.length < 10) return address;
	return `${address.slice(0, 6)}...${address.slice(-4)}`;
}
