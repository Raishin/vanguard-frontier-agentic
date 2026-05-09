# Workflow and output contract

Use this reference only when designing a full migration plan, cutover sequence, or rollback strategy.

## Migration areas to check

- SMC server migration: agent installation, replication status, incremental sync lag, ECS target spec selection
- DTS data migration: source/target engine compatibility, full migration status, incremental sync lag, replication user privileges
- OSSImport: source storage type, transfer progress, checksum verification, incremental sync configuration
- Cutover sequencing: phase order (data first, DNS/LB last), go/no-go criteria, maintenance window
- Rollback paths: per-phase rollback procedures, rollback time estimates, backup verification status
- Post-cutover verification: data integrity checks, application smoke tests, performance baseline comparison

## Safe workflow

1. **Frame scope** — confirm source environment, target region, migration tool, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live replication status from SMC/DTS consoles; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius of a failed cutover? what is the rollback time? what data can be lost?
4. **Recommend safest action** — narrow scope, staged cutover, rollback path for each phase

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Migration: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
