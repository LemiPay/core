export function generateRandomAddress(): string {
	const randomBytes = new Uint8Array(20);
	globalThis.crypto.getRandomValues(randomBytes);
	const randomHex = Array.from(randomBytes, (byte) => byte.toString(16).padStart(2, '0')).join('');
	return '0x' + randomHex;
}
export function shortenAddress(address: string) {
	if (address.length < 10) return address;
	return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

export function copyToClipboard(text: string) {
	navigator.clipboard.writeText(text);
}
