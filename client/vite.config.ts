import path from 'node:path';
import { fileURLToPath } from 'node:url';

import tailwindcss from '@tailwindcss/vite';
import { loadEnv } from 'vite';
import { defineConfig } from 'vitest/config';
import { playwright } from '@vitest/browser-playwright';
import { sveltekit } from '@sveltejs/kit/vite';

const clientDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(clientDir, '..');

/**
 * Prefer repo-root .env over client/.env.
 * process.env already set by the shell/CI still wins over both
 * (Vite never overwrites existing process.env keys from .env files).
 */
function preferRootEnv(mode: string) {
	const rootEnv = loadEnv(mode, repoRoot, '');

	for (const [key, value] of Object.entries(rootEnv)) {
		if (process.env[key] === undefined) {
			process.env[key] = value;
		}
	}
}

export default defineConfig(({ mode }) => {
	preferRootEnv(mode);

	return {
		// Keep loading client/.env as usual; root values already in process.env win.
		envDir: clientDir,
		plugins: [tailwindcss(), sveltekit()],
		test: {
			expect: { requireAssertions: true },
			projects: [
				{
					extends: true,
					test: {
						name: 'client',
						browser: {
							enabled: true,
							provider: playwright(),
							instances: [{ browser: 'chromium', headless: true }]
						},
						include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
						exclude: ['src/lib/server/**']
					}
				},

				{
					extends: true,
					test: {
						name: 'server',
						environment: 'node',
						include: ['src/**/*.{test,spec}.{js,ts}'],
						exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
					}
				}
			]
		}
	};
});
