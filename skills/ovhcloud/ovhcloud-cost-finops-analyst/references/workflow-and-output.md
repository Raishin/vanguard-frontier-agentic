# Workflow and output contract

Use this reference only when performing the full cost review, spend forecast, optimization roadmap, or Savings Plan coverage analysis.

## Review domains

Check these areas before giving a verdict:

- Public Cloud project scope, region breakdown, and active service inventory
- Idle or unattached resources: stopped instances still billed, unattached volumes, unused floating IPs, orphaned snapshots
- Savings Plans coverage: commitment level, term, matching instance types, coverage gaps vs. on-demand spend
- Instance rightsizing signals: CPU and memory utilization baselines, instance flavor choices vs. workload requirements
- Tagging hygiene: untagged resources blocking showback, chargeback attribution, or cost allocation
- Backup, monitoring, and redundancy components that must not be cut without explicit risk acceptance

## Safe workflow

1. **Frame scope**
   - Project ID(s) and region(s) in scope:
   - Current monthly spend baseline or billing period:
   - Business criticality and reliability requirements:
   - Required outcome (reduce spend / explain spike / forecast / tagging governance):
   - Explicit non-goals (e.g., do not touch production databases):
2. **Collect evidence**
   - Prefer OVHcloud billing export, Public Cloud usage APIs, or Terraform state evidence if available.
   - Otherwise inspect repository `ovh_cloud_project` or related resources, sanitized billing screenshots, or official OVHcloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What resource deletion could break backups, log retention, monitoring, or redundancy?
   - What rightsizing recommendation could cause performance degradation or SLA breach?
   - What commitment cancellation could incur penalty charges?
   - What evidence is missing that prevents confident savings estimates?
4. **Recommend the smallest safe action**
   - Prefer snapshot-before-delete, staged rightsizing, and validated commitment coverage before recommending cuts.
   - Separate confirmed waste from estimated potential savings; never present projected savings as guaranteed.

## Output contract

Return this structure:

```markdown
# OVHcloud Cost FinOps Review: <project or scope>
## Executive verdict
- Status: OPTIMIZED / IMPROVEMENT AVAILABLE / RISKS PRESENT / NEEDS EVIDENCE
- Biggest waste signal:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Estimated monthly saving | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, prerequisite: <check>, rollback: <rollback>
## Savings summary
- Confirmed waste: <amount or unknown>
- Projected savings (not guaranteed): <range>
## Residual risk
- <risk or explicit none>
```
