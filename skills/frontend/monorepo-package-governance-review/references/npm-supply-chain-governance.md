# npm Dependency-Confusion & Supply-Chain Governance

Use this reference for reviewing npm registry configuration (`.npmrc`), lockfile commitment/enforcement, and lifecycle-script exposure as a dependency-confusion and supply-chain surface — distinct from (and complementary to) the pnpm-catalog/npm-`overrides` drift-consolidation concerns in `dependency-lockfile-governance.md`. This reference is static-review-only: read and grep `.npmrc`, `package.json`, `package-lock.json`, and CI workflow files; never run `npm install`, `npm ci`, or any registry-touching command.

> Version note: all claims below are grounded in npm CLI docs (`scope.md`, `npmrc.md`, `npm-ci.md`, `npm-approve-scripts.md`) via Context7 on the current stable npm CLI (v10+) documentation set. Label any claim not directly confirmed there as `documentation-based (unverified this session)` rather than asserting it as current default behavior.

## What people get wrong

The naive story is:

> "If it's in `package.json` and installs cleanly, the dependency is trusted."

Wrong. A scoped internal package name (`@myco/internal-lib`) is only routed to a private registry if `.npmrc` (or CI's registry config) explicitly maps that scope to it. Absent that mapping, npm's default resolution falls back to the public registry — and an attacker who publishes a same-named (or higher-versioned) public package under that scope can have it installed instead of the intended internal package. This is **dependency confusion**, and it is a configuration-file finding, not an install-time anomaly you'd only catch by running the install.

## Officially grounded facts

From npm docs (`scope.md`, `npmrc.md`, `npm-ci.md`, `npm-approve-scripts.md`):

1. **Scope-to-registry mapping is the dependency-confusion prevention gate.** `@scope:registry=https://private-registry.example.com` in `.npmrc` (or set via `npm config set @myco:registry=...` / `npm login --registry=... --scope=@myco`) routes every package under that scope to the private registry. A scoped package referenced in `package.json` with no matching `@scope:registry` entry anywhere in the effective `.npmrc` chain (project, user, CI env) resolves against the public npm registry by default.
2. **`npm ci` requires and enforces a committed `package-lock.json`; `npm install` does not.** `npm ci` is documented as designed for automated/CI environments: it requires an existing lockfile, errors if the lockfile's dependencies don't match `package.json`, and never modifies `package.json`/`package-lock.json` — "ensuring a consistent and frozen state." `npm install` can silently re-resolve and rewrite the lockfile on drift. A CI workflow that runs `npm install` instead of `npm ci` gives up frozen-install enforcement even if a lockfile is committed.
3. **Dependency lifecycle scripts (`preinstall`/`install`/`postinstall`/`prepare`) are blocked by default and gated by the `allowScripts` field.** `npm approve-scripts` manages an `allowScripts` field in the project's own `package.json`; by default, dependency install scripts are blocked and npm silently skips them for packages not listed in `allowScripts`. A newly added or bumped dependency that ships a `postinstall` script is only a live risk if it is (a) already present in `allowScripts`, or (b) the project has disabled this gate (e.g., an older npm major without the block, or `--ignore-scripts` not enforced the other way). Confirm which npm major is in use before asserting the default-blocked behavior applies — treat as `documentation-based (unverified this session)` if the installed version can't be confirmed from repo evidence.
4. **Registry-auth credentials must be scoped to a registry host/path, never set bare.** npm docs explicitly contrast a "bad config" (`_authToken=MYTOKEN`, unscoped, applies globally) against "good config" scoped forms: `//registry.example.com/:_authToken=MYTOKEN` (applies to that host) or the narrower `//registry.example.com/myorg/:_authToken=MYTOKEN1` (applies only under that org path). The settings `_auth`, `_authToken`, `username`, `_password`, `email`, `cafile`, `certfile`, and `keyfile` "must all be scoped to a specific registry" — an unscoped `_authToken` line in `.npmrc` is itself a finding, independent of whether the token value is a live secret.
5. **A lockfile with no committed `package-lock.json` at all is a stronger finding than a stale one.** If the repo has no `package-lock.json` (or it is `.gitignore`d), there is nothing for `npm ci` to enforce against — every install re-resolves ranges live, and the install-time set of resolved versions is unverifiable from the repo alone. Check `.gitignore` for a `package-lock.json` entry as part of this review, not just its presence in the working tree.

## Non-negotiable design rules

### 1. Every referenced scope needs a traceable registry mapping

For each `@scope/name` package referenced anywhere in `package.json` (dependencies, devDependencies, peerDependencies) that is not a well-known public-scope package (e.g., `@types/*` is normally fine unmapped), confirm the scope has a corresponding `@scope:registry=` entry in a `.npmrc` file in scope (project-level `.npmrc` committed to the repo, or documented CI environment/secrets config). If you cannot find the mapping in the repo and the user has not pointed you at a CI-side equivalent, report it as an open finding — do not assume it exists in CI secrets you cannot see.

### 2. Lockfile presence, commitment, and CI enforcement are three separate checks

Do not collapse "there's a `package-lock.json`" into "lockfile governance is fine." Check, separately: (a) does `package-lock.json` exist in the working tree, (b) is it tracked by git (not `.gitignore`d, and appears in `git log`/`git ls-files` history rather than only untracked), and (c) does the CI workflow invoke `npm ci` (or an equivalent frozen-install command) rather than `npm install`. All three must hold for the lockfile to function as a supply-chain control.

### 3. New/bumped dependencies are checked for lifecycle scripts before merge, not after

When a diff adds or bumps a dependency, check that dependency's `package.json` (in `node_modules` if present, or its published manifest) for `preinstall`/`install`/`postinstall`/`prepare`. If present, quote the script content verbatim (same rule as in `dependency-lockfile-governance.md`) and check whether it is already listed in the root `package.json`'s `allowScripts`. A script silently added to `allowScripts` in the same diff that adds the dependency is worth flagging on its own — it means a reviewer approved script execution for a package they may not have separately vetted.

### 4. Unscoped credential lines in `.npmrc` are a blocking finding regardless of the token's live validity

Do not wait to determine whether a token found in `.npmrc` is still active before flagging it. An unscoped `_authToken=...` (or `_auth=`, `_password=`, etc.) line is a structural misconfiguration per npm's own documented guidance, independent of whether the credential is currently valid, because it would apply to *every* registry request npm makes, including ones to the public registry.

### 5. Version-range looseness on sensitive dependencies is a supply-chain question, not a churn question

For dependencies with elevated blast radius (auth, crypto, serialization, CI/build tooling itself), an unpinned wide range (`^1.0.0`, `*`, `latest`) combined with either no lockfile or a non-frozen CI install means the actual installed version is effectively whatever the registry serves at install time. Flag this pairing specifically — a wide range alone, with a committed lockfile and enforced frozen install, is a narrower and less urgent finding than the same range with no enforcement.

## Minimal safe review flow

1. List every `@scope/name` dependency in `package.json`; for each non-public-convention scope, search `.npmrc` (project, and any repo-documented CI config) for a matching `@scope:registry=` entry. Flag any scope with no mapping found in repo evidence.
2. Confirm `package-lock.json` exists, is tracked by git, and is not listed in `.gitignore`.
3. Search CI workflow files for the install command; flag `npm install` where `npm ci` is expected, and flag any workflow with no explicit install-frozen step at all.
4. For new/bumped dependencies in the diff, check for lifecycle scripts and cross-reference the root `package.json`'s `allowScripts` array; quote any script found verbatim.
5. Grep `.npmrc` for `_authToken`, `_auth`, `_password`, `_password`, `email`, `cafile`, `certfile`, `keyfile` keys; flag any instance not prefixed with a `//host/[path]:` scope fragment.
6. Report findings with exact file:line citations; propose the `.npmrc` scope entry / CI command / `allowScripts` diff without applying it.

## Greppable sinks (static patterns, no install required)

1. **Missing scope-to-registry mapping for an internal scoped package**
   - Dangerous pattern: a scoped package name appears in `package.json` (e.g., `grep -E '"@[a-z0-9-]+/[a-z0-9.-]+"' package.json` matching a non-`@types` scope) but `.npmrc` has zero `@scope:registry=` lines for that scope (`grep '@<scope>:registry' .npmrc` returns nothing).
   - Safe pattern: every non-public scope referenced in `package.json` has a matching `@scope:registry=` line in a committed `.npmrc`, or the user has confirmed an equivalent CI-side registry config exists.

2. **Unscoped auth token or credential in `.npmrc`**
   - Dangerous pattern: a line at the start of `.npmrc` sets `_authToken`, `_auth`, or `_password` directly (no `//host/path:` prefix) — greppable as an unprefixed `_authToken`, `_auth`, or `_password` key assignment.
   - Safe pattern: credentials appear only as `//<registry-host>/[optional-path]:_authToken=...` (host- or path-scoped), matching npm's documented good-config form.

3. **CI install step using `npm install` instead of `npm ci` with a committed lockfile present**
   - Dangerous pattern: `package-lock.json` is tracked in git, but the CI workflow contains `npm install` (not `npm ci`) ahead of build/test steps — searchable via `grep -n 'npm install' .github/workflows/*.yml` (or the equivalent CI config path) with no adjacent `npm ci`.
   - Safe pattern: CI workflow's dependency-install step is `npm ci`, and a committed `package-lock.json` exists for it to enforce against.

4. **New dependency lifecycle script with no corresponding `allowScripts` review**
   - Dangerous pattern: diff adds a dependency whose `package.json` contains `"postinstall"` (or `preinstall`/`prepare`), and the root `package.json`'s `allowScripts` array either doesn't exist or doesn't list that package — meaning the reviewer has not made an explicit allow/deny decision visible in the diff.
   - Safe pattern: the new dependency's lifecycle script is either absent, or present and accompanied by an explicit, reviewed `allowScripts` entry (or a documented `--ignore-scripts` policy) in the same diff.

## Adversarial checklist

Before closing out a supply-chain review as clean, answer these:

- For every scoped package referenced in the diff, did I find an explicit `@scope:registry=` mapping in repo evidence, or am I assuming one exists in CI secrets I cannot see?
- Is `package-lock.json` both present *and* tracked by git *and* enforced by `npm ci` in CI — not just one or two of the three?
- Did I search `.npmrc` for every credential-shaped key (`_auth`, `_authToken`, `_password`, `email`, `cafile`, `certfile`, `keyfile`), not just `_authToken`?
- For any new/bumped dependency, did I check its actual `package.json` for lifecycle scripts rather than assuming a well-known package name is safe?
- Did I confirm which npm major version is in play before asserting the default-blocked lifecycle-script behavior applies, or did I label it `documentation-based (unverified this session)` because the version isn't visible from repo evidence?

If you cannot answer these from repo evidence, say so rather than declaring the supply-chain surface clean.

## When to push back

Push back if the user asks to:

- treat a missing `@scope:registry` mapping as low-priority because "we've never had an incident" — dependency confusion is a pre-registration attack against a name that doesn't yet need to exist maliciously; absence of a past incident is not evidence of absence of exposure,
- accept `npm install` in CI because "the lockfile is there anyway" — a committed lockfile with no frozen-install enforcement provides no guarantee about what CI actually installed,
- allowlist a dependency's lifecycle script "for now" without reading its content, to unblock a merge — this converts a reviewable static finding into an unreviewed trust decision,
- treat an unscoped `_authToken` in `.npmrc` as fine because "it's a read-only token" — the structural risk is which hosts receive the credential, independent of the token's granted permissions.

Those are not shortcuts. They trade a diagnosable, static, pre-install finding for an assumption that only fails once a malicious package is actually published or a credential actually leaks to the wrong host.
