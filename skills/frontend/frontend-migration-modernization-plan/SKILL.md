---
name: frontend-migration-modernization-plan
description: Build a phased, reversible plan to migrate or modernize a legacy frontend surface (jQuery/Backbone/AngularJS to a modern framework, CRA/Webpack to Vite, or a same-framework major-version bump) using strangler-fig sequencing with measurable exit criteria per phase, without defaulting to a full rewrite.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Frontend Migration & Modernization Plan

## Purpose

Most full frontend rewrites blow their timeline/budget or never ship, while unmanaged incremental migrations rot into permanent dual-stack debt. This skill exists to produce a bounded, reversible, metric-gated migration plan without stuffing the entire migration playbook (strangler patterns, legacy-jQuery inventory tactics, framework-upgrade risk analysis, rollback design) into every response — those load only when the task needs them.

## When to use

Use this skill when the user asks to:

- plan a migration off a legacy stack (jQuery, Backbone, AngularJS, old CRA/Webpack setup) to a modern framework or bundler,
- sequence a same-framework major-version upgrade (React 17→19, Angular N→N+2, Vue 2→3) as a phased rollout,
- design a strangler-fig boundary (route table, module federation seam, adapter layer) between legacy and modern code,
- define rollback and legacy-decommission exit criteria for an in-flight migration.

## Lean operating rules

- First classify the migration type: cross-framework (paradigm shift, high risk) vs. same-framework major-version (breaking-change surface, narrower risk). They need different reference material — do not blend them.
- Do not recommend a full rewrite as the default. Only recommend it when incremental strangling is demonstrably infeasible (e.g. the runtime model itself is incompatible), and say so explicitly with the reason.
- Every phase must have a stated rollback action completable within one deploy cycle and a measurable exit metric (not a date-only gate).
- Treat the legacy-decommission step as mandatory and time-boxed; a migration plan with no decommission criteria is incomplete.
- Verify target-framework migration APIs/codemods via Context7 or official docs before citing them; never invent a codemod name or flag.
- Never propose a migration phase that merges auth/session state across old and new stacks without a named security review gate.
- Load `references/strangler-boundary-design.md` only when the user needs the actual seam/proxy design, not for a high-level phase summary.
- Load `references/rollback-and-exit-criteria.md` only when defining or auditing phase gates.
- Load `references/rewrite-vs-incremental-decision.md` only when the user is undecided between rewrite and incremental strangling.

## Context7 Documentation Protocol

Every framework-specific migration API, codemod name, config flag, or "recommended path" claim in a response must be traceable to one of:

1. **Context7-verified** — resolved via `mcp__Context7__resolve-library-id` then confirmed with `mcp__Context7__query-docs` against the specific library (e.g. `/reactjs/react.dev`, `/vercel/next.js`) in this session or a prior verification you can cite by source URL.
2. **Official docs (direct citation)** — a specific docs URL, used when Context7 has no coverage for that library/version.
3. **Inference** — explicitly labeled as such; never presented as a verified codemod/flag.

Do not invent codemod names, CLI flags, or config keys. If Context7 and official docs both lack coverage for a specific claim (e.g. an exact flag for a niche bundler), say so and mark it `inference — verify against installed tooling` rather than guessing. Re-verify before citing if the last check is not from the current session, since migration tooling and recommended paths change across releases (e.g. Next.js has shipped multiple generations of App Router migration guidance; React Compiler adoption guidance is still evolving).

Confirmed in this skill's authoring session (re-verify if stale):

- Next.js ships `npx @next/codemod cra-to-next` for CRA → Next.js migration, and a `rewrites().fallback` config for proxying unmigrated routes during incremental strangler cutover. Source: `vercel/next.js` docs (`01-app/02-guides/upgrading/codemods.mdx`, `01-app/03-api-reference/05-config/01-next-config-js/rewrites.mdx`).
- Next.js explicitly supports `app/` and `pages/` directories coexisting during App Router migration and recommends breaking the migration into small incremental steps. Source: `vercel/next.js` docs (`01-app/02-guides/migrating/app-router-migration.mdx`).
- React Compiler supports incremental adoption via `compilationMode: 'annotation'` plus `"use memo"` / `"use no memo"` directives, letting teams opt components in/out individually before flipping to full inference mode. Source: `reactjs/react.dev` docs (`reference/react-compiler/directives.md`, `learn/react-compiler/incremental-adoption.md`).

## References

Load these only when needed:

- [Strangler boundary design](references/strangler-boundary-design.md) — use when the user needs the actual technical seam (proxy/route-table, module federation, adapter layer) between legacy and modern code, not just a phase list.
- [Rollback and exit criteria](references/rollback-and-exit-criteria.md) — use when defining or auditing what makes a phase safe to promote, and what makes legacy code eligible for decommission.
- [Rewrite vs. incremental decision](references/rewrite-vs-incremental-decision.md) — use only when the user is weighing a full rewrite against strangler-fig incrementalism and needs the decision criteria, not for already-decided migrations.

## Response minimum

Return, at minimum:

- the migration type classification (cross-framework vs. same-framework) and why,
- the phase sequence with entry/exit criteria per phase,
- the rollback design per phase,
- the legacy-decommission criteria and time-box,
- evidence level for any framework API/codemod cited (Context7-verified vs. inference).
