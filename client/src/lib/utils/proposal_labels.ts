import type { ProposalStatus } from '$lib/types/endpoints/proposals.types';
import type { ProposalStatusDisplay } from '$lib/utils/proposal_status';
import { getProposalStatusDisplay } from '$lib/utils/proposal_status';

export type ProposalKind = 'new_member' | 'withdraw' | 'fund_round' | 'investment' | 'all';

export type ProposalKindDisplay = {
	label: string;
	classes: string;
};

const KIND_LABELS: Record<Exclude<ProposalKind, 'all'>, string> = {
	new_member: 'Nuevo miembro',
	withdraw: 'Retiro',
	fund_round: 'Ronda de fondos',
	investment: 'Inversión'
};

const KIND_CLASSES: Record<Exclude<ProposalKind, 'all'>, string> = {
	new_member:
		'border-violet-200 bg-violet-50 text-violet-700 dark:border-violet-400/30 dark:bg-violet-500/15 dark:text-violet-200',
	withdraw:
		'border-orange-200 bg-orange-50 text-orange-700 dark:border-orange-400/30 dark:bg-orange-500/15 dark:text-orange-200',
	fund_round:
		'border-sky-200 bg-sky-50 text-sky-700 dark:border-sky-400/30 dark:bg-sky-500/15 dark:text-sky-200',
	investment:
		'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/30 dark:bg-emerald-500/15 dark:text-emerald-200'
};

function normalizeKind(kind: string): Exclude<ProposalKind, 'all'> | null {
	const normalized = kind
		.toLowerCase()
		.replace(/([a-z])([A-Z])/g, '$1_$2')
		.toLowerCase();

	if (normalized === 'newmember' || normalized === 'new_member') return 'new_member';
	if (normalized === 'withdraw') return 'withdraw';
	if (normalized === 'fundround' || normalized === 'fund_round') return 'fund_round';
	if (normalized === 'investment') return 'investment';
	return null;
}

export function getProposalKindDisplay(kind: string): ProposalKindDisplay {
	const normalized = normalizeKind(kind);
	if (!normalized) {
		return {
			label: 'Desconocido',
			classes:
				'border-slate-200 bg-slate-50 text-slate-700 dark:border-slate-400/30 dark:bg-slate-500/15 dark:text-slate-200'
		};
	}

	return {
		label: KIND_LABELS[normalized],
		classes: KIND_CLASSES[normalized]
	};
}

export function getGovernanceStatusDisplay(
	status: ProposalStatus | string | null | undefined
): ProposalStatusDisplay {
	const normalized = typeof status === 'string' ? status.toLowerCase() : status;

	switch (normalized) {
		case 'active':
			return {
				label: 'Aprobada',
				classes:
					'border-blue-200 bg-blue-50 text-blue-700 dark:border-blue-400/30 dark:bg-blue-500/15 dark:text-blue-200'
			};
		case 'matured':
			return {
				label: 'Vencida',
				classes:
					'border-slate-200 bg-slate-50 text-slate-700 dark:border-slate-400/30 dark:bg-slate-500/15 dark:text-slate-200'
			};
		case 'withdrawn':
			return {
				label: 'Retirada',
				classes:
					'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/30 dark:bg-emerald-500/15 dark:text-emerald-200'
			};
		default:
			return getProposalStatusDisplay(status as ProposalStatus);
	}
}
