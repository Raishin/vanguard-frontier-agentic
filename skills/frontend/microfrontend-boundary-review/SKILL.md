---
name: microfrontend-boundary-review
description: Reviews micro-frontend/module-federation boundary contracts for shared-dependency versioning safety, runtime isolation, and ownership clarity before adoption or extension of a distributed frontend architecture.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Micro-Frontend Boundary Review

## Purpose

Gate the adoption or extension of a micro-frontend / module-federation architecture on three things that determine whether it actually reduces coupling or merely relocates it: an explicit shared-dependency version-compatibility contract, a runtime-isolation level matched to each remote's data sensitivity, and unambiguous per-remote ownership with a bounded blast radius. Micro-frontends are frequently adopted to solve an organizational coupling problem (multiple teams shipping one deployable) but, done carelessly, they trade that problem for a worse one: distributed runtime coupling where one team's broken bundle or dependency bump breaks a sibling team's remote in production. This skill exists to make that trade-off explicit before it ships, not to rubber-stamp micro-frontends as a default architecture.

## When to use

Use this skill when the user asks to:

- adopt micro-frontends or module federation for the first time in a project that is currently a single deployable,
- add a new remote to an existing micro-frontend host (including a third-party-built or externally-owned remote),
- audit shared-dependency version drift across existing remotes that is causing or risks causing runtime breakage,
- review the isolation level (iframe vs. same-runtime composition) chosen for a remote against the sensitivity of the data it handles.

Do not use this skill for:

- module/package boundary review within a single deployable application — that is `frontend-platform-architecture-review`,
- reviewing whether server-side aggregation belongs in a BFF versus client-side composition — that is `frontend-bff-boundary-review`,
- component-level React architecture (props, composition, state placement) inside one remote or host — that is `react-component-architecture-review`,
- approving micro-frontends as a default architectural choice; this skill's default posture is to make the team justify the distributed-coupling trade-off, not to assume it is warranted.

## Context7 Documentation Protocol

- Resolve `/reactjs/react.dev` with `resolve-library-id` before evaluating any same-runtime composition pattern (module federation without iframes, build-time composition, or any design where a host and one or more remotes mount into the same page) that puts multiple React roots on one page or that requires the host and a remote to interoperate at runtime.
- Query the resolved docs for `createRoot` and `useId` behavior before approving that pattern. Official React docs confirm `createRoot` is designed to be called multiple times on one page — "a page that uses 'sprinkles' of React for parts of the page may have as many separate roots as needed" — so multiple independently-mounted roots is a supported pattern, not a hack. Docs also confirm that independent React applications sharing a page must pass a distinct `identifierPrefix` to `createRoot` (or `hydrateRoot`) to prevent `useId`-generated identifier collisions between them; a same-runtime composition with two or more `createRoot` calls and no distinct `identifierPrefix` per root is a concrete, checkable defect, not a style preference.
- React's official docs do not state a cross-major-version compatibility guarantee for multiple React copies coexisting in one page (e.g., a host on React 18 and a remote on React 19 sharing a runtime). Treat any claim that "different React versions across remotes is safe because each root is independent" as `documentation-based, unconfirmed` — flag it as a required verification item rather than asserting it is safe or unsafe from memory.
- Where a claim depends on module-federation-tooling behavior specifically (Webpack Module Federation shared-scope negotiation, Vite plugin federation's `shared` config, singleton flags, `strictVersion` behavior), require the proposer to cite that tool's own current documentation. That tooling is outside React/Next.js core and is not assumed covered by the React docs grounding above — do not extend React-docs-grounded claims to cover federation-tool-specific version-negotiation behavior.
- If Context7 is unavailable, state every runtime-isolation and shared-dependency claim as `documentation-based, unconfirmed` rather than asserting it from memory, and require the user to verify against the installed React version and the federation tool's current docs before treating the boundary as approved.

## Lean operating rules

- First establish the composition mechanism in scope (module federation, iframes, or build-time/server-side composition) — the isolation and versioning risks differ by mechanism, and advice given for one mistakenly applied to another is unreliable.
- Determine the isolation level required from each remote's data sensitivity before reviewing anything else. A remote that renders or handles data that should not share a CSP/JavaScript-runtime trust boundary with sibling remotes needs iframe isolation or an equivalent mitigation; same-runtime composition is not a neutral default for that remote regardless of how convenient it is.
- Do not treat "it uses module federation" as evidence of isolation. Module federation and other non-iframe composition share the same JavaScript runtime and the same CSP context across host and remotes — a vulnerability or a broken bundle in one remote can affect the host and every sibling remote sharing that runtime.
- Require an explicit, documented shared-dependency version-compatibility policy (pinned versions, a defined compatible range, or singleton/strictVersion enforcement in the federation config) before approving adoption or a new remote. Unmanaged, unmonitored version drift across remotes is the most common cause of micro-frontend production incidents and is not acceptable as "we'll handle it if it breaks."
- Require a bounded blast radius for every remote: an error boundary or equivalent isolation around the remote's mount point so an uncaught error or broken bundle in that remote does not take down the host or sibling remotes, and an independent deployment/rollback path so one remote's release cannot force a host-wide rollback.
- Require unambiguous, single-team ownership per remote. A remote with shared or unclear ownership has no clear on-call responsibility when it breaks in production, which defeats the organizational-coupling benefit micro-frontends were adopted for in the first place.
- Do not accept "micro-frontends reduce coupling" as self-evidently true. Ask what coupling moved from build-time (a monorepo/shared-deploy dependency graph, which is visible and tooling-enforced) to runtime (a shared-dependency and shared-CSP-context graph, which is often invisible until it breaks in production).
- Never execute, build, or run application code, module-federation dev servers, or bundler configs as part of this review; this is a static-review skill (Read/Grep/Glob only) — review the composition config, shared-dependency manifest, and ownership documentation as given.
- Treat any hardcoded credential, API key, or token found in a remote's config, shared module, or example fixture as a HIGH-severity finding requiring immediate escalation, separate from the boundary-contract verdict.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step boundary-review procedure (isolation, versioning, ownership, blast radius) and the required output shape.
- [Shared-runtime security and CSP boundary](references/shared-runtime-security.md) — use only when a remote handles sensitive data and same-runtime (non-iframe) composition is proposed or in place, to ground the CSP trust-boundary collapse risk and isolation mitigations.

## Response minimum

Return, at minimum:

- the composition mechanism in scope (module federation, iframes, build-time/server-side composition) and whether it was confirmed from config or asserted by the user,
- the isolation-level verdict for each remote in scope, matched against its data sensitivity,
- the shared-dependency version-compatibility policy status (present/absent, and its mechanism if present),
- the blast-radius and ownership status per remote (error-boundary/isolation mitigation present, independent deploy path present, owning team named),
- ranked findings with file:line or config-key evidence where applicable, risk class, and fix,
- verdict: approve / approve-with-notes / block,
- evidence level (including whether React-version or federation-tool-version claims were confirmed or left `documentation-based, unconfirmed`) and open questions.
