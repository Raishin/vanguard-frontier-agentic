# Audit Evidence Mapping Protocol — Detailed Workflow and Output Contract

## Overview

This document provides the step-by-step workflow, decision tree, and output contract for the `audit-evidence-mapping-protocol` skill. It is intended for orchestrating agents and human compliance owners who need to understand how evidence mapping proceeds, where gates fire, and what the deliverable looks like.

---

## Detailed Workflow

### Phase 1 — Scope and Ingest

**Step 1.1 — Receive control framework**
- Input: control IDs, framework name (ISO 27001, SOC 2 Type II, NIST CSF, internal), and control description
- Action: normalize control IDs to a canonical internal format
- Output: `control_register[]` with fields: `control_id`, `control_description`, `workload_hint`, `evidence_type_hint`

**Step 1.2 — Confirm workload scope**
- Input: list of Microsoft 365 services and Dynamics 365 / Power Platform environments in scope
- Check: confirm Microsoft Purview licensing tier for each workload
  - Audit Standard: 180-day default retention (as of October 17, 2023)
  - Audit Premium (E5 or Purview Suite): 1-year default retention for Entra ID, Exchange, SharePoint, OneDrive
  - Audit Premium + 10-year add-on: up to 10-year retention with explicit policy
- Output: `workload_scope[]` with `service`, `environment_id`, `audit_tier`, `default_retention_days`

**Step 1.3 — Load retention policy inventory**
- Source: Microsoft Purview Audit log retention policies (exported summary, not live query requiring credentials)
- Action: match each workload to its applicable custom retention policy; fall back to licensing default
- Output: `retention_policy_map[]` with `workload`, `policy_name`, `retention_days`, `priority`

---

### Phase 2 — Evidence Discovery

**Step 2.1 — Map controls to evidence artifacts**
For each control in `control_register[]`:
- Identify the primary evidence type:
  - Audit log entry (Purview Unified Audit Log)
  - Configuration export (Power Platform DLP policy, Entra ID Conditional Access policy)
  - Access review record (Entra ID Access Reviews)
  - Policy document or runbook (SharePoint)
  - Dataverse / Dynamics 365 audit trail entry
- Record: `evidence_artifact[]` with `control_id`, `artifact_type`, `source_workload`, `location_hint`, `expected_retention_required`

**Step 2.2 — Verify evidence availability**
- For each artifact: confirm whether the artifact exists within the retention window
- Flag: `evidence_status` = `complete` | `partial` | `missing` | `retention_expired`

**Step 2.3 — Check legal-hold coverage**
- If a legal-hold case reference is provided:
  - Confirm that each in-scope artifact is covered by an active eDiscovery hold in Microsoft Purview
  - Flag uncovered artifacts as `hold_status` = `covered` | `not_covered` | `unknown`
- If no legal-hold case reference: record `hold_status` = `not_applicable`

---

### Phase 3 — Gap Analysis and Escalation Gates

**Step 3.1 — Build gap register**
- Populate `gap_register[]` with every artifact where:
  - `evidence_status` ≠ `complete`, OR
  - `retention_days` < `expected_retention_required`, OR
  - `hold_status` = `not_covered` (when a hold case is active)

**Gate A — Evidence Completeness Gate**
```
IF gap_register[] is non-empty:
  → PAUSE workflow
  → Escalate to: compliance owner + m365-maestro-agent
  → Action required: remediate gaps before audit window opens
  → Do NOT assemble attestation package until gaps are resolved or explicitly accepted with risk note
```

**Gate B — Retention / Legal-Hold Deficiency Gate**
```
IF any artifact has retention_days < expected_retention_required
OR any artifact has hold_status = not_covered AND legal hold is active:
  → STOP workflow
  → Escalate to: Purview compliance administrator + legal owner
  → Action required: place legal hold or extend retention policy
  → Do NOT proceed until human confirmation that deficiency is resolved or formally accepted
```

**Gate C — Privilege Sensitivity Gate**
```
IF any artifact is flagged as potentially privileged (attorney-client, work product):
  → Flag artifact as privilege_sensitivity = HIGH
  → Route to legal counsel before including in attestation package
  → Do NOT transmit privileged artifact to external auditor without counsel sign-off
```

