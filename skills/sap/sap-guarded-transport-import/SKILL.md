---
name: sap-guarded-transport-import
description: Execute SAP transport request imports using a mandatory 17-step guarded mutation sequence covering classification, target confirmation, criticality, requester, approver, ticket, scope, read-only current state, diff and dry-run, blast radius, rollback plan, segregation of duties, approval gate, execute approved imports only, verify, audit, and report. Refuse if any step is missing. Use when a confirmed and approved transport import into a non-development SAP system (quality, pre-production, or production) must be executed under traceable controls.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: delivery
  lifecycle: experimental
  execution_tier: mutating-runtime
---

# SAP Guarded Transport Import

## Purpose

Execute SAP transport request imports into non-development systems under a strict 17-step guarded mutation sequence. Every step must be completed, documented, and confirmed before the import execution step is reached. If any step is absent, the import is refused until it is completed. This skill treats every transport import into a quality, pre-production, or production system as a risk-critical mutating operation requiring explicit approval and a documented rollback path.

## When to use

Use this skill when:

- a transport request (TR) must be imported into a quality, pre-production, or production SAP system,
- a change manager or project lead has confirmed the import is approved and requires traceable execution,
- post-import verification and audit evidence are required for change management or compliance purposes,
- the user needs a structured, risk-controlled execution protocol for SAP CTS+, STMS, or cloud transport management imports.

## When not to use

- When the import target is a development system (DEV) — development imports are lower risk and do not require this full sequence.
- When the user wants to *plan* a transport without executing — advise on the sequence but do not reach the execution step.
- When any of the 17 steps cannot be completed — refuse execution until all steps are resolved.
- When no explicit approval from an authorized approver has been obtained — refuse execution.

## Lean operating rules

- 17 steps, no shortcuts. The sequence is mandatory and must be completed in order. Steps cannot be skipped or reordered.
- Refuse before step 13. Never reach the execution step (step 14) without a documented approval from an authorized approver on record in this session.
- Segregation of duties enforced. The person who requests the transport (step 5) must not be the same person who approves it (step 12). If SoD is violated, refuse execution.
- Read current state before any change. Step 8 (read-only current state) must be completed before any import is proposed. Do not propose an import on undocumented target state.
- Dry-run before live execution. Step 9 (diff/dry-run) must precede execution. If the SAP system does not support dry-run for the transport type, document the reason and require enhanced approval.
- Blast radius documented before execution. Step 10 (blast radius) must list all objects, programs, and business processes potentially affected. Imports with undocumented blast radius are refused.
- One transport at a time. Do not batch-import multiple transports in a single execution step unless each has completed its own full 17-step sequence.
- Never hardcode credentials. All system access uses pre-configured RFC destinations, STMS users, or CTS+ API service keys — never credentials in plain text.
- Evidence labeling mandatory. Every step must produce labeled evidence before the next step proceeds.

## The 17-step guarded mutation sequence

