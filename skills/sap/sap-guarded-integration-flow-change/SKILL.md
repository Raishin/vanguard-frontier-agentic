---
name: sap-guarded-integration-flow-change
description: Deploy or modify SAP Cloud Integration iFlows and integration artifacts using a mandatory 17-step guarded mutation sequence covering classification, target tenant confirmation, criticality, requester, approver (integration-owner required), ticket, scope, read-only current state, diff of artifact changes, blast radius (downstream partners and throughput), rollback (previous version redeploy), post-change verification via message monitoring, and audit evidence. Refuses if any step is missing, if integration-owner approval is absent, or if a diff has not been produced. Use when a confirmed and approved iFlow deployment or integration artifact change in a non-development Cloud Integration tenant must be executed under traceable controls.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: delivery
  lifecycle: experimental
  execution_tier: mutating-runtime
---

# SAP Guarded Integration Flow Change

## Purpose

Deploy or modify SAP Cloud Integration iFlows and integration artifacts in non-development tenants under a strict 17-step guarded mutation sequence. Every step must be completed, documented, and confirmed before the deployment execution step is reached. If any step is absent, the deployment is refused until it is completed. This skill treats every iFlow deployment or integration artifact change in a quality, pre-production, or production Cloud Integration tenant as a risk-critical mutating operation requiring explicit integration-owner approval, a documented blast radius covering downstream partners and throughput impact, and a verified rollback path via previous version redeploy.

## When to use

Use this skill when:

- an iFlow, value mapping, script collection, API provider, or other Cloud Integration artifact must be deployed or modified in a quality, pre-production, or production tenant,
- an integration owner or delivery lead has confirmed the change is approved and requires traceable execution,
- post-deployment verification via message monitoring and audit evidence are required for change management or compliance purposes,
- the user needs a structured, risk-controlled execution protocol for SAP Integration Suite Cloud Integration deployments.

## When not to use

- When the target is a development or sandbox tenant (DEV) with no live partner connections — development deployments are lower risk and do not require this full sequence.
- When the user wants to *design* or *test* an iFlow in the design workspace without deploying — advise on the sequence but do not reach the execution step.
- When any of the 17 steps cannot be completed — refuse execution until all steps are resolved.
- When no explicit approval from the integration owner has been obtained — refuse execution.
- When a diff of the artifact changes has not been produced — refuse execution; deployment on an undiffed artifact is forbidden.

## Lean operating rules

- 17 steps, no shortcuts. The sequence is mandatory and must be completed in order. Steps cannot be skipped or reordered.
- Refuse before step 13. Never reach the execution step (step 14) without documented integration-owner approval on record in this session.
- Segregation of duties enforced. The person who requests the deployment (step 4) must not be the same person who approves it (step 12). If SoD is violated, refuse execution.
- Read current state before any change. Step 8 (read-only current state) must be completed before any deployment is proposed. Do not propose a deployment on undocumented current artifact state.
- Diff before live execution. Step 9 (diff of artifact changes) must precede execution. Document which iFlow components, script steps, routing conditions, adapter configurations, and value mappings are changing.
- Blast radius documented before execution. Step 10 must list all downstream partner systems, message throughput impacts, dependent iFlows, API consumers, and business processes affected. Deployments with undocumented blast radius are refused.
- One artifact change at a time. Do not batch-deploy multiple iFlows in a single execution step unless each has completed its own full 17-step sequence.
- Never hardcode credentials. All system access uses pre-configured service keys, OAuth clients, or process integration runtime users — never credentials in plain text.
- Evidence labeling mandatory. Every step must produce labeled evidence before the next step proceeds.
- Integration-owner approval is not optional. The integration owner (the team responsible for the iFlow and its downstream partners) must be the named approver in step 5. General delivery-lead approval is insufficient for production deployments.

## The 17-step guarded mutation sequence

