# Workflow and output contract

Use this reference only when performing a full security hardening review, incident triage, or MLPS 2.0 compliance gap analysis.

## Security hardening areas to check

- Security Center: agent deployment coverage, tier (Basic/Advanced/Enterprise), active vulnerability alerts (CVE severity), baseline check results, ransomware protection status
- WAF: rule group coverage (OWASP Top 10, bot, custom), IP whitelist/blacklist entries and justification, CC attack defense thresholds, HTTPS certificate binding
- Anti-DDoS Pro: protection tier vs. expected attack surface, protected IPs, mitigation thresholds
- Cloud Firewall (north-south): internet-facing ECS/SLB policy review, open port inventory, allow/deny rules
- Cloud Firewall (east-west): VPC-to-VPC rules, intra-VPC lateral movement controls
- NTA: VPC flow log collection to SLS, anomaly detection rule coverage
- MLPS 2.0 Level 3 controls: boundary protection (Cloud Firewall + WAF), intrusion detection (Security Center HSS), audit log (ActionTrail + SLS 180-day retention)

## Safe workflow

1. **Frame scope** — confirm target account/workload, compliance driver, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live Security Center scan results and Cloud Firewall policy exports; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is unpatched? what traffic is uncontrolled? what MLPS controls are missing?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path; test Cloud Firewall rules in non-production first

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Security Hardening: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
