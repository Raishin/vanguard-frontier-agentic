# Compliance Evidence Chain Guide

Use this reference for AWS compliance evidence mapping across Security Hub controls, Config rules/conformance packs, Audit Manager, AWS Artifact, CloudTrail, manual evidence, exceptions, owners, and control narratives.

## What people get wrong

The lazy story is:

> Export dashboard findings and call it audit evidence.

Wrong. Audit evidence needs scope, time period, source, control mapping, ownership, completeness, exception handling, and reproducibility. A dashboard screenshot is usually weak evidence.

Common bad assumptions:

- Security Hub passed controls prove compliance.
- AWS Config conformance packs cover the full control objective.
- AWS Artifact reports prove customer workload compliance.
- Manual evidence is acceptable without owner and timestamp.
- Audit Manager evidence is complete without checking source coverage.
- Remediated finding means control operated effectively for the whole period.

## Compliance-evidence failure modes

- Evidence period does not match audit period.
- Account/Region/resource scope excludes material workloads.
- Control mapping confuses AWS responsibility with customer responsibility.
- Exceptions lack risk acceptance, expiry, owner, and compensating controls.
- Evidence cannot be reproduced from source systems.
- Sensitive evidence leaks account IDs, customer data, secrets, or vulnerability details unnecessarily.

## Minimum safe workflow

1. Identify framework, control, audit period, in-scope accounts/Regions/workloads, and evidence owner.
2. Map technical sources to control assertions: Config, Security Hub, CloudTrail, AWS Backup, IAM, logs, tickets, and manual evidence.
3. Label evidence quality: direct, indirect, sampled, stale, manual, missing, or inference.
4. Check completeness by scope and time period, not just current pass/fail status.
5. Separate AWS Artifact/provider evidence from customer workload evidence.
6. Produce evidence package with source, timestamp, owner, control mapping, gaps, exceptions, and safe redaction.
7. Avoid remediation recommendations unless routed to the relevant technical hardening skill.

## Verification targets

- AWS Config rules/conformance packs, aggregators, compliance history, and resource scope
- Security Hub controls/findings, workflow status, standards, suppression rules, and account/Region coverage
- Audit Manager assessment/evidence folders where available, noting current service availability/maintenance constraints
- AWS Artifact reports mapped to shared-responsibility claims only
- CloudTrail organization trail, log integrity, S3/KMS protection, and evidence retention
- tickets, approvals, exception register, risk acceptance, remediation proof, and control-owner signoff

## When to push back

Push back if the user asks to:

- treat a screenshot as complete evidence
- use provider reports as proof of customer controls
- hide exceptions, stale data, or missing accounts
- include secrets/customer data in evidence packs
- claim compliance from current-state checks for a historical audit period
- turn evidence mapping into unapproved remediation