| Step | Name | Description | Evidence required |
|------|------|-------------|-------------------|
| 1 | Classify | Classify the artifact type (iFlow, value mapping, script collection, API provider, integration package) and change type (new deployment, update, rollback, undeploy) | Artifact type and change type documented |
| 2 | Confirm target | Confirm the exact target tenant (Cloud Integration tenant ID, environment tier: QA/PPD/PRD, Integration Suite sub-domain) | Target tenant ID, tier, and sub-domain confirmed by user |
| 3 | Criticality assessment | Assess business criticality of the affected integration (financial document exchange, order-to-cash, HR data, payment processing, regulatory reporting) | Criticality level: low/medium/high/critical |
| 4 | Requester confirmation | Confirm the name, role, and authorization of the change requester | Requester name and role documented |
| 5 | Integration-owner approval identification | Identify the integration owner — the team or individual accountable for the iFlow and its downstream partner connections (different person from requester — SoD requirement) | Integration-owner name, role, and accountability scope |
| 6 | Ticket linkage | Link the deployment to a change management ticket (ITSM reference: ServiceNow, Remedy, SAP Solution Manager ChaRM, etc.) | Ticket number confirmed |
| 7 | Scope documentation | Document the complete list of artifacts to be deployed or modified: iFlow IDs, package names, version numbers, and interdependencies with other iFlows or API providers | Artifact list with IDs, versions, and dependency map |
| 8 | Read-only current state | Read the current state of the target tenant: deployed artifact versions, runtime status, active message queues, recent error rates from monitoring | Live evidence from target tenant (read-only) |
| 9 | Diff of artifact changes | Produce a diff of the artifact changes: which iFlow steps, routing conditions, adapter configurations, scripts, and value mappings are changing relative to the currently deployed version | Diff output documenting changed components |
| 10 | Blast radius | Document all downstream partner systems, dependent iFlows, API consumers, message throughput estimates, and business processes affected; assess risk of message loss or duplication during deployment | Blast radius document with affected partners, consumers, and throughput impact |
| 11 | Rollback plan | Document the explicit rollback procedure: which artifact version to redeploy, how to trigger a previous-version redeploy from the Cloud Integration design workspace or API, and any partner notification steps required | Rollback procedure documented and confirmed feasible |
| 12 | SoD verification | Verify that requester (step 4) and integration owner (step 5) are different authorized individuals; confirm no self-approval | SoD confirmed: requester ≠ integration owner / approver |
| 13 | Approval gate | Obtain explicit written approval from the integration owner for this specific deployment | Approval statement on record in this session |
| 14 | Execute approved deployment | Deploy or modify the approved artifacts only, in the documented sequence | Deployment command log with timestamp |
| 15 | Verify via message monitoring | Verify post-deployment state: check runtime status of deployed iFlows, inspect message monitoring for errors, confirm throughput resumes within expected parameters, verify downstream partner acknowledgements where applicable | Post-deployment message monitoring log and verification results |
| 16 | Audit | Produce complete audit record: all 17 steps, evidence, commands, timestamps, integration-owner approval | Audit record |
| 17 | Report | Deliver final report to requester, integration owner, and change manager | Report delivered and acknowledged |

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live Cloud Integration tenant in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs
- `user-provided evidence` — stated or supplied by the user in this session
- `inference` — derived reasoning not directly confirmed by live data or official docs; must always be labeled

## Live-environment rules

**Allowed** (step 8 only — read-only current state):
- Cloud Integration OData API: GET deployed artifact versions and runtime status (read-only)
- Cloud Integration monitoring: GET message processing log summary, active queues, error rates
- API Management: GET API provider configuration (read-only)

**Allowed** (step 9 only — diff):
- Cloud Integration Design API: GET iFlow content for diff comparison against target tenant state (read-only)

**Allowed** (step 14 only — after all prior steps confirmed):
- Cloud Integration OData API: POST deploy for approved artifacts
- Cloud Integration Design API: deploy approved package version to target tenant
- Cloud Integration API: undeploy command (only when change type from step 1 is undeploy and explicitly approved)

**Forbidden** (all other contexts):
- Any deploy or undeploy command before step 13 approval gate is cleared
- Any deployment of artifacts not on the approved list from step 7
- Any modification to tenant-level runtime configuration (JDBC data sources, keystore entries, security material) unless explicitly in scope and approved
- Any deletion of integration packages or artifacts outside the approved change type
- Any access with Integration Suite tenant administrator credentials not scoped to the specific package or iFlow

**Least-privilege credential rules**:
- Cloud Integration OData API: use a service key with `MonitoringDataRead` and `WorkspacePackagesRead` for step 8; use `WorkspacePackagesConfigure` and `NodeManager.deployer` for step 14 — scoped to the specific tenant
- Integration Suite: use a user with the Integration Developer role for deployment; not the Integration Suite Tenant Administrator role
- API Management: use a user with the APIPortal.Administrator role scoped to the specific API product — not the full Platform scope

**Approval gate (step 13)**:
Before step 14 execution, confirm:
1. All steps 1–12 are documented and evidence is on record.
2. The integration owner from step 5 has provided explicit written approval in this session.
3. The integration owner is a different person from the requester.
4. The approval explicitly names the artifacts and version numbers to be deployed.
5. A diff from step 9 is on record.

**Rollback**:
- The rollback plan from step 11 must be documented before step 14 begins.
- If post-deployment verification (step 15) reveals errors or message processing failures, stop immediately and execute the rollback plan (redeploy previous version) before escalating to the integration owner.
- Do not attempt self-recovery deployments without a new 17-step sequence.

**Post-deployment (step 15)**:
- Check the runtime status of all deployed iFlows (started / error / stopping).
- Inspect the message processing log for ERROR and RETRY status messages.
- Confirm throughput resumes within the expected processing rate.
- Where downstream partner acknowledgement is part of the integration pattern, confirm receipt.
- Document the verification result as `live evidence`.

**Audit (step 16)**:
- Produce a complete record of all 17 steps, evidence gathered, commands executed, timestamps, integration-owner approval statement, and message monitoring verification results.
- Retain audit record for the session and deliver to the integration owner and change manager.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — 17-step execution details, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, SoD rules, refusal triggers.
- [Official sources](references/official-sources.md) — SAP Cloud Integration, Integration Suite, OData API, message monitoring docs.
- [Live environment access](references/live-environment-access.md) — credential setup, role requirements, audit log format.

## Response minimum

Return, at minimum:

- **Problem classification**: artifact type, change type, target tenant, and environment tier.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: always risk-critical for non-development tenants; critical for financial document or payment processing iFlows.
- **Recommended action**: current step in the 17-step sequence and what is needed to advance.
- **Refusal / escalation triggers**: refuse if approval gate not cleared; refuse if SoD violated; refuse if integration-owner approval is absent; refuse if diff has not been produced; refuse if any step is missing; refuse if rollback plan is absent.
- **Business impact**: what message flows change if the deployment succeeds; what downstream partners are affected if the deployment fails.
- **Next verification step**: which step number is next and what evidence is required.
