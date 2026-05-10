# Workflow and output contract

Use this reference only when performing a full IONOS Cloud cost review or optimization analysis.

## Review domains

Check these areas before giving a verdict:

- Idle and underutilized compute: servers running at consistently low CPU and memory with no scheduled workload
- Orphaned and oversized volumes: detached volumes, volumes larger than their actual usage, snapshot accumulation
- Managed service tier fit: is the contracted PostgreSQL, MariaDB, or MongoDB tier appropriate for observed workload?
- Backup and snapshot cost: is the retention policy proportionate to RPO requirements, or are old snapshots accumulating?
- Pricing tier and contract strategy: is the resource under a monthly, hourly, or reserved contract and is that optimal?
- Cross-region consolidation feasibility: can resources be consolidated into fewer regions without violating GDPR constraints?
- Cost attribution: are resources tagged consistently enough to support showback or chargeback?

## Safe workflow

1. **Frame scope**
   - Billing period and scope (entire account, specific datacenter, specific resource type):
   - GDPR data residency constraints and declared regions:
   - Business criticality of resources under review:
   - Required outcome (reduce spend, explain spike, attribute costs):
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer billing API data or cost dashboard export if available.
   - Otherwise inspect IaC resource definitions, user-provided utilization screenshots, or pricing documentation.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test optimization risk**
   - What can reduce reliability or availability if this resource is downsized or removed?
   - What backup or recovery capability is lost if a snapshot policy is tightened?
   - What GDPR constraint blocks a cross-region consolidation?
   - What is the rollback cost if a decommission turns out to be premature?
   - What utilization data is missing that would change the recommendation?
4. **Recommend the safest optimization first**
   - Prefer low-risk actions (snapshot cleanup, orphaned volume removal) before risky ones (production server downsizing).
   - Never recommend removing backups, disabling encryption, or eliminating audit logging without explicit risk acceptance.
   - Always state the estimated saving and the specific risk per recommendation.

## Output contract

Return this structure:

```markdown
# IONOS Cost Optimization Analysis: <scope>
## Executive verdict
- Estimated optimization potential: <range>
- Evidence level:
- GDPR constraint summary:
## Scope and assumptions
- Billing period:
- Regions covered:
- Confirmed:
- Unknown:
- Out of scope:
## Waste findings
| Priority | Category | Resource | Estimated saving | Risk | Evidence | Recommended action |
|---|---|---|---|---|---|---|
## Recommended actions (ranked by risk)
1. <action> — saving: <estimate>, risk: <level>, validation: <check>, rollback: <rollback>
## GDPR and reliability constraints
- <constraint that limits consolidation options, or "none">
## Evidence gaps
- <gap or explicit none>
```
