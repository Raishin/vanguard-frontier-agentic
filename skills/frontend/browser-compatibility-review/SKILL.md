---
name: browser-compatibility-review
description: Audit JS/CSS/HTML feature usage against the project's declared Browserslist/supported-browser matrix using Baseline and caniuse status data, flag unguarded non-Baseline usage, and verify feature-detection or polyfill fallback coverage, with per-feature caniuse/Baseline lookups loaded only for features actually in question.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: platform
---

# Browser Compatibility Review

## Purpose

"Works on my browser" is not a compatibility strategy. This skill checks used web-platform features against the org's actual declared supported-browser matrix (Browserslist config or an explicit browser-version list) using Baseline/caniuse status, and verifies that any feature outside "widely available" has a real feature-detection or polyfill fallback rather than a silent failure.

## When to use

Use this skill when the user asks to:

- review a PR using a new JS API, CSS feature, or HTML element for cross-browser risk,
- audit the overall Baseline-status distribution of features used in a codebase,
- decide whether a feature needs a polyfill, feature-detection gate, or is safe to use unguarded,
- propose narrowing or widening the org's supported-browser matrix,
- triage a browser-specific bug report.

## Context7 Documentation Protocol

Baseline status, caniuse support tables, and Browserslist/tooling behavior change on their own release cadences, independent of this skill's version — never assert a feature's Baseline tier, a browser's support version, or a config-syntax detail from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` for `web-features` (prefer `/web-platform-dx/web-features`) when grounding a Baseline-status claim (widely/newly/limited availability, `baseline_low_date`/`baseline_high_date` semantics) — do not paraphrase Baseline's tiering from memory.
3. Call `mcp__Context7__query-docs` for the specific feature or mechanism in question — e.g. "compute Baseline status for a compat key", "getStatus for a web-features id" — before stating a feature's tier as fact. Do this per review, not once from a prior session's memory.
4. For the exact per-feature per-browser support matrix (which browser version added support), prefer live lookups against `https://caniuse.com/` and MDN's browser-compatibility tables in `official_docs` over Context7 paraphrase — Context7's `web-features` package computes Baseline tiers from that same underlying data but is not the source of truth for a single browser-version cell.
5. Browserslist's own config syntax and query semantics are not consistently resolvable in Context7 (verified: no dedicated `browserslist` library was found when this skill was authored) — treat any Browserslist query-syntax claim as `documentation-based (Context7 unavailable for this library)` and confirm it against the project's actual `.browserslistrc` / `package.json` `browserslist` key and the official `browserslist/browserslist` GitHub README rather than inventing query syntax.
6. If Context7 returns no relevant match for a claim, fall back to the `official_docs` URLs and mark the claim `documentation-based (Context7 unavailable)` instead of presenting it as freshly verified.
7. Never invent a Baseline tier, a caniuse support percentage, or a Browserslist query keyword that no queried source confirms.

## Lean operating rules

- Always check the feature against the project's actual declared Browserslist config or explicit browser-version list — never approve based on the reviewer's own current browser, and never assume a default matrix (e.g. `> 0.5%, last 2 versions, Firefox ESR, not dead`) applies without reading the project's own config.
- Distinguish Baseline "Newly available" (recently reached cross-engine support, but by definition still excludes older browsers within the ~2.5-year newly-to-widely window) from "Widely available" (safe to use unguarded for most matrices) from "Limited availability" (requires a fallback). Treat these as the three tiers computed from `baseline_low_date` / `baseline_high_date`, not as marketing labels.
- For any Limited-Availability feature, or a Newly-Available feature whose window excludes a browser/version actually in the project's matrix, verify a real `@supports`/feature-detection/polyfill exists and correctly gates the risky code path — do not accept an assertion that a fallback "exists" without reading the code that implements it.
- Weigh polyfill bundle-size cost against the actual percentage of affected users (from real analytics/RUM data if available) rather than blanket-recommending every polyfill; a polyfill added for a browser with near-zero real traffic is often a worse tradeoff than accepting the gap.
- Treat any hard failure (thrown exception, blank render, broken checkout) as strictly higher severity than cosmetic degradation (missing rounded corners, no animation), and prioritize findings accordingly.
- Recommend supported-browser-matrix changes only as data-backed proposals to product/analytics owners — this skill does not unilaterally decide the org's matrix, it surfaces the gap and the tradeoff.
- Never recommend disabling a security-relevant browser default (mixed-content blocking, SameSite cookie defaults, CSP enforcement, Permissions Policy) as a "compatibility workaround" — that is a security regression, not a fix, and must be flagged as such if proposed by the user.
- Load reference files only for the review question actually in scope (single-feature triage vs full-codebase Baseline sweep vs matrix-change proposal); do not preload the full feature catalog for a one-feature check.

## References

Load these only when needed:

- [Baseline status model](references/baseline-status-model.md) — use when determining or explaining the exact Baseline tier (widely/newly/limited) of a specific feature and what that tier does and does not guarantee for the project's matrix.
- [Fallback verification patterns](references/fallback-verification-patterns.md) — use when a finding requires checking whether an actual feature-detection gate or polyfill exists and correctly covers the risky code path, across JS, CSS, and HTML.
- [Browserslist matrix interpretation](references/browserslist-matrix-interpretation.md) — use when reading, interpreting, or proposing a change to the project's `.browserslistrc`/`browserslist` config, or reconciling it with build-tool (Autoprefixer/Babel/postcss-preset-env) targets.

## Response minimum

Return, at minimum:

- the feature(s) in scope and their exact Baseline status (widely / newly / limited, with the underlying date window if newly available),
- the specific browsers in the org's declared matrix that lack support, if any, cited against the project's actual Browserslist/matrix config,
- current fallback status (none / feature-detected / polyfilled) with the actual code shown, not an assertion,
- severity distinction between hard failure and cosmetic degradation,
- polyfill cost-vs-affected-user tradeoff note if a polyfill is recommended,
- evidence label for every claim (`live evidence` from project config/code, `documentation-based`, or `inference`), and an explicit note when Context7 was unavailable for a cited claim.
