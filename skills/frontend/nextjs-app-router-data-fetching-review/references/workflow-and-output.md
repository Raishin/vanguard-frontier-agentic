# Review workflow and findings contract

Use this reference for the full boundary/authorization review procedure and the required output shape.

## What people get wrong

The naive story is:

> `'use client'` just means "this component runs in the browser too" — eyeball the file, check it doesn't obviously import a DB client, and move on. Server Actions "run on the server," so whatever they do is trusted.

Wrong, on both halves.

- `'use client'` marks a **module-graph boundary**, not a single-file property. Once a file has `'use client'`, every module it imports and every component it directly renders is pulled into the client bundle — including transitive imports through shared barrel files (`lib/index.ts` re-exporting both a pure util and a DB client). A file that "looks clean" can still leak a secret three imports deep.
- A Server Action's function body runs on the server, but its **inputs are fully attacker-controlled**. Nothing stops a caller from invoking the action directly with crafted `FormData`, bypassing the UI, any client-side validation, and any upstream page-level auth check entirely. "It runs on the server" says nothing about whether the action re-verifies who is calling it.

The review has to operate at both the **import-graph** level (boundary/bundle-leak) and the **argument-trust** level (Server Action authorization), and it must not let a passing check on one half stand in for the other.

## Workflow

1. **Confirm the Next.js major version**
   - Read `package.json` for the exact Next.js version. Server Component restrictions and the availability of the Taint API (`experimental_taintObjectReference`, `experimental_taintUniqueValue`, gated behind `experimental.taint`) are version-sensitive — see the Context7 Documentation Protocol in SKILL.md.

2. **Enumerate every `'use client'` file in scope**
   - For each, read its import list.
   - Trace transitively: if it imports a local module, open that module and check its imports too, until you reach either a leaf (no further local imports) or a module that imports `server-only`, reads a non-`NEXT_PUBLIC_`-prefixed environment variable, touches the filesystem, or instantiates a DB/ORM client.
   - Record the exact chain (file A imports file B imports file C which reads `DATABASE_URL`) — a bundle-leak finding without a traced chain is not a finding, it is a guess.

3. **Enumerate every Server Action (`'use server'`) in scope**
   - This includes files with a top-level `'use server'` directive and inline `async function` bodies marked `'use server'` inside a Server Component.
   - For each action, identify: what mutation/read does it perform, and what value gates whether it is allowed to proceed?

4. **Classify each Server Action's authorization source**
   - **Session-derived (correct)**: the action calls a session/auth helper (`auth()`, `verifySession()`, a `cookies()`-backed session read) and uses the identity/role returned from that call to gate the mutation.
   - **Input-derived (defect)**: the action reads a user ID, role, or "isAdmin"-style flag from its own parameters or from `formData.get(...)` and uses that value — not a freshly re-derived session — to gate the mutation or to decide *whose* data to act on.
   - **Missing (defect)**: the action performs a mutation with no authorization check at all, relying solely on an upstream page-level check that does not extend into the action.
   - **Resource-scoped (verify ownership, not just auth)**: even with a valid session, if the action acts on a specific resource ID (e.g. `deletePost(postId)`), confirm it checks that the session's user actually owns/may act on that resource — an authenticated-but-unauthorized mutation is an IDOR (Insecure Direct Object Reference), still Broken Access Control.

5. **Check for boundary-placement inefficiency (non-security)**
   - If a `'use client'` directive sits on a large subtree (e.g. an entire page or layout) where only a small leaf component actually needs interactivity/state/browser APIs, note it as a MEDIUM bundle-size finding — recommend pushing the boundary down to the smallest interactive leaf. Do not treat this the same as a traced server-only leak.

6. **Produce ranked findings**
   - Order by blast radius: confirmed bundle leaks and Broken Access Control findings first (HIGH), then missing resource-ownership checks, then boundary-placement/bundle-size notes (MEDIUM/LOW).

## Decision tree

