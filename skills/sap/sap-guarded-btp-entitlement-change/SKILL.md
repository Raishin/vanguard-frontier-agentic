---
name: sap-guarded-btp-entitlement-change
description: Change SAP BTP entitlements, quotas, and service plan subscriptions using a mandatory 17-step guarded mutation sequence covering classification, target global account and subaccount confirmation, criticality, requester, approver (platform-owner plus FinOps required), ticket, scope, read-only current state, diff of entitlement changes, blast radius (cost and dependent applications), rollback, post-change verification, and audit evidence. Refuses if any step is missing, if FinOps approval is absent for entitlement increases, or if a cost-impact assessment has not been produced. Use when a confirmed and approved BTP entitlement, quota, or subscription change must be executed under traceable controls.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: platform
  lifecycle: experimental
  execution_tier: mutating-runtime
---

# SAP Guarded BTP Entitlement Change

## Purpose

Change SAP BTP entitlements, service plan quotas, and service subscriptions in global accounts and subaccounts under a strict 17-step guarded mutation sequence. Every step must be completed, documented, and confirmed before the execution step is reached. If any step is absent, the entitlement change is refused until it is completed. This skill treats every BTP entitlement increase, quota adjustment, or service subscription change as a risk-critical mutating operation requiring dual approval from both the platform owner and FinOps, a documented cost-impact assessment, and a verified rollback path.

Entitlement increases that add billable consumption are never approved on the basis of technical necessity alone — FinOps approval is a hard gate.

## When to use

Use this skill when:

- a BTP service plan entitlement must be assigned, increased, or removed in a global account or subaccount,
- a service plan quota must be adjusted for a specific subaccount,
- a service subscription (a service plan enabled for a subaccount) must be added or removed,
- a platform owner and FinOps approver have confirmed the change is approved and requires traceable execution,
- post-change verification and audit evidence are required for cost governance, compliance, or capacity planning purposes,
- the user needs a structured, risk-controlled execution protocol for SAP BTP cockpit or BTP Entitlements API operations.

## When not to use

- When the change targets a trial or free-tier account with no billable consumption — trial accounts are lower risk, but FinOps review is still recommended for resource planning.
- When the user wants to *view* or *audit* current entitlements without making changes — advise on the sequence but do not reach the execution step.
- When any of the 17 steps cannot be completed — refuse execution until all steps are resolved.
- When FinOps approval has not been obtained for any entitlement increase — refuse execution.
- When a cost-impact assessment (part of step 9 diff) has not been produced — refuse execution.

## Lean operating rules

- 17 steps, no shortcuts. The sequence is mandatory and must be completed in order. Steps cannot be skipped or reordered.
- Refuse before step 13. Never reach the execution step (step 14) without documented dual approval (platform owner + FinOps) on record in this session.
- FinOps approval is mandatory for all entitlement increases. Any change that adds billable capacity or enables a paid service plan requires explicit FinOps approval documented in step 13. Technical justification alone does not clear this gate.
- Segregation of duties enforced. The person who requests the entitlement change (step 4) must not be the same person who approves it (step 12). If SoD is violated, refuse execution.
- Read current state before any change. Step 8 (read-only current state) must be completed before any entitlement change is proposed. Do not propose changes on undocumented current entitlement state.
- Cost-impact assessment required. Step 9 (diff) must include a documented cost delta — the estimated additional monthly cost from the proposed entitlement increase, based on current pricing. Changes with undocumented cost impact are refused.
- Blast radius documented before execution. Step 10 must list all dependent applications and services that will be affected if the entitlement change succeeds or fails. Changes with undocumented blast radius are refused.
- One subaccount change at a time. Do not batch-change entitlements across multiple subaccounts in a single execution step unless each has completed its own full 17-step sequence.
- Never hardcode credentials. All system access uses pre-configured service accounts, BTP cockpit SSO, or BTP APIs with scoped OAuth tokens — never credentials in plain text.
- Evidence labeling mandatory. Every step must produce labeled evidence before the next step proceeds.

