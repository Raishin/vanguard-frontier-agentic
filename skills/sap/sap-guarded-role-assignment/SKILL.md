---
name: sap-guarded-role-assignment
description: Assign or revoke SAP role collections and authorizations using a mandatory 17-step guarded mutation sequence covering classification, target tenant/system confirmation, criticality, requester, approver, ticket, scope, read-only current state, SoD pre-check, diff of effective permissions, blast radius, rollback plan, approval gate, execute approved changes only, verify, audit, and report. Refuses if any step is missing, if an SoD conflict is detected, or if self-approval is attempted. Use when a confirmed and approved role assignment or revocation in a non-development SAP system must be executed under traceable controls.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: security
  lifecycle: experimental
  execution_tier: mutating-runtime
---

# SAP Guarded Role Assignment

## Purpose

Assign or revoke SAP role collections and authorizations in non-development systems under a strict 17-step guarded mutation sequence. Every step must be completed, documented, and confirmed before the execution step is reached. If any step is absent, the assignment or revocation is refused until it is completed. This skill treats every role assignment or revocation in a quality, pre-production, or production SAP system as a risk-critical mutating operation requiring explicit approval, a mandatory SoD pre-check, and a documented rollback path.

SoD violations are a hard stop: if the proposed assignment would create a Segregation of Duties conflict, the assignment is refused regardless of who approves it. No role is granted for convenience, expediency, or "temporary access" without completing the full sequence.

## When to use

Use this skill when:

- a user, service account, or group must be granted or have revoked one or more SAP role collections, composite roles, or single authorization objects in a quality, pre-production, or production system,
- a security owner or compliance officer has confirmed the change is approved and requires traceable execution,
- post-change verification and audit evidence are required for compliance, audit, or access review purposes,
- the user needs a structured, risk-controlled execution protocol for SAP BTP role collections, S/4HANA PFCG roles, or cloud-service authorization management.

## When not to use

- When the target is a development system (DEV) with no business data — development role assignments do not require this full sequence, though SoD checks remain good practice.
- When the user wants to *review* or *audit* existing role assignments without making changes — advise on the sequence but do not reach the execution step.
- When any of the 17 steps cannot be completed — refuse execution until all steps are resolved.
- When no explicit approval from an authorized approver has been obtained — refuse execution.
- When the proposed assignment would create an SoD conflict — refuse execution entirely; do not proceed even with approval.

## Lean operating rules

- 17 steps, no shortcuts. The sequence is mandatory and must be completed in order. Steps cannot be skipped or reordered.
- Refuse before step 13. Never reach the execution step (step 14) without a documented approval from an authorized approver on record in this session.
- SoD pre-check is mandatory. Step 9 (SoD pre-check / diff of effective permissions) must be completed before any assignment is proposed. An assignment that creates an SoD conflict is refused unconditionally — approval does not override an SoD violation.
- Segregation of duties enforced. The person who requests the role change (step 4) must not be the same person who approves it (step 12). If SoD is violated, refuse execution.
- Read current state before any change. Step 8 (read-only current state) must be completed before any assignment is proposed. Do not propose a change on undocumented current access state.
- Diff effective permissions before live execution. Step 9 must include a diff of effective permissions the user will hold after the change. Undocumented net-new permissions are a refusal trigger.
- Blast radius documented before execution. Step 10 must list all business processes, transactions, and data objects that will become accessible. Assignments with undocumented blast radius are refused.
- One user/group change at a time. Do not batch-assign roles to multiple users in a single execution step unless each has completed its own full 17-step sequence.
- Never hardcode credentials. All system access uses pre-configured service users, API service keys, or SSO tokens — never credentials in plain text.
- Evidence labeling mandatory. Every step must produce labeled evidence before the next step proceeds.
- Never grant broad admin for convenience. Never assign SAP_ALL, Administrator role collections, or unrestricted authorization profiles as a workaround for missing fine-grained roles.

## The 17-step guarded mutation sequence

| Step | Name | Description | Evidence required |
|------|------|-------------|-------------------|
| 1 | Classify | Classify the role change type (assign / revoke), role type (BTP role collection, PFCG composite role, single authorization object, cloud service entitlement), and scope (user, group, service account) | Role change type, role type, and scope documented |
| 2 | Confirm target | Confirm the exact target system (BTP subaccount/tenant, S/4HANA SID + client, cloud service instance, environment tier: QA/PPD/PRD) | Target system, subaccount/client, and tier confirmed by user |
| 3 | Criticality assessment | Assess business criticality of the roles being assigned or revoked (financial posting, payment execution, payroll, system administration, integration platform access, compliance-relevant data) | Criticality level: low/medium/high/critical |
| 4 | Requester confirmation | Confirm the name, role, and authorization of the change requester | Requester name and role documented |
| 5 | Approver identification | Identify the authorized approver (security owner or compliance officer — different person from requester — SoD requirement) | Approver name, role, and authorization level |
| 6 | Ticket linkage | Link the role change to a change management or access request ticket (ITSM reference: ServiceNow, Remedy, SAP Cloud Identity Access Governance, GRC access request, etc.) | Ticket number confirmed |
| 7 | Scope documentation | Document the complete list of roles, role collections, or authorization objects to be assigned or revoked, including the target user(s) or group(s) and the validity period | Role list with descriptions, target user/group, and validity period |
| 8 | Read-only current state | Read the current role assignments for the target user(s) or group(s): current role collections, assigned PFCG roles, active authorization objects, last access review date | Live evidence from target system (read-only) |
| 9 | SoD pre-check and diff | Run a Segregation of Duties pre-check against the proposed assignment; produce a diff of effective permissions the user will hold after the change; document any SoD conflicts found — refuse if any conflict exists | SoD pre-check result (pass/fail) and effective-permission diff |
| 10 | Blast radius | Document all transactions, business objects, data sets, and business processes that will become accessible (for assignment) or inaccessible (for revocation) after the change | Blast radius document with affected access scope |
| 11 | Rollback plan | Document the explicit rollback procedure: which roles to revoke or reassign, validity-period reversion, or restore-from-snapshot path | Rollback procedure documented and confirmed feasible |
| 12 | SoD verification | Verify that requester (step 4) and approver (step 5) are different authorized individuals; confirm no self-approval; confirm SoD pre-check from step 9 returned no conflicts | SoD confirmed: requester ≠ approver; SoD pre-check passed |
| 13 | Approval gate | Obtain explicit written approval from the authorized approver for this specific role assignment or revocation | Approval statement on record in this session |
| 14 | Execute approved change | Execute the role assignment or revocation for approved roles only, for approved users or groups only, within the documented validity period | Execution log with timestamp |
| 15 | Verify | Verify post-change state: confirm role assignments are active, run effective-permission check, confirm business process access matches expectations, confirm no unintended SoD conflicts introduced | Post-change log and verification results |
| 16 | Audit | Produce complete audit record: all 17 steps, evidence, commands, timestamps, approvals, SoD pre-check result | Audit record |
| 17 | Report | Deliver final report to requester, approver, security owner, and compliance team | Report delivered and acknowledged |

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live SAP system in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs
- `user-provided evidence` — stated or supplied by the user in this session
- `inference` — derived reasoning not directly confirmed by live data or official docs; must always be labeled

