# Workflow and output contract

Use this reference only when executing the full daily operations briefing, generating the briefing report, or preparing action item assignments.

## Briefing domains

Check these areas before giving a recommendation:

- Cost delta: CN-* Cost Manager vs international Cost Manager, day-over-day comparison, >15% spike causes
- ActionTrail anomalies: unusual CreateAccessKey, AssumeRole, DeleteBucket, or bulk resource deletion events in last 24 hours
- ACK health: pod failure scope (single-AZ vs multi-AZ), crash loop causes, resource exhaustion
- Quota utilization: ECS instances per region, EIP per VPC, RDS instances per account — >80% threshold
- Security Center findings: HIGH/CRITICAL severity, finding age, owner assignment status
- Open action items: owner, due date, status from prior briefing cycle

## Safe briefing workflow

1. **Cost delta review**
   - Pull CN-* Cost Manager and international Cost Manager data separately.
   - Compare to prior day baseline.
   - If >15% delta: assign investigation owner before proceeding.
   - Common spike causes: MaxCompute on-demand job runs, CDN traffic spikes, ECS spot replacement.
2. **ActionTrail anomaly triage**
   - Query last 24 hours for: CreateAccessKey, AssumeRole from new principals, DeleteBucket, bulk ECS stop/delete.
   - If anomaly found: escalate to security team immediately; do not defer to next cycle.
3. **ACK and application health**
   - Identify pod failures and classify by AZ scope.
   - Single-AZ failure → app team owner.
   - Multi-AZ failure → platform team escalation.
4. **Quota utilization check**
   - Check ECS instance quota per region, EIP quota per VPC, RDS instance quota per account.
   - >80% utilization → file quota increase request immediately.
5. **Security Center finding triage**
   - List HIGH and CRITICAL findings.
   - Flag findings older than 24 hours without owner assignment.
   - Escalate missed SLA findings to security team lead at the briefing.
6. **Assign open action items**
   - Every finding must have an owner and due date before briefing ends.
   - No finding may be carried forward without explicit owner acknowledgment.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Daily Operations Briefing: <date>
## Cost delta summary
- CN-* cost delta (vs prior day):
- International cost delta (vs prior day):
- Spike investigation (>15% delta): owner: <owner>, investigation due: <time>
- Spike cause candidates:
## ActionTrail anomaly triage
- Anomalies detected (last 24h):
- Security escalation required:
## ACK and application health
| Cluster | Failed pods | AZ scope | Owner |
|---|---|---|---|
- Cluster-level escalation triggered:
## Quota utilization warnings
| Resource | Current utilization | Threshold | Action required |
|---|---|---|---|
## Security Center finding triage
| Finding | Severity | Age | Owner | SLA status |
|---|---|---|---|---|
- Missed SLA escalation:
## Open action items
| Item | Owner | Due | Status |
|---|---|---|---|
## Next 24-hour risk summary
- Top risks:
- Pre-emptive actions:
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