- A `'use client'` file's traced import graph reaches a module importing `server-only`, reading a non-`NEXT_PUBLIC_` env var, or instantiating a DB/ORM client → **HIGH: bundle-leak risk.** Cite the exact import chain.
- A Server Action gates its mutation/authorization using a value taken from its own parameters or `FormData` instead of a freshly re-derived session → **HIGH: Broken Access Control (OWASP A01).** Load `references/owasp-a01-broken-access-control.md` and frame the finding per that reference.
- A Server Action re-derives session correctly but never checks that the session's user owns/may act on the specific resource ID it was given → **HIGH: IDOR / Broken Access Control (OWASP A01).**
- A Server Action has no authorization check at all, relying on an upstream page-level check → **HIGH: Broken Access Control (OWASP A01).** Note explicitly that page-level checks do not extend into Server Actions.
- `'use client'` is placed on a subtree larger than the interactive leaf requires, with no traced server-only leak → **MEDIUM: avoidable bundle-size cost.** Recommend pushing the boundary down; do not escalate to HIGH.
- A Client Component genuinely needs conversion consideration but relies on browser-only APIs, local state, effects, or event handlers → do not recommend converting it to a Server Component; note the constraint instead.

## Output contract

Return:

1. Next.js major version confirmed (or explicitly noted as unconfirmed)
2. Per-boundary table: `'use client'` file | traced import-graph result (clean / leak with chain) | evidence (file:line)
3. Per-Server-Action table: action | authorization source (session-derived / input-derived / missing / resource-scoped-unverified) | verdict
4. Ranked findings, each with:
   - file:line evidence
   - risk class (bundle-leak / broken-access-control / IDOR / missing-auth-check / avoidable-bundle-size)
   - concrete fix, scoped to the narrowest sufficient change
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
5. Verdict: approve / approve-with-notes / block
6. Open questions or explicitly out-of-scope items (e.g. Pages Router API routes encountered, or rendering/caching concerns deferred to `nextjs-rendering-caching-review`)

## Validation gates

- Every bundle-leak finding traces the actual import chain from the `'use client'` file to the leaked server-only module — no finding asserts bundle contents without a traced chain.
- Every authorization finding identifies exactly which client-controlled value (parameter name, `FormData` key) is being trusted in place of a session re-derivation.
- No finding recommends converting a Client Component to a Server Component without first confirming it has no browser-only API, state, effect, or event-handler dependency.
- No Broken Access Control finding is downgraded to a style note or MEDIUM severity for "code cleanliness" reasons — the security-notes hard gate in `metadata.json` applies regardless of how small the fix looks.

## Common failure modes

- Flagging every `'use client'` directive as a problem regardless of whether interactivity is actually needed there.
- Missing transitive imports — checking only the top-level imports of a `'use client'` file and stopping, rather than following a shared `lib/db.ts` imported through an intermediate barrel file.
- Assuming Server Actions are inherently safe "because they run on the server," while ignoring that their inputs are fully client-controlled and can be invoked directly, bypassing the UI.
- Treating a page-level or layout-level auth check as sufficient for a Server Action invoked from that page, without checking the action re-verifies independently.
- Confusing "authenticated" with "authorized for this specific resource" — a valid session does not imply the session's user may act on the resource ID supplied.

## Adversarial checklist

Before finalizing a finding, answer these:

- Does a Server Action re-derive the acting user's identity from the server session, or trust an id/role passed in from the client?
- Does the import graph of every `'use client'` file actually avoid server-only modules, traced transitively, not just at the top level?
- Is `'use client'` placed at the smallest necessary leaf, or does it needlessly convert a large subtree?
- Would this Server Action behave safely if called directly with crafted `FormData` bypassing the UI entirely?
- For resource-scoped actions, does the check confirm the session's user owns/may act on the specific resource ID, not just that a session exists?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect, except for Broken Access Control findings with clear file:line evidence of client-controlled authorization, which stay HIGH regardless.
