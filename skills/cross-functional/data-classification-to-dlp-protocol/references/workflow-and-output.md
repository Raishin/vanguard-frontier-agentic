# Data Classification to DLP Protocol — Detailed Workflow and Output Contract

## Overview

This document provides the step-by-step workflow, decision tree, and output contract for the `data-classification-to-dlp-protocol` skill. It is the reference for `m365-copilot-readiness-governance-agent`, `power-platform-governance-dataverse-security-agent`, `m365-maestro-agent`, and human Purview compliance administrators who need to understand the taxonomy gate, the DLP coverage mapping process, and the classification report format.

---

## Detailed Workflow

### Phase 1 — Data Discovery and Scoping

**Step 1.1 — Ingest data types and regulatory context**
- Input: data type list (e.g., customer PII, financial records, health data, intellectual property)
- Input: regulatory or contractual requirements (GDPR, HIPAA, PCI DSS, internal policy)
- Map each data type to a regulatory obligation tier:
  - Tier 1 (Restricted): special-category personal data, payment card data, health data — highest protection required
  - Tier 2 (Highly Confidential): confidential business data, internal credentials, contracts
  - Tier 3 (Confidential): internal business data with limited sharing scope
  - Tier 4 (Internal): general internal use data
  - Tier 5 (Public): publicly released data
- Output: `data_type_register[]` with `data_type`, `regulatory_tier`, `applicable_regulations[]`

**Step 1.2 — Scope workloads**
- Identify which Microsoft 365 workloads and Power Platform environments are in scope
- For each workload: confirm whether Microsoft Purview Information Protection labeling is enabled
- Output: `workload_scope[]` with `service`, `labeling_enabled: true|false`, `dlp_policy_location_supported: true|false`

---

### Phase 2 — Taxonomy Review and Design

**Step 2.1 — Review existing sensitivity label taxonomy**
- Input: current sensitivity label list from Microsoft Purview (exported summary — no live query requiring credentials)
- For each existing label: record `label_name`, `scope` (files, emails, meetings, schema assets), `encryption_configured: true|false`, `content_marking: true|false`, `sublabel_of`
- Compare to data type register: identify gaps (data types with no matching label, overlapping label scopes, outdated label configurations)
- Output: `taxonomy_gap_list[]` with `data_type`, `gap_type` (missing label, overlapping scope, outdated config), `recommended_action`

**Step 2.2 — Taxonomy design recommendation**
Microsoft Purview sensitivity label best practices applied:
- Use a hierarchical label structure (parent label → sublabels for tiers)
- Configure encryption for Tier 1 and Tier 2 labels using Azure Rights Management
- Configure content marking (headers, footers, watermarks) for Tier 1–3 labels
- Confirm auto-labeling compatibility for each label (sensitive information types, trainable classifiers, exact data match)
- Output: `taxonomy_design_recommendation` with recommended label structure, encryption settings, and auto-labeling approach

Note: taxonomy design is a recommendation only; do not create or modify labels — route to Purview compliance administrator for implementation.

**Gate 1 — Classification Taxonomy Gate**
```
IF taxonomy_gap_list[] contains:
  - Missing labels for Tier 1 or Tier 2 data types
  - Contradictory label scopes (multiple labels covering same data without clear hierarchy)
  - Labels without encryption for Tier 1 data types:
  → PAUSE
  → Escalate to: data owner + compliance owner
  → Do NOT proceed to DLP coverage mapping until taxonomy gaps are resolved or explicitly accepted with risk note
```

---

### Phase 3 — DLP Coverage Mapping

**Step 3.1 — Map labels to DLP policies**
For each sensitivity label:
- Identify which DLP policies reference the label (via Content contains > Sensitivity labels condition)
- Identify which workload locations are covered (Exchange, SharePoint, OneDrive, Teams, Devices, Copilot/Copilot Chat, Power Platform)
- Record: `dlp_coverage_map[]` with `label_name`, `covered_locations[]`, `uncovered_locations[]`, `protective_action` (block, restrict, audit-only, warn), `policy_name`

**Step 3.2 — Identify coverage gaps**
Coverage gap types:
- Regulated data type (Tier 1/2) with no DLP policy
- DLP policy exists but protective action is audit-only for regulated data types (insufficient)
- Workload location not covered (e.g., Devices or Teams not included in policy scope)
- Power Platform connectors handling sensitive data not restricted by a connector DLP policy

**Gate 2 — DLP Coverage Gate**
```
IF any Tier 1 or Tier 2 data type has:
  - No DLP policy
  - DLP policy with audit-only action (no block or restrict)
  - Critical workload location not covered:
  → STOP
  → Escalate to: Purview compliance administrator + data owner
  → Do NOT assemble final report until gaps are resolved or formally accepted with risk note by compliance owner
```

**Step 3.3 — Assess auto-labeling opportunity**
- For each Tier 1 and Tier 2 data type: assess whether a sensitive information type (SIT), trainable classifier, or exact data match (EDM) configuration is available for auto-labeling
- Output: `auto_labeling_opportunities[]` with `data_type`, `classifier_available: true|false`, `classifier_name`, `recommended_policy_type` (simulation first, then enforcement)

---

### Phase 4 — Power Platform and Dataverse Alignment

**Step 4.1 — Power Platform connector DLP assessment**
- `power-platform-governance-dataverse-security-agent` reviews:
  - Connector classification in each in-scope environment DLP policy (Business, Non-Business, Blocked)
  - Connectors handling Tier 1 or Tier 2 data that are classified as Non-Business or unclassified
  - Environment groups and default DLP policies for tenant-level coverage
