# Incident-to-Remediation Protocol — Detailed Workflow and Output Contract

## Overview

This document provides the step-by-step workflow, decision tree, and output contract for the `incident-to-remediation-protocol` skill. It is the reference for `m365-identity-zero-trust-agent`, `copilot-governance-maestro-agent`, `microsoft-maestro-agent`, and human incident commanders who need to understand the gate structure, the incident record format, and post-incident review requirements.

Zero Trust assume-breach is applied at every phase: the scope of compromise is assumed to be larger than initial signals suggest until forensic evidence explicitly narrows it.

---

## Detailed Workflow

### Phase 1 — Detection and Triage

**Step 1.1 — Ingest incident signal**
- Source: Microsoft Defender portal incident queue, manual report, or alert from a connected workload
- Record: `incident_id`, `detection_source`, `initial_alert_count`, `affected_entity_types[]` (users, devices, mailboxes, apps, Dataverse environments), `incident_type_hypothesis`
- Apply assume-breach immediately: treat affected scope as potentially wider than initial entity list

**Step 1.2 — Severity classification**
Microsoft Defender XDR severity scale:
- **Informational**: no immediate risk; monitor
- **Low**: limited scope; standard response cadence
- **Medium**: moderate scope or data sensitivity; elevated response
- **High**: significant scope or high-value asset at risk; escalate immediately
- **Critical**: organization-wide threat, ransomware, or mass credential compromise; immediate escalation

Blast radius factors:
- Number of affected entities
- Sensitivity classification of accessed data (e.g., Purview sensitivity labels)
- Production environment impact
- Presence of lateral movement indicators

Output: `severity_assessment` with `severity_level`, `blast_radius_estimate`, `lateral_movement_suspected: true|false`, `data_exfiltration_suspected: true|false`

**Step 1.3 — Cross-workload scope mapping**
- Map affected entities to workloads: Entra ID, Exchange Online, SharePoint, Teams, Dynamics 365 / Dataverse, Copilot Studio
- Identify which Microsoft Defender workloads have signals (Defender for Identity, Defender for Endpoint, Defender for Office 365, Defender for Cloud Apps)
- Output: `workload_scope_map` with per-workload `affected: true|false`, `signal_source`, `entity_list[]`

---

### Phase 2 — Escalation and Containment Gates

**Gate 1 — Severity Triage Gate**
```
IF severity_level = High OR Critical:
  → ESCALATE IMMEDIATELY to:
    - Incident commander (human)
    - microsoft-maestro-agent (for Defender XDR SecOps coordination)
    - copilot-governance-maestro-agent (if Copilot surface area is in scope)
  → Do NOT proceed to containment planning without incident commander acknowledgment
  → Record: escalation_timestamp, escalation_target, commander_acknowledgment_reference
```

**Gate 2 — Containment Approval Gate**
```
FOR EVERY containment action (user disable, device isolation, IP/domain block, session revoke):
  → REQUEST explicit human approval from incident commander or security owner
  → Record: action_description, approver_id, approval_timestamp, approval_reference
  → Do NOT execute any containment action without recorded approval
  → If approval cannot be obtained within the required timeframe for a Critical incident:
    → Escalate to security leadership for emergency authorization
```

**Step 2.1 — Containment planning**
Identify containment actions per affected entity type:
| Entity type | Possible containment actions |
|---|---|
| User identity | Disable account, revoke sessions, reset credentials, enforce MFA via Conditional Access |
| Device | Isolate device via Microsoft Defender for Endpoint |
| Mailbox | Block external forwarding, remove malicious inbox rules |
| IP / domain | Block in Microsoft Defender portal |
| Dataverse environment | Revoke service principal access, disable affected flows |
| Copilot surface | Disable affected agent instance, revoke data source connections |

---

### Phase 3 — Investigation

**Step 3.1 — Attack story analysis**
- Use Microsoft Defender XDR correlated incident view to trace the attack timeline
- Identify: initial access vector, privilege escalation path, lateral movement, persistence mechanisms, data accessed or exfiltrated
- Map findings to MITRE ATT&CK techniques where applicable
- Output: `attack_story_summary` with `initial_access_vector`, `techniques[]`, `affected_data_classification`, `exfiltration_evidence: true|false|suspected`

**Step 3.2 — Identity and Zero Trust posture assessment**
- `m365-identity-zero-trust-agent` assesses:
  - Conditional Access coverage gaps that enabled the attack
  - Entra ID risk event details for affected identities
  - Privileged Identity Management (PIM) anomalies
  - Zero Trust gap: which verify-explicitly or least-privilege control failed
- Output: `identity_zt_assessment` with `ca_gap[]`, `risk_events[]`, `pim_anomalies[]`, `zt_gap_summary`

**Step 3.3 — Audit log and evidence preservation**
- Confirm relevant audit logs are preserved (Purview Unified Audit Log)
- If legal or regulatory investigation is likely: escalate to compliance owner for legal-hold placement in Microsoft Purview eDiscovery
- Do NOT allow audit logs to be deleted or retention shortened during active investigation

