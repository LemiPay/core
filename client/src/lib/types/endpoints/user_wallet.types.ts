export type WalletInfo = {
	address: string;
	currencies: {
		wallet_id: string;
		ticker: string;
		balance: string;
	}[];
};
