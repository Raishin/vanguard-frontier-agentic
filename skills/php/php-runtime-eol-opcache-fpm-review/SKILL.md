---
name: php-runtime-eol-opcache-fpm-review
description: Use this skill to review PHP runtime upgrade readiness — whether the target or running PHP version is past php.net's published four-year support window (active support, then security-only, then EOL) — and to review production OPcache (enable, validate_timestamps, memory sizing) and PHP-FPM (pm, max_children, max_requests) hardening. Use when production PHP could be running an EOL or soon-security-only-EOL version, or when OPcache/FPM configuration could serve stale code or let a traffic spike exhaust workers. An EOL runtime is always a blocking finding. Static review only; it never installs, upgrades, or restarts a PHP runtime.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-16"
  category: operational
  lifecycle: experimental
---

# PHP Runtime EOL, OPcache & PHP-FPM Review

## Purpose

Review a PHP service's runtime lifecycle status and production hardening so a service does not run on an unsupported PHP version or an unhardened OPcache/PHP-FPM configuration without the team knowing. The dominant failure modes are a PHP version past php.net's published support window (EOL, or security-only and approaching its own EOL), OPcache left disabled or misconfigured for the deployment model, and PHP-FPM pool settings that leave worker concurrency and recycling unbounded.

## When to use

Use this skill when the user asks to:

- review whether a service's target or running PHP version is EOL, security-only, or actively supported per php.net,
- assess an upgrade path and timeline against php.net's published support-window dates,
- review production `opcache.enable`, `opcache.validate_timestamps`, `opcache.memory_consumption`, and `opcache.max_accelerated_files` configuration,
- review production PHP-FPM `pm`, `pm.max_children`, and `pm.max_requests` configuration for resource-exhaustion and worker-leak exposure.

## When not to use

Do not use this skill for:

- performing the PHP version upgrade, editing application code for compatibility, or installing/restarting any PHP, OPcache, or PHP-FPM process — this skill is static review only; it recommends the path and hands implementation to the owning team.
- reviewing application-level vulnerabilities, Composer dependency supply chain, or WordPress-specific hardening — those belong to other PHP-domain skills in this catalog.
- asserting a PHP version's support status from memory or estimation; if php.net's supported-versions page does not list the version, say so rather than guessing a date.
- judging lifecycle status against the wall clock; judgment is based on the target/committed version and the published dates only, so findings stay reproducible.

## Preconditions

- The PHP version actually targeted or running in production (`composer.json` `require.php`, Dockerfile/base-image tag, CI runtime matrix, or deployment manifest).
- The production `php.ini` OPcache directives in scope: `opcache.enable`, `opcache.validate_timestamps`, `opcache.revalidate_freq`, `opcache.memory_consumption`, `opcache.max_accelerated_files`.
- The production PHP-FPM pool configuration: `pm`, `pm.max_children`, `pm.max_requests`, and the `pm.start_servers`/`pm.min_spare_servers`/`pm.max_spare_servers` triad if `pm` is `dynamic`.
- The deployment model (immutable container image replaced per deploy vs. in-place file sync), since it determines whether `opcache.validate_timestamps=0` is safe without a compensating invalidation step.
- Approximate available memory per worker host/container, if `pm.max_children` sizing guidance is requested.

## Lean operating rules

- Resolve the exact PHP version from the strongest available evidence and state that evidence tier before classifying its lifecycle status.
- Classify status precisely as active support, security-only (with the published end date), or EOL — never collapse these into "outdated."
- Treat any EOL classification as blocking, regardless of code quality elsewhere; EOL means no fixes are published, including for actively exploited vulnerabilities.
- Treat a security-only classification as blocking only when its published security-support end date falls within the stated release horizon with no tracked upgrade plan; otherwise report it as dated advisory guidance.
- Confirm `opcache.enable=1` in production, and check `opcache.validate_timestamps` against the actual deployment model rather than assuming one universally correct value.
- Confirm `pm.max_children` is evidently bounded by available memory and `pm.max_requests` is nonzero (or has a documented rationale for `0`).
- Never fabricate a PHP version, support date, or CVE; encode lifecycle dates only from php.net's supported-versions page.
- Label every claim `repo evidence`, `documentation-based`, or `inference`.

## Context7 documentation protocol

