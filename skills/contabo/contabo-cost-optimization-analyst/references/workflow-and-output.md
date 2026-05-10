# Workflow and output contract

Use this reference only when performing a full cost review, billing impact assessment, contract period analysis, or rightsizing recommendation for a Contabo environment.

## Review domains

Check these areas before giving a verdict:

- Contract period obligations: active periods (1/3/6/12 months), renewal dates, early-termination implications
- Instance sizing: VPS tier vs. VDS dedicated, CPU/RAM/storage fit for the declared workload
- Storage VPS vs. Object Storage: cost and capability tradeoffs for storage-heavy workloads
- Addon utilization: Private Networking, Additional IPs, Extra Storage, Custom Images — used vs. provisioned
- Billing event horizon: upcoming renewals, period changes, or instance operations that will create new obligations
- Spending anomalies: unexpected charges, orphaned addons, redundant instances

## Safe workflow

1. **Frame scope**
   - Account or instance(s) under review:
   - Business context (workload type, team, budget owner):
   - Optimization goal (reduce cost, right-size, consolidate, forecast):
   - Current contract periods and renewal exposure:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer user-provided billing summaries, instance lists, or addon inventories.
   - Use read-only Contabo API calls for instance and addon state if live access is available.
   - Ground pricing claims in official Contabo documentation; label pricing as `documentation-based` since published prices may change.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What contract period change creates an irreversible billing obligation?
   - What addon is provisioned but unused or underutilized?
   - What instance is oversized relative to observed or stated utilization?
   - What renewal is approaching that requires a decision within the next billing cycle?
   - What evidence is missing that would change the cost assessment?
4. **Recommend the smallest safe action**
   - Prefer changes that reduce cost without creating new contractual lock-in.
   - State the financial impact and obligation of each recommendation explicitly.
   - If the safest action is to wait for a renewal window before changing periods, say that plainly.

## Output contract

Return this structure:

```markdown
# Contabo Cost Optimization Review: <scope>
## Executive verdict
- Status: OPTIMIZED / OPTIMIZED WITH GAPS / COST RISK / NEEDS EVIDENCE
- Biggest cost risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Priority | Finding | Evidence | Financial impact | Recommended action |
|---|---|---|---|---|
## Recommended actions
1. <action> — billing impact: <impact>, contract risk: <risk>, validation: <check>
## Billing event horizon
- Upcoming renewals or decisions:
## Residual risk
- <risk or explicit none>
```
