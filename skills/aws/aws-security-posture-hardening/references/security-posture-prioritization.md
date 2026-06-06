# Security Posture Prioritization Guide

Use this reference when prioritizing AWS Security Hub CSPM, GuardDuty, Inspector, Macie, Config, CloudTrail, IAM, public exposure, vulnerability, and compliance posture findings.

## What people get wrong

The lazy story is:

> Sort Security Hub findings by severity and fix the highs first.

Wrong. Finding severity is an input, not a priority model. Real risk depends on exploitability, exposure, asset criticality, compensating controls, account scope, and remediation blast radius.

Common bad assumptions:

- Security Hub enabled means all relevant controls are covered.
- GuardDuty finding absence means no threat activity.
- Inspector vulnerability severity equals business priority.
- Public exposure findings are all equally urgent.
- CloudTrail configured in one account/Region proves audit coverage.
- Auto-remediation is safe for posture hardening.

## Security-posture failure modes

- Delegated administrator and organization coverage miss member accounts or Regions.
- Security Hub standards are enabled but findings are suppressed, stale, or unowned.
- GuardDuty, Inspector, Macie, Config, or CloudTrail are partially deployed or not routed to responders.
- IAM findings are remediated with broad deny/allow changes that break workloads.
- Public S3, security group, ALB, API Gateway, or CloudFront exposure lacks data classification context.
- Remediation deletes evidence, disables logging, rotates secrets without consumers, or changes KMS policies unsafely.

## Minimum safe workflow

1. Confirm organization/account/Region scope and which services are actually enabled.
2. Normalize findings by resource, owner, business criticality, exposure path, data sensitivity, and active threat evidence.
3. Separate detection gaps from resource misconfigurations and vulnerability remediation.
4. Prioritize by risk: internet exposure plus sensitive data, identity escalation, logging gaps, exploitable vulnerabilities, and active GuardDuty signals.
5. Recommend least-privilege, reversible remediation with validation and rollback notes.
6. Track exceptions, suppressions, compensating controls, and due dates explicitly.
7. Do not execute auto-remediation from this review skill.

## Verification targets

- Security Hub/CSPM standards, finding workflow status, suppression rules, and delegated admin/account coverage
- GuardDuty detectors, findings, malware protection, trusted/threat lists, and alert routing
- Inspector coverage for EC2/ECR/Lambda and vulnerable package/resource context
- Macie jobs/classification findings for sensitive data exposure
- Config recorders, conformance packs, aggregators, and CloudTrail organization trails/log integrity
- IAM Access Analyzer, public access findings, security group exposure, S3 Block Public Access, KMS key policy, and secret rotation evidence

## When to push back

Push back if the user asks to:

- treat compliance score as actual security risk
- suppress findings without owner, reason, expiry, and compensating control
- run broad auto-remediation on production resources
- widen IAM/network access to clear a finding
- claim organization coverage from a single account sample
- remove logs/evidence while remediating
