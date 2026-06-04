# Azure Migration Cutover Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling an assessment green and assuming the cutover is ready.
- Grouping servers by owner or convenience while ignoring runtime dependencies.
- Using stale discovery, missing performance data, or incomplete inventory for final go/no-go.
- Cutting over into an unfinished landing zone with weak DNS, routing, monitoring, backup, or identity.
- Treating rollback as “restore from backup” without timing and data-loss proof.

## Officially grounded service shape

Microsoft Learn evidence says Azure Migrate supports decide, plan, and execute phases: discovery, business case, Azure readiness, right-sizing, cost estimation, dependency analysis, replication, migration, and modernization. Wave planning quality depends on complete discovery, accurate inventory, dependency analysis, application grouping, metadata enrichment, and assessments. Planning agents can interpret migration data but execution actions remain in Azure Migrate workflows.

- Azure Migrate discovery collects inventory and performance data used for business cases and assessments.
- Assessments provide readiness, right-sizing, cost estimates, and blockers, but need current data and correct assumptions.
- Dependency analysis helps group workloads and avoid missed application dependencies.
- Landing-zone readiness includes identity, network, governance, security, management, and operations foundations.
- Cutover readiness adds wave sequencing, freeze windows, validation, rollback, and ownership.

## Non-negotiable design rules

- Require current discovery and assessment timestamps before go/no-go.
- Require dependency evidence for every workload group in a wave.
- Require target landing-zone controls before migration execution.
- Require least-privilege migration permissions and named operators.
- Require rollback checkpoints, data-loss boundary, and post-cutover validation owner.

## Minimal safe implementation flow

- Scope migration wave, workloads, source platform, target region/subscription, landing zone, and business owner.
- Collect assessment, dependency, business case, replication/test migration, landing-zone, and runbook evidence.
- Classify blockers by readiness, dependency, permissions, data, network, identity, monitoring, backup, and rollback risk.
- Decide go, no-go, or conditional go with explicit cutover criteria.
- Define post-cutover validation, monitoring, handover, and rollback deadline.

## Safe verification targets

- Discovery covers all in-scope workloads with current data.
- Assessment assumptions, sizing, cost, and readiness blockers are reviewed.
- Dependency groups align to application behavior and outage window.
- Landing zone has connectivity, DNS, identity, policy, monitoring, backup, and support ownership.
- Rollback has tested procedure, decision point, and accepted RPO/RTO impact.

## When to push back

- The plan lacks current Azure Migrate evidence.
- Dependency mapping is guessed or incomplete.
- Landing-zone controls are “coming later.”
- Rollback cannot be executed within the business recovery window.
