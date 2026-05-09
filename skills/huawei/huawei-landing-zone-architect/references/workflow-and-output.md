# Workflow and output contract

Use this reference only when performing a full landing zone design review or governance implementation guidance.

## Landing zone areas to check

- Organizations: master account designation, member account inventory, organizational unit structure, SCP attachment map
- SCP baseline: active SCPs, deny guardrail coverage (dangerous services blocked, data exfiltration paths blocked), allowlist scope
- IAM baseline: MFA enforcement rate, password policy settings, access key age distribution, privileged user count
- Enterprise projects: project inventory, permission boundary per project, cost attribution tag coverage, project deletion risk
- Account isolation: workload accounts vs management accounts, blast radius isolation, cross-account access paths (agencies)
- Logging and audit: CTS enabled per account, log storage region, retention period

## Safe workflow

1. **Frame scope** — confirm master account, member accounts, compliance requirements, and non-goals
2. **Collect evidence** — prefer live Organizations and IAM export; label all evidence types
3. **Stress-test** — SCP blast radius on member accounts, MFA bypass risk, cross-project privilege leakage
4. **Recommend safest action** — staged SCP rollout (simulate → test member account → production), IAM baseline remediation

## Output contract

Return this structure:

```markdown
# Huawei Cloud Landing Zone: <scope>
## Scope and evidence level
## Organizations structure and SCP inventory
## IAM baseline compliance
## Enterprise project governance model
## Account vs enterprise project isolation analysis
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
