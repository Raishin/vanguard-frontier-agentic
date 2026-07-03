---
name: nextjs-app-router-data-fetching-review
description: Statically review Next.js App Router Server/Client Component boundaries and Server Action data mutations for correct data-fetching placement, bundle-leak risk, and authorization-trust integrity, escalating client-trusted authorization to a security finding rather than a style note.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Next.js App Router Data Fetching Review

## Purpose

Review the Server Component / Client Component boundary (`'use client'` placement and its import graph) and Server Action (`'use server'`) authorization logic in a Next.js App Router codebase, without re-litigating rendering mode, `fetch()` caching, or component decomposition in every response. This skill exists so bundle-leak risk and Server Action authorization-trust defects stay the focus, and so those adjacent concerns stay out of scope.

## When to use

Use this skill when the user asks to:

- review a `'use client'`/`'use server'` boundary in a diff or PR,
- determine whether a Server Action correctly authorizes its caller,
- investigate a report that server-only code or a secret appears to be reaching the browser bundle.

Do not use this skill for:

- pure UI/styling changes with no data-fetching or boundary change,
- Pages Router API routes (`pages/api/*`) — different security model, not Server Actions; use general API-route review instead,
- rendering-mode selection (static/ISR/dynamic) or `fetch()` cache-configuration review — that is `nextjs-rendering-caching-review`,
- component decomposition or state-placement review with no boundary or Server Action involved — that is `react-component-architecture-review`.

## Context7 Documentation Protocol

- Resolve `/vercel/next.js` with `resolve-library-id` before citing any Server Component restriction or Server Action security claim.
- Before asserting what is or is not safe to import into a Client Component, or what a Server Action must re-verify, read the repo's `package.json` to confirm the installed Next.js major version, then call `query-docs` scoped to that version for "Server Actions security" and "Server Components restrictions." The exact bundling/serialization rules and the availability of the Taint API (`experimental_taintObjectReference` / `experimental_taintUniqueValue`, gated behind `experimental.taint` in `next.config`) are version-specific.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- Server Components are the App Router default; `'use client'` is an opt-in boundary marker at the top of a file, above imports. Once a file is marked `'use client'`, everything it imports and directly renders is included in the client bundle — trace that import graph, do not eyeball the single file.
- Treat a Client Component's import graph reaching a server-only module (a DB/ORM client, a module reading a non-`NEXT_PUBLIC_`-prefixed env var, filesystem access, or any module importing `server-only`) as a HIGH-severity bundle-leak finding. This is the hard security gate for this skill's boundary half.
- A Server Action's arguments (including `FormData`) are fully client-controlled, even though the function body runs on the server. Treat any Server Action that derives its authorization decision from its own input parameters or `FormData` — instead of re-deriving identity from the server-side session (`cookies()`, an `auth()`/session helper) — as a HIGH-severity Broken Access Control finding (OWASP Top Ten A01), not a style note.
- Page-level or layout-level authentication/authorization checks do not extend into a Server Action invoked from that page. Each Server Action must independently re-verify session and, for resource-scoped mutations, ownership/role — absence of that re-check is a defect even if an upstream page already checked auth.
- Do not recommend converting a Client Component to a Server Component without first checking it doesn't rely on browser-only APIs, local state, effects, or event handlers — that breaks functionality, it does not fix a boundary defect.
- Do not flag every `'use client'` directive as a problem. Only flag it when the boundary is placed higher than the smallest interactive leaf needs, or when its import graph leaks server-only code.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Treat any hardcoded API key, token, session secret, or credential found in a Server Action, Client Component, or example data as a HIGH-severity finding requiring immediate escalation, not a boundary note.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the bundle-leak trace method, the authorization decision tree, and the required output shape.
- [OWASP A01 — Broken Access Control](references/owasp-a01-broken-access-control.md) — load only when a Server Action authorization finding is present, to ground the finding's severity and framing.

## Response minimum

Return, at minimum:

- per-boundary table (`'use client'` file, traced import-graph leak or clean, evidence file:line),
- per-Server-Action table (action, authorization source used, verdict),
- ranked findings with file:line, risk class, and fix,
- the Next.js major version the claims were verified against,
- verdict: approve / approve-with-notes / block,
- evidence level and open questions.
