# Workflow and output contract

Use this reference only when performing a full compliance and sovereignty review or implementation guidance.

## Compliance areas to check

- MLPS 2.0 Level 3 controls: login audit (LTS), boundary protection (CFW/WAF), intrusion detection (HSS/SecMaster), data backup (CBR), MFA enforcement (IAM)
- Data localization: workload region mapping, replication targets, backup destinations for CN-* data
- Trusted Cloud (CAICT): certification scope, applicable services, attestation currency
- Government cloud: dedicated tenancy, region-specific configuration, approval requirements
- Incident reporting obligations: data destruction reporting within 24 hours under MLPS Level 3
- Cross-border data movement: legal basis assessment, data transfer agreements

## Safe workflow

1. **Frame scope** — confirm workload classification, MLPS level, region, and non-goals
2. **Collect evidence** — prefer live state; label all evidence types
3. **Stress-test** — regulatory gaps, missing controls, cross-border movement risk
4. **Recommend safest action** — prioritized remediation with compliance deadlines

## Output contract

Return this structure:

```markdown
# Huawei Cloud Compliance and Sovereignty: <scope>
## Scope and evidence level
## MLPS 2.0 Level 3 control coverage
## Data localization posture
## Trusted Cloud (CAICT) applicability
## Identified gaps and regulatory risk
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