**Gate D — Special-Category Personal Data Gate**
```
IF any artifact contains health, biometric, racial/ethnic origin, or other special-category data:
  → STOP
  → Confirm jurisdiction and privacy owner
  → Do NOT proceed until jurisdiction confirmed and privacy owner engaged
```

---

### Phase 4 — Attestation Package Assembly

**Step 4.1 — Compile attestation records**
For each control:
```
{
  "control_id": "<framework>-<id>",
  "control_description": "...",
  "evidence_artifact_ref": "<location or log reference>",
  "source_workload": "Exchange Online | SharePoint | Entra ID | Dataverse | ...",
  "retention_expiry": "<ISO date>",
  "legal_hold_status": "covered | not_covered | not_applicable",
  "evidence_quality": "complete | partial | missing",
  "privilege_sensitivity": "HIGH | STANDARD",
  "privacy_sensitivity": "HIGH | STANDARD",
  "open_questions": ["..."],
  "gap_notes": "..."
}
```

**Step 4.2 — Assemble package header**
```
{
  "attestation_package_id": "<uuid>",
  "skill_id": "audit-evidence-mapping-protocol",
  "skill_version": "0.1.0",
  "invoked_by": "<agent or human id>",
  "audit_window": { "start": "<date>", "end": "<date>" },
  "control_framework": "<name>",
  "workloads_in_scope": ["..."],
  "total_controls": <n>,
  "complete_evidence": <n>,
  "partial_evidence": <n>,
  "missing_evidence": <n>,
  "retention_gaps": <n>,
  "hold_gaps": <n>,
  "escalations_fired": ["Gate A", "Gate B", ...],
  "package_status": "ready_for_review | blocked_pending_escalation",
  "timestamp": "<ISO datetime>"
}
```

**Step 4.3 — Attach do-not-do list**
Every attestation package includes:
- Do not transmit evidence to an external auditor or regulator without human compliance owner sign-off.
- Do not modify retention policies, legal holds, or eDiscovery cases to close gaps without Purview admin and legal owner approval.
- Do not include potentially privileged communications in the package without legal counsel review.
- Do not request or retain personal data beyond the minimum necessary to complete the attestation.
- Do not interpret this package as legal advice or as a legal opinion on compliance status.

---

## Decision Tree (Condensed)

```
Receive control list
  └─ Scope workloads + check licensing tier
       └─ Map controls to evidence artifacts
            └─ Verify retention and legal-hold status
                 ├─ [Gap register non-empty] → Gate A: pause, escalate
                 ├─ [Retention or hold deficiency] → Gate B: stop, escalate
                 ├─ [Privileged content detected] → Gate C: flag, route to counsel
                 ├─ [Special-category data, unknown jurisdiction] → Gate D: stop
                 └─ [All gates clear] → Assemble attestation package
                                           └─ Compliance owner review + sign-off
                                                └─ Authorized external transmission (human only)
```

---

## Output Contract

| Field | Type | Required | Description |
|---|---|---|---|
| `attestation_package_id` | string (UUID) | Yes | Unique package identifier |
| `skill_id` | string | Yes | Must be `audit-evidence-mapping-protocol` |
| `skill_version` | string | Yes | Semantic version |
| `audit_window` | object | Yes | `start` and `end` ISO dates |
| `control_framework` | string | Yes | Framework name and version |
| `workloads_in_scope` | string[] | Yes | List of Microsoft 365 / D365 workloads |
| `attestation_records` | object[] | Yes | One record per control (schema above) |
| `gap_register` | object[] | Yes | Empty if no gaps; populated if Gate A fired |
| `escalations_fired` | string[] | Yes | Which gates fired |
| `package_status` | enum | Yes | `ready_for_review` or `blocked_pending_escalation` |
| `do_not_do_list` | string[] | Yes | Mandatory refusal items |
| `open_questions` | string[] | Yes | Unresolved items requiring human judgment |
| `timestamp` | string (ISO) | Yes | Package assembly datetime |

---

## Audit Log Fields

`attestation_package_id`, `skill_id`, `skill_version`, `invoked_by`, `input_hash`, `evidence_quality_summary`, `gates_fired`, `package_status`, `timestamp`
