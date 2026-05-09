# Workflow and output contract

Use this reference only when performing a full database administration review or implementation guidance.

## Database areas to check

- GaussDB/RDS instances: engine type, version, flavor, AZ HA mode, read replica count, maintenance window
- DDS: topology (replica set vs sharded), shard key design, WiredTiger configuration
- Database proxy: connection pool size, read/write split ratio, failover behavior, health check endpoint
- CBR backup: snapshot schedule, log backup enabled (PITR), retention period, last successful backup, recovery test
- HA failover: primary-standby configuration, RPO/RTO targets, failover test history
- GaussDB for Oracle: compatibility test results, PL/SQL procedures tested, data type mapping gaps

## Safe workflow

1. **Frame scope** — confirm target database engines, HA requirements, compliance needs, and non-goals
2. **Collect evidence** — prefer live instance status and backup history; label all evidence types
3. **Stress-test** — backup gaps, Oracle compatibility risks, proxy single-point-of-failure, failover coordination
4. **Recommend safest action** — staged changes, backup verification first, failover test scheduling

## Output contract

Return this structure:

```markdown
# Huawei Cloud Database Administration: <scope>
## Scope and evidence level
## Instance inventory and HA configuration
## CBR backup coverage and PITR status
## Database proxy configuration
## GaussDB for Oracle compatibility assessment
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
