# Routing Conventions: File Roles and Precedence

Use this reference only when a review also needs to assess `+layout`/`+page`/`+server` file precedence or route-tree structure — not for a load-placement-only review (see `workflow-and-output.md` for that).

## Officially grounded file roles

Per SvelteKit's routing documentation:

- **`+page.svelte`** — the component rendered for a route.
- **`+page.js`** — exports a universal `load` (typed `PageLoad`). Runs on the server during SSR/hydration, then again in the browser on subsequent client-side navigations (subject to page options).
- **`+page.server.js`** — exports a server `load` (typed `PageServerLoad`), always server-only. Renaming `+page.js` to `+page.server.js` is the documented mechanism for moving a load function that needs a database or private env var out of universal-execution scope. Can also export form `actions`.
- **`+layout.svelte`** — wraps child routes; layouts nest by directory structure.
- **`+layout.js` / `+layout.server.js`** — same universal/server split as page equivalents, but the returned data is available to the layout's own component and to every child route via `await parent()`.
- **`+server.js`/`+server.ts`** — exports HTTP method handlers (`GET`, `POST`, etc.) for a route, making it an API endpoint rather than a page. Always server-only.

## Precedence and data flow

- If a route has both a server `load` and a universal `load` (e.g., `+page.server.js` and `+page.js` in the same directory), **the server `load` runs first**. Its return value becomes the `data` property on the `LoadEvent` passed into the universal `load`. Do not describe these as running "in parallel" or "independently" — they are sequential and dependent.
- A server `load`'s return value must be serializable via devalue (JSON-representable types plus `BigInt`, `Date`, `Map`, `Set`, `RegExp`, and repeated/cyclical references). A universal `load` has no such restriction and may return non-serializable values (component constructors, class instances, functions) — this asymmetry is a useful signal during review: if a "server load" is returning something devalue cannot serialize, either the classification is wrong or the code will fail at runtime, not just leak.
- A `+layout.server.js`'s data is available to every nested route's `load` via the `parent()` function — trace `parent()` calls when checking whether a leak in a layout's server `load` return value actually reaches a specific deeply nested universal `load`.
- Prerendered routes invoke `load` at build time, not per-request — flag this distinction only if the review scope includes prerendering/page-option correctness; it does not change the leak-tracing rules above.

## Review-relevant precedence pitfalls

- Do not assume a `+page.server.js` with no corresponding `+page.js` implies the route has no client-reachable data flow — the server `load`'s return value still reaches the client as `data`/`$page.data` for the page component itself, even with no universal `load` present. Apply the same leak-tracing rules from `workflow-and-output.md` regardless of whether a sibling universal `load` exists.
- Do not assume `+layout.js`/`+layout.server.js` data is scoped narrowly — it is available to the entire subtree beneath that layout, which widens the blast radius of any leak found there compared to a single `+page.js`.
- A `+server.ts` in the same directory as a `+page.js`/`+page.server.js` is a separate concern from page load functions; do not conflate an API endpoint's own request-handling logic with the page's load-function review unless the page's universal `load` calls that endpoint via `fetch()`, in which case trace the endpoint's response shape for any leaked field the same way you would trace a server `load`'s return value.

## When to push back

Push back if the user asks you to:

- "just put it in `+page.js` so it fetches faster" when the data source requires a private credential or database access — that is not faster, it is a credential-exposure defect.
- treat a `+layout.server.js` leak as low-severity because "it's just the layout" — layout-scoped leaks reach a wider client-visible subtree than a single page's leak, not a narrower one.
- skip verifying the installed SvelteKit version's `$env`/server-only-module enforcement because "the docs say it's blocked" — documentation describes the current release's behavior; it does not prove the repo's pinned version enforces it the same way.
