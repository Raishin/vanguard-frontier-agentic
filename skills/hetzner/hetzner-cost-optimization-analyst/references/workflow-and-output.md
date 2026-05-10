# Workflow and output contract

Use this reference only when performing a full cost review, spend spike investigation, rightsizing analysis, or producing a structured cost optimization response.

## Review domains

Check these areas before giving a verdict:

- **Server type selection** — shared CX vs dedicated CCX, Intel vs ARM (CAX), oversized instance types, spot vs on-demand trade-offs not applicable (Hetzner has no spot instances)
- **Idle and unattached resources** — Volumes not attached to a server, Primary IPs not attached to a server, Floating IPs with no active routing
- **Load Balancer utilization** — single-target LBs (likely unnecessary), LBs with no active connections, LB plan tier vs actual traffic
- **Storage Box consumption** — Storage Box plans vs actual usage, Snapshot Plans accumulating more snapshots than needed, manual snapshot retention
- **Snapshot accumulation** — server image snapshots not associated with active recovery paths, stale snapshot buildup
- **Cost attribution** — missing labels preventing cost attribution to project, team, or environment

## Safe cost review workflow

1. **Frame scope**
   - Hetzner project and environment:
   - Billing period or cost spike timeframe:
   - Business criticality and owner:
   - Required outcome (waste identification, rightsizing plan, cost attribution):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer live Hetzner MCP read-only evidence if available.
   - Otherwise inspect user-provided sanitized API output, billing exports, or official Hetzner docs.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What savings recommendations would remove the only recovery path?
   - What deletions are irreversible without a prior snapshot?
   - What rightsizing would break a workload's CPU or memory contract?
   - What evidence is missing that would change the cost verdict?

4. **Recommend the smallest safe action**
   - Prefer staged rightsizing over immediate deletion.
   - Never recommend deleting a Volume or snapshot that is the only recovery path without explicit risk acceptance from the user.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Hetzner Cloud Cost Review: <scope>
## Executive verdict
- Status: OPTIMIZED / SAVINGS AVAILABLE / WASTE FOUND / NEEDS EVIDENCE
- Estimated monthly waste:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Resource | Evidence | Estimated monthly saving | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, reversible: <yes/no>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
