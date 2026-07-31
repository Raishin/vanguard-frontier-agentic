---
name: react-component-architecture-review
description: Statically review React component trees for composition, prop-interface, and state-placement defects (God-components, prop drilling, overbroad context, hook-rule violations) against React's own composition guidance, producing ranked file:line findings.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# React Component Architecture Review

## Purpose

Review React component decomposition, prop-interface design, and state-placement decisions (local vs lifted vs context vs external store) without re-litigating styling, live performance profiling, or state-management library selection in every response. This skill exists so those adjacent concerns stay out of scope and the review stays focused on structural defects that erode testability and maintainability.

## When to use

Use this skill when the user asks to:

- review a React component or feature PR for architecture before merge,
- answer "is this component doing too much",
- audit prop-drilling or context usage across a component subtree,
- assess whether new or existing components should be split, merged, or recomposed.

Do not use this skill for:

- pure styling/CSS review — no architecture concern is in scope,
- performance profiling that requires live browser traces — that needs a runtime tool, not static review,
- state-management library selection with no existing code to review — that is a design conversation, not a review.

## Context7 Documentation Protocol

- Resolve the React library ID with `resolve-library-id` (matched result: `/reactjs/react.dev`) before citing any React-specific claim.
- Before asserting a component-splitting or state-placement recommendation is "React's recommended pattern," call `query-docs` against the repo's actual React major version (read `package.json` first) and cite the doc section — do not assert from memory.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Never assume the latest React docs apply to an older major version in the repo; hook rules and context APIs have changed across majors (e.g., `Context.Provider` vs. `<Context value={...}>`).

## Lean operating rules

- First read `package.json` to confirm the installed React major version. Do not make an API-availability claim without confirming the version.
- Classify each in-scope component as presentational, container, or compound before evaluating it. Do not apply a single decomposition rule to all three uniformly.
- Context usage is not automatically a smell. Values that change rarely and are needed broadly (theme, auth, locale) are an appropriate use of context per React's own guidance; only flag context when it causes overbroad re-renders or is used in place of straightforward prop passing at shallow depth.
- Do not recommend a specific state-management library (Redux, Zustand, Jotai, etc.) unless one is already a dependency in the repo. A decomposition problem is not a tooling problem.
- Do not fabricate re-render counts or performance claims without a live profiler; label such estimates `inference, not measured`.
- Treat deep prop chains that match an intentional compound-component or render-prop API as a design pattern, not a defect — verify intent before flagging.
- Cap the blast radius of a single review: if the review would require rewriting more than 5 components, stop and flag it as "requires a dedicated refactor plan" rather than prescribing the rewrite inline.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Treat any hardcoded API key, token, or secret found in component props, default values, or example data as a HIGH-severity finding requiring immediate escalation, not a style note.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the decision tree for split/context/compound-API calls, and the required output shape.
- [Composite widget patterns](references/composite-widget-patterns.md) — load only when the component in scope is a composite widget (combobox, tabs, dialog, listbox) where deep prop chains may be an intentional ARIA APG pattern rather than drilling.

## Response minimum

Return, at minimum:

- the component(s) and files in scope,
- ranked findings with file:line evidence and a concrete refactor sketch per finding,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "requires a dedicated refactor plan" cap, missing live re-render data).
