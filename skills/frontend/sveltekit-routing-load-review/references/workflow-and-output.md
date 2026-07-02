# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the leak-tracing decision tree, and the required output shape for a SvelteKit routing/load review.

> Version note: `$env/*` client-import enforcement and server-only module protection are Vite-plugin-driven and have evolved across SvelteKit releases. Verify the installed `@sveltejs/kit` version in `package.json` before asserting exact enforcement behavior; do not assume the latest documented behavior applies to an older installed version.

## What people get wrong

The naive assumption is:

> "`+page.js` is the client file and `+page.server.js` is the server file — as long as I don't put a DB call directly in `+page.js`, I'm safe."

That is incomplete in two ways:

1. `+page.js` (and `+layout.js`) are **universal** — SvelteKit runs them on the server for the first SSR render, then re-runs them **in the browser** for every subsequent client-side navigation. It is not "the client file"; it is "the file that runs in both places," and the browser-execution half is where secrets leak.
2. A secret can leak without ever being imported into a universal `load` file directly — it leaks the moment it appears anywhere inside the plain object a server `load` *returns*, because that object is serialized (via devalue) and shipped to the browser as `data`/`$page.data` regardless of whether anything downstream "needed" it.

## Step-by-step workflow

1. **Inventory route files.** For every route directory in scope, list `+page.js`, `+page.server.js`, `+layout.js`, `+layout.server.js`, and `+server.ts`/`+server.js` present. Classify each by filename suffix alone: `.server.` → server load, otherwise → universal load. Do not infer classification from file content or route name.
2. **Confirm page options.** Read any `export const ssr` / `export const csr` in `+page.js`/`+layout.js` for the route. If `ssr = false`, the "universal load runs server-first" assumption is void — the universal load runs client-only, in a SPA-style mount, from the start. If `csr = false`, the universal load never re-runs client-side on navigation (full-page reloads instead) — note this because it changes (lowers) the leak surface but should still be verified rather than assumed.
3. **Trace each universal load's imports.** For every `+page.js`/`+layout.js` in scope, follow its import graph, including transitive `$lib` imports, looking for:
   - direct imports from `$env/static/private` or `$env/dynamic/private` (SvelteKit's Vite plugin should block this at build time for client-reachable files — verify the repo's SvelteKit version actually enforces this, do not assume),
   - imports of a module under `$lib/server/` or named `*.server.js`/`*.server.ts` (SvelteKit's server-only module protection should block this too — same verification caveat),
   - manual `process.env.<SECRET>` reads that bypass both of the above guards,
   - a database client (Prisma, Drizzle, raw driver, etc.) instantiated or imported directly.
4. **Trace server-`load` return values.** For every `+page.server.js`/`+layout.server.js` in scope, inspect the object literal(s) returned by `load`. Flag any field that is a raw secret, API key, session token, or full DB-record dump not intended for client consumption — that object becomes `data` in the child universal `load` and `$page.data` in every component, i.e. it is client-visible the instant it is returned, independent of whether the universal `load` "does" anything with it.
5. **Check route-tree precedence** only if `+layout`/`+page`/`+server` structural correctness is also in question — load `references/routing-conventions.md` for that sub-review.
6. **Rank and report findings** per the output shape below.

## Leak-tracing decision tree

- Universal `load` (or a module it transitively imports) directly imports `$env/static/private`, `$env/dynamic/private`, or reads `process.env.<SECRET>` → **HIGH**. State whether the repo's SvelteKit version's build-time guard would catch this import path or whether it is a bypass (e.g., `process.env` read, or a non-`.server`-suffixed module outside `$lib/server/` that re-exports a value originally sourced from a private env var).
- Universal `load` imports a `$lib/server/*` or `*.server.js` module directly → **HIGH** unless the repo's verified SvelteKit version enforces server-only module protection for that exact import path, in which case this is a build-time-blocked case; still flag it as a MEDIUM code-smell (the import should never have been attempted) rather than close the finding silently.
- Server `load` (`+page.server.js`/`+layout.server.js`) returns an object containing a raw secret, token, or credential intended only for server-to-service calls → **HIGH**, regardless of whether any universal `load` or component currently reads that field. The leak is the serialization to the client, not the eventual consumption.
- Server `load` output is consumed unchanged by a child universal `load` that then spreads or forwards it further (e.g., into a client-side third-party SDK init call) → **HIGH**, trace and cite both hops.
- `+page.server.js`/`+layout.server.js` contains a DB call or secret access with no client-visible leak in its return value → **not a finding for this skill**; this is expected, correct placement.
- Duplicate near-identical server `load` logic repeated across sibling `+page.server.js` files that could be consolidated into a shared `+layout.server.js` → **MEDIUM**, maintainability/duplication, not a security finding.
- `+server.ts` (API route) referenced by a universal `load`'s `fetch()` call, where the `+server.ts` handler itself correctly guards secrets server-side → **not a finding**; note it as an alternative-pattern observation only if the review scope includes route-tree precedence.

## Adversarial checklist

Before closing a review with no HIGH findings, confirm:

- Did you classify every file by filename suffix, not by assumption?
- For every universal `load`, did you check `ssr`/`csr` page options before asserting when/whether it runs server-side vs. client-side?
- Did you follow *transitive* `$lib` imports, not just the direct imports in the load file itself?
- Did you inspect the actual object returned by every server `load`, not just whether it "looks like" it fetches sensitive data?
- Did you verify the installed SvelteKit version's enforcement behavior for `$env/*/private` and server-only modules rather than assuming current docs apply unconditionally?
- If you found zero leaks, is that because none exist, or because you didn't trace far enough?

## Output shape

Every review response must include:

1. **Scope** — routes and files reviewed, each labeled universal-load or server-load (or both, if the route has paired files).
2. **Findings** — ranked HIGH → MEDIUM → LOW, each with `file:line`, the classification (misplacement vs. leak vs. duplication), the full import/data-flow trace for any leak finding, and a concrete fix sketch (e.g., "move to `+page.server.js`" or "strip `apiKey` field from the returned object before it reaches `data`").
3. **Evidence level** per finding: `repo evidence` (read the actual file/import), `documentation-based` (asserting SvelteKit's enforcement behavior from docs without confirming the installed version), or `inference` (plausible but unverified, e.g., a dynamic import you could not statically resolve).
4. **Verdict** — approve / approve-with-notes / block. A single HIGH leak finding is a block.
5. **Open questions** — anything the review could not verify (unread `svelte.config.js`, unresolved dynamic import, unconfirmed SvelteKit version).
