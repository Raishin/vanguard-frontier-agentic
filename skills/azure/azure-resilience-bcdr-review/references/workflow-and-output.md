# Workflow and Output Contract

## Safe Workflow

1. **Define business recovery scope**: workload, environment, region footprint, critical transactions, dependency chain, and who declares disaster.
2. **Force target clarity**: stated RTO, RPO, maximum tolerated data loss, manual-versus-automatic failover expectations, and failback requirements.
3. **Map the recovery design**: zone redundancy, regional redundancy, backups, replication, warm standby, pilot light, active-active, or restore-only pattern.
4. **Check shared responsibility**: separate Azure platform resilience from application correctness, data protection, identity dependencies, DNS/traffic management, and operator runbook obligations.
5. **Collect evidence**: architecture docs, runbooks, drill history, Azure Monitor alerts, Resource Health, Service Health, and any live Azure MCP posture signals.
6. **Stress-test service realities**: identify where services do not provide cross-region failover, where failover is manual, where recovery is eventual, or where data consistency guarantees are weaker than assumed.
7. **Judge runbook quality**: prerequisites, decision points, rollback path, failback sequence, contact chain, evidence capture, and last tested date.
8. **Return a verdict**: READY, READY WITH RISKS, or NOT READY, with explicit blockers, evidence labels, and next drills or design changes.

## Role-Specific Stress Checks

- Reject any design that promises near-zero RTO or zero RPO without explicit architecture, cost, operational ownership, and test evidence.
- Challenge assumptions that zone redundancy equals cross-region disaster recovery.
- Challenge assumptions that backups alone satisfy low-RTO requirements.
- Challenge assumptions that Azure-managed failover automatically covers application state, integration endpoints, secrets, DNS, certificates, third-party dependencies, or identity-plane dependencies.
- Check whether failback is materially harder than failover and whether the plan admits that.
- Treat untested runbooks as weak evidence, not readiness proof.
- Treat health and alert signals as detection inputs only; they are not recovery execution.
- Call out single-region control dependencies, manual approval bottlenecks, and undocumented human handoffs.

## Output Template

```markdown
# Azure Resilience BCDR Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest recovery risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Business targets
- Critical service:
- Region or topology:
- Required RTO:
- Required RPO:
- Disaster declaration owner:

## Recovery design summary
- Pattern:
- Failover mode:
- Failback mode:
- Dependencies:
- Shared-responsibility boundary:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Runbook and test posture
- Last tested:
- Test type:
- Gaps:
- Missing proof:

## Service-level recovery gaps
- ...

## Safe next actions
1.
2.
3.

## Assumptions and unknowns
- ...
```

## Red Flags

- The plan says "geo-redundant" but never states real RTO, real RPO, or failback behavior.
- The answer assumes every Azure service in scope supports automatic cross-region recovery.
- The design has replicated infrastructure but no tested data or identity recovery path.
- The runbook has failover steps but no failback criteria, rollback branch, or ownership chain.
- Recovery readiness is claimed from architecture diagrams alone, with no drill evidence.
- Monitoring exists, but no alert-to-decision workflow shows who acts, when, and with what authority.
