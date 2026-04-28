# Workflow and Output Contract

## Safe Workflow

1. **Fix the scope first**: management group, subscription, resource group, billing scope, or mixed; name the decision owner and reporting audience.
2. **Check visibility before optimization**: confirm cost analysis usage, scope coverage, tag coverage, amortized-versus-actual view awareness, and whether exported data exists for durable reporting.
3. **Inspect ownership and accountability**: who receives alerts, who approves spend exceptions, who owns tag quality, who acts on Advisor findings, and who reviews recurring exports.
4. **Review spend controls**: budgets, threshold alerts, forecast alerts, cadence, and whether thresholds are aligned to actual governance response rather than vanity numbers.
5. **Review optimization evidence**: Advisor cost recommendations, pricing posture, reservation awareness, savings-plan awareness, idle or oversized patterns, and known blind spots.
6. **Review allocation and reporting quality**: tag strategy, missing owner/cost-center tags, export automation, dataset choice, and downstream reporting consumers.
7. **Give a prioritized governance verdict**: visibility gaps first, ownership gaps second, control gaps third, optimization actions fourth. Do not reverse that order unless evidence is overwhelming.

## Role-Specific Stress Checks

- Do not promise savings without utilization or recommendation evidence.
- Do not confuse price estimation with governance. A pricing quote is not a cost-control operating model.
- Do not call budgets “enforcement.” Budgets alert; they do not stop resource consumption.
- Do not ignore actual-versus-amortized cost views when reservations or savings plans matter.
- Do not accept “we have tags” as success unless tag coverage, consistency, and ownership are real.
- Do not recommend exports without naming dataset consumers, refresh cadence, storage path ownership, and verification checks.
- Do not treat Azure Advisor recommendations as self-justifying. Check ownership, feasibility, and risk before calling them savings.
- Do not accept governance claims when no one owns alerts, no one reviews exports, and no one triages cost anomalies.

## Output Template

```markdown
# Azure Cost Governance Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest cost-governance gap:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Billing or resource scope:
- Owner:
- Audience:
- Requested action:

## Visibility and ownership
| Area | Current state | Risk | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|
| Cost analysis |  |  |  |  |  |
| Budgets and alerts |  |  |  |  |  |
| Tags for cost |  |  |  |  |  |
| Exports and reporting |  |  |  |  |  |
| Reservation and savings-plan awareness |  |  |  |  |  |
| Advisor and optimization review |  |  |  |  |  |

## Prioritized controls
1.
2.
3.

## Safe next actions
1.
2.
3.

## Open questions
- 
```

## Red Flags

- The request asks for “cost optimization” but no scope owner, budget owner, or reporting audience exists.
- The answer claims savings based only on pricing pages or intuition, without usage evidence, Advisor findings, or documented governance gaps.
- The design depends on reservations or savings plans but ignores amortized reporting.
- The team has budgets but no alert recipients with accountability.
- The team exports data but cannot name who consumes it or how failed exports are detected.
- The plan depends on tags for cost allocation but tag coverage or ownership is unknown.
