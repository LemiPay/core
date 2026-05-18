import { json, error, type RequestHandler } from '@sveltejs/kit';
import { sendEmail } from '$lib/server/email';
import { env } from '$env/dynamic/private';
import type { MailRequest } from '$lib/types/endpoints/api.types';

export const POST: RequestHandler = async ({ request }) => {
	let req: unknown;

	try {
		req = await request.json();
	} catch {
		error(400, 'Invalid JSON');
	}

	const internalSecret = request.headers.get('x-internal-secret');

	if (internalSecret !== env.MAIL_SERVER_SECRET) {
		error(401, 'Unauthorized');
	}

	try {
		const parsedReq = parseMailRequest(req);

		await sendEmail(parsedReq);
		return json({ ok: true });
	} catch {
		error(400, 'Invalid mail request');
	}
};

function parseMailRequest(req: unknown): MailRequest {
	if (typeof req !== 'object' || req === null) {
		throw new Error('Invalid mail request');
	}

	const obj = req as Record<string, unknown>;

	if (
		typeof obj.subject !== 'string' ||
		typeof obj.text !== 'string' ||
		typeof obj.to !== 'string'
	) {
		throw new Error('Invalid mail request');
	}

	return {
		subject: obj.subject,
		text: obj.text,
		to: obj.to
	};
}
