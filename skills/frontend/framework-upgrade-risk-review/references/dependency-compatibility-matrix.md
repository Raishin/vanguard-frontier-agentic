# Dependency Compatibility Matrix

Use this reference when the actual upgrade blocker is a third-party dependency's peer-dependency constraint, not a breaking change in the core framework itself.

## What people get wrong

The naive assumption is:

> "The framework's own migration guide says the upgrade is straightforward, so the upgrade is straightforward."

Wrong. The framework's own breaking-change list only covers the framework. It says nothing about whether the state-management library, UI component library, testing library, or build plugin the project actually depends on has published a compatible release yet. A framework-major upgrade is frequently blocked not by the framework's own breaking changes but by an ecosystem library that has not shipped peer-dependency support for the new major, sometimes for months after the framework's release.

## The actual compatibility surface

For a same-framework major upgrade, the dependencies most likely to gate the upgrade are:

- **UI component libraries** built directly against the framework's internals (not just its public API) — these are the most common blockers because internal API changes (e.g. React's internal fiber/reconciler changes, Angular's internal change-detection changes) can break a component library even when the library's *usage* of the public API looks unchanged.
- **State-management libraries** with framework-version-pinned peer dependencies.
- **Testing-library integrations** (React Testing Library, Angular TestBed harnesses, Vue Test Utils) — these often require a matching major version to support new rendering/hydration internals, and a stale version can produce misleading green tests that do not reflect real runtime behavior.
- **Build-tool plugins/loaders** that wrap the framework's compiler (e.g. a bundler plugin implementing framework-specific fast-refresh or SSR transforms) — these must track framework-internal changes closely and are a common source of "works with `next dev` but breaks under the plugin" issues.
- **Meta-framework version coupling** — e.g. Next.js major versions are coupled to a specific React major/minor floor; do not assess a Next.js upgrade's React compatibility from memory, check the specific Next.js major's stated React peer-dependency range in its own official docs for that release.

## Procedure

1. Enumerate the project's direct dependencies with the framework's own package as a peer dependency (read `package.json` `peerDependencies` fields of the top N most framework-coupled packages actually installed, not a generic list).
2. For each, check whether a release compatible with the target framework major has shipped. Prefer checking the dependency's own official changelog/release notes over inferring from its semver range, since a library may have shipped a compatible peer-dependency range before actually fixing all the runtime breakage that range implies is safe.
3. If a dependency has *not* shipped compatible support, this is a hard blocker independent of how clean the framework's own breaking-change list is — say so explicitly and do not let a clean framework migration guide imply a clean overall upgrade.
4. If a dependency requires a beta/RC release to get compatibility, flag that as an elevated-risk path (pinning to a pre-release for a production dependency) rather than presenting it as equivalent to a stable compatible release.
5. Distinguish "peer-dependency range technically allows the new major" from "the maintainer has stated they tested against the new major" — a permissive semver range is not evidence of compatibility, it is often just an unmaintained range that was never tightened.

## When to push back

Push back if the user wants to force-install (`--legacy-peer-deps` / `--force`) past a peer-dependency conflict as the resolution rather than as a temporary unblock — that suppresses the signal that a real incompatibility may exist and should be logged as residual risk, not treated as "resolved."

Push back if the user assumes a component library "probably still works" because it hasn't announced incompatibility — the absence of an announcement is not evidence of compatibility, especially for internal-API-coupled libraries; recommend an actual smoke test of the specific components in use.

Push back if the plan is to upgrade the framework and defer checking ecosystem-dependency compatibility until after merge — for framework-internal-coupled dependencies (UI kits, test harnesses, build plugins) this ordering inverts the actual risk: the framework's own official migration guide is usually the well-tested, well-documented part, while third-party catch-up compatibility is the least-documented and most likely source of surprise regressions.
