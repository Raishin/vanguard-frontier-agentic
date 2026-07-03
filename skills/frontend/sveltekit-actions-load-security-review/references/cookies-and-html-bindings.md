# Cookies and {@html} Bindings Review

Use this reference only when the review scope includes a `cookies.set()` call or an `{@html}` template binding.

## Cookie policy: `cookies.set()`

SvelteKit's `cookies.set()` ships with secure-by-default options: `httpOnly` and `secure` both default to `true` (with `secure` relaxed to `false` only when the request is over plain HTTP on `localhost`, a development convenience), and `sameSite` defaults to `'lax'`. Since SvelteKit v2, `path` must be passed explicitly to `cookies.set()`/`cookies.delete()`/`cookies.serialize()` — the framework will not silently infer one.

### Non-negotiable design rules

1. **A call that explicitly sets `httpOnly: false` on a session/identity cookie is always a HIGH finding.** It opts out of the framework's own secure default specifically to make the cookie readable from client-side JavaScript — there is no legitimate reason to do this for a session token.
2. **A call that explicitly sets `secure: false` outside of a documented local-development context is a HIGH finding** on any cookie that carries session or identity information — it permits the cookie to be sent over plain HTTP.
3. **A missing `sameSite` is not automatically a finding** (the framework default `'lax'` applies), but an explicit `sameSite: 'none'` on a session cookie without a matching `secure: true` is a HIGH finding — `SameSite=None` requires `Secure` per current browser cookie-handling rules.
4. **A missing `path` is at least a MEDIUM finding** — SvelteKit requires it explicitly since v2 precisely because omitting it previously led to ambiguous, broader-than-intended cookie scoping.
5. **Do not flag a `cookies.set()` call that only sets non-sensitive, non-identity state** (e.g., a UI preference like `theme=dark`) with the same severity as a session-cookie finding — check what the cookie actually carries before assigning severity.

## XSS: `{@html}`

Svelte's `{@html ...}` tag injects the given string as raw HTML into the DOM with no escaping or sanitization — it is the direct analog of setting `innerHTML`. Any user-reachable input reaching this tag unsanitized is a stored or reflected XSS vector.

### Non-negotiable design rules

1. **Trace the `{@html}` argument to its origin.** Follow it backward through component props, `load()` return data, form submissions, and third-party API responses. If any hop in that chain includes content a user or attacker can influence (a comment body, a profile bio, a query parameter reflected into a response, a webhook payload rendered later), the source counts as user-reachable.
2. **Require a named sanitizer call on the exact traced expression**, not merely present somewhere in the codebase. `{@html DOMPurify.sanitize(value)}` clears the bar; `{@html value}` with a `DOMPurify` import sitting unused nearby does not.
3. **Fully origin-controlled content is not a finding** — static marketing copy or developer-authored strings with no code path for a user to influence them can be rendered via `{@html}` safely. State this explicitly in the review output rather than silently skipping it, so the scope decision is visible.
4. **Do not accept "we sanitize on the way into the database" as sufficient** unless you can show the specific write path that populates this exact field always runs through the sanitizer — a second write path (an admin tool, a migration script, a different API route) that bypasses it reintroduces the vulnerability at the same render site.

## Minimal safe implementation patterns

```js
// src/routes/login/+page.server.js — safe cookie policy
export const actions = {
	login: async ({ cookies, request }) => {
		const data = await request.formData();
		const user = await db.getUser(data.get('email'), data.get('password'));
		const token = await db.createSession(user);
		cookies.set('sessionid', token, {
			path: '/',
			httpOnly: true,
			secure: true,
			sameSite: 'lax'
		});
		return { success: true };
	}
};
```

```svelte
<!-- src/lib/components/CommentBody.svelte — safe: sanitized on the exact traced path -->
<script>
	import DOMPurify from 'dompurify';
	let { comment } = $props();
</script>

<div class="comment-body">
	{@html DOMPurify.sanitize(comment.body)}
</div>
```

Anti-patterns (do not approve):

```js
// WRONG: httpOnly disabled, cookie readable from any injected/XSS'd script
cookies.set('sessionid', token, { path: '/', httpOnly: false, secure: true, sameSite: 'lax' });
```

```svelte
<!-- WRONG: user-submitted comment.body rendered with no sanitizer on this path -->
<div class="comment-body">
	{@html comment.body}
</div>
```

## Verification targets

- Grep for `cookies.set(` across `+page.server.js`, `+layout.server.js`, `+server.js`, and form action code; for each match, read the full options object and confirm `httpOnly`, `secure`, `sameSite`, and `path` are all explicit.
- Grep `.svelte` files for `{@html` and, for each match, trace the bound expression backward through the component's props and any `load()`/store data that feeds it.
- Grep for `DOMPurify`, `sanitize-html`, or an equivalent sanitizer import; confirm it is actually called inline on the specific `{@html}` expression under review, not merely imported.
