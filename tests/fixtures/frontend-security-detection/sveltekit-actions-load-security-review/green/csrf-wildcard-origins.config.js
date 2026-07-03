// svelte.config.js — Safe: names specific trusted origins instead of a wildcard.
import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		csrf: {
			trustedOrigins: ['https://trusted-partner.example.com']
		}
	}
};

export default config;
