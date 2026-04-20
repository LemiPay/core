import type { ProposalStatus } from '$lib/types/endpoints/proposals.types';

export type ProposalStatusDisplay = {
	label: string;
	// Tailwind classes for the badge wrapper (borde + fondo + texto).
	classes: string;
};

// Mapeo del enum de ProposalStatus a su representación visual en el front.
export function getProposalStatusDisplay(status: ProposalStatus): ProposalStatusDisplay {
	switch (status) {
		case 'Approved':
			return {
				label: 'En curso',
				classes: 'border-blue-200 bg-blue-50 text-blue-700'
			};
		case 'Executed':
			return {
				label: 'Finalizada',
				classes: 'border-green-200 bg-green-50 text-green-700'
			};
		case 'Canceled':
			return {
				label: 'Cancelada',
				classes: 'border-red-200 bg-red-50 text-red-600'
			};
		case 'Pending':
			return {
				label: 'Pendiente',
				classes: 'border-gray-200 bg-gray-50 text-gray-600'
			};
		case 'Rejected':
			return {
				label: 'Rechazada',
				classes: 'border-red-200 bg-red-50 text-red-600'
			};
		case 'Expired':
			return {
				label: 'Expirada',
				classes: 'border-gray-200 bg-gray-50 text-gray-600'
			};
		case 'Failed':
			return {
				label: 'Fallida',
				classes: 'border-red-200 bg-red-50 text-red-600'
			};
	}
}
