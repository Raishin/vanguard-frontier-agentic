---
name: react-rsc-data-boundary-review
description: Statically review React Server Components code for data leaks across the server-to-client serialization boundary — secrets passed as props to Client Components, server-only modules missing the `server-only` guard, `use server` actions with no authorization check, non-public environment variables read in `use client` modules, and tainted values crossing the boundary unnarrowed — grounded in React's and Next.js's own documentation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# React RSC Data Boundary Review

## Purpose

Review React Server Components (RSC) code for the concrete, documented ways sensitive server-side data leaks across the server-to-client serialization boundary: a secret passed as a prop such as `password={SECRET_API_KEY}` on a Client Component, a server-only data module with no `server-only` guard that a client bundle could accidentally pull in, a `'use server'` Server Action that mutates data with no session/ownership check (missing the `session?.user` check pattern), a `'use client'` module reading a non-`NEXT_PUBLIC_` environment variable, or a value that should have been marked with `experimental_taintUniqueValue` and narrowed before crossing the boundary. This skill exists so the review stays anchored to these five documented defect classes instead of drifting into a general "React code review" of component design, hooks usage, or rendering performance.

## When to use

Use this skill when the user asks to:

- review a Server Component that passes props to a Client Component,
- audit a data-access module (`lib/`, `data/`, or similar) that is meant to run only on the server,
- review a `'use server'` Server Action or Server Function for authorization gaps,
- check whether a `'use client'` module is reading environment variables safely,
- perform a pre-launch security review of a Next.js App Router or other RSC-based application's data flow.

Do not use this skill for:

- generic client-side XSS review (`dangerouslySetInnerHTML`, unsanitized `innerHTML`, DOM-based injection) — that is a distinct defect class covered by `frontend-dom-xss-csp-review`, not this skill,
- Next.js rendering/caching-strategy review (revalidation, cache tags, `fetch` cache options) with no data-boundary security angle — use `nextjs-rendering-caching-review` or `nextjs-app-router-data-fetching-review` instead,
- confirming that a leak has actually been exploited in production — static analysis proves the structural risk (a secret is reachable across the boundary) and not that an attacker has already captured it; that requires live request/network interception, which this skill does not perform.

## Context7 Documentation Protocol

- Resolve the React library ID with `resolve-library-id` (matched result: `/reactjs/react.dev`) before citing any `experimental_taintUniqueValue` or Server/Client Component serialization claim. Use `query-docs` against it to confirm the exact tainting API shape and the documented anti-pattern (a secret such as `process.env.API_PASSWORD` passed directly as a prop) before flagging a finding as `documentation-based`.
- Resolve the Next.js library ID with `resolve-library-id` (matched result: `/vercel/next.js`) before citing any `server-only` package usage, `NEXT_PUBLIC_` environment-variable convention, or Server Action authorization pattern claim. Use `query-docs` against it for the `server-only` install/import pattern and the documented Server Action authorization example (`auth()` + ownership check) before labeling a finding `documentation-based`.
- `experimental_taintUniqueValue` is an experimental React API, not yet stable — label any finding or fix sketch that depends on it as `documentation-based (experimental API)`, and note that the equally valid non-experimental fix is simply omitting the sensitive field from the object passed to the Client Component.
- Confirm which surface is in scope before citing a documented rule: a "Server Component passing props" claim only applies where a Server Component (no `'use client'` directive, or an RSC-context module) actually renders a Client Component; a `'use server'` claim only applies to a function or file carrying that exact directive; a `NEXT_PUBLIC_` claim is Next.js–specific and does not automatically apply to a different meta-framework's env-var convention.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- All five defect categories default to HIGH severity. This is a security-scoped skill: do not downgrade an untraced secret-prop pass, a missing `server-only` guard, or an unauthorized `use server` mutation to MEDIUM just because it has not been observed exploited yet — the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this prop might leak a secret" without naming the specific prop, the specific Server Component that renders it, and the specific Client Component receiving it is not a valid finding — it is a guess.
- Do not flag every prop passed from a Server Component to a Client Component. Only a prop whose value traces back to an environment variable, a credential, a token, a full unfiltered config/response object, or any other server-only sensitive value is a finding. A plain string, number, or narrowed non-sensitive field (e.g. `config.SERVICE_API_VERSION`) is not a finding.
- Do not approve a `'use server'` action that mutates or deletes data unless a session check (the `session?.user` pattern) and, where the mutation targets a specific resource, an ownership check comparing the resource's owner to the authenticated user, are both visibly present on that exact function's path. A session check existing in a different action does not clear this bar — trace the specific function under review.
- Do not treat a `server-only` import anywhere in the codebase as covering every server-only module. Confirm the exact file that reads the sensitive value (`process.env.*`, a database credential, an internal API token) has `server-only` imported at its own top, not merely that some other file in the project has it.
- Watch for whole-object prop forwarding: a Server Component that fetches a config/response object and forwards it unnarrowed as a same-named prop (e.g. `config={config}`) crosses the boundary with whatever sensitive fields the object holds, even if no single field is individually named "secret" or "password" in the forwarding code. Narrowing to specific non-sensitive fields, or applying `experimental_taintUniqueValue` to the sensitive fields before any possible pass-through, are the two documented mitigations.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the decision tree across all five defect categories, and the required output shape.
- [Boundary data leaks and the taint API](references/boundary-data-leaks-and-taint.md) — load only when the review scope includes a prop passed from a Server Component to a Client Component, a `server-only` guard question, or a value that should have been tainted.
- [Server Action authorization and client environment exposure](references/server-actions-and-env-exposure.md) — load only when the review scope includes a `'use server'` directive or a `'use client'` module reading `process.env`.

## Response minimum

Return, at minimum:

- the Server Component(s), Client Component prop boundaries, `'use server'` action(s), and/or `'use client'` module(s) in scope,
- ranked findings with file:line evidence, defect category (`boundary-data-leak`, `missing-server-only-guard`, `action-authz-missing`, `env-exposure-in-client`, or `taint-boundary-violation`), the concrete data-flow trace (the sensitive value's origin and every hop to the boundary crossing or missing guard), and a fix sketch matching React's or Next.js's documented pattern,
- guard status per finding: an explicit statement of whether a `server-only` import, an `experimental_taintUniqueValue` call, or a session/ownership check is present on the traced path — never approve on the assumption one exists elsewhere,
- evidence level per finding (`repo evidence`, `documentation-based`, or `structural-risk`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming data leakage requires live request interception (network inspection), not static review" or "taint API coverage requires a Context7 audit of all async boundaries where promises serialize, beyond this review's scope").
