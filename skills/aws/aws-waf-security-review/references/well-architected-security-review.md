# Well-Architected Security Review Guide

Use this reference for AWS Well-Architected Framework Security Pillar reviews. In this repository, `aws-waf-security-review` means Well-Architected Framework review, not AWS Web Application Firewall configuration.

## What people get wrong

The lazy story is:

> Security review is checking IAM, encryption, and public access.

Wrong. Security posture is an end-to-end control system: identity, detection, infrastructure protection, data protection, vulnerability management, and incident response must all have evidence.

Common bad assumptions:

- No Security Hub finding means secure.
- Encryption enabled means data protection is complete.
- IAM managed policies are least privilege.
- CloudTrail exists, so detective controls are covered.
- Network private means workload secure.
- Incident response is ready because a runbook exists.

## Security-specific failure modes

- Multi-account boundaries, SCPs, permission boundaries, and break-glass controls are undefined.
- Static credentials, long-lived access keys, or broad cross-account trust remain active.
- Logging/detection coverage is incomplete across accounts, Regions, services, or data planes.
- KMS key policies, bucket policies, secrets, and data residency controls are unreviewed.
- Public exposure hides behind CDN/API/LB layers or misconfigured security groups.
- Findings have no owner, SLA, exception record, or verification after remediation.

## Minimum safe workflow

1. Confirm workload scope, accounts, Regions, data classification, regulatory context, and threat model.
2. Review identity first: human access, workload roles, federation, privilege boundaries, and break-glass.
3. Check detective controls: CloudTrail, Config, GuardDuty, Security Hub, IAM Access Analyzer, log retention, and alert routing.
4. Review infrastructure and data protection: network exposure, KMS, secrets, storage policies, backups, and vulnerability posture.
5. Check incident readiness: runbooks, contacts, containment paths, forensics logging, and game-day evidence.
6. Prioritize findings by exploitability, blast radius, business impact, and remediation safety.
7. Keep recommendations non-mutating unless a separate guarded operator workflow is invoked.

## Verification targets

- Organization/account structure, SCPs, permission boundaries, identity center/federation, MFA, and break-glass controls
- IAM policies, trust policies, access analyzer findings, unused access, and credential age
- CloudTrail, Config, GuardDuty, Security Hub, VPC Flow Logs, log destinations, retention, and alert paths
- public exposure evidence across ALB/API Gateway/CloudFront/S3/security groups/routes
- KMS key policies, secret rotation, S3/data-store encryption, backups, and data classification
- incident response runbooks, contacts, tabletop/game-day evidence, and exception register

## When to push back

Push back if the user asks to:

- claim security readiness from questionnaire answers alone
- ignore unsupported accounts, Regions, or log gaps
- widen IAM or network access as a shortcut
- hide critical findings behind generic “best practice” language
- treat encryption as proof of authorization or data minimization
- confuse this Well-Architected Framework review with AWS Web Application Firewall rules
