# Workflow and output contract

Use this reference only when performing a full ECS compute review or lifecycle implementation guidance.

## Compute areas to check

- ECS instances: instance inventory, flavor, AZ placement, EIP binding, security group rules
- AS groups: scaling policy type (alarm/scheduled/periodic), min/max/desired capacity, cool-down period, scale-in protection
- IMS: private image inventory, patch baseline status, image sharing policy, version lifecycle
- DeH: host inventory, ECS placement, BYOL compliance status, migration history
- CSBS: backup policy coverage, backup frequency, retention period, last successful backup, recovery test history
- Resize paths: online resize eligibility, offline resize procedure, data disk impact

## Safe workflow

1. **Frame scope** — confirm target instances, enterprise project, and non-goals
2. **Collect evidence** — prefer live instance and backup status; label all evidence types
3. **Stress-test** — backup gaps, AS scale-in stateful risk, DeH compliance exposure, IMS patch gaps
4. **Recommend safest action** — staged changes with CSBS backup verification before each destructive operation

## Output contract

Return this structure:

```markdown
# Huawei Cloud ECS Compute: <scope>
## Scope and evidence level
## ECS instance inventory
## AS scaling policy and health
## IMS golden image posture
## CSBS backup coverage
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
