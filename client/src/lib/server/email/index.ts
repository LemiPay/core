import { env } from '$env/dynamic/private';
import type { MailRequest } from '$lib/types/endpoints/api.types';

import { EmailClient } from '@azure/communication-email';

const env_vars = {
	conn: env.AZURE_CONNECTION_STRING,
	sender: env.AZURE_EMAIL
};

export async function sendEmail({ to, subject, text }: MailRequest) {
	if (!env_vars.sender) {
		throw new Error('AZURE_EMAIL is not set');
	}

	if (!env_vars.conn) {
		throw new Error('AZURE_CONNECTION_STRING is not set');
	}

	const client = new EmailClient(env_vars.conn);

	const message = {
		senderAddress: env_vars.sender,
		content: {
			subject,
			html: text
		},
		recipients: {
			to: [{ address: to }]
		}
	};

	const poller = await client.beginSend(message);
	const result = await poller.pollUntilDone();

	return result;
}
