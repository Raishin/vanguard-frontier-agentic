// svelte.config.js — DANGER: disables SvelteKit's built-in CSRF protection.
// Every POST/PUT/PATCH/DELETE form submission from any origin will now be
// accepted, defeating the framework's default cross-site request forgery guard.
import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		csrf: {
			checkOrigin: false
		}
	}
};

export default config;
