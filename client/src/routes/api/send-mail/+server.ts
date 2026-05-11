import { json, error, type RequestHandler } from '@sveltejs/kit';
import { sendEmail } from '$lib/server/email';
import { env } from '$env/dynamic/private';

export const POST: RequestHandler = async ({ request, locals }) => {
	const req = await request.json();

	// Log
	console.log('Received request to send email');
	console.log(req);

	const internalSecret = request.headers.get('x-internal-secret');

	if (internalSecret !== env.MAIL_SERVER_SECRET) {
		throw error(401, 'Unauthorized');
	}

	await sendEmail(req[0]);

	return json({ ok: true });
};
