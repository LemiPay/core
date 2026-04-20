export type WalletCurrency = {
	wallet_id: string;
	address: string;
	currency_id: string;
	balance: string;
	ticker: string;
};

export type AddressGroup = {
	address: string;
	currencies: WalletCurrency[];
};