## Live-environment rules

**Allowed** (step 8 only — read-only current state):
- BTP cockpit or BTP APIs: GET role collection assignments for user or group (read-only)
- S/4HANA PFCG display (transaction PFCG in display mode, SU01 display)
- SAP Cloud Identity Services (IAS/IPS) GET user role assignments
- SAP GRC / Cloud Identity Access Governance: GET current access assignments and SoD risk report

**Allowed** (step 9 only — SoD pre-check and diff):
- SAP GRC Access Control or Cloud Identity Access Governance: SoD risk simulation for proposed roles
- BTP role collection effective-permission diff (read-only API calls)
- PFCG authorization trace (read-only simulation mode)

**Allowed** (step 14 only — after all prior steps confirmed):
- BTP cockpit or BTP API: POST/PUT role collection assignment for approved users or groups
- S/4HANA SU01 or PFCG: role assignment execution for approved users
- SAP Cloud Identity Services: group or role assignment for approved service accounts

**Forbidden** (all other contexts):
- Any assignment or revocation command before step 13 approval gate is cleared
- Any assignment of roles not on the approved list from step 7
- Any assignment that produces an SoD conflict identified in step 9, regardless of approval
- Any grant of SAP_ALL, Administrator role collections, or unrestricted authorization profiles
- Any modification to role definitions or authorization object values (this skill covers assignment only, not role design)
- Any self-approval (requester = approver)
- Any system access with basis administrator or tenant administrator credentials not scoped to role assignment only

**Least-privilege credential rules**:
- BTP: use a user with User and Role Administrator role collection scoped to the specific subaccount only; not Global Account Administrator
- S/4HANA: use a user with authorization for SU01 (user maintenance) and PFCG (role assignment) scoped to the target client only; not SAP_ALL or BASIS profile
- SAP Cloud Identity Services (IAS/IPS): use a service account with User Administrator scope for the specific tenant; not Tenant Administrator with system configuration rights
- SAP GRC / Cloud IAG: use a risk analyst or access request processor role scoped to the relevant business unit; not GRC system administrator

**Approval gate (step 13)**:
Before step 14 execution, confirm:
1. All steps 1–12 are documented and evidence is on record.
2. The approver from step 5 has provided explicit written approval in this session.
3. The approver is a different person from the requester.
4. The SoD pre-check from step 9 returned no conflicts.
5. The approval explicitly names the roles and the target user(s) or group(s).

**Rollback**:
- The rollback plan from step 11 must be documented before step 14 begins.
- If post-change verification (step 15) reveals unintended access or SoD conflicts, stop immediately and execute the rollback plan before escalating to the approver.
- Do not attempt self-recovery without a new 17-step sequence.

**Post-change (step 15)**:
- Run an effective-permission check for the target user after the assignment.
- Confirm the user can access only the intended transactions and business objects.
- Run a second SoD check to confirm no conflicts were introduced.
- Document the verification result as `live evidence`.

**Audit (step 16)**:
- Produce a complete record of all 17 steps, evidence gathered, commands executed, timestamps, SoD pre-check result, approval statement, and verification results.
- Retain audit record for the session and deliver to the approver, security owner, and compliance team.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — 17-step execution details, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, SoD rules, refusal triggers.
- [Official sources](references/official-sources.md) — SAP BTP role collections, PFCG, IAS/IPS, GRC docs.
- [Live environment access](references/live-environment-access.md) — credential setup, role requirements, audit log format.

## Response minimum

Return, at minimum:

- **Problem classification**: role change type (assign/revoke), role type, target system, and environment tier.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: always risk-critical for non-development targets; critical for any financial, payroll, or system-administration role.
- **Recommended action**: current step in the 17-step sequence and what is needed to advance.
- **Refusal / escalation triggers**: refuse if approval gate not cleared; refuse if SoD violation detected (step 9 or step 12); refuse if self-approval attempted; refuse if any step is missing; refuse if rollback plan is absent; refuse if broad admin role requested.
- **Business impact**: what access changes if the assignment succeeds; what business processes are affected if the revocation is unintended.
- **Next verification step**: which step number is next and what evidence is required.
