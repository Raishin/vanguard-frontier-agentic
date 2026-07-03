// svelte.config.js — Safe: relies on SvelteKit's default CSRF behavior.
// checkOrigin defaults to true, so the property is left out entirely rather
// than explicitly toggled — cross-site form submissions are rejected by default.
import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter()
		// csrf.checkOrigin intentionally omitted — the framework default (true) applies.
	}
};

export default config;
