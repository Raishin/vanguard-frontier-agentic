# Abandoned and Advisory Governance

## Why this matters

An abandoned Packagist package is not necessarily vulnerable today, but it will not receive a fix if a vulnerability is found tomorrow, and it will not receive a fix now for anything already discovered. Composer can be configured to surface this risk (or block it outright), but the defaults have changed across versions, and a project pinned to an older Composer release may be relying on weaker default behavior than its authors expect. This reference exists so a reviewer checks the actual configured (or defaulted) behavior rather than assuming abandonment is handled.

## NORMATIVE facts

- `config.policy.abandoned.audit` controls whether and how `composer audit` reports abandoned packages. Allowed values are `ignore`, `report`, and `fail`, matching the same semantics as the advisories policy: `ignore` does not report, `report` reports without a non-zero exit code, and `fail` causes a non-zero exit code. This setting applies only to audit reporting, not to whether abandoned packages can be installed or updated.
- The default for `config.policy.abandoned.audit` became `fail` in Composer 2.7; it defaulted to `report` in Composer 2.6. A project's Composer version constraint therefore determines which default behavior actually applies unless the key is explicitly set.
- `config.policy.abandoned.block` is a separate key that, when enabled, prevents abandoned packages from being installed during `update`, `require`, or `remove` operations. Its default value is `false` — meaning by default, Composer does not stop an abandoned package from being installed or updated, it can only be flagged during an audit.

## Reviewer evidence criteria

- Read `config.policy.abandoned.audit` and `config.policy.abandoned.block` directly from `composer.json`. Do not assume a default without first confirming the key is genuinely unset.
- If `config.policy.abandoned.audit` is unset, determine the project's Composer version constraint (from `composer.json` `require.composer` or lockfile metadata, if present) to establish which default (`report` for 2.6, `fail` for 2.7+) actually applies, and flag the ambiguity if the version constraint is broad enough to span both.
- Treat `config.policy.abandoned.block` left at its default `false` as expected baseline behavior, not itself a finding — but combine it with the next check: search for evidence of abandoned packages already present (audit output, lockfile package names cross-referenced against known-abandoned status if such evidence is provided) and confirm each has a tracked replacement plan (a migration ticket, a comment, or documented accepted-risk exception). An abandoned package with no replacement plan and no compensating audit-gating configuration is a blocking finding.
- Never fabricate a package's abandoned status. Only report a package as abandoned when the repository provides direct evidence (an audit run's output, a Composer warning captured in a log, or an explicit note in project documentation) — a reviewer without that evidence should report the gap ("abandonment status could not be confirmed from available evidence") rather than guessing.
- When a replacement plan does exist, verify it names a concrete path (a specific replacement package, a scheduled removal, or an accepted-risk sign-off with an expiry or review date) rather than an open-ended "we know about it."

## Context: general OSS risk (report figure only)

Black Duck's Open Source Security and Risk Analysis (OSSRA) report, published February 25, 2025, found that 86% of commercial codebases evaluated contained open source software vulnerabilities, and 90% of audited codebases had open source components more than four years out of date. This is population-level report context motivating why abandoned and unpatched dependencies deserve deliberate governance — it is never evidence about the specific repository under review, and must not be cited as a finding about this codebase.

## Sources

- `config.policy.abandoned.audit` and `config.policy.abandoned.block` keys, allowed values, and version-dependent defaults: [Composer config documentation](https://getcomposer.org/doc/06-config.md).
- General OSS-risk context (86% vulnerable codebases, 90% components 4+ years out of date), cited as report-figure motivation only: [Black Duck 2025 OSSRA report announcement](https://news.blackduck.com/2025-02-25-New-Black-Duck-Report-86-of-Commercial-Codebases-Contain-Vulnerable-Open-Source,-Exposing-Organizations-to-Security-Risks).

Last verified: 2026-07-16
