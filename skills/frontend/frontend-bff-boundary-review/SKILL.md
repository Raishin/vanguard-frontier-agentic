---
name: frontend-bff-boundary-review
description: Determines and reviews whether aggregation/shaping logic belongs in a Backend-for-Frontend layer versus client-side composition, and audits existing BFF boundaries for scope creep, duplicated aggregation logic, and leaked backend topology or pass-through authorization.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Frontend BFF Boundary Review

## Purpose

Decide whether a multi-backend data need belongs in a Backend-for-Frontend (BFF) route or in client-side composition, and audit an existing BFF layer for the two failure modes that matter most: organic scope creep (duplicated or one-off aggregation logic scattered across routes) and trust-boundary erosion (a BFF that forwards client-supplied authorization claims or credentials without re-verifying them, or that leaks internal backend topology into its responses). This skill exists so the boundary decision and the trust-boundary audit stay the focus, and so field-level contract review of an already-scoped endpoint, or client-side cache/store design once data has landed in the browser, stay out of scope.

## When to use

Use this skill when the user asks to:

- decide whether a feature needing data from multiple backend services should aggregate server-side (a BFF route) or client-side (parallel query-library calls),
- audit an existing BFF layer, or a specific BFF route, that has grown organically over time,
- review whether a proposed new BFF route or service duplicates an existing one's aggregation logic,
- check whether a BFF route re-authenticates/re-authorizes the caller or merely passes through client-supplied claims/tokens to backend services.

Do not use this skill for:

- reviewing the field-level shape, versioning, or authorization contract of a single already-scoped API endpoint — that is `api-integration-contract-review`,
- client-side cache/store design (query-library cache keys, state colocation) once data has already been fetched — that is `state-management-decision-review`,
- rendering-mode or `fetch()` cache-directive selection for a Next.js route with no cross-service aggregation involved — that is `nextjs-rendering-caching-review`,
- Server Action authorization or `'use client'`/`'use server'` boundary review with no BFF-scope question involved — that is `nextjs-app-router-data-fetching-review`.

## Context7 Documentation Protocol

- Resolve `/vercel/next.js` with `resolve-library-id` before citing any Route Handler capability, caching directive, or runtime behavior as grounds for a BFF-vs-client-composition recommendation.
- Before recommending a Next.js Route Handler as the BFF implementation vehicle, read the repo's `package.json` to confirm the installed Next.js major version, then call `query-docs` scoped to that version for "Route Handler caching" and "route segment config revalidate fetchCache." Caching semantics changed materially across major versions — Route Handler `GET` methods are cached by default through Next.js 14 but are **not** cached by default starting in Next.js 15, requiring an explicit `export const dynamic = 'force-static'` to opt back in. A BFF route assumed to cache aggregated responses on an unverified version can silently hit every backend on every request instead of reducing round-trips as intended.
- If the version cannot be confirmed, or Context7 is unavailable, state the caching claim as `documentation-based, version-unconfirmed` and recommend the user verify `export const dynamic` / `export const revalidate` / `export const fetchCache` behavior against their installed version before relying on it for load-reduction claims.
- Do not assume a Route Handler behaves like a `fetch()` call inside a Server Component; segment-level config (`dynamic`, `revalidate`, `fetchCache`) governs the Route Handler's own caching, and it is set independently of caching used for calls the handler itself makes.

## Lean operating rules

- A BFF is a trust boundary, not a convenience layer for reshaping JSON. The default posture for any BFF route is that it terminates the client's authentication context and establishes its own — it does not relay whatever the client sent forward.
- Default toward BFF aggregation when a feature needs data from two or more backend services with different authorization models or error shapes. Default toward client-side composition only when the backends involved are already safe to call directly from the browser (same trust level as the client, already CORS-exposed, no internal-only topology).
- Before proposing a new BFF route, search for an existing route already serving an overlapping need. A second BFF route re-implementing the same aggregation is a maintenance and drift risk, not a fresh feature.
- Treat any BFF route that reads a client-supplied authorization claim (a role, user ID, or permission flag taken from a request body, query string, or an unverified header) and uses it directly to gate a backend call as a HIGH-severity finding — this is pass-through authorization, not delegation.
- Treat any BFF route that forwards a client-supplied bearer token or credential straight to a downstream backend, in place of the BFF re-authenticating the session and minting its own downstream credential, as a HIGH-severity finding, unless the system is an explicit, documented token-exchange/delegation design (e.g. OAuth token exchange) with its own re-verification step.
- Treat any BFF response that exposes internal-only backend hostnames, service names, stack traces, or service-specific error codes/shapes verbatim to the browser as a MEDIUM-to-HIGH finding depending on sensitivity — the BFF exists in part to prevent this leak, and a thin pass-through response defeats that purpose.
- Do not recommend client-side composition when it would require the browser to hold credentials for, or make direct network calls to, a backend that is not already intended to be internet-reachable — that is a bigger security regression than the aggregation-placement question being asked.
- Do not flag every multi-call client-side data-fetching pattern as a problem. Client-side composition of two or three already-public, already-authorized endpoints is a legitimate, lower-latency choice; only escalate when trust boundaries or topology are actually crossed.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Treat any hardcoded API key, service token, or credential found in BFF route source, environment file references, or example data as a HIGH-severity finding requiring immediate escalation, separate from the boundary-scope verdict.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step boundary-decision procedure, the existing-BFF audit method, and the required output shape.
- [OWASP API Security — trust-boundary risks](references/owasp-api-trust-boundary.md) — load only when a pass-through-authorization or topology-leak finding is present, to ground the finding's OWASP API Security Top 10 classification and severity framing.

## Response minimum

Return, at minimum:

- the boundary decision (BFF aggregation vs. client-side composition) with justification tied to the number of backends, their auth models, and their reachability from the browser,
- for any new/extended BFF route: an explicit scope statement and a check for an existing overlapping route,
- a trust-boundary audit result (pass-through-authorization check, credential-forwarding check, topology-leak check) for any BFF route in scope,
- ranked findings with file:line evidence, risk class, and fix,
- the Next.js major version the caching claims were verified against, if a Route Handler is the proposed implementation,
- verdict: approve / approve-with-notes / block,
- evidence level and open questions.