---

### Phase 4 — Remediation

**Step 4.1 — Remediation planning**
For each confirmed compromise or misconfiguration:
- Identify remediation action (credential reset, device re-image, inbox rule cleanup, Conditional Access policy correction, service principal removal)
- Confirm action is least-privilege (does not require broader permissions than necessary)
- Confirm action is reversible where possible; flag irreversible actions for extra approval
- Output: `remediation_plan[]` with `action`, `target_entity`, `reversible: true|false`, `approval_required: true|false`

**Step 4.2 — Remediation execution**
- Execute each approved remediation action
- Record: `action`, `executor`, `timestamp`, `result: success|failed|partial`
- Verify each action with confirmation evidence (e.g., sign-in log showing no further anomalous activity, device compliance status restored)

**Step 4.3 — Zero Trust posture hardening**
- Based on the Zero Trust gap identified in Step 3.2, recommend and apply (with human approval) Conditional Access policy corrections, PIM role scope reductions, or MFA enforcement changes
- Do NOT apply Zero Trust hardening that would lock out legitimate users without testing in report-only mode first

---

### Phase 5 — Post-Incident Review

**Gate 3 — Post-Incident Review Gate**
```
Mandatory before incident closure:
  → Complete structured post-incident review (items below)
  → Do NOT close incident in Microsoft Defender portal without completed review
  → Hand off review findings to security owner for playbook and policy updates
```

**Post-incident review checklist:**
1. Confirmed attack type, MITRE ATT&CK techniques, and impact summary
2. Root cause: which control failed, was misconfigured, or was absent
3. Detection gap: how long between initial access and detection; what reduced MTTD
4. Response gap: what slowed containment; what improved MTTC
5. Data accessed or exfiltrated: confirmed or suspected; sensitivity classification
6. Regulatory or legal notification obligation: yes / no / under assessment (escalate to legal if yes)
7. Zero Trust gap findings and recommended Conditional Access or identity hardening
8. Playbook updates required
9. Policy changes required (with owner and timeline)

---

## Decision Tree (Condensed)

```
Incident detected
  └─ Severity triage
       ├─ High / Critical → Gate 1: escalate to IC + maestro → awaiting acknowledgment
       └─ Low / Medium → containment planning
            └─ Gate 2: approval for each containment action
                 └─ Execute approved containment
                      └─ Investigation (attack story, identity ZT assessment, audit log)
                           └─ Remediation planning + Gate 2 (approval for each action)
                                └─ Execute remediation
                                     └─ Gate 3: post-incident review (mandatory)
                                          └─ Resolve incident in Defender portal
```

---

## Output Contract

### Incident Record

| Field | Type | Required | Description |
|---|---|---|---|
| `incident_record_id` | string (UUID) | Yes | Unique record identifier |
| `skill_id` | string | Yes | Must be `incident-to-remediation-protocol` |
| `skill_version` | string | Yes | Semantic version |
| `incident_id` | string | Yes | Microsoft Defender portal incident ID |
| `severity_level` | enum | Yes | `Informational | Low | Medium | High | Critical` |
| `blast_radius_estimate` | string | Yes | Affected entity count and data sensitivity summary |
| `workload_scope_map` | object | Yes | Per-workload affected status and entity list |
| `containment_actions` | object[] | Yes | Each action with approval reference and result |
| `attack_story_summary` | object | Yes | Initial access, techniques, data accessed |
| `identity_zt_assessment` | object | Yes | CA gaps, risk events, ZT gap summary |
| `remediation_plan` | object[] | Yes | Each remediation action with approval and result |
| `escalations_fired` | string[] | Yes | Which gates fired |
| `post_incident_review` | object | Yes (before closure) | Full review checklist completed |
| `incident_status` | enum | Yes | `open | contained | remediated | closed` |
| `do_not_do_list` | string[] | Yes | Mandatory refusal items |
| `open_questions` | string[] | Yes | Unresolved items for human judgment |
| `timestamp` | string (ISO) | Yes | Record creation datetime |

### Do-Not-Do List (always attached)

- Do not execute containment actions (user disable, device isolation, IP block) without recorded human approval.
- Do not close an incident without a completed post-incident review.
- Do not allow audit logs to be deleted or retention shortened during an active investigation.
- Do not minimize the assumed scope of compromise until forensic evidence explicitly narrows it.
- Do not treat this protocol as covering data breach regulatory notification obligations; escalate breach scenarios to the privacy owner and legal counsel.
- Do not request credentials, session tokens, or personal data to perform investigation; work from sanitized incident signals only.

---

## Audit Log Fields

`incident_record_id`, `skill_id`, `skill_version`, `invoked_by`, `incident_id`, `severity_level`, `gates_fired`, `containment_actions_count`, `remediation_actions_count`, `post_incident_review_complete`, `incident_status`, `timestamp`
