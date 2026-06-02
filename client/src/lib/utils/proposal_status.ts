import type { ProposalStatus } from '$lib/types/endpoints/proposals.types';

export type ProposalStatusDisplay = {
	label: string;
	// Tailwind classes for the badge wrapper (borde + fondo + texto).
	classes: string;
};

// Mapeo del enum de ProposalStatus a su representación visual en el front.
export function getProposalStatusDisplay(
	status: ProposalStatus | string | null | undefined
): ProposalStatusDisplay {
	switch (status) {
		case 'Approved':
			return {
				label: 'En curso',
				classes:
					'border-blue-200 bg-blue-50 text-blue-700 dark:border-blue-400/30 dark:bg-blue-500/15 dark:text-blue-200'
			};
		case 'Executed':
			return {
				label: 'Finalizada',
				classes:
					'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-400/30 dark:bg-emerald-500/15 dark:text-emerald-200'
			};
		case 'Canceled':
			return {
				label: 'Cancelada',
				classes:
					'border-red-200 bg-red-50 text-red-700 dark:border-red-400/30 dark:bg-red-500/15 dark:text-red-200'
			};
		case 'Pending':
			return {
				label: 'Pendiente',
				classes:
					'border-amber-200 bg-amber-50 text-amber-700 dark:border-amber-400/30 dark:bg-amber-500/15 dark:text-amber-200'
			};
		case 'Rejected':
			return {
				label: 'Rechazada',
				classes:
					'border-red-200 bg-red-50 text-red-700 dark:border-red-400/30 dark:bg-red-500/15 dark:text-red-200'
			};
		case 'Expired':
			return {
				label: 'Expirada',
				classes:
					'border-slate-200 bg-slate-50 text-slate-700 dark:border-slate-400/30 dark:bg-slate-500/15 dark:text-slate-200'
			};
		case 'Failed':
			return {
				label: 'Fallida',
				classes:
					'border-red-200 bg-red-50 text-red-700 dark:border-red-400/30 dark:bg-red-500/15 dark:text-red-200'
			};
		default:
			return {
				label: 'Desconocido',
				classes:
					'border-slate-200 bg-slate-50 text-slate-700 dark:border-slate-400/30 dark:bg-slate-500/15 dark:text-slate-200'
			};
	}
}
