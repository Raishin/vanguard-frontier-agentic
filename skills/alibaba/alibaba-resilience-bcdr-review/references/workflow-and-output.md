# Workflow and output contract

Use this reference only when performing a full BCDR review or production-readiness assessment.

## Review domains

Check these areas before giving a recommendation:

- Workload criticality classification (mission-critical, business-critical, standard)
- Stated RTO/RPO targets and evidence of tested recovery against those targets
- HA architecture: RDS HA Edition, PolarDB GDN, ACK multi-zone, ECS cross-region
- HBR backup coverage: vault region, backup frequency, application-consistent snapshot policy
- Cross-region redundancy: separate ACK clusters, GSLB/GTM routing, DR region readiness
- Runbook completeness: documented failover steps, owner assignment, last tested date

## Safe workflow

1. **Frame requirements**
   - Workload name and criticality classification:
   - Stated RTO target:
   - Stated RPO target:
   - Regions in use (primary and DR):
   - Last DR drill date and outcome:
2. **Collect evidence**
   - Prefer live console or aliyun CLI evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What happens if the primary region becomes unavailable?
   - What single points of failure exist in the HA design?
   - Are HBR vaults in a separate region?
   - Is there evidence of a successful failover test?
   - What runbook steps are missing or unowned?
4. **Recommend the smallest safe next step**
   - Prioritize by risk: untested recovery > missing cross-region vault > no GSLB > aspirational RTO claims.
   - If the safest action is to schedule a DR drill, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud BCDR Review: <workload scope>
## Executive summary
- BCDR posture verdict:
- Evidence level:
- Critical gaps:
## Workload profile
- Criticality:
- RTO target (stated):
- RPO target (stated):
- Primary region:
- DR region:
## HA architecture assessment
| Component | Current configuration | HA classification | Gaps |
|---|---|---|---|
## HBR backup coverage
- Vault region:
- Backup frequency:
- Application-consistent snapshots:
- Cross-region vault confirmed:
## Recovery test evidence
- Last DR drill date:
- Scope tested:
- Outcome:
- Gaps identified during drill:
## Runbook completeness
- Failover steps documented:
- Owner assigned:
- Last reviewed date:
## Prioritized BCDR improvements
1. <improvement> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