- Output: `pp_dlp_gap_list[]` with `connector_name`, `current_classification`, `recommended_classification`, `risk_tier`

**Step 4.2 — Dataverse column security assessment**
- Review Dataverse column security profiles for fields containing sensitive data types
- Flag fields with sensitive data that lack column-level security profiles
- Output: `dataverse_security_gaps[]` with `table_name`, `column_name`, `data_type`, `column_security_enabled: true|false`

**Gate 3 — Label Adoption Gate**
```
IF label adoption data is available AND adoption rate for Tier 1/2 data types is below threshold (defined by compliance owner):
  → FLAG as risk item
  → Escalate to: data owner for adoption improvement actions
  → Record adoption gap in final report
  → Do NOT treat low adoption as a passed state; it is an open risk
```

**Gate 4 — Special-Category Data Gate**
```
IF any Tier 1 data type is health, biometric, genetic, racial/ethnic origin, or other special-category:
  → Confirm jurisdiction and privacy owner
  → Do NOT proceed without jurisdiction confirmed
  → Ensure applicable regulation (GDPR Art. 9, HIPAA) is reflected in label and DLP protective action
```

**Gate 5 — Copilot Surface Exposure Gate**
```
IF sensitivity label configuration affects Microsoft 365 Copilot or Copilot Chat data grounding:
  → Escalate to: m365-copilot-readiness-governance-agent
  → Ensure DLP policy for Copilot/Copilot Chat location is configured for Tier 1/2 labels
  → Verify that Copilot cannot process files or emails with Tier 1 labels where the DLP policy blocks processing
```

---

### Phase 5 — Report Assembly and Handoff

**Step 5.1 — Compile classification and DLP report**
```
{
  "report_id": "<uuid>",
  "skill_id": "data-classification-to-dlp-protocol",
  "skill_version": "0.1.0",
  "data_types_assessed": <n>,
  "taxonomy_gaps": <n>,
  "dlp_coverage_gaps": <n>,
  "pp_dlp_gaps": <n>,
  "dataverse_security_gaps": <n>,
  "auto_labeling_opportunities": <n>,
  "gates_fired": ["Gate 1", "Gate 2", ...],
  "report_status": "ready_for_review | blocked_pending_escalation",
  "timestamp": "<ISO datetime>"
}
```

**Step 5.2 — Attach do-not-do list**
Every report includes:
- Do not create, modify, or delete sensitivity labels or DLP policies without Purview compliance administrator approval.
- Do not remove encryption from labels covering Tier 1 or Tier 2 data without legal and compliance owner review.
- Do not deliberately under-classify regulated personal data to evade a regulatory obligation.
- Do not treat audit-only DLP actions as sufficient protection for Tier 1 regulated data types.
- Do not request credentials, tenant IDs, or personal data to perform the classification assessment.
- Do not interpret this report as legal advice on regulatory compliance status.

---

## Decision Tree (Condensed)

```
Ingest data types + regulatory requirements
  └─ Scope workloads
       └─ Review existing taxonomy → identify gaps
            └─ Gate 1: taxonomy complete? → No → PAUSE, escalate
                 └─ Map labels to DLP policies → identify coverage gaps
                      └─ Gate 2: DLP coverage for Tier 1/2? → No → STOP, escalate
                           └─ Assess auto-labeling + Power Platform DLP + Dataverse security
                                ├─ Gate 3: label adoption below threshold? → FLAG
                                ├─ Gate 4: special-category data? → confirm jurisdiction
                                ├─ Gate 5: Copilot surface exposure? → escalate to copilot-readiness agent
                                └─ [All gates resolved] → Assemble report → Purview admin review + sign-off
```

---

## Output Contract

### Classification and DLP Report

| Field | Type | Required | Description |
|---|---|---|---|
| `report_id` | string (UUID) | Yes | Unique report identifier |
| `skill_id` | string | Yes | Must be `data-classification-to-dlp-protocol` |
| `skill_version` | string | Yes | Semantic version |
| `data_type_register` | object[] | Yes | All data types with regulatory tier |
| `taxonomy_gap_list` | object[] | Yes | Gaps in existing label taxonomy |
| `taxonomy_design_recommendation` | object | Yes | Recommended label structure (not for direct implementation) |
| `dlp_coverage_map` | object[] | Yes | Per-label DLP policy coverage |
| `dlp_gap_list` | object[] | Yes | Coverage gaps requiring remediation |
| `auto_labeling_opportunities` | object[] | Yes | Auto-labeling candidates |
| `pp_dlp_gap_list` | object[] | Yes | Power Platform connector DLP gaps |
| `dataverse_security_gaps` | object[] | Yes | Dataverse column security gaps |
| `gates_fired` | string[] | Yes | Which gates fired |
| `report_status` | enum | Yes | `ready_for_review` or `blocked_pending_escalation` |
| `do_not_do_list` | string[] | Yes | Mandatory refusal items |
| `open_questions` | string[] | Yes | Unresolved items for human judgment |
| `timestamp` | string (ISO) | Yes | Report creation datetime |

---

## Audit Log Fields

`report_id`, `skill_id`, `skill_version`, `invoked_by`, `data_types_assessed`, `taxonomy_gaps`, `dlp_coverage_gaps`, `gates_fired`, `report_status`, `timestamp`
