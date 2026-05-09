# Workflow and Output Contract

Use this reference when performing the full WAF Security Pillar review or formatting the final assessment.

## Review domains

Work through these six security design principles in order:

1. **Strong identity foundation** — IAM credential types (root, IAM users, roles, federated), least-privilege policies, Permission Boundaries, SCPs, IAM Access Analyzer findings
2. **Traceability** — CloudTrail (all regions, log file validation, S3 MFA delete), AWS Config recording, Security Hub standards enabled, GuardDuty active
3. **Security at all layers** — VPC security groups (no 0.0.0.0/0 management ports), WAF rules, Shield Advanced, Network Firewall, EC2/container security
4. **Automation of security best practices** — IaC policy checks (cfn_nag, Checkov, tfsec), Binary Authorization/ECR scanning, Config Rules for detective controls
5. **Data protection** — KMS CMKs vs AWS-managed keys for regulated data, S3 encryption and Block Public Access, Secrets Manager, Macie PII discovery
6. **Incident response readiness** — IR playbooks, GuardDuty automated response, Security Hub Insights, Systems Manager Incident Manager runbooks

## Safe workflow

1. **Frame scope**: account IDs (sanitized), Organization structure, Regions in use, compliance drivers (PCI/HIPAA/SOC2)
2. **Gather evidence**: Security Hub score and standards, GuardDuty findings, CloudTrail status, IAM Access Analyzer findings
3. **Prioritize findings**: by exploitability × blast radius × data sensitivity
4. **Draft recommendations**: each with severity, rollback path, and validation command
5. **Confirm before acting**: require explicit approval for any IAM, network, KMS, or compliance-impacting change

## Response shape

1. Scope and multi-account structure
2. Security Hub / GuardDuty / CloudTrail / Config coverage
3. IAM and identity posture
4. Network and infrastructure protection
5. Data protection and encryption
6. Incident response readiness
7. Prioritized findings (Critical → High → Medium)
8. Open risks and blockers