This skill's allowed tools are Read, Grep, and Glob only — it does not call Context7 or any live documentation lookup at review time. Every PHP-lifecycle, OPcache, or PHP-FPM behavioral claim must instead be grounded in the bundled reference files, which are sourced directly from php.net and dated at authoring time. If a claim cannot be traced to a bundled reference or to evidence in the repository, label it an assumption rather than asserting it as documented behavior. See [PHP version lifecycle](references/php-version-lifecycle.md), [OPcache production configuration](references/opcache-production-config.md), and [PHP-FPM pool tuning](references/php-fpm-pool-tuning.md).

## Workflow

Follow the review in this order: (1) resolve the exact PHP version targeted or running, and its evidence tier; (2) classify that version against php.net's current supported-versions table as active-support, security-only, or EOL, citing the exact dates; (3) if security-only, compare its security-support end date against the stated release horizon; (4) read production OPcache directives and check `enable`/`validate_timestamps` against the deployment model, and `memory_consumption`/`max_accelerated_files` against the actual codebase size; (5) read production PHP-FPM pool configuration and check `pm.max_children` sizing and `pm.max_requests` recycling; (6) emit findings with evidence tiers, concrete remediation, and ownership handoffs.

## Decision gates

- Block on any EOL PHP version, unconditionally.
- Block on a security-only version whose published security-support end date falls within the stated release horizon with no tracked upgrade plan.
- Block on production OPcache disabled, or `validate_timestamps` inconsistent with the deployment model with no compensating invalidation step.
- Block on PHP-FPM with no evidently bounded `pm.max_children` or an unbounded `pm.max_requests` with no documented rationale.
- Every lifecycle and directive claim traces to a bundled reference file or explicit repository evidence, never memory.
- The PHP version upgrade and any application-code compatibility work are handed to the owning engineering team, never performed here.

## Evidence classification

Label each finding `repo evidence` (seen directly in `php.ini`, FPM pool config, Dockerfile, CI, or a version banner), `documentation-based` (a php.net-published support date or directive default from the bundled references), or `inference` (a reasonable conclusion not directly observed, e.g. estimated memory footprint per worker). A documented default never proves what a specific deployment's actual configuration is — check the file before asserting it.

## Security and privacy constraints

Static review only. Never install, upgrade, downgrade, or restart any PHP, OPcache, or PHP-FPM process, and never make any network call to php.net or any other service. Never fabricate a PHP version, support date, or CVE — report missing evidence instead of guessing. Treat any credential found in configuration files as a redact-and-flag finding, never echoed in output.

## Escalation conditions

Escalate to incident response on any evidence the failure is already live — an EOL PHP version confirmed serving production traffic, or a PHP-FPM pool observed exhausting workers under load. Escalate a confirmed EOL or approaching-security-only-EOL finding to the owning engineering team as a tracked upgrade item, with the recommended target version and the exact php.net-published dates driving the timeline.

## References

Load these only when needed:

- [PHP version lifecycle](references/php-version-lifecycle.md) — the php.net support-policy definitions and the current per-branch active-support/security-support/EOL dates.
- [OPcache production configuration](references/opcache-production-config.md) — `opcache.enable`, `opcache.validate_timestamps`, and sizing directives, and how to review them against a deployment model.
- [PHP-FPM pool tuning](references/php-fpm-pool-tuning.md) — `pm`, `pm.max_children`, and `pm.max_requests` review criteria for resource-exhaustion and worker-leak exposure.

## Response minimum

Return, at minimum:

- the resolved PHP version, its evidence tier, and its lifecycle classification (active support / security-only with end date / EOL) with the exact php.net-published dates;
- the production OPcache hardening state (`enable`, `validate_timestamps` versus deployment model, sizing);
- the production PHP-FPM hardening state (`pm.max_children` sizing, `pm.max_requests` recycling);
- concrete remediation and an exact verification step for each finding;
- ownership handoffs and any incident-response escalation.

## Anti-goals

- Never fabricate or guess a PHP version's support-window dates; encode them only from php.net's supported-versions page.
- Never judge lifecycle status against the wall clock; use the target/committed version and the published dates only.
- Do not rewrite application code or perform the PHP version upgrade; recommend the path and hand implementation to the owning team.
- Do not execute, restart, or reload any PHP, OPcache, or PHP-FPM process, and do not contact php.net or any other service.
- Do not treat a version absent from the current supported-versions table as EOL by assumption; say the evidence is missing instead.