## The 17-step guarded mutation sequence

| Step | Name | Description | Evidence required |
|------|------|-------------|-------------------|
| 1 | Classify | Classify the change type (assign entitlement, increase quota, decrease quota, remove entitlement, add subscription, remove subscription), the service name, service plan, and billing model (free / metered / subscription / block) | Change type, service name, service plan, and billing model documented |
| 2 | Confirm target | Confirm the exact target global account name/ID, subaccount name/ID, and environment (production global account or non-production) | Target global account ID, subaccount ID, and environment tier confirmed by user |
| 3 | Criticality assessment | Assess business criticality of the affected service (production application dependency, compliance-required service, revenue-critical integration) and cost sensitivity (metered billing scale risk) | Criticality level: low/medium/high/critical; cost sensitivity: low/medium/high |
| 4 | Requester confirmation | Confirm the name, role, and authorization of the change requester | Requester name and role documented |
| 5 | Dual approver identification | Identify both the platform owner (accountable for the global account architecture) and the FinOps approver (accountable for BTP cost governance) — both must be different from the requester | Platform owner name and role; FinOps approver name and role |
| 6 | Ticket linkage | Link the change to a change management or capacity planning ticket (ITSM or FinOps budget approval reference) | Ticket number confirmed |
| 7 | Scope documentation | Document the complete list of entitlement changes: service names, service plans, quota amounts (from → to), and target subaccounts | Entitlement change list with service, plan, quota delta, and subaccount |
| 8 | Read-only current state | Read the current entitlement state of the global account and target subaccount: assigned entitlements, current quota allocations, active subscriptions, current metered consumption | Live evidence from BTP cockpit or Entitlements API (read-only) |
| 9 | Diff and cost-impact assessment | Produce a diff of entitlement changes (quota before and after per service plan); document the estimated monthly cost delta based on SAP BTP pricing for the billing model; identify any entitlements being removed that have active consumers | Diff with quota delta and cost-impact assessment (monthly cost delta) |
| 10 | Blast radius | Document all applications, services, and integrations that depend on the affected entitlements; assess risk of service disruption for quota decreases or subscription removals; assess cost overrun risk for quota increases | Blast radius document with dependent applications, disruption risk, and cost risk |
| 11 | Rollback plan | Document the explicit rollback procedure: which entitlements to restore, which subscriptions to re-enable, and how to revert quota allocations via the Entitlements API or cockpit | Rollback procedure documented and confirmed feasible |
| 12 | SoD verification | Verify that requester (step 4) is different from both the platform owner and FinOps approver (step 5); confirm no self-approval by any of the three parties | SoD confirmed: requester ≠ platform owner ≠ FinOps approver |
| 13 | Dual approval gate | Obtain explicit written approval from both the platform owner and the FinOps approver for this specific entitlement change, naming the services, plans, and quota amounts | Dual approval statements on record in this session |
| 14 | Execute approved change | Execute the entitlement assignments, quota adjustments, or subscription changes for approved services only, in the documented sequence | Execution log with timestamp |
| 15 | Verify | Verify post-change state: confirm entitlements are active in the target subaccount, confirm dependent applications are functioning, confirm metered consumption tracking is active, check for any unexpected billing events | Post-change entitlement snapshot and application health verification |
| 16 | Audit | Produce complete audit record: all 17 steps, evidence, commands, timestamps, dual approval statements, cost-impact assessment | Audit record |
| 17 | Report | Deliver final report to requester, platform owner, FinOps approver, and cost center owner | Report delivered and acknowledged |

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live BTP global account or subaccount in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs or SAP BTP pricing documentation
- `user-provided evidence` — stated or supplied by the user in this session
- `inference` — derived reasoning not directly confirmed by live data or official docs; must always be labeled

## Live-environment rules

