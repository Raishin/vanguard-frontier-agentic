---
name: frontend-platform-architecture-review
description: Reviews cross-cutting frontend architecture decisions (module boundaries, rendering topology, technology adoption) against a rewrite-averse, evidence-grounded standard before they are approved, producing an ADR-quality verdict rather than a stylistic opinion.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Frontend Platform Architecture Review

## Purpose

Review proposed or existing frontend architecture — module/package boundaries, monorepo topology, rendering-strategy choice, technology adoption — for duplication, migration safety, and cross-team consistency, without re-litigating implementation-level state-management, routing, API-contract, or SSR-mechanics detail in every response. This skill exists so those adjacent concerns stay out of scope and the review stays focused on the cross-cutting, org-level decision: should this architectural change be approved, and on what terms.

## When to use

Use this skill when the user asks to:

- review a proposal to adopt a new frontend framework, library, or build tool,
- resolve two teams solving the same problem with different primitives (state management, routing, styling),
- redraw a monorepo/polyrepo module boundary or ownership line,
- review a rendering-strategy change (CSR to SSR, SSR to SSG/ISR/streaming/PPR),
- audit an existing codebase for architectural entropy before a scaling or hiring push.

Do not use this skill for:

- implementation-level state-management review — route to `state-management-decision-review`,
- routing/navigation-specific review — route to `routing-navigation-review`,
- API-contract or data-fetching review — route to `api-integration-contract-review`,
- SSR/hydration mechanics debugging — route to `ssr-hydration-streaming-diagnosis`,
- responsive/visual UI design review — that needs a design-system/visual skill, not architecture review.

## Context7 Documentation Protocol

- Before evaluating any version-sensitive technical claim in the proposal (e.g., "Next.js Partial Prerendering lets us do X," "React 19 Suspense enables Y"), call `resolve-library-id` for the exact library, then `query-docs` against the repo's confirmed version — read `package.json` first to confirm the installed major version before trusting a claim about API availability.
- Matched library IDs for this skill's default grounding: React is `/reactjs/react.dev`, Next.js is `/vercel/next.js`. Resolve fresh for any other framework named in a proposal (Angular, Vue, SvelteKit, etc.) rather than assuming these two cover every case.
- Never approve a version-sensitive technical claim without Context7 verification. If Context7 is unavailable, mark the claim `documentation-based — unverified this session` in the verdict and require the proposer to confirm the claim before final approval.
- Documentation proves what a framework *supports*; it does not prove the proposal's specific repo can adopt it safely. Pair every Context7-grounded capability claim with a repo-evidence check (actual installed version, actual existing patterns) before treating it as settled.

## Lean operating rules

- First classify the proposal: new capability, migration of an existing capability, or boundary redraw. Do not evaluate a migration as if it were greenfield.
- Check for an existing in-repo equivalent before evaluating the proposal on its own terms. A proposal that duplicates a capability the repo already solves is a duplication defect regardless of how well-argued the new approach is.
- Require at least two alternatives with tradeoffs before treating a single-option proposal as reviewable. A proposal with no alternatives considered is not ready for an architecture verdict — send it back.
- Default against "rewrite" framing. If an incremental strangler-fig or boundary-first path exists, require it over a big-bang rewrite; do not accept "the codebase is too messy to migrate incrementally" without the proposer demonstrating why.
- Treat accessibility and security posture as first-class, blocking review criteria — not items to defer to a follow-up ticket. An architecture proposal silent on a11y/security is incomplete, not merely imperfect.
- Treat Core Web Vitals budget impact as mandatory for any rendering-strategy change; a proposal with no stated LCP/INP/CLS impact (lab or field) has not been evaluated for its primary tradeoff.
- Never fabricate a performance or vitals number without a stated measurement source; label estimates `inference, not measured` and require the proposer to supply lab or field data before final approval.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only) — verdicts are based on document, code, and config evidence, not live measurement you generate yourself.
- Flag any architecture proposal that stores secrets or tokens in client-reachable bundles or build-time-inlined env vars, allows postinstall scripts from unpinned/unaudited dependencies, or introduces a CSP-incompatible pattern (`unsafe-eval`, dynamic `Function()`) as a blocking finding, not a note — this cannot be approved-with-conditions into a later cleanup.

## References

Load these only when needed:

- [Review workflow and ADR output contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the approve/approve-with-conditions/reject decision tree, and the required ADR-format output shape.
- [Rendering topology and cross-cutting budgets](references/rendering-topology-and-budgets.md) — load only when the proposal changes rendering strategy (CSR/SSR/SSG/ISR/streaming/PPR) or when Core Web Vitals, a11y, or security posture needs grounding against current framework/WCAG guidance.

## Response minimum

Return, at minimum:

- the architectural change and files/modules/teams in scope,
- duplication check result (existing in-repo equivalent found or none),
- every version-sensitive claim labeled `Context7-verified` or `documentation-based — unverified this session`,
- verdict (approve / approve-with-conditions / reject-with-reasoning) with the specific unresolved conditions if any,
- rollback or incremental-migration path referenced explicitly,
- open questions the review could not resolve from available evidence.
