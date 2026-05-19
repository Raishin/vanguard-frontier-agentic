# Legal-HR Case Capsule — Schema and Redaction Rules

The capsule is the only sanctioned cross-agent handoff record. It has 30
required fields. A capsule with any field omitted, blank, or set to a
placeholder is not ready to send.

## Field definitions

### Identity and routing

| Field | Type | Rule |
|---|---|---|
| `case_id` | string | Stable opaque identifier. No employee name, no email. |
| `source_agent` | string | Agent id that produced this capsule. |
| `receiving_agent` | string | Agent id this capsule is handed to. |
| `primary_agent` | string | The single agent accountable for synthesis. Exactly one. |
| `secondary_agents` | string[] | Agents running parallel review. May be empty. |
| `matter_type` | enum | See risk taxonomy `matter_type` values. |
| `employee_or_party_identifiers_redacted` | string | Role + business unit only, e.g. "IC engineer, BU-West". Never a name or ID. |
| `jurisdiction_or_location_if_known` | string | Country / state / "Unknown". |
| `business_unit` | string | Org unit reference, not a person. |
| `timeline` | string | Dated sequence of events, effective dates, deadlines. |

### Evidence discipline

| Field | Type | Rule |
|---|---|---|
| `facts` | string[] | Confirmed and corroborated only. |
| `allegations` | string[] | Claims made but unproven; record who made each. |
| `assumptions` | string[] | Treated as plausible but unverified. |
| `inferences` | string[] | Conclusions drawn from facts; mark each as inference. |
| `missing_evidence` | string[] | Materially relevant facts not provided. |
| `evidence_quality` | enum | `strong` / `mixed` / `weak` / `insufficient`. |

### Risk posture

| Field | Type | Rule |
|---|---|---|
| `risk_rating` | enum | `Critical` / `High` / `Medium` / `Low` / `Unknown`. |
| `privilege_sensitivity` | enum | `none` / `possible` / `likely-privileged`. |
| `privacy_sensitivity` | enum | `low` / `moderate` / `high` / `special-category`. |
| `retaliation_risk` | enum | `none-observed` / `possible` / `elevated` / `unknown`. |
| `discrimination_or_harassment_risk` | enum | `none-observed` / `possible` / `elevated` / `unknown`. |
| `regulatory_risk` | enum | `none-observed` / `possible` / `elevated` / `unknown`. |
| `litigation_hold_needed` | enum | `no` / `recommended` / `yes` / `unknown`. |
| `data_minimization_notes` | string | What was excluded and why. |

### Ownership and action

| Field | Type | Rule |
|---|---|---|
| `decision_owner` | string | Accountable human role, e.g. "Employment Counsel". Exactly one. |
| `human_approval_required` | boolean | `true` for any adverse, irreversible, or cross-domain action. |
| `escalation_required` | boolean | `true` whenever an escalation gate is triggered. |
| `recommended_next_action` | string | A recommendation, never a directive. |
| `do_not_do_list` | string[] | Explicit prohibited actions. Must be non-empty. |
| `audit_log_summary` | string | One-line pointer to the audit-log event for this handoff. |

## Redaction rules

1. Default to role + business-unit references. A name appears in a capsule only
   when a named human owner role requires it, and never an employee subject.
2. No government IDs, no contact details, no credentials, no medical detail, no
   protected-class data unless the matter strictly requires it and the
   `privacy_sensitivity` field is set to `special-category` with a documented
   reason in `data_minimization_notes`.
3. Privileged or investigation text is summarized, never pasted. If a capsule
   would extend circulation of privileged material, set `privilege_sensitivity`
   to `likely-privileged` and narrow the content.
4. `do_not_do_list` always includes, at minimum, the actions outside agent
   authority: approve, deny, terminate, discipline, settle, file, notify a
   regulator, make a public disclosure, send an employee communication, or
   mutate an HR/legal system.

## Worked example (sanitized)

```json
{
  "case_id": "CAP-2026-0142",
  "source_agent": "hr-maestro-agent",
  "receiving_agent": "hr-termination-readiness-agent",
  "primary_agent": "hr-termination-readiness-agent",
  "secondary_agents": ["legal-employment-law-risk-agent", "hr-employee-relations-agent"],
  "matter_type": "termination-with-retaliation-risk",
  "employee_or_party_identifiers_redacted": "IC engineer, BU-West",
  "jurisdiction_or_location_if_known": "Unknown",
  "business_unit": "BU-West Engineering",
  "timeline": "Complaint filed 2026-04-02; PIP opened 2026-04-09; termination proposed 2026-05-10",
  "facts": ["A PIP was opened", "A complaint was filed before the PIP"],
  "allegations": ["Manager states performance was failing pre-complaint (uncorroborated)"],
  "assumptions": ["PIP criteria were applied consistently with peers"],
  "inferences": ["Timing proximity could be read as retaliatory by a fact-finder"],
  "missing_evidence": ["Contemporaneous performance records predating the complaint", "Comparator data"],
  "evidence_quality": "weak",
  "risk_rating": "High",
  "privilege_sensitivity": "possible",
  "privacy_sensitivity": "moderate",
  "retaliation_risk": "elevated",
  "discrimination_or_harassment_risk": "unknown",
  "regulatory_risk": "possible",
  "litigation_hold_needed": "recommended",
  "data_minimization_notes": "Complaint text excluded; only sequence retained",
  "decision_owner": "Employment Counsel",
  "human_approval_required": true,
  "escalation_required": true,
  "recommended_next_action": "Pause the proposed termination; route to employment counsel for retaliation analysis",
  "do_not_do_list": ["Do not proceed with termination", "Do not backdate or retroactively create performance records", "Do not contact the complainant about the complaint"],
  "audit_log_summary": "EVT-2026-0142-03 capsule handoff hr-maestro -> termination-readiness"
}
```