**Allowed** (step 8 only — read-only current state):
- BTP Entitlements API: GET entitlements for global account and subaccount (read-only)
- BTP cockpit: view current entitlement assignments, quota allocations, and active subscriptions (read-only)
- BTP Usage API: GET current metered consumption for context

**Allowed** (step 9 only — diff and cost-impact):
- BTP Entitlements API: GET entitlement catalog for pricing metadata (read-only)
- BTP cockpit: entitlement comparison view (read-only)

**Allowed** (step 14 only — after all prior steps confirmed):
- BTP Entitlements API: PUT/PATCH to assign or adjust entitlements for approved services and quota amounts
- BTP cockpit: entitlement assignment and quota editing for approved services only
- BTP Subscription API: POST to enable or DELETE to remove approved service subscriptions

**Forbidden** (all other contexts):
- Any entitlement assignment, quota change, or subscription modification before step 13 dual approval gate is cleared
- Any change to services not on the approved list from step 7
- Any entitlement increase without FinOps approval, regardless of other approvals
- Any global account configuration changes (trust configuration, directory structure, subaccount creation) — this skill covers entitlement and subscription management only
- Any access with global account administrator credentials for step 14 if a scoped entitlement manager role is available

**Least-privilege credential rules**:
- BTP Entitlements API: use a service account with the `Entitlements Administrator` role collection scoped to the specific global account; not Global Account Administrator
- BTP cockpit: use a user with the `Global Account Viewer` role for steps 8–13 (read-only) and `Entitlements Administrator` role for step 14 only
- BTP Subscription API: use a service account with `Subaccount Service Administrator` scoped to the specific subaccount; not Global Account Administrator

**Approval gate (step 13)**:
Before step 14 execution, confirm:
1. All steps 1–12 are documented and evidence is on record.
2. The platform owner from step 5 has provided explicit written approval in this session.
3. The FinOps approver from step 5 has provided explicit written approval in this session.
4. Both approvers are different from the requester.
5. The cost-impact assessment from step 9 is on record.
6. The approval explicitly names the services, service plans, and quota amounts to be changed.

**Rollback**:
- The rollback plan from step 11 must be documented before step 14 begins.
- If post-change verification (step 15) reveals application disruption or unexpected billing events, stop immediately and execute the rollback plan before escalating to the platform owner and FinOps.
- Do not attempt self-recovery changes without a new 17-step sequence.

**Post-change (step 15)**:
- Retrieve entitlement snapshot for the target subaccount and confirm assigned services match the approved change list.
- Confirm dependent applications are running and have not lost access to required services.
- For metered billing services, confirm that consumption tracking is active and reporting to the expected cost center.
- Document the verification result as `live evidence`.

**Audit (step 16)**:
- Produce a complete record of all 17 steps, evidence gathered, commands executed, timestamps, dual approval statements, cost-impact assessment, and post-change verification results.
- Retain audit record for the session and deliver to the platform owner, FinOps approver, and cost center owner.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — 17-step execution details, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, SoD rules, refusal triggers.
- [Official sources](references/official-sources.md) — SAP BTP entitlements, quotas, subscriptions, Entitlements API docs.
- [Live environment access](references/live-environment-access.md) — credential setup, role requirements, audit log format.

## Response minimum

Return, at minimum:

- **Problem classification**: change type, service name, service plan, billing model, and target global account / subaccount.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: always risk-critical for production global accounts; critical for any metered billing service where quota increase carries significant cost exposure.
- **Recommended action**: current step in the 17-step sequence and what is needed to advance.
- **Refusal / escalation triggers**: refuse if dual approval gate (platform owner + FinOps) not cleared; refuse if FinOps approval absent for any entitlement increase; refuse if SoD violated; refuse if cost-impact assessment absent; refuse if any step is missing; refuse if rollback plan is absent.
- **Business impact**: what changes in service availability and cost if the entitlement change succeeds; what applications are disrupted if the change is rolled back.
- **Next verification step**: which step number is next and what evidence is required.
