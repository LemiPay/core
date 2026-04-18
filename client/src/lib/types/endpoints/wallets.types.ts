export type WalletCurrency = {
	wallet_id: string;
	address: string;
	balance: string;
	ticker: string;
};

export type AddressGroup = {
	address: string;
	currencies: WalletCurrency[];
};
