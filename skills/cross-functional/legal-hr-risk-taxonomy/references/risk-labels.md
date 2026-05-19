# Legal-HR Risk Labels and Audit-Log Schema

## matter_type values

The `matter_type` field on a case capsule uses one of these classes. Classes
marked **E** are escalation-grade by default.

| Value | Domain | E |
|---|---|---|
| `contract-review` | Legal | |
| `privacy-data-protection` | Legal | |
| `litigation-hold-discovery` | Legal | E |
| `regulatory-compliance` | Legal | |
| `ip-open-source` | Legal | |
| `vendor-procurement-risk` | Legal | |
| `ethics-investigation` | Legal | E |
| `policy-governance` | Legal | |
| `public-disclosure` | Legal | E |
| `employee-relations` | HR | |
| `workplace-investigation` | HR | E |
| `performance-management` | HR | |
| `termination-readiness` | HR | |
| `termination-with-retaliation-risk` | HR + Legal | E |
| `leave-accommodation` | HR | E |
| `recruiting-selection` | HR | |
| `compensation-equity` | HR | E |
| `benefits-payroll` | HR | |
| `workforce-reduction` | HR + Legal | E |
| `worker-classification` | HR + Legal | E |
| `harassment-discrimination` | HR + Legal | E |
| `retaliation` | HR + Legal | E |
| `whistleblower` | Legal + HR | E |
| `executive-misconduct` | Legal + HR | E |
| `union-labor` | HR + Legal | E |
| `employee-data-breach` | Legal + HR | E |
| `people-data-analytics` | HR | |
| `hris-process-controls` | HR | |
| `unclassified` | maestro | |

## Label definitions

- `evidence_quality`
  - `strong` — corroborated, contemporaneous, consistent.
  - `mixed` — some corroboration, some gaps.
  - `weak` — largely uncorroborated or single-source.
  - `insufficient` — cannot assess; treat conclusions as Unknown.
- `privilege_sensitivity`
  - `none` — no privileged material implicated.
  - `possible` — privilege may attach; flag and narrow circulation.
  - `likely-privileged` — route through counsel; do not widen distribution.
- `privacy_sensitivity`
  - `low` — no personal data beyond role references.
  - `moderate` — ordinary employee data.
  - `high` — sensitive employee data (performance, discipline).
  - `special-category` — medical, disability, immigration, protected-class, or
    similar; documented justification required.

## Audit-log schema

One event per handoff or escalation. Minimum necessary fields only.

| Field | Rule |
|---|---|
| `event_id` | Stable opaque event identifier. |
| `case_id` | The capsule this event belongs to. |
| `timestamp` | ISO 8601. |
| `initiating_agent` | Agent id that raised the event. |
| `receiving_agent` | Agent id or human owner role receiving it. |
| `human_owner` | Accountable human role. |
| `matter_type` | A value from the table above. |
| `risk_rating` | Critical / High / Medium / Low / Unknown. |
| `escalation_status` | `none` / `recommended` / `escalated` / `paused`. |
| `data_sensitivity` | The `privacy_sensitivity` label. |
| `privilege_sensitivity` | The `privilege_sensitivity` label. |
| `action_recommended` | One-line recommendation, never a directive. |
| `action_prohibited` | The capsule `do_not_do_list` summary. |
| `evidence_summary` | One line; no raw sensitive content. |
| `open_questions` | Material questions still unanswered. |
| `decision_status` | `pending-human-approval` / `approved-by-owner` / `not-required`. |
| `retention_category` | Records-retention class for the event. |

## Rules

1. The audit log carries labels and summaries only. Never raw medical,
   privileged, credential, or protected-class content.
2. `decision_status` is never `approved` unless a named `human_owner` approved
   it. Agents never self-approve.
3. When facts are missing, `risk_rating` is `Unknown` and `escalation_status`
   is at least `recommended`.
4. An escalation-grade `matter_type` forces `escalation_status` to `escalated`
   or `paused`, never `none`.
