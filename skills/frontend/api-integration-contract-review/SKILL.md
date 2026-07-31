---
name: api-integration-contract-review
description: Reviews frontend-to-backend API contracts — BFF route handlers and direct backend calls — for data-minimization, server-side object-level authorization enforcement, error-shape leakage, CORS misconfiguration, and backward-compatible versioning before they ship.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# API Integration Contract Review

## Purpose

Review any new or changed API contract consumed by the frontend — a direct backend call or a BFF (backend-for-frontend) route handler — for data-minimization (no field returned to a client that the caller is not authorized to see), object-level authorization enforced independently on the server, error-shape safety (no upstream internals leaking to the client), CORS correctness, and backward-compatible versioning for existing consumers. This skill exists so those four concerns get a disciplined, security-severity review every time a contract is introduced or changed, instead of being waved through as "just wiring."

## When to use

Use this skill when the user asks to:

- review a new API endpoint or BFF route handler before it ships,
- review a change to an existing response shape, status-code contract, or error format,
- audit whether the frontend fetches more fields than it renders,
- investigate a reported data-exposure or authorization-bypass (BOLA) concern,
- review CORS configuration for an endpoint that accepts credentialed requests.

Do not use this skill for:

- the frontend's caching/store logic once data has already arrived — route to `state-management-decision-review`,
- BFF-vs-direct-call topology or new-BFF-service ownership decisions at the platform level — route to `frontend-platform-architecture-review`; use this skill for the contract itself once the boundary decision is made,
- SSR/hydration mechanics — route to the relevant SSR skill,
- general Next.js data-fetching patterns unrelated to authorization/data-shape — route to `nextjs-app-router-data-fetching-review`.

## Context7 Documentation Protocol

- Before assessing a Next.js Route Handler's caching configuration, query Next.js docs for the current caching-directive semantics (`dynamic`, `revalidate`, `fetchCache`, `runtime`) against the repo's confirmed major version — read `package.json` first. As of Next.js 15+, `GET` Route Handlers are no longer cached by default; caching requires an explicit `export const dynamic = 'force-static'`. A route relying on pre-15 default-caching behavior to protect against overexposure (or that assumes it is uncached when it is actually configured `force-static`) is a version-sensitive misconfiguration risk, not a stylistic detail — verify the version before trusting either claim.
- Matched library ID for this skill's default grounding: Next.js is `/vercel/next.js`. Resolve fresh via `resolve-library-id` for any other backend/BFF framework named in the review (Express, Fastify, NestJS, etc.) rather than assuming Next.js conventions transfer.
- Before approving a query-key or cache-key design that scopes data by session/role, query TanStack Query docs for query-key structuring guidance — object-based key segments are order-independent and `undefined` properties are dropped during serialization, so a key intended to separate two roles/users can silently collide if one property is `undefined` for one caller and omitted for another. Matched library ID: `/tanstack/query`.
- Documentation proves what the framework *supports* (e.g., that `force-static` exists and changes default caching). It does not prove this specific route handler is configured correctly, or that authorization is actually enforced server-side. Pair every Context7-grounded capability claim with a repo-evidence check (the actual route handler code, the actual authorization middleware) before treating a finding as resolved.
- Never approve a version-sensitive caching or contract claim without Context7 verification. If Context7 is unavailable, label the claim `documentation-based — unverified this session` and require confirmation before final sign-off.

## Lean operating rules

- Treat every unjustified field in a response as a data-minimization defect, not a style note. "We return the whole object because it's simpler" is not a justification.
- Treat client-supplied identifiers (a URL param, a body field, a bearer-token claim the client can influence) as untrusted for authorization decisions. Object-level authorization must be re-derived server-side from the authenticated session, independent of what the client claims to be requesting.
- Escalate every finding of client-side-only authorization enforcement or excessive data exposure to security severity, not a style/lint-level note — these map directly to OWASP API Security Top 10 categories (Broken Object Level Authorization, Excessive Data Exposure).
- Treat raw upstream error forwarding (stack traces, internal hostnames, DB driver errors, vendor error payloads) as a blocking finding. Error responses reaching the client must be shaped and sanitized, not passed through for "easier debugging."
- Treat wildcard CORS (`Access-Control-Allow-Origin: *`) combined with `Access-Control-Allow-Credentials: true` as an automatic blocking finding — this combination is invalid per the CORS spec in browsers that enforce it correctly, and where it is not rejected outright it defeats the purpose of credentialed requests.
- For a breaking contract change (removed field, renamed field, changed status-code meaning, changed error shape), require an identified list of existing consumers and a stated deprecation window before approval. Do not accept "nothing should be calling this yet" without evidence.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only) — verdicts are based on route-handler code, authorization middleware, and documented contract evidence, not live requests you generate yourself.
- Label every claim `repo evidence`, `Context7-verified`, `documentation-based — unverified this session`, or `inference`. Documentation proves framework capability; it does not prove this endpoint's authorization is correctly wired.

## References

Load these only when needed:

- [Contract review workflow and verdict contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the block / block-with-conditions / approve decision tree, and the required output shape.
- [Authorization and data-minimization patterns](references/authorization-and-data-minimization.md) — use when the contract involves per-object access control, role-scoped fields, or a BOLA/excessive-data-exposure concern; grounds server-side enforcement patterns and field-justification review.
- [Error shape, CORS, and versioning](references/error-shape-cors-versioning.md) — use when reviewing error-handling code, CORS configuration, or a breaking/backward-compatible contract change with existing consumers.

## Response minimum

Return, at minimum:

- the endpoint/route handler and consumer(s) in scope,
- a per-field data-minimization justification (or the unjustified fields flagged),
- the object-level authorization enforcement mechanism and whether it is independently server-verified,
- error-shape and CORS findings, each labeled by severity,
- verdict (approve / approve-with-conditions / block) with the specific unresolved conditions if any,
- versioning/deprecation plan status for any breaking change,
- every version-sensitive framework claim labeled `Context7-verified` or `documentation-based — unverified this session`.
