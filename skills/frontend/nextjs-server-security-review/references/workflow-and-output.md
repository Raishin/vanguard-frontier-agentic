# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the middleware, Server Action, config, or environment file under review actually raises.

## Prerequisites

- Read `package.json` to confirm the Next.js major version and whether the app uses the App Router or Pages Router. `experimental.serverActions` configuration keys, middleware conventions, and rewrite/redirect APIs differ across major versions and routing modes; do not apply App Router API names to a Pages Router codebase or vice versa.
- Locate every candidate file in scope: `middleware.ts`/`middleware.js` at the project root, `next.config.js`/`next.config.mjs`, any `.env*` file, and any file containing a `'use server'` directive or exported Server Action.

## Workflow

1. **Locate `middleware.ts`/`.js` and read its exported `config.matcher`.** Determine whether the `matcher` pattern excludes any path via a negative lookahead (e.g. `(?!api|_next)`). See `references/middleware-and-server-actions.md` for the decision tree.
2. **For every path the matcher excludes, locate that path's own handlers** (Route Handlers, Server Actions) and confirm each one independently verifies the session/auth state inside its own function body — never assume middleware covers it.
3. **Locate `next.config.js` and read any `experimental.serverActions` block.** If the deployment topology involves a reverse proxy or multi-zone setup (check deployment docs, `rewrites()`/proxy config, or ask if unclear), confirm `allowedOrigins` is present in that block and lists the actual production origins.
4. **Grep every `.env*` file and every `process.env.NEXT_PUBLIC_*` usage** for a variable name suggesting a secret (`KEY`, `SECRET`, `TOKEN`, `PASSWORD`, `CREDENTIAL`, or equivalent). See `references/env-and-ssrf-surfaces.md`.
5. **Grep `next.config.js` for `images.dangerouslyAllowLocalIP`.** If `true`, confirm dynamic `src` values passed to `<Image>` are validated against a hardcoded external-hostname allowlist somewhere on their data-flow path.
6. **Grep for `NextResponse.rewrite(` and `rewrites()` destinations.** For each, trace the destination value's origin backward. If it is built from user-controlled input (query parameters, headers, request body) with no hostname allowlist check before the rewrite call, this is an SSRF/open-redirect finding.
7. **Produce ranked findings** using the output contract below.

## Decision tree

- Middleware `matcher` excludes a path via negative lookahead, and that path's own handler has no independent session check → **HIGH** finding, `middleware-auth-gap`. Cite the documented execution-order rule directly (`documentation-based`): a Proxy matcher that excludes a path also skips Server Function calls on that path.
- Middleware `matcher` excludes a path, but that path's own handler (Server Action or Route Handler) demonstrably verifies the session itself → not a finding for that path; note it explicitly as reviewed-and-clear.
- `serverActions` block exists, deployment is cross-origin (reverse proxy/multi-zone), and `allowedOrigins` is absent from that block → **HIGH** finding, `csrf-origin`.
- `serverActions` block exists and includes `allowedOrigins` listing the actual production origins, or the deployment is same-origin only and the key is legitimately omitted → not a finding.
- A `NEXT_PUBLIC_`-prefixed variable name matches a secret-shaped pattern (`KEY`/`SECRET`/`TOKEN`/`PASSWORD`/`CREDENTIAL`) → **HIGH** finding, `secret-leak`, regardless of whether the value has been rotated since — the bundle already shipped it to every prior client that loaded the page.
- A `NEXT_PUBLIC_`-prefixed variable holds genuinely non-secret data (an analytics ID, a public feature flag) → not a finding.
- `images.dangerouslyAllowLocalIP: true` with no demonstrable `src` allowlist validation → **HIGH** finding, `ssrf-redirect`.
- `images.dangerouslyAllowLocalIP` is `false` or absent (the default) → not a finding.
- `NextResponse.rewrite()`/rewrite destination built from user-controlled input with no hostname allowlist check before the call → **HIGH** finding, `ssrf-redirect`.
- Rewrite destination is a fixed literal path, or a dynamic value is checked against a hardcoded allowlist before the rewrite call → not a finding.

## Output contract

Every response from this skill must return:

1. **Scope** — the middleware file(s), Server Action(s), `next.config.js`, and/or environment file(s) reviewed.
2. **Ranked findings** — each with file:line, defect category (`middleware-auth-gap` / `csrf-origin` / `secret-leak` / `ssrf-redirect`), the concrete data-flow trace (matcher pattern and excluded-path handler status, serverActions config and deployment topology, environment variable declaration and usage site, or origin-to-sink path for SSRF/redirect), and a fix sketch matching Next.js's documented pattern.
3. **Middleware-auth-gap status per excluded path** — an explicit statement of whether the excluded path's own handler independently verifies the session; never infer one does.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly — do not imply confirmed exploitation without live evidence (e.g., a captured cross-origin CSRF success, a confirmed SSRF callback).
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming actual cross-origin CSRF exploitation requires a live cross-origin request, not static review," or "component-level data-fetching patterns in this same file are out of scope — recommend a general Next.js review if needed."

## When to push back

Push back if the user asks to:

- approve a middleware `matcher` that excludes `/api` (or another path) as sufficient authorization coverage without checking whether that excluded path's own handlers verify the session — "middleware is there" is not evidence for a path middleware never runs on,
- skip the `serverActions.allowedOrigins` check because "we haven't seen a CSRF incident" — this defect class is structural and often invisible until an attacker actually attempts a cross-origin form submission,
- clear a `NEXT_PUBLIC_`-prefixed secret because "we're going to rotate it" — rotation does not undo exposure already shipped to every client that loaded a build containing the old value, and the review is about the structural leak, not the current key's live validity,
- downgrade an untraced SSRF/rewrite finding to informational because "it's probably fine" — this skill's default is HIGH for exactly this class of unproven claim.
