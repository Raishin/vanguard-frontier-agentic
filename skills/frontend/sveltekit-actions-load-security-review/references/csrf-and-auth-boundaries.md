# CSRF and Auth-Boundary Review

Use this reference only when the review scope includes `svelte.config.js`'s `csrf` block, a form action, or an auth-guard trace across `load()`/`+layout.server.js`/`hooks.server.js`.

## CSRF: `checkOrigin` and `trustedOrigins`

SvelteKit blocks cross-site form submissions by default. At request time, it rejects a form-content-type `POST`/`PUT`/`PATCH`/`DELETE` request whenever the request's origin does not match the app's own origin and is not present in a trusted-origins allowlist. Two config knobs control this:

- `kit.csrf.checkOrigin` — `true` by default. Setting it to `false` disables the origin check entirely.
- `kit.csrf.trustedOrigins` — an allowlist of additional origins permitted to submit forms cross-site. Setting it to `['*']` disables the check just as thoroughly as `checkOrigin: false`, regardless of what `checkOrigin` is set to — SvelteKit's own config resolution only enables the check when `checkOrigin` is true **and** `trustedOrigins` does not contain `'*'`.

### Non-negotiable design rules

1. **Treat `checkOrigin: false` and a `'*'` entry in `trustedOrigins` as the same severity of finding.** Both fully defeat the CSRF guard; a reviewer who only greps for `checkOrigin` will miss the wildcard-origin bypass.
2. **A named `trustedOrigins` list is the correct pattern for legitimate cross-site integrations** (e.g., a trusted partner site embedding a form that posts to this app) — flag only the wildcard, not the presence of the option itself.
3. **The check only applies to form-content-type requests with a body-bearing method.** JSON API requests handled by `+server.js` endpoints using `fetch` with a custom header are a different trust boundary (typically same-origin `fetch` plus `SameSite` cookie behavior) — do not conflate the two when scoping a finding.

## Auth boundary: where does the guard actually run?

SvelteKit's own documentation is explicit that `load()`-based auth guards have sharp edges:

- Layout `load()` functions do **not** necessarily re-run on every request — in particular, client-side navigation between child routes can skip re-running a parent layout's `load()`.
- Layout and page `load()` functions run **concurrently** by default. A child `load()` does not automatically see or wait for the parent's result unless it explicitly calls `await parent()`.
- The two supported ways to guarantee an auth check runs before protected code: (a) enforce it in `hooks.server.js`'s `handle` function, which runs before any `load()` for every matching request, or (b) put the guard directly in the specific `+page.server.js`/`+server.js` that needs it.

### Non-negotiable design rules

1. **Do not accept "the layout checks auth" as proof for a specific child page.** Confirm the child's own `load()` calls `await parent()` and inspects the result (e.g., redirects or throws if `parent()`'s data shows no user), or confirm `hooks.server.js` enforces the guard independently of any `load()`.
2. **`hooks.server.js`'s `handle` is the strongest guarantee** because it runs before `resolve(event)`, which is what triggers `load()` execution — a guard here covers every route it matches regardless of individual `load()` implementations.
3. **A raw `cookies.get(...)` value is not an identity.** Whatever helper resolves "who is this user" must perform a verifying lookup (session store, signed/encrypted cookie, database check) — passing the cookie's raw string value straight into a data query or a trust decision is an unguarded read keyed on attacker-controlled input, not authentication.
4. **Form actions need their own check.** A `load()` guard on the page does not protect the co-located `export const actions` handlers — SvelteKit invokes an action directly on form submission; verify each action independently.

## Minimal safe implementation pattern

```js
// svelte.config.js — safe: default checkOrigin, named trusted origins only if needed
const config = {
	kit: {
		// csrf.checkOrigin defaults to true; omit unless you have a documented reason to touch it.
		csrf: {
			trustedOrigins: ['https://trusted-partner.example.com']
		}
	}
};
```

```js
// src/hooks.server.js — safe: guard enforced before any load() runs
export async function handle({ event, resolve }) {
	event.locals.user = await getUserFromSession(event.cookies.get('sessionid'));
	return resolve(event);
}
```

```js
// src/routes/account/+page.server.js — safe: guard re-checked at the exact protected path
import { requireLogin } from '$lib/server/auth';

export async function load(event) {
	const user = requireLogin(event); // throws/redirects if event.locals.user is absent
	return { message: `hello ${user.name}!` };
}
```

Anti-pattern (guard only in the parent layout, never confirmed by the child):

```js
// src/routes/(protected)/+layout.server.js
export async function load({ locals }) {
	if (!locals.user) throw redirect(303, '/login');
	return { user: locals.user };
}
```

```js
// src/routes/(protected)/account/+page.server.js — WRONG: never calls parent(),
// so this page's own load() has no guard of its own, and if this layout load
// is skipped on a given navigation, nothing here catches it.
import * as db from '$lib/server/db';

export async function load({ cookies }) {
	return { account: await db.getAccount(cookies.get('sessionid')) };
}
```

## Verification targets

- Grep `svelte.config.js` for `checkOrigin` and `trustedOrigins`; read the exact boolean/array value.
- Grep every `+page.server.js`/`+layout.server.js`/`+server.js` for `cookies.get(` and follow each result to its next use.
- Grep for `requireLogin`, `locals.user`, or an equivalent verifying-helper name to confirm a guard call actually exists on the traced path — its absence, combined with a `cookies.get(` feeding a data call, is the clearest structural signal of this defect class.
- Grep child `load()` functions in a protected route tree for `parent()` to confirm they actually consume the parent's guard result rather than relying on it implicitly.
