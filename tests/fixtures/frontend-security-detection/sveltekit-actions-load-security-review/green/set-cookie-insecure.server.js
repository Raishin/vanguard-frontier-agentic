// src/routes/login/+page.server.js — Safe: httpOnly, secure, sameSite, and
// path are all explicit and set to their secure values, matching SvelteKit's
// documented cookie defaults.
import * as db from '$lib/server/db';

/** @satisfies {import('./$types').Actions} */
export const actions = {
	login: async ({ cookies, request }) => {
		const data = await request.formData();
		const user = await db.getUser(data.get('email'));
		const token = await db.createSession(user);
		cookies.set('sessionid', token, { path: '/', httpOnly: true, secure: true, sameSite: 'lax' });
		return { success: true };
	}
};
