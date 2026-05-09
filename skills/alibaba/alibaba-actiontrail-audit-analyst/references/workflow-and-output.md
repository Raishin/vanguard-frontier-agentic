# Workflow and output contract

Use this reference only when performing a full ActionTrail audit review, compliance evidence generation, or incident triage.

## ActionTrail audit areas to check

- Trail configuration: enabled status, event categories (management vs. data events), delivery to OSS and SLS
- SLS logstore integration: logstore TTL vs. 180-day MLPS 2.0 requirement, index configuration, alert rules
- Multi-account coverage: Resource Directory management account trail vs. per-account trails
- Anomaly detection: alert rules for off-hours admin access, privilege escalation, mass deletions, unusual source IPs
- Compliance coverage: MLPS 2.0 Level 3, internal policy gaps, evidence packaging for auditors
- Recent admin activity review: high-risk API calls (RAM policy changes, deletion events, cross-border transfers)

## Safe workflow

1. **Frame scope** — confirm target account/organization, compliance driver, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what is missing? what compliance gap exists?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud ActionTrail Audit: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
