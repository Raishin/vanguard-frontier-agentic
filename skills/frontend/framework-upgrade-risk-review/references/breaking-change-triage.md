# Breaking-Change Triage

Use this reference when classifying specific breaking changes from an official migration guide into blocking vs. cosmetic buckets, and when mapping each change to the code paths it actually touches.

> Version note: breaking-change lists are per-release and change across minor/patch versions within a major too (e.g. deprecation warnings added in a `.1` release ahead of removal in the next major). Re-pull the current official migration guide for the exact source and target versions before triaging — do not reuse a prior session's list without re-verifying it still matches.

## What people get wrong

The naive assumption is:

> "The migration guide lists N breaking changes, so there are N things to fix, roughly equal effort each."

Wrong. Migration guides mix at least three fundamentally different categories of severity, and treating them as equivalent produces either false alarm (blocking a low-risk upgrade over a warning) or false confidence (shipping an upgrade that silently changes runtime behavior).

## The three-tier classification

### 1. Build-blocking

The code will not compile or the build will fail outright. These are the safest category paradoxically — the build tells you immediately, and CI catches them before merge.

Examples grounded in official docs:
- React 19 removing `propTypes`/`defaultProps` on function components — this is a type/lint-level break in TypeScript codebases, not always a hard compile failure in plain JS, so verify whether the project uses TypeScript strict mode or just runtime PropTypes checks before calling it "build-blocking" vs. "silently degraded."
- Any removed export, removed CLI flag, or removed config key — grep the repo for the literal symbol/flag name to confirm actual usage before flagging it as in-scope; do not flag "this symbol was removed" as risk if the repo never imports it.

### 2. Runtime-blocking (behavioral break, not compile break)

The code compiles and builds fine, but behaves differently or throws at runtime. These are more dangerous than build-blocking changes because CI without adequate integration/e2e coverage will not catch them before production.

Grounded example: Next.js 15 making `cookies()`, `headers()`, `draftMode()`, and `params`/`searchParams` asynchronous. Old synchronous call sites do not necessarily fail to build in JS (they may still "work" until the returned Promise is used incorrectly) — they fail or misbehave at runtime, and the failure mode may only surface on paths exercised by real traffic, not by every test.

Grounded example: Next.js 15 changing `fetch` to be uncached by default. This produces no build error and no runtime crash — it produces a silent regression in data freshness or, more dangerously, in cache-hit-dependent performance/cost assumptions. This class of change is the one triage must not under-weight, because there is no error to signal it happened.

For every runtime-blocking change, identify:
- the specific API/behavior that changed (cite the migration guide section),
- whether an automated codemod exists and is officially named for it (do not assume — check `references/dependency-compatibility-matrix.md`-adjacent codemod listings in the framework's own upgrade docs),
- what test coverage (if any) would actually exercise the changed code path — if none, say so explicitly as a residual risk, not a solved problem.

### 3. Cosmetic deprecation

The code still works exactly as before; the framework emits a warning (console warning, lint warning, or doc note) that the API is scheduled for removal in a *future* major, not this one.

Do not present these as this-upgrade risk. List them separately as "deprecation debt to schedule before the *next* major," since conflating them with real blockers dilutes attention from what actually needs fixing before merge.

## Mapping to blast radius

For every build-blocking and runtime-blocking item (never cosmetic-only items, to keep the list actionable):

1. Identify the literal API/symbol/config key changed.
2. Search the repo (`Grep`) for actual usage of that symbol — imports, JSX usage, config keys, CLI invocations in `package.json` scripts.
3. Count distinct files/modules touched; do not estimate — enumerate.
4. Flag any usage inside test fixtures or mocks separately from production source, since test-only usage changes the risk profile (breaks CI, not production).
5. If the symbol is re-exported or wrapped by an internal abstraction layer, note that fixing the wrapper once may resolve many call sites — do not multiply file count by naive per-callsite fix cost.

## When to push back

Push back if the user asks you to://"just estimate the risk without checking the actual official migration guide for this version" — a risk assessment built from memory of a different major version (or a different framework entirely) is not a risk assessment, it is a guess wearing a risk assessment's clothes.
Push back if the user wants to skip the blast-radius search step because "it's probably fine" — "probably fine" is exactly the failure mode this skill exists to replace with an enumerated count.
Push back if the user wants to bundle a same-framework major upgrade together with unrelated dependency upgrades in one PR — that makes the blast radius of a failure ambiguous (which upgrade caused the regression?) and should be flagged as a review/rollback risk, not silently accepted.
