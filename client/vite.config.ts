import path from 'node:path';
import { fileURLToPath } from 'node:url';

import tailwindcss from '@tailwindcss/vite';
import { loadEnv, type Plugin, type Rollup } from 'vite';
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

/** RSC directives ("use client" / "use server") are meaningless outside Next.js and spam the build log. */
function stripRscDirectives(): Plugin {
	const directiveRe = /^['"]use (?:client|server)['"];?\r?\n?/;

	return {
		name: 'strip-rsc-directives',
		enforce: 'pre',
		transform(code, id) {
			if (!id.includes('node_modules') || !directiveRe.test(code)) return;
			return { code: code.replace(directiveRe, ''), map: null };
		},
		configResolved(config) {
			const options = config.build.rollupOptions;
			const userOnWarn = options.onwarn;

			options.onwarn = (warning: Rollup.RollupLog, defaultHandler) => {
				if (
					warning.code === 'MODULE_LEVEL_DIRECTIVE' ||
					(typeof warning.message === 'string' &&
						warning.message.includes('Module level directives'))
				) {
					return;
				}
				if (userOnWarn) {
					userOnWarn(warning, defaultHandler);
				} else {
					defaultHandler(warning);
				}
			};
		}
	};
}

export default defineConfig(({ mode }) => {
	preferRootEnv(mode);

	return {
		// Keep loading client/.env as usual; root values already in process.env win.
		envDir: clientDir,
		plugins: [stripRscDirectives(), tailwindcss(), sveltekit()],
		build: {
			// Web3 deps (wagmi/reown) produce large vendor chunks by design.
			chunkSizeWarningLimit: 1500
		},
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
