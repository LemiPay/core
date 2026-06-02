import type { FundRoundStatusResponse } from '$lib/types/endpoints/fund_rounds.types';
import type { WalletCurrency } from '$lib/types/endpoints/wallets.types';

export interface StatusDisplay {
	label: string;
	classes: string;
}

export interface FundRoundCardProps {
	status: FundRoundStatusResponse;

	expandedFundRoundId: string | null;
	selectedContribWalletId: string;

	recommended: number;
	myContribution: number;

	ticker: string;

	compatibleWallets: WalletCurrency[];

	contribLoading: boolean;
	contribError: string;

	memberCount: number;

	currentUserId: string | null;

	statusDisplay: StatusDisplay;

	formatAmount: (value: number) => string;
	shortenAddress: (address: string) => string;

	onToggleAccordion: (proposalId: string) => void;
	onCancelRound: (proposalId: string) => void;
	onContribute: (status: FundRoundStatusResponse) => Promise<void>;
}
