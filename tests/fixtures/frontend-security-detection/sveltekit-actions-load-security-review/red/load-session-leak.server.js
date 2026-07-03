// src/routes/account/+page.server.js — DANGER: the raw session cookie is
// passed straight into the database lookup with no auth-guard call anywhere
// in this file. Any request carrying a guessable/forged `sessionid` cookie
// value gets a full account record back — there is no verification step,
// only a lookup.
import * as db from '$lib/server/db';

/** @type {import('./$types').PageServerLoad} */
export async function load({ cookies }) {
	const account = await db.getAccount(cookies.get('sessionid'));
	return { account };
}
