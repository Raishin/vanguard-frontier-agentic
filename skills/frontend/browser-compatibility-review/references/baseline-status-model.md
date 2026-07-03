# Baseline Status Model

Use this reference when determining or explaining the exact Baseline tier of a specific feature and what that tier does — and does not — guarantee for the project's own matrix.

## What people get wrong

The naive story is:

> "Baseline: Widely available" means it works everywhere I need it to.

Wrong. Baseline is computed against a fixed reference set of core browser engines (Chrome/Edge, Firefox, Safari — desktop and their mobile counterparts), not against the org's actual supported-browser matrix. A feature can be "Widely available" in Baseline terms and still be broken for a real user on an unsupported engine (older WebViews, embedded browsers, some enterprise-locked builds) that Baseline does not track at all. Baseline tells you about cross-engine standards convergence; it does not tell you about the org's own matrix.

## Officially grounded tiers

Per the `web-features` project's `compute-baseline` package (verified via Context7, `/web-platform-dx/web-features`):

- Every computed status carries a `baseline` field with one of three effective values:
  - `'high'` — **Widely available**. The feature has been supported by all core browser engines for roughly 30 months (2.5 years).
  - `'low'` — **Newly available**. The feature just became supported across all core engines, but has not yet cleared the widely-available window.
  - `false` — **Limited availability**. At least one core browser engine does not yet support the feature.
- A `'low'`/newly-available result carries a `baseline_low_date` (when cross-engine support was first reached) and, once promoted, a `baseline_high_date` (when it crosses into widely-available, typically ~30 months after `baseline_low_date`).
- `getStatus(featureId, compatKey)` returns the Baseline status for a feature that has completed the project's editorial review; `computeBaseline({ compatKeys, checkAncestors })` computes status for a set of raw compat keys, including features that have not undergone editorial review — treat the latter as provisional and say so.
- `checkAncestors: true` matters for features that are only reachable through a parent capability gated behind a flag or prefix (e.g. an API method whose containing interface is itself experimental) — check ancestors before declaring a leaf API "supported."

## Non-negotiable rules

1. **Never collapse "Newly available" into "safe to use unguarded."** A feature that just crossed into `baseline: 'low'` explicitly excludes browser versions released before the cross-engine convergence date — if the project's matrix includes any browser version older than `baseline_low_date`'s corresponding release, the feature is Limited-Availability *for that project*, regardless of its global Baseline label.
2. **Baseline is not the org's matrix.** Always re-derive the actual compatibility verdict against the project's declared Browserslist/matrix config (see `browserslist-matrix-interpretation.md`), not against Baseline's fixed reference set alone.
3. **Editorial-reviewed vs computed-only status are not the same confidence level.** `getStatus` results have been through the web-features project's review process; raw `computeBaseline` results have not. Label computed-only results as provisional in the finding.
4. **A single low-support browser in the matrix downgrades the whole finding.** If the matrix declares support for even one browser version below what the feature needs, the finding is "unguarded use of a Limited-Availability feature for this project," not "Widely available, no action needed."
5. **Do not confuse Baseline tier with caniuse's raw percentage-of-global-usage figures.** caniuse usage stats answer "how many people, globally, use a supporting browser"; Baseline answers "has this converged across core engines." Both matter, but they answer different questions — cite whichever one actually grounds the claim being made, and do not substitute one for the other.

## Verification targets

- The Baseline `baseline` value and, if `'low'`, the `baseline_low_date` for the feature in question (via `mcp__Context7__query-docs` against `/web-platform-dx/web-features`, or the live `web-features` dataset / `https://web.dev/baseline`).
- The per-browser support table for the feature on `https://caniuse.com/` or MDN's browser-compatibility table, to identify exactly which matrix-declared browser versions lack support.
- The project's own Browserslist/matrix config (see `browserslist-matrix-interpretation.md`) to determine whether any excluded browser is actually in scope.

## When to push back

Push back if the user asks to:

- approve a feature as "safe" solely because a badge or article calls it "Baseline" without stating which tier,
- treat a computed-only (`computeBaseline`, no editorial review) result as an authoritative, final verdict,
- ignore the project's own matrix because "Baseline says it's fine" — Baseline's reference set and the project's matrix are not the same thing, and conflating them produces false-negative compatibility reviews.
