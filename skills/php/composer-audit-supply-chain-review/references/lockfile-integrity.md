# Lockfile Integrity

## Why this matters

`composer.lock` is what turns a dependency tree from "whatever the constraints in `composer.json` happen to resolve to today" into a reproducible, reviewable set of exact versions. Without it, or with a drifted or bypassed one, the versions actually running in production are not the versions anyone reviewed — which is precisely the gap OWASP's Vulnerable and Outdated Components category (A06) describes: components are used with unknown or unmanaged versions, making it impossible to know what is actually deployed.

## NORMATIVE facts

- `composer update` resolves dependencies against `composer.json` and writes the exact resolved package versions to `composer.lock`.
- `composer install`, when a lock file is present, uses the exact versions recorded in `composer.lock` rather than re-resolving from `composer.json` — this is what makes installs consistent across a CI server, production machines, and every developer's environment.
- Composer displays a warning when running `install` if `composer.lock` has not been updated since changes were made to `composer.json` that could affect dependency resolution — i.e., the lock file and the manifest have drifted apart.
- For libraries, committing the lock file is documented as not necessary; this exception does not apply to applications, where the lock file is what makes deployment reproducible.

## Reviewer evidence criteria

- Confirm `composer.lock` exists in the repository for any project that is deployed as an application (not merely a library consumed by others). Its absence is a blocking finding on its own: without it, `composer install` re-resolves against `composer.json` constraints every time, so the exact versions running anywhere are not fixed, recorded, or reviewable.
- If `composer.lock` exists, check for evidence of drift from `composer.json` — a captured CI log showing the "lock file is not up to date" warning, a lock content-hash field that does not match the current `composer.json`, or `composer.json` constraints whose ranges no longer contain the versions actually locked. Report drift as a distinct finding from lock absence; a stale lock file is not the same failure as no lock file, and the remediation differs (run `composer update` for the affected packages and commit the refreshed lock, rather than generating a lock file from scratch).
- With or without an up-to-date lock file, separately review `composer.json` version constraints for packages at security-sensitive boundaries (authentication, cryptography, session handling, serialization/deserialization, payment processing, or any package with a documented prior advisory). Flag unpinned or unusually wide constraints (`*`, unconstrained `dev-` branch references, or a `~`/`^` floor wide enough to span multiple major or minor lines) — a lock file freezes today's resolution, but an unattended future `composer update` against a wide constraint can still move far without anyone reviewing the jump.
- When recommending remediation, name the exact Composer command that would resolve the finding (e.g. `composer update <package>` to refresh a stale lock entry, or tightening a constraint in `composer.json`) for the owning engineer to run — this skill never executes it.
- Treat this as an OWASP A06 (Vulnerable and Outdated Components) instance: the underlying risk is that unmanaged or unknown dependency versions make it impossible to assess exposure, independent of whether any specific advisory currently applies.

## Sources

- `composer.lock` purpose, `composer install` vs `composer update` behavior, the stale-lock warning, and the library exception: [Composer basic usage documentation](https://getcomposer.org/doc/01-basic-usage.md).
- Vulnerable and Outdated Components as a named risk category: [OWASP Top Ten](https://owasp.org/www-project-top-ten/).

Last verified: 2026-07-16
