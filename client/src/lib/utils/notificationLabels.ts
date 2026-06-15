export const EVENT_LABELS: Record<string, string> = {
	withdraw_proposal_created: 'Propuesta de retiro',
	proposal_approved: 'Propuesta aprobada',
	proposal_rejected: 'Propuesta rechazada',
	proposal_executed: 'Propuesta ejecutada',
	fund_round_created: 'Ronda de fondeo creada',
	investment_created: 'Inversión creada',
	investment_matured: 'Inversión madurada',
	new_member_added: 'Invitación a un grupo',
	expense_created: 'Gasto creado',
	welcome: 'Bienvenida',
	login_alert: 'Alerta de inicio de sesión'
};

export type ActivityVariant = 'purple' | 'blue' | 'green' | 'yellow' | 'lime' | 'amber';

const EVENT_VARIANTS: Record<string, ActivityVariant> = {
	withdraw_proposal_created: 'purple',
	proposal_approved: 'green',
	proposal_rejected: 'amber',
	proposal_executed: 'green',
	fund_round_created: 'lime',
	investment_created: 'blue',
	investment_matured: 'lime',
	new_member_added: 'amber',
	expense_created: 'yellow',
	welcome: 'purple',
	login_alert: 'blue'
};

export function getEventLabel(eventName: string): string {
	return EVENT_LABELS[eventName] ?? eventName;
}

export function getNotificationTitle(eventName: string, groupName: string | null): string {
	return getEventLabel(eventName);
}

export function getNotificationDetail(eventName: string, groupName: string | null): string {
	if (groupName) {
		return groupName;
	}
	return 'Notificación del sistema';
}

export function getEventVariant(eventName: string): ActivityVariant {
	return EVENT_VARIANTS[eventName] ?? 'blue';
}

export function getEventIconContainerClasses(variant: ActivityVariant): string {
	if (variant === 'green') return 'bg-emerald-400/15 text-emerald-700 dark:text-emerald-300';
	if (variant === 'yellow') return 'bg-amber-400/15 text-amber-700 dark:text-amber-300';
	if (variant === 'purple') return 'bg-violet-400/15 text-violet-700 dark:text-violet-300';
	if (variant === 'lime') return 'bg-lime-400/15 text-lime-700 dark:text-lime-300';
	if (variant === 'amber') return 'bg-orange-400/15 text-orange-700 dark:text-orange-300';
	return 'bg-sky-400/15 text-sky-700 dark:text-sky-300';
}

export function getActivityVariantClasses(variant: ActivityVariant): string {
	return getEventIconContainerClasses(variant);
}

export function formatRelativeTime(createdAt: string): string {
	const date = new Date(createdAt.includes('T') ? createdAt : `${createdAt}Z`);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMin = Math.floor(diffMs / 60_000);

	if (diffMin < 1) return 'Ahora';
	if (diffMin < 60) return `Hace ${diffMin} min`;

	const diffHours = Math.floor(diffMin / 60);
	if (diffHours < 24) return `Hace ${diffHours} h`;

	const isToday = date.toDateString() === now.toDateString();
	if (isToday) return 'Hoy';

	const yesterday = new Date(now);
	yesterday.setDate(now.getDate() - 1);
	if (date.toDateString() === yesterday.toDateString()) return 'Ayer';

	return date.toLocaleDateString('es-AR', { day: 'numeric', month: 'short' });
}
