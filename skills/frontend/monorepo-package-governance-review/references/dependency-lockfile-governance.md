# Dependency and Lockfile Governance

Use this reference for reviewing dependency version policy, pnpm catalogs, npm overrides, lockfile/manifest consistency, and lifecycle-script risk in a monorepo workspace.

> Version note: pnpm `catalog`/`catalogs` syntax and the `catalog:` protocol are documented at `pnpm.io/catalogs` and `pnpm.io/pnpm-workspace_yaml`. Confirm the installed pnpm major version supports catalogs (a relatively recent feature) via Context7 before recommending a migration to it.

## What people get wrong

The naive story is:

> "The lockfile is just an implementation detail; reviewing `package.json` is enough."

Wrong. A `package.json` range like `^4.2.0` and a lockfile pinned to `4.2.0` can silently diverge from what actually gets installed the next time `install` runs without `--frozen-lockfile` (or the pnpm/npm equivalent). Dependency governance review has at least three separate concerns, and collapsing them into "check package.json" misses two of the three:

1. **declared range** — what `package.json` (or a pnpm catalog entry) says is acceptable,
2. **resolved version** — what the lockfile actually pins right now,
3. **install-time behavior** — whether CI enforces the lockfile (frozen install) or silently re-resolves and drifts.

## Officially grounded shape

From pnpm docs (`catalogs`, `pnpm-workspace_yaml`, `settings`):

- `pnpm-workspace.yaml` top-level `catalog:` defines a default catalog of shared dependency versions, referenced in any package's `package.json` via the `catalog:` protocol (or `catalog:default` explicitly).
- `catalogs:` (plural) defines named catalogs (e.g., `react17`, `react18`) for cases where different packages in the workspace intentionally need different major versions of the same dependency during a migration.
- The `catalog:` protocol can also be used inside `pnpm-workspace.yaml`'s own `overrides` block (`overrides: { foo: "catalog:" }`) to force a single resolved version workspace-wide while keeping the source of truth in one catalog entry.
- Catalogs solve version **drift** (same dependency, different ranges in different packages) but do not by themselves solve version **pinning** (exact vs range) — those are separate governance questions.

npm's dependency-governance primitives (`overrides` in `package.json`, `resolutions` in Yarn) serve a similar drift-consolidation role but operate differently: `overrides` forces a resolved version across the entire tree regardless of what nested dependencies request, which can mask a legitimate need for two different major versions if applied too broadly.

## Non-negotiable design rules

### 1. Lockfile changes must be explainable by a manifest change

If a lockfile diff shows resolved-version changes with no corresponding `package.json`/`pnpm-workspace.yaml` diff, treat that as suspicious until explained (could be a legitimate transitive-dependency update from a fresh `install`, or could indicate an out-of-band lockfile edit or a compromised registry response). Ask for the `install` command/log that produced the diff before approving.

### 2. Catalog/override consolidation is a security control, not just tidiness

Multiple packages independently specifying loose ranges (`^4.0.0`) for the same security-sensitive dependency means each package can independently resolve to a different patch/minor version over time, widening the set of versions that must be trusted. Consolidating via a pnpm catalog entry or npm `overrides` forces one resolved version, shrinking the audit surface — recommend this specifically for auth, crypto, or serialization libraries, not indiscriminately for every dependency.

### 3. Pinning policy depends on the team's automation, not a blanket rule

Do not require exact-version pins (`4.2.0` instead of `^4.2.0`) unless asked, unless the dependency is security-sensitive, or unless there is no automated update+CI-gate pipeline (Renovate/Dependabot with required status checks) in place. A team with working automated updates and a frozen-lockfile CI gate already has reproducibility; blanket exact-pinning there adds churn without adding safety. Ask about the update-automation setup before recommending a pinning policy change.

### 4. Lifecycle scripts are quoted, not summarized

When a new or bumped dependency introduces a `postinstall`, `preinstall`, or `prepare` script, quote the script's actual content from the dependency's `package.json` (or lockfile-recorded script) in the finding. Do not infer safety or danger from the package name or its stated purpose — a script literally reading `node scripts/postinstall.js` requires opening that file, not accepting the name as self-explanatory.

### 5. Frozen installs are the enforcement mechanism — verify CI actually uses one

A correct lockfile is meaningless if CI runs a non-frozen install (allowing silent re-resolution). Check the CI workflow for the frozen-install flag appropriate to the package manager (e.g., pnpm's `--frozen-lockfile`, npm's `ci` command) before concluding that lockfile governance is actually enforced end-to-end.

## Minimal safe review flow

1. Identify the package manager and workspace layout (`pnpm-workspace.yaml`, npm/yarn `workspaces` field in root `package.json`).
2. For the dependency/package in scope, compare the declared range(s) across every package that references it — flag drift (same dependency, different ranges, no catalog/override tying them together).
3. Cross-check the lockfile's resolved version(s) against declared ranges; flag any resolved version outside a declared range (should not normally happen, but indicates a manifest/lockfile edit order-of-operations problem) and any case with multiple distinct resolved versions for what should be one consolidated dependency.
4. For any new/bumped dependency in the diff, check its `package.json` for `postinstall`/`preinstall`/`prepare` and quote the script content verbatim if present.
5. Check the CI workflow for a frozen/`ci`-style install command; flag if absent.
6. Report findings with exact file:line citations; propose the catalog/override/pin diff without applying it.

## Adversarial checklist

Before recommending a dependency-governance change, answer these:

- Does the lockfile diff have a corresponding manifest diff, or is it unexplained?
- For the dependency being consolidated, is there a legitimate reason two packages need different major versions (active migration), or is the split accidental drift?
- Does CI actually enforce the lockfile with a frozen-install flag, or can a fresh `install` silently re-resolve?
- If a lifecycle script is present, have I read its actual content, not just its filename?
- Is the pinning policy I'm recommending consistent with whether this team already runs automated dependency updates with CI gating?

If you cannot answer these from repo evidence, say so rather than issuing a blanket pinning recommendation.

## When to push back

Push back if the user asks to:

- "just pin everything to exact versions" without checking whether an update-automation + CI-gate pipeline already provides reproducibility — this trades update velocity for a pinning policy that may not add real safety,
- consolidate every dependency into one catalog entry regardless of legitimate multi-version migration needs (e.g., forcibly unifying `react17`/`react18` catalogs mid-migration),
- accept a lockfile diff with no explainable manifest change "because CI passed" — a passing CI run does not prove the lockfile change was legitimate, only that the resolved versions installed successfully,
- ignore a `postinstall` script because "it's a well-known package" — package reputation does not substitute for reading the actual script content in the version being introduced.

Those are not shortcuts. They trade a diagnosable supply-chain question for an assumption that will not hold under a compromised-dependency scenario.
