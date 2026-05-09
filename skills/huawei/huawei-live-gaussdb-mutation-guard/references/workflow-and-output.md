# Workflow and Output — Huawei Live GaussDB Mutation Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm account ID, enterprise project, and active IAM principal.
2. Describe the target instance: engine type, version, region, HA mode.
3. Identify MLPS Level 3 classification for the database.

### Phase 2: CBR Backup Verification

4. Check CBR vault for the target instance:
   - CBR console > Vaults > select vault > Backups tab
5. Confirm the most recent backup is within 24 hours and shows status `Available`.
6. Verify the backup is restorable: attempt a test restore metadata check if possible.

### Phase 3: Dependent Application Enumeration

7. List all applications connecting to this instance (connection strings, database proxies).
8. List any read replicas or DRS replication tasks sourced from this instance.
9. Identify all downstream data pipelines reading from this instance.

### Phase 4: Blast Radius Assessment

10. For deletion: total data volume at risk, application downtime duration, DRS replication chain break.
11. For spec downgrade: connection pool headroom reduction, query concurrency impact.
12. For backup policy removal: RPO gap created, duration without recovery protection.

### Phase 5: Approval Gate

13. Present all evidence to the operator: instance identity, backup status, MLPS classification, dependent apps, blast radius.
14. Require explicit written approval including MLPS incident obligation acknowledgment if applicable.
15. Do not proceed until approval is received.

### Phase 6: Execution

16. Execute the approved operation via GaussDB/RDS console.
17. For deletion: confirm data export or final backup before proceeding.

### Phase 7: Post-Change Verification

18. Verify instance state reflects the executed action.
19. Check CES alarms for anomalies.
20. If MLPS Level 3 data destruction: initiate incident report process within 24 hours.

## Expected Output Format

```
INSTANCE IDENTITY
  Account:            <account-id>
  Enterprise Project: <enterprise-project>
  Instance:           <instance-name>
  Engine:             <engine-type>/<version>
  Region:             <region>

CBR BACKUP STATUS
  Latest backup:      <date>
  Status:             [AVAILABLE / UNAVAILABLE / NOT CONFIGURED]
  Restorable:         [YES / NO / UNVERIFIED]

MLPS CLASSIFICATION
  Level:              [MLPS Level 3 / Not classified / Unknown]
  Incident obligation: [YES — 24h report required / NO]

DEPENDENT APPLICATIONS
  Applications:       <list or NONE>
  Replicas:           <count>
  DRS tasks:          <count>

BLAST RADIUS
  Data at risk:       <size>
  Application impact: <description>

APPROVAL STATUS
  Operator:           <identity>
  Approved:           [YES / NO / PENDING]
  MLPS acknowledged:  [YES / NO / N/A]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

VERIFICATION
  Instance state:     <confirmed state>
  CES anomalies:      [NONE / <description>]
  Incident filed:     [YES / NO / N/A]
```
