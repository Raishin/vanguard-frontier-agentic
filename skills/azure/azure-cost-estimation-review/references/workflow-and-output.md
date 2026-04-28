# Workflow and Output Contract

## Safe Workflow

1. **Define the estimate boundary**: identify billing scope, target environment, currency, timeframe, and whether the estimate is for rough planning, architecture comparison, or approval.
2. **Separate retail estimate from real payable cost**: check whether the estimate uses retail pricing only, logged-in agreement pricing, savings-plan assumptions, or reserved-capacity assumptions.
3. **Inspect workload assumptions**: uptime, scale units, storage growth, bandwidth, transaction volume, backup, HA/DR, and environment count.
4. **Inspect SKU and region realism**: verify that the chosen region, service tier, VM family, database tier, redundancy option, and network path match the stated workload and compliance constraints.
5. **Challenge nonproduction shortcuts**: confirm whether dev/test assumptions are being incorrectly reused for production, especially around uptime, redundancy, throughput headroom, support, and recovery expectations.
6. **Check omission risk**: look for missing networking, monitoring, backup, egress, public IP, log ingestion, premium storage, or disaster-recovery components that make the estimate falsely low.
7. **Check uncertainty labeling**: require explicit statements about what is estimated, what is unknown, what depends on future utilization, and what may change by region, agreement, or discount posture.
8. **Return an estimate-credibility verdict**: call the estimate credible only if scope, assumptions, omissions, and uncertainty are explicit.

## Role-Specific Stress Checks

- Do not accept calculator output as truth merely because it came from Microsoft tooling. Garbage assumptions still produce garbage estimates.
- Do not confuse retail pricing with negotiated enterprise pricing or final invoice cost.
- Do not accept production estimates that omit HA, DR, logging, backup, or support expectations.
- Do not accept “same SKU in every region” thinking without latency, availability, and price variation review.
- Do not let teams reuse nonproduction right-sizing assumptions for production without headroom and resilience justification.
- Do not accept monthly totals without checking whether the estimate assumes 24x7 runtime, partial-hours usage, autoscale behavior, or paused resources.
- Do not accept savings-plan or reservation claims unless the ownership, scope, term, and utilization confidence are explicit.
- Do not hide uncertainty. If storage growth, throughput, egress, or retention is unknown, say the estimate is weak.

## Output Template

```markdown
# Azure Cost Estimation Review: <scope>

## Verdict
- Status: CREDIBLE / CREDIBLE WITH RISKS / NOT CREDIBLE
- Biggest estimate risk:
- Evidence level: live evidence / documentation-based / user-provided sanitized evidence / inference

## Scope
- Billing or resource scope:
- Environment: dev / test / prod / mixed
- Time horizon:
- Currency:
- Requested action:

## Assumption review
| Area | Current assumption | Risk | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|
| Region |  |  |  |  |  |
| SKU / tier |  |  |  |  |  |
| Runtime / uptime |  |  |  |  |  |
| Scale / quantity |  |  |  |  |  |
| Storage / retention |  |  |  |  |  |
| Network / egress |  |  |  |  |  |
| HA / DR |  |  |  |  |  |
| Savings plan / reservation |  |  |  |  |  |
| Missing components |  |  |  |  |  |
| Uncertainty labeling |  |  |  |  |  |

## Safe next actions
1.
2.
3.

## Open questions
- 
```

## Red Flags

- The estimate is being used for approval, but nobody can explain the region, SKU, or runtime assumptions.
- The estimate mixes nonproduction and production expectations into one number.
- The estimate assumes savings-plan or reserved pricing without ownership or utilization confidence.
- The estimate excludes DR, monitoring, backup, or network egress for a production workload.
- The estimate presents a single monthly number with no uncertainty band or missing-assumption list.
- The estimate claims invoice accuracy even though it is based only on retail pricing or incomplete sizing inputs.
