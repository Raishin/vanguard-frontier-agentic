# Workflow and output contract

Use this reference only when executing the full live RDS/PolarDB mutation gate — instance deletion, spec downgrade, or backup policy removal.

## Database mutation guard areas to check

- Instance identity: instance ID, engine type, region, account confirmed
- Backup verification: most recent backup ID, creation time, status (Completed), retention period, restore test history
- Spec downgrade risk: current CPU/memory/IOPS utilization vs. target spec limits; 20% headroom rule
- Blast radius: dependent applications, connection strings, downstream services, maintenance window notification
- Operator authorization: identity confirmed; explicit written confirmation covering all required statements

## Safe workflow

1. **Frame scope** — confirm instance identity and the specific mutation requested
2. **Collect evidence** — verify backup existence and restore readiness; label: `live evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what is unrecoverable? what applications fail?
4. **Gate** — require explicit written confirmation from the operator covering all required confirmation statements
5. **Execute and verify** — execute in the confirmed maintenance window; confirm result; monitor metrics

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Live RDS/PolarDB Mutation Guard: <scope>
## Instance identity confirmed
## Backup verification
## Blast radius assessment
## Confirmation received
## Execution result
## Post-change monitoring
```

Each section must include an evidence level label. Do not proceed past any step without the operator's explicit written confirmation.
