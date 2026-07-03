// src/routes/account/+page.server.js — Safe: the session cookie is resolved
// and verified through requireLogin() before any data lookup happens, so an
// invalid or forged session id fails the auth check instead of reaching db.
import * as db from '$lib/server/db';
import { requireLogin } from '$lib/server/auth';

/** @type {import('./$types').PageServerLoad} */
export async function load(event) {
	const user = requireLogin(event);
	const account = await db.getAccount(user.id);
	return { account };
}
