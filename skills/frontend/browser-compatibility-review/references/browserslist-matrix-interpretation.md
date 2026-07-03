# Browserslist Matrix Interpretation

Use this reference when reading, interpreting, or proposing a change to the project's `.browserslistrc` / `package.json` `browserslist` config, or reconciling it with build-tool targets (Autoprefixer, Babel `preset-env`, `postcss-preset-env`, esbuild `target`).

> Version note (uncertainty flag): Context7 did not resolve a dedicated `browserslist` library at the time this skill was authored (only adjacent tooling like `es-check`, which integrates with Browserslist, was found). Treat Browserslist query-syntax details below as `documentation-based (Context7 unavailable for this library)`, and always confirm exact current syntax against the project's own resolved config output and the official `browserslist/browserslist` GitHub README before relying on it for a production recommendation.

## What people get wrong

The common bad assumption is:

> There's no `.browserslistrc` file, so this project doesn't have a declared matrix — I'll just eyeball it.

Wrong. Browserslist resolves configuration from multiple possible locations, in a defined precedence, and most frontend build tools (Autoprefixer, Babel, ESLint's `eslint-plugin-compat`, PostCSS presets) read it implicitly. Before declaring "no matrix exists," check all of the standard resolution points:

- a `browserslist` key in the project's `package.json`,
- a `.browserslistrc` file at the project root or a parent directory,
- a `BROWSERSLIST` environment variable (rare, but overrides file-based config when set),
- a shared/extended config referenced via `extends` in either of the above.

If genuinely none exist, Browserslist falls back to its own defaults (`> 0.5%, last 2 versions, Firefox ESR, not dead`) — that fallback *is* the effective matrix, and should be reported as such rather than treated as "no matrix."

## Reading the matrix correctly

- Browserslist queries combine like `> 0.5%, last 2 versions, not dead` — each comma-separated clause is a query, and by default clauses are combined with logical OR (a browser matching *any* clause is included), unless explicitly joined with `and`. Do not assume AND semantics for comma-separated queries without confirming against the project's resolved output.
- `not dead` excludes browsers without official support or security updates for the past 24 months (per Browserslist's documented `dead` query) — a matrix without `not dead` may be implicitly including officially unsupported browsers, which is itself worth flagging.
- Percentage-based queries (`> 0.5%`) are usage-share-relative and will silently shift over time as global usage shifts — a matrix expressed this way is not a fixed, auditable list; if the review needs a fixed matrix (e.g. for a compliance/procurement commitment), recommend resolving the query to a concrete browser list and pinning that, or flag the drift risk explicitly.
- To see the *actual resolved list* of browsers/versions a config expands to (not just the raw query string), the standard approach is running the project's own Browserslist resolution tooling (commonly exposed via the `browserslist` CLI or a project script) rather than manually reasoning about what a query string expands to — verify the exact command locally instead of assuming a specific CLI invocation, since this detail was not confirmed via Context7 for this skill.

## Non-negotiable rules

1. **Never approve or reject a feature's compatibility using an assumed matrix.** Always resolve the project's actual config (file-based, `package.json`-based, or the documented Browserslist default) before making a claim.
2. **Distinguish the declared query from the resolved browser list.** "last 2 versions" is not itself a browser list; it must be resolved against current release data to know which concrete versions are in scope, and that resolution changes over time as new versions ship.
3. **Reconcile the Browserslist matrix with the build tool's actual behavior separately per tool.** Babel `preset-env`, Autoprefixer, and `postcss-preset-env` each consume the same Browserslist config but apply it differently (transpilation targets vs CSS prefixing vs CSS feature polyfilling) — do not assume that because Babel is configured correctly, CSS-level compatibility is automatically covered, or vice versa.
4. **A percentage- or "last N versions"-based query is a moving target.** Flag this explicitly when a stakeholder wants a fixed compliance commitment; recommend either pinning explicit browser/version pairs or re-verifying the resolved list on a defined cadence.
5. **Proposing a matrix change is a data-backed recommendation, not a unilateral edit.** Base any proposed change on real usage/analytics data for the org's actual audience, and route the recommendation to the product/analytics owner rather than silently narrowing or widening support.

## Verification targets

- The actual `browserslist` key / `.browserslistrc` content in the project (read directly).
- The resolved concrete browser/version list the query expands to, obtained via the project's own tooling — verify the exact invocation locally rather than assuming one.
- Whether Babel, Autoprefixer, and any CSS preset-env-style tool in the build pipeline are all wired to the same Browserslist config (shared root config) rather than divergent per-tool overrides.

## When to push back

Push back if the user asks to:

- widen or narrow the supported-browser matrix without any usage/analytics justification,
- treat a percentage-based query as a fixed, auditable list for a compliance statement without resolving and pinning it,
- add a browser-compatibility "fix" that actually just silences a linter (`eslint-plugin-compat` disable comment) rather than resolving the underlying gap.
