# Workflow and output contract

Use this reference when performing a full cost review, rightsizing analysis, reserved instance evaluation, or idle resource audit.

## Review domains

Check these areas before giving a verdict:

- Instance rightsizing: type family (DEV1, GP1, ENT1, RENDER, COPARM, STARDUST), utilization vs provisioned capacity, lifecycle (hourly vs reserved)
- Reserved instance commitments: monthly vs annual, utilization rate, non-refundability risk, instance family match
- Object Storage: bucket lifecycle rules, cold data tiering, egress patterns, orphaned or unnamed buckets
- SBS block storage: unattached volumes, over-provisioned volume sizes, snapshot accumulation
- Serverless functions: invocation count vs cold-start frequency, memory allocation vs actual usage, idle functions
- RDB instances: instance class vs connection count and IOPS, backup retention window vs cost
- Cockpit observability: active plan tier vs actual metrics/log/trace ingestion, unused dashboards or alert rules

## Safe workflow

1. **Frame scope**
   - Projects and environments in scope:
   - Cost period or billing month under review:
   - Budget target or saving goal:
   - Required outcome:
   - Explicit non-goals (e.g., do not touch backups):
2. **Collect evidence**
   - Prefer sanitized billing exports, Terraform state, or user-provided cost summaries.
   - Label each finding as `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
   - If billing data or resource utilization data was not provided, state that explicitly before proceeding.
3. **Stress-test risk**
   - Which reserved instance commitments are non-refundable and at risk of under-utilization?
   - Which cuts would remove backup, security, logging, or observability coverage?
   - Which volume or instance deletions are irreversible?
   - What savings estimate is based on incomplete utilization data?
4. **Recommend the smallest safe cost reduction action**
   - Prefer rightsizing, lifecycle rules, and snapshot cleanup before recommending deletion of active resources.
   - Flag every non-refundable commitment before recommending purchase.
   - If the safest action is to gather utilization data before recommending changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Scaleway Cost Optimization Review: <scope>

## Cost posture verdict
- Status: OPTIMIZED / OPTIMIZATION OPPORTUNITIES FOUND / NEEDS EVIDENCE
- Biggest waste category:
- Evidence level:

## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:

## Findings
| Severity | Resource type | Finding | Evidence | Estimated saving | Minimum safe action |
|---|---|---|---|---|---|

## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback risk: <risk>

## Irreversible or commitment-locked items
- <reserved instance purchases, volume deletions, or similar — or explicit none>

## Residual risk
- <risk or explicit none>
```
