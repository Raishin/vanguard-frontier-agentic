# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the RSC code under review actually raises.

## Prerequisites

- Identify the RSC boundary in scope: which files are Server Components (no `'use client'` directive, or files in an RSC-only context such as a Next.js App Router `app/` tree without the directive), which are Client Components (`'use client'` at the top), which files carry a `'use server'` directive (Server Actions/Functions), and which data-access modules are expected to be server-only.
- Read `package.json` to confirm the framework and version in use (Next.js App Router, a different RSC-capable meta-framework, or plain `react-server-dom-*` wiring) — API names and exact conventions (e.g. `NEXT_PUBLIC_` is Next.js-specific) differ by framework; do not assume a Next.js convention applies to a non-Next.js RSC setup.

## Workflow

1. **Locate every Server Component → Client Component prop boundary.** For each Server Component that renders a Client Component, list every prop passed and trace each value's origin (a literal, an env var, a database/API response field, a full object).
2. **Enumerate server-only data-access modules.** For each module that reads `process.env`, a database credential, or an internal API token, check whether `import 'server-only'` is the first statement in that file. See `references/boundary-data-leaks-and-taint.md` for the decision tree.
3. **Check for tainting or narrowing on sensitive values.** For any Server Component that reads a config/response object containing sensitive fields before rendering a Client Component, determine whether the object is narrowed to only non-sensitive fields before being passed as a prop, or whether `experimental_taintUniqueValue` is applied to the sensitive fields upstream.
4. **Enumerate every `'use server'` function.** For each, check whether a session check (`auth()` or equivalent, tested via a `session?.user` style guard) and — for mutations targeting a specific resource — an ownership check comparing the resource owner to the authenticated user, both execute before any read/write/delete against the data layer. See `references/server-actions-and-env-exposure.md`.
5. **Enumerate every `'use client'` module that reads `process.env`.** For each environment variable read, confirm the name is prefixed `NEXT_PUBLIC_` (or the framework's equivalent public-env convention); flag any that is not.
6. **Produce ranked findings** using the output contract below.

## Decision tree

- A Server Component passes a prop to a Client Component whose value traces back to an environment variable, a database credential, an API token, or any other value your review classifies as server-only sensitive → **HIGH** finding, `boundary-data-leak`. Cite React's documented anti-pattern (`documentation-based`).
- A Server Component passes an entire config/response object (not narrowed to specific non-sensitive fields) to a Client Component, and that object is known or suspected to carry a sensitive field → **HIGH** finding, `taint-boundary-violation`. Note whether `experimental_taintUniqueValue` was available and unused, or whether simple narrowing was the missed opportunity.
- A prop's value is a plain literal, a non-sensitive string/number, or a field explicitly narrowed out of a larger object (e.g. `config.SERVICE_API_VERSION`) → not a finding.
- A data-access module reads `process.env`/a credential and has no `import 'server-only'` as its first statement → **HIGH** finding, `missing-server-only-guard`. This holds even if no Client Component currently imports the module — the guard is the structural control against a future accidental import, and its absence is the finding regardless of present-day reachability.
- A `'use server'` function performs a read/write/delete against the data layer with no session check visible in its body → **HIGH** finding, `action-authz-missing`.
- A `'use server'` function has a session check but, for a mutation scoped to a specific resource (delete/update by ID), has no ownership check comparing the resource's owner to the authenticated user → **HIGH** finding, `action-authz-missing` (IDOR-shaped gap) — a session check alone proves *someone* is logged in, not that they own the resource being mutated.
- A `'use client'` module reads a `process.env` variable not prefixed `NEXT_PUBLIC_` → **MEDIUM** finding, `env-exposure-in-client` (the value is `undefined` at runtime in the browser bundle, and the read itself signals a broken mental model of the boundary that may mask a worse leak elsewhere).
- A `'use client'` module reads only `NEXT_PUBLIC_`-prefixed variables → not a finding.

## Output contract

Every response from this skill must return:

1. **Scope** — the Server Component(s), Client Component prop boundaries, `'use server'` action(s), and/or `'use client'` module(s) reviewed.
2. **Ranked findings** — each with file:line, defect category (`boundary-data-leak` / `missing-server-only-guard` / `action-authz-missing` / `env-exposure-in-client` / `taint-boundary-violation`), the concrete data-flow trace (naming every hop from the sensitive value's origin to the boundary crossing or missing guard), and a fix sketch matching React's or Next.js's documented pattern.
3. **Guard status per finding** — an explicit statement of whether `server-only`, `experimental_taintUniqueValue`, or a session/ownership check is present on the traced path; never infer one exists elsewhere in the codebase.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `structural-risk`. Label structural risk findings as structural risk explicitly — do not imply confirmed exploitation without live evidence (e.g., a captured network payload showing the secret in the client bundle).
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming the secret actually reached a browser requires live request/network interception, not static review," or "this file also has a client-side XSS-shaped concern unrelated to the RSC boundary — out of scope for this skill, recommend `frontend-dom-xss-csp-review`."

## When to push back

Push back if the user asks to:

- approve a prop pass because "the Client Component doesn't actually use that field" — an unused sensitive prop still serializes into the client payload; unused-in-render is not the same as never-transmitted,
- treat a `server-only` import anywhere in the project as covering a different file that itself reads the sensitive value with no guard of its own,
- skip the ownership check on a `'use server'` mutation because "the session check already runs" — a session check proves authentication, not authorization over the specific resource being mutated,
- downgrade an untraced whole-object prop pass to informational because "the object probably doesn't have anything sensitive in it" — trace the object's actual shape before clearing it, or state explicitly that the shape could not be confirmed.
