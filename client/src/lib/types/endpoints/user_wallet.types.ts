export type WalletInfo = {
	address: string;
	currencies: {
		wallet_id: string;
		ticker: string;
		balance: string;
	}[];
};

export type Wallet = {
	wallet_id: string;
	address: string;
	currency_id: string;
	balance: string;
};
