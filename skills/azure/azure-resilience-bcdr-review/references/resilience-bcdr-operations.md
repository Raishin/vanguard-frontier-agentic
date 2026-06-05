# Azure Resilience BCDR Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Claiming zero RTO or zero RPO without proving cost, latency, consistency, and failover mechanics.
- Treating availability zones as a complete disaster recovery plan.
- Testing backup creation but never testing restore time and data correctness.
- Writing failover steps but forgetting failback.
- Storing DR runbooks, scripts, or credentials only in the failed region or failed platform path.

## Officially grounded service shape

- Microsoft Learn evidence says DR plans must align to recovery targets and cover all components and the system as a whole.
- RTO and RPO are business-defined recovery metrics; aiming for zero downtime or zero data loss is difficult and costly and must be agreed by technical and business stakeholders.
- DR is not an automatic feature of Azure. Azure services provide capabilities that must be mapped to a workload-specific DR plan.
- Well-Architected guidance requires business impact prioritization, disaster thresholds, communication protocols, recovery-aware architecture, backup strategy, drills, current plans, accessible DR assets, and safe automation.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident state, or production readiness.

## Non-negotiable design rules

- Define workload tier, business impact, RTO, RPO, and disaster declaration threshold before architecture review.
- Review each dependency, not only the primary compute or database service.
- Keep failback as a separate documented process from failover.
- Test restores, failover, failback, operator access, monitoring, and communication paths.
- Treat DR automation as high risk unless trained operators, approvals, and circuit breakers are defined.

## Minimal safe implementation flow

- Scope business service, components, dependencies, regions/zones, data stores, and recovery owners.
- Map target RTO/RPO to replication, backup, failover, and restore mechanisms per component.
- Review runbooks, communication plan, escalation path, DR asset availability, and access model.
- Assess drill evidence and gaps against component-level and workload-level recovery targets.
- Return blockers, conditional recovery posture, safe next tests, and required plan updates.

## High-risk assumptions to kill

- Zero RTO or zero RPO is not a realistic default; it must be business-approved with cost, consistency, latency, and operational tradeoffs visible.
- Availability zones, backups, or paired regions alone are not a disaster recovery plan for the workload as a system.
- Backup success is weak evidence until restore duration, data correctness, identity access, and dependency recovery are tested.
- Failover without failback is half a DR plan and can strand production in an unplanned state.
- DR assets stored only in the primary region, primary tenant path, or failed platform path are not available DR assets.

## Safe command/code verification targets

- Verify business tier, RTO, RPO, disaster threshold, dependency map, and recovery owner for each component.
- Check backup, replication, failover, failback, restore-test, and drill evidence against documented targets.
- Confirm runbooks include communication plan, escalation path, operator access, monitoring, and manual decision points.
- Validate DR automation has approvals, circuit breakers, and current credentials or break-glass paths outside the failed dependency.
- Label every untested recovery claim as conditional or unproven, not production-ready.

## Safe verification targets

- RTO/RPO are documented and tied to business criticality.
- Failover and failback have separate runbooks and decision owners.
- Backups restore within target RTO and meet target RPO in tested evidence.
- DR scripts, pipelines, credentials, and docs remain accessible during regional outage scenarios.
- Latest drill evidence covers technical steps and human process steps.

## When to push back

- The user wants DR approval without restore or drill evidence.
- The plan assumes Azure platform resilience equals workload continuity.
- Failback is undocumented or deferred.
- Recovery assets are stored only in the primary region or one operational path.
