---
name: framework-upgrade-risk-review
description: Assess breaking-change and regression risk for a same-framework major-version upgrade (React, Next.js, Angular, Vue, or core build tooling), grounding every claimed breaking change in the framework's official release notes/migration guide, and separate upgrade-blocking issues from cosmetic deprecation noise.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: resilience
---

# Framework Upgrade Risk Review

## Purpose

Framework major-version upgrades fail in production not because the upgrade was a bad idea, but because the breaking-change surface was assessed from stale memory instead of the actual release notes for the actual installed version. This skill exists to ground every claimed breaking change in current official docs and separate what will actually break the build/runtime from what is merely deprecated-but-still-working — without stuffing every framework's entire changelog history into every response.

## When to use

Use this skill when the user asks to:

- assess risk before upgrading a framework or core build tool to a new major version,
- explain what specifically will break when moving from version X to version Y,
- decide whether a security-driven forced upgrade (CVE fix in a new major version) outweighs the migration effort,
- triage a failing build/test suite after an upgrade attempt to determine which failures are upgrade-caused vs. pre-existing.

## Lean operating rules

- Always resolve the exact currently-installed version from the lockfile (`package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`) before assessing risk; "upgrading React" without knowing the current pinned version produces meaningless risk output.
- Query Context7/official release notes for the target version's actual breaking-change list before writing any risk assessment — never assemble this from training-data memory alone, since breaking-change lists change per release and are exactly the kind of fact that goes stale.
- Separate hard breaking changes (code will not compile/run) from soft deprecations (still works, warns, scheduled for future removal) — conflating them causes both false alarm and false confidence.
- Treat a breaking change tied to a security fix as a forced-upgrade driver that overrides normal cost/benefit sequencing; also flag if the current pinned version is already past its security-support window regardless of migration effort.
- Do not assume a codemod exists for a given breaking change unless it is named in the official migration guide; do not invent codemod commands, CLI flags, or config keys.
- Treat multi-major-version jumps (e.g. React 16 → 19, Next.js 12 → 15) as requiring sequential per-major risk assessment, not a single diffed comparison — intermediate majors carry their own breaking changes that compound.
- Load `references/breaking-change-triage.md` only when classifying specific breaking changes as blocking vs. non-blocking and mapping them to affected code paths.
- Load `references/dependency-compatibility-matrix.md` only when third-party dependencies (not the core framework itself) are the upgrade blocker.

## Context7 Documentation Protocol

Every framework-specific breaking-change claim, codemod name, removed API, or "recommended migration path" statement in a response must be traceable to one of:

1. **Context7-verified** — resolved via `mcp__Context7__resolve-library-id` then confirmed with `mcp__Context7__query-docs` against the specific library (e.g. `/reactjs/react.dev`, `/vercel/next.js`) in this session or a prior verification you can cite by source URL.
2. **Official docs (direct citation)** — a specific docs/changelog URL, used when Context7 has no coverage for that library/version.
3. **Inference** — explicitly labeled as such and flagged for human verification against the installed toolchain; never presented as a verified breaking change.

Do not invent codemod names, CLI flags, or config keys. If Context7 and official docs both lack coverage for a specific claim (e.g. an exact flag for a niche bundler plugin), say so and mark it `inference — verify against installed tooling` rather than guessing. Re-verify before citing if the last check is not from the current session — breaking-change lists and codemod tooling are actively evolving (React ships incremental React Compiler / Server Components guidance; Next.js has shipped multiple async-API and caching-default changes across recent majors).

Confirmed in this skill's authoring session (re-verify if stale):

- React 19 requires migrating `ReactDOM.render` to `createRoot`/`hydrateRoot`, removes `propTypes`/`defaultProps` support on function components (replace with TypeScript types/ES6 default parameters), and removes string refs in favor of ref callbacks. React ships `npx codemod@latest react/19/migration-recipe` as a combined codemod, plus a targeted `npx codemod@latest react/19/replace-string-ref`. Source: `reactjs/react.dev` docs (`blog/2024/04/25/react-19-upgrade-guide.md`).
- Next.js 15 makes previously synchronous dynamic APIs (`cookies()`, `headers()`, `draftMode()`, and `params`/`searchParams` in pages, layouts, and `generateMetadata`/`generateViewport`) asynchronous — all call sites must `await` them or use `React.use()` in Client Components. Next.js ships `npx @next/codemod@latest next-async-request-api .` to automate this migration. Source: `vercel/next.js` docs (`01-app/02-guides/upgrading/version-15.mdx`, `01-app/02-guides/upgrading/codemods.mdx`).
- Next.js 15 also changes the default caching behavior: `fetch` requests are no longer cached by default and require explicit `{ cache: 'force-cache' }` to opt back into the previous caching behavior. This is a runtime/behavioral breaking change, not a compile-time one, so it will not surface as a build failure — it surfaces as stale-vs-fresh data regressions. Source: `vercel/next.js` docs (`01-app/02-guides/upgrading/version-15.mdx`).

## References

Load these only when needed:

- [Breaking-change triage](references/breaking-change-triage.md) — use to classify each breaking change from the official migration guide as build-blocking, runtime-blocking, or cosmetic-deprecation, and to map each to affected code paths via repo search.
- [Dependency compatibility matrix](references/dependency-compatibility-matrix.md) — use when the risk driver is a third-party library's peer-dependency constraint rather than the core framework's own breaking changes.

## Response minimum

Return, at minimum:

- the exact current version and target version being assessed (sourced from the lockfile, not assumed),
- the breaking-change list sourced from official release notes/Context7 (with citation), split into blocking vs. cosmetic,
- any security-advisory-driven upgrade urgency or security-support-window expiration,
- an estimated blast radius (files/modules touching each blocking change, found via repo search),
- evidence level for every claimed breaking change (Context7-verified, official-docs-cited, or inference — inference must be flagged for human verification against the installed toolchain).
