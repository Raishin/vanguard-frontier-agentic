# Workflow and output contract

Use this reference only when performing a full capacity review, quota exhaustion assessment, growth trajectory model, or producing a structured capacity planning response.

## Review domains

Check these areas before giving a verdict:

- **Per-project resource limits** — servers, Volumes, Load Balancers, Floating IPs, Primary IPs, Networks, SSH keys, certificates — compare current counts against Hetzner Cloud per-project quotas
- **Quota exhaustion timeline** — growth rate estimation, time-to-quota based on current trajectory
- **Server type upgrade paths** — shared CX to dedicated CCX progression, ARM CAX options, minimum downtime resize paths
- **Region distribution** — deployment spread across fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), hel1 (Helsinki FI), single-region risk for production
- **No auto-scaling caveat** — Hetzner has no native auto-scaling; manual provisioning lead time is ~2–5 minutes per server; growth plans must account for this
- **Storage Box capacity** — Storage Box plan limits, Snapshot Plan configuration (requires `hour` and `minute` parameters), snapshot retention accumulation
- **Project split strategy** — when to split a monolithic Hetzner project into multiple projects to reset per-project quota counters

## Safe capacity planning workflow

1. **Frame scope**
   - Hetzner project and environment:
   - Current resource inventory (or note if not queried):
   - Growth driver (traffic forecast, new workloads, seasonal peak):
   - Planning horizon (30/90/180 days):
   - Required outcome (quota headroom check, scaling design, region strategy):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer live Hetzner MCP read-only evidence if available.
   - Otherwise inspect user-provided sanitized API output, infrastructure-as-code, or official Hetzner docs.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Which resource type reaches quota exhaustion first on current trajectory?
   - Is any region at risk of being the single point of failure?
   - What is the minimum provisioning lead time the workload can tolerate?
   - What evidence is missing that would change the capacity verdict?

4. **Recommend the smallest safe action**
   - Prefer proactive quota increase requests over reactive panic provisioning.
   - Prefer multi-region distribution over single-region vertical scaling.
   - If the safest action is to gather current resource counts first, say that plainly.

## Output contract

Return this structure:

```markdown
# Hetzner Cloud Capacity Plan: <scope>
## Executive verdict
- Status: HEALTHY / HEADROOM TIGHT / QUOTA RISK / NEEDS EVIDENCE
- Most constrained resource:
- Estimated time to quota exhaustion (if data available):
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Resource inventory vs limits
| Resource type | Current count | Per-project limit | Headroom | Risk |
|---|---|---|---|---|
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, timeline: <when>, validation: <check>
## Region distribution assessment
- fsn1 (Falkenstein DE): <count / risk>
- nbg1 (Nuremberg DE): <count / risk>
- hel1 (Helsinki FI): <count / risk>
- Recommendation:
## Residual risk
- <risk or explicit none>
```