| Step | Name | Description | Evidence required |
|------|------|-------------|-------------------|
| 1 | Classify | Classify the transport type (workbench, customizing, Support Package, cloud transport) and content category (ABAP, Fiori, configuration, master data) | Transport type + content category documented |
| 2 | Confirm target | Confirm the exact target system (SID, client, environment tier: QA/PPD/PRD) | Target SID, client, and tier confirmed by user |
| 3 | Criticality assessment | Assess business criticality of the transported objects (financial postings, payroll, integration interfaces, core customizing) | Criticality level: low/medium/high/critical |
| 4 | Requester confirmation | Confirm the name, role, and authorization of the change requester | Requester name and role documented |
| 5 | Approver identification | Identify the authorized approver (different person from requester — SoD requirement) | Approver name, role, and authorization level |
| 6 | Ticket linkage | Link the import to a change management ticket (ITSM reference: ServiceNow, Remedy, SAP Solution Manager, etc.) | Ticket number confirmed |
| 7 | Scope documentation | Document the complete list of transport requests in the import queue, their descriptions, and interdependencies | TR list with descriptions and sequence |
| 8 | Read-only current state | Read the current state of the target system: transport queue, import queue status, active objects, last import log | Live evidence from target system (read-only) |
| 9 | Diff and dry-run | Perform a diff of transported objects against current target system content; run dry-run import if system supports it | Diff output or documented dry-run result |
| 10 | Blast radius | Document all ABAP programs, configuration entries, Fiori apps, interfaces, and business processes potentially affected | Blast radius document with affected components |
| 11 | Rollback plan | Document the explicit rollback procedure: which transports to re-import, configuration steps, or system restore path | Rollback procedure documented and confirmed feasible |
| 12 | SoD verification | Verify that requester (step 4) and approver (step 5) are different authorized individuals; confirm no self-approval | SoD confirmed: requester ≠ approver |
| 13 | Approval gate | Obtain explicit written approval from the authorized approver for this specific import sequence | Approval statement on record in this session |
| 14 | Execute approved import | Execute the import for approved transport requests only, in the documented sequence | Import command log with timestamp |
| 15 | Verify | Verify post-import system state: check import log for errors, run smoke tests, confirm business process availability | Post-import log and verification results |
| 16 | Audit | Produce complete audit record: all 17 steps, evidence, commands, timestamps, approvals | Audit record |
| 17 | Report | Deliver final report to requester, approver, and change manager | Report delivered and acknowledged |

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live SAP system in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs
- `user-provided evidence` — stated or supplied by the user in this session
- `inference` — derived reasoning not directly confirmed by live data or official docs; must always be labeled

## Live-environment rules

**Allowed** (step 8 only — read-only current state):
- STMS display transactions (read-only transport queue inspection)
- CTS+ API GET calls to read import queue status
- SAP Cloud Transport Management Service GET API calls
- Read-only import log inspection

**Allowed** (step 14 only — after all prior steps confirmed):
- STMS import execution for approved transport requests
- CTS+ API import trigger for approved requests
- SAP Cloud Transport Management Service import trigger for approved requests

**Forbidden** (all other contexts):
- Any import command before step 13 approval gate is cleared
- Any import of a transport not on the approved TR list from step 7
- Any modification to transport route configuration
- Any deletion of transport requests from the queue without explicit user instruction
- Any ABAP system access with developer-level credentials (use transport administrator role only)

**Least-privilege credential rules**:
- STMS: use a user with `S_TRANSPRT` import authorization for the specific target system only; not BASIS administrator
- CTS+: use a service user with CTS+ transport manager role scoped to the specific transport route
- SAP Cloud TMS: use a Transport Operator role scoped to the specific transport node; not Transport Administrator with delete rights

**Approval gate (step 13)**:
Before step 14 execution, confirm:
1. All steps 1–12 are documented and evidence is on record.
2. The approver from step 5 has provided explicit written approval in this session.
3. The approver is a different person from the requester.
4. The approval explicitly names the transport requests to be imported.

**Rollback**:
- The rollback plan from step 11 must be documented before step 14 begins.
- If post-import verification (step 15) reveals errors, stop immediately and escalate to the approver before any further action.
- Do not attempt self-recovery imports without a new 17-step sequence.

**Post-import (step 15)**:
- Read the import log for all ERROR, WARNING, and ABAP short dump entries.
- Run the smoke test or business process check specified in the blast radius document.
- Document the verification result as `live evidence`.

**Audit (step 16)**:
- Produce a complete record of all 17 steps, evidence gathered, commands executed, timestamps, approval statement, and verification results.
- Retain audit record for the session and deliver to the approver and change manager.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — 17-step execution details, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, SoD rules, refusal triggers.
- [Official sources](references/official-sources.md) — SAP TMS, CTS+, Cloud TMS, STMS docs.
- [Live environment access](references/live-environment-access.md) — credential setup, STMS/CTS+ role requirements, audit log format.

## Response minimum

Return, at minimum:

- **Problem classification**: transport type, content category, and target environment tier.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: always risk-critical for non-development targets.
- **Recommended action**: current step in the 17-step sequence and what is needed to advance.
- **Refusal / escalation triggers**: refuse if approval gate not cleared; refuse if SoD violated; refuse if any step is missing; refuse if rollback plan is absent.
- **Business impact**: what changes if the import succeeds; what breaks if it fails.
- **Next verification step**: which step number is next and what evidence is required.
