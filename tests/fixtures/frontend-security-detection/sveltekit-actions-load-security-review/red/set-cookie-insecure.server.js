// src/routes/login/+page.server.js — DANGER: httpOnly is explicitly disabled,
// so the session cookie is readable from client-side JavaScript. Any XSS
// elsewhere in the app (or a malicious third-party script) can exfiltrate it.
import * as db from '$lib/server/db';

/** @satisfies {import('./$types').Actions} */
export const actions = {
	login: async ({ cookies, request }) => {
		const data = await request.formData();
		const user = await db.getUser(data.get('email'));
		const token = await db.createSession(user);
		cookies.set('sessionid', token, { path: '/', httpOnly: false, secure: true, sameSite: 'lax' });
		return { success: true };
	}
};
