// svelte.config.js — DANGER: trustedOrigins: ['*'] disables the origin check
// entirely (per SvelteKit's own csrf_check_origin resolution: it is true only
// when checkOrigin is true AND trustedOrigins does not contain '*').
import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		csrf: {
			trustedOrigins: ['*']
		}
	}
};

export default config;
