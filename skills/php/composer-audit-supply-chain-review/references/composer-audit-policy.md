# Composer Audit Policy

## Why this matters

`composer audit` is the built-in mechanism for catching known-vulnerable, abandoned, or malware-flagged dependencies before they ship. A repository can have this command available and still ship a vulnerable dependency to production if the command is never invoked in CI, or if it runs but its result is not allowed to fail the build. The review criteria below exist to catch exactly that gap — a policy that exists on paper but is not actually enforced.

## NORMATIVE facts

- `composer audit` checks installed packages against known security advisories, abandoned status, and malware flags.
- Exit code `0` means no issues were found.
- Exit code `1` means the command found packages matching the configured dependency policy (advisories and/or abandoned status, depending on `config.policy` settings) or failed due to missing required packages.
- `config.policy.advisories.audit` controls how `composer audit` treats packages with security advisories. Its allowed values are `ignore` (advisories not reported), `report` (advisories reported but do not cause a non-zero exit code), and `fail` (advisories cause a non-zero exit code). The documented default is `fail`.
- The `audit` command supports flags including `--no-dev` (skip `require-dev` packages), `--format` (table, plain, json, or summary), `--locked` (audit from the lock file instead of the installed `vendor` directory), and `--ignore-severity` (filter out advisories below a given severity).

## Reviewer evidence criteria

- Locate every CI job that builds, tests, or deploys the project. For each one, search for an invocation of `composer audit` (or `composer audit --locked`, or an equivalent wrapper script).
- If no such invocation exists in any relevant pipeline, this is a blocking finding: no CI gate exists at all.
- If an invocation exists, check whether its exit code is allowed to fail the job. Look for patterns that suppress failure: a trailing `|| true`, a shell `set +e` around the call, a CI step marked `continue-on-error: true` (or equivalent), or output redirected to a report file with no subsequent check of the exit code. Any of these means the audit runs but does not gate — treat it as equivalent to no gate, and cite the specific suppression pattern found.
- Check `composer.json` for an explicit `config.policy.advisories.audit` value. If set to `ignore` or `report`, this weakens or removes the gate regardless of what CI does with the exit code — flag it, and look for an accompanying documented accepted-risk rationale before treating it as intentional.
- If `config.policy.advisories.audit` is unset, the documented default (`fail`) applies — verify this is consistent with the project's actual Composer version constraint before relying on it (see abandoned-and-advisory-governance.md for the version-sensitivity of related defaults).
- If `--locked` is not used and the pipeline audits only the installed `vendor` directory, note that a `vendor` directory that is regenerated fresh in CI from `composer.lock` should produce equivalent results, but flag if the pipeline's install step and audit step could diverge (e.g. cached `vendor` directories, conditional installs).
- Never assert a specific advisory ID or CVE number exists for a package unless it is directly present in retrieved audit output, a lockfile annotation, or another concrete artifact in the repository.

## RECOMMENDATION

- Run `composer audit --locked` in CI immediately after dependency installation, as an explicit step whose failure is not caught or suppressed by later pipeline logic.
- Keep `config.policy.advisories.audit` at its documented default (`fail`) unless there is a written, time-bound, accepted-risk exception for a specific advisory.

## Sources

- `composer audit` behavior, exit codes, and CLI flags: [Composer CLI documentation](https://getcomposer.org/doc/03-cli.md).
- `config.policy.advisories.audit` key, allowed values, and default: [Composer config documentation](https://getcomposer.org/doc/06-config.md).

Last verified: 2026-07-16
