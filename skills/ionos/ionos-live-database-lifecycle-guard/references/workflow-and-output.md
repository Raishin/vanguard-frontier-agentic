# Workflow and output contract

Use this sequence when the request involves any IONOS DBaaS lifecycle operation against a live cluster.

## Review domains

Before executing or advising any database lifecycle operation, confirm:

- Target cluster: cluster name or UUID is confirmed and unambiguous
- Named approving identity: a full name or authenticated account identifier (not a role, alias, or ticket number alone)
- Current backup: backup exists with a verified timestamp within the RPO window
- RPO/RTO targets: documented and accepted by the workload owner
- Rollback or recovery plan: a documented recovery path if the operation produces unexpected results
- Regional endpoint correctness: the IONOS DBaaS endpoint region matches the declared GDPR processing location
- Operation scope: failover, replica promotion, horizontal scale, vertical scale, backup schedule change, PITR, or cluster deletion

## Safe workflow

1. **Confirm target and approval**
   - Cluster name or UUID:
   - Datacenter region and DBaaS endpoint:
   - Named approving identity (full name or authenticated account ID):
   - If any of the above is missing or ambiguous → declare a hard stop
2. **Verify backup and recovery posture**
   - Most recent backup timestamp:
   - Backup age within RPO window: yes / no / unknown
   - RPO target (maximum acceptable data loss):
   - RTO target (maximum acceptable downtime):
   - PITR availability: yes / no / unknown
   - If backup verification is absent or RPO/RTO undocumented → declare a hard stop
3. **Validate regional endpoint**
   - PostgreSQL endpoints follow `https://postgresql.<region>.ionos.com`
   - MariaDB and MongoDB endpoints follow the equivalent regional pattern
   - Cross-region endpoint use may constitute a GDPR violation — flag and stop if mismatch
4. **Execute with the smallest bounded change**
   - Prefer staged operations (one replica at a time, rolling scale) over wholesale cluster changes
   - Run the operation only after all hard-stop conditions are confirmed
   - After every mutation, verify cluster state and emit a recovery path
5. **Post-mutation verification**
   - Confirm cluster status returned to healthy
   - Confirm replication lag is within acceptable bounds (for failover/promotion)
   - Confirm backup schedule is intact after scaling
   - Emit next rollback or recovery path explicitly

## Output shape

Return sections in this order:

1. Hard-stop declaration (if any prerequisite is missing) or "Prerequisites confirmed"
2. Target confirmation: cluster ID, region, endpoint
3. Backup verification: timestamp, RPO status, RTO target
4. Approval status: named identity confirmed
5. Rollback or recovery plan
6. Proposed or executed operation scope
7. Post-mutation cluster state
8. Open risks or refusal reason

Keep command evidence sanitized. Do not paste database connection strings, credentials, or customer account identifiers.
