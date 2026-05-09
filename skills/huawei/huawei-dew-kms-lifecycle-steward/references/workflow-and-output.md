# Workflow and output contract

Use this reference only when performing a full DEW key lifecycle review or encryption governance implementation guidance.

## DEW areas to check

- KMS keys: key inventory, rotation schedule, version count, pending deletion flags, consumer services
- CSMS secrets: secret inventory, rotation status, FunctionGraph rotation function health, consumer dependency map
- CBH: session recording enabled/disabled, retention period, access policy scope, MLPS compliance status
- DBSS: encryption coverage per database, SQL audit policy enabled/disabled, audit log retention
- Integration: OBS SSE key binding, ECS disk encryption key, RDS/GaussDB CMK encryption

## Safe workflow

1. **Frame scope** — confirm target DEW services, enterprise project, and non-goals
2. **Collect evidence** — prefer live state; label all evidence types
3. **Stress-test** — encrypted resource enumeration, rotation failure modes, key deletion blast radius
4. **Recommend safest action** — staged key rotation, deletion window review, MLPS gap remediation

## Output contract

Return this structure:

```markdown
# Huawei Cloud DEW Key Lifecycle: <scope>
## Scope and evidence level
## KMS key inventory and rotation status
## CSMS secret rotation posture
## CBH session recording configuration
## DBSS SQL audit status
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
