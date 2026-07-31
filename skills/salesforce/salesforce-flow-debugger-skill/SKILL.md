---
name: salesforce-flow-debugger-skill
description: "Diagnoses Salesforce Flow failures from pasted error messages or (in T1 mode) live Flow Interview logs fetched via sf CLI. Identifies the failing node, root cause, and provides specific fix recommendations including fault path design, data type corrections, and null handling. TRIGGER when: user says debug this flow error, flow failed with, flow interview error, why did my flow fail, flow is not working, flow throws error, flow interview fault. Trigger phrases: flow error, interview log, fault path, flow failed on. DO NOT TRIGGER when: writing new flows from scratch (route to generating-flow skill), reviewing static Flow configuration for quality (use salesforce-flow-automation-review-skill), deploying Flows to a new environment (use salesforce-deployment-validator-skill)."
license: MIT
allowed-tools: Bash(sf data query:*) Bash(sf org list metadata:*) Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-21"
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["View Setup and Configuration", "View Setup"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex"]
---

# salesforce-flow-debugger-skill

Hybrid T0/T1 skill that diagnoses Salesforce Flow failures. Works in two modes:

- **T0 mode (default):** Takes a pasted Flow error message or pasted Flow
  Interview log export. No org connection needed.
- **T1 mode (optional):** Fetches the Flow definition via
  `sf org list metadata` and queries `FlowInterviewLog` records via
  `sf data query` to retrieve live failure context.

Identifies the failing node, root cause, fix recommendation, and fault path
design guidance. Outputs a structured diagnosis with an audit envelope.

## When This Skill Owns the Task

Use `salesforce-flow-debugger-skill` when the work is to **diagnose why
a Flow failed or is not working**:

- "My Flow failed with: 'This record failed to save because...'"
- "Here's the Flow Interview log — what went wrong?"
- "Why does my Account Before-Save Flow throw a null pointer on the Decision node?"
- "Flow: Order Notification fails on Send Email Action with UNHANDLED_FAULT"
- "How do I add a fault path to prevent this error?"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Building a new Flow from scratch | `generating-flow` (sf-skills) or MCP pipeline |
| Static review of Flow configuration and best practices | `salesforce-flow-automation-review-skill` |
| Deploying a fixed Flow to another org | `salesforce-deployment-validator-skill` |
| Apex exception inside a Flow-invoked Apex action | `salesforce-apex-log-analyzer-skill` |

---

## Required Context to Gather First

Before diagnosing, gather:

1. **Error message text** — the exact error string from the flow failure
   notification, debug log, or Flow Interview log entry.
2. **Flow API name** — the developer name of the failing Flow, if known.
3. **Flow type** — Screen Flow, Auto-launched Flow, Record-Triggered Flow,
   Scheduled Flow, or Subflow.
4. **Trigger context** — what action or event triggered the Flow? User
   action, record save, scheduled batch, or REST API call?
5. **Failing node name** — the element name (if visible in the error or
   the Flow builder debug view).
6. **Org type** — sandbox or production. Production errors require stricter
   redaction.
7. **Recent changes** — was the Flow recently modified or deployed?
   What changed?

In T1 mode, additionally confirm:
- Target org alias recognized by `sf org list`
- Org is sandbox (T1 mode is restricted to sandbox for live log fetch)

---

## Recommended Workflow

### Step 1 — Parse the error message

Extract from the error text:

- **Fault type** — UNHANDLED_FAULT, NullPointerException, DML Exception,
  Governor Limit, Type Mismatch, Recursive Flow
- **Failing element** — the element name or API action mentioned
- **Variable state context** — any variable values or record ID fragments
  in the error (mask immediately per redaction rules)
- **Stack trace** — if present, extract the top frame

Apply redaction (see Redaction Rules) before continuing analysis.

### Step 2 (T1 mode only) — Fetch Flow metadata

```bash
sf org list metadata \
  --metadata-type Flow \
  --target-org <alias>
```

Filter for the failing Flow by API name. Use the result to confirm the
Flow version deployed and retrieve the element count.

### Step 3 (T1 mode only) — Query FlowInterviewLog

```bash
sf data query \
  --query "SELECT Id, FlowApiName, InterviewLabel, CurrentElement, ErrorCode, ErrorMessage, StartTime, EndTime FROM FlowInterviewLog WHERE FlowApiName = '<FlowApiName>' AND Status = 'Fault' ORDER BY StartTime DESC LIMIT 10" \
  --target-org <alias> \
  --result-format json
```

Redact all record IDs and variable values before analysis.
Note: `FlowInterviewLog` and `FlowInterviewLogEntry` are available
in orgs with Flow Interview Logging enabled (must be activated in Setup →
Process Automation Settings).

### Step 4 — Classify the root cause

Map the parsed error to a root cause pattern
(see `references/flow-error-patterns.md`):

| Error pattern | Root cause category |
|---|---|
| `UNHANDLED_FAULT` on Action element | Missing fault connector on the action |
| `NullPointerException` on Assignment | Variable used before being set; loop ran zero iterations |
| `DML Exception` on Update Records | Validation rule blocked the save; trigger re-entry; locked record |
| `EXCEEDED_ID_LIMIT` or `TOO_MANY_SOQL_QUERIES` | DML or SOQL inside a loop |
| `INSUFFICIENT_ACCESS_ON_CROSS_REFERENCE_ENTITY` | Running user lacks object or record access |
| `FLOW_LOOP_COUNT_LIMIT` | Recursive Flow invocation; self-trigger via record update |
| `INVALID_TYPE` or `WRONG_CONTROLLER_STATE` | Merging incompatible variable types in an Assignment |
| Subflow not found | Subflow version not active or not deployed in this org |

### Step 5 — Generate specific fix recommendation

For each root cause, generate:
1. **Immediate fix** — what to change in the Flow builder right now
2. **Fault path recommendation** — should a fault connector be added?
   Where should the fault path go? (See `references/fault-path-design.md`)
3. **Null guard** — if null variables are involved, where should the
   null check Decision element be placed before the failing element?
4. **Data type fix** — if type mismatch, what types need to match and how
   to add an explicit conversion or intermediate variable

### Step 6 — Emit structured diagnosis with audit envelope

Produce the full output block per the Output Format section below.

---

## Quality Scoring Rubric (100-point)

Score every diagnosis before emitting. Threshold: 80+ ship, 60–79 ship
with caveat, below 60 reject and request more context.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Root cause clarity** | 30 | Specific error type identified; failing element named; cause-and-effect chain explained in plain language |
| **Fix suggestion specificity** | 25 | Tells admin which element to click, which property to change, and what value to set — not "check the data" |
| **Fault path recommendation** | 20 | Recommends adding fault connector where missing; describes where fault path should end (screen, log, email) |
| **Data type analysis** | 15 | Identifies mismatched variable types; names correct types and where to set them |
| **Redaction quality** | 10 | All record IDs, variable values, and user IDs masked in output and audit envelope |

**Scoring penalties:**
- Generic recommendation ("check the error") with no specific element: -25
- Missing fault path guidance when UNHANDLED_FAULT is the error type: -20
- Unredacted record ID or user ID in output: -30 (immediate caveat)
- Incorrect fault type classification: -20

---

## T0/T1 Contract

### T0 Mode (default, no org connection)

- Takes pasted error message or pasted FlowInterviewLog export as input.
- All analysis is static — no org connection.
- `allowed-tools: Read Grep Glob` only.
- No `sf` CLI commands executed.

### T1 Mode (optional, read-only runtime)

- Activates when user confirms org alias and explicitly requests live
  log fetch.
- OAuth scopes: `api` and `refresh_token` only.
- Run As permissions: `View Setup and Configuration`, `View Setup`.
- Denied: `ModifyAllData`, `ViewAllData`, `ViewEncryptedData`,
  `ModifyMetadata`, `AuthorApex`.
- **Restricted to sandbox orgs** — do not query FlowInterviewLog on
  production in T1 mode without explicit user confirmation and production
  safety review.
- Maximum query: 10 most recent fault records. No bulk log extraction.

---

## Refusal Triggers

Stop and decline if:

- The request is to write a new Flow from scratch (out of scope).
- The request is to deploy the fixed Flow to a production org (route to
  `salesforce-deployment-validator-skill`).
- In T1 mode: target org appears to be production and user has not
  explicitly confirmed the production safety review step.
- The audit envelope cannot be populated in T1 mode (org alias
  unresolvable, flow name missing).
- The user requests that redaction be skipped.

---

## Audit Envelope Schema

Every T1 execution emits an audit envelope. T0 mode emits a reduced
envelope (no org fields).

```yaml
audit_envelope:
  matter_id: "<caller-provided-or-generated-uuid>"
  skill_id: "salesforce-flow-debugger-skill"
  skill_version: "0.1.0"
  mode: "<T0-static | T1-live>"
  target_org_alias: "<alias or 'N/A for T0'>"
  flow_api_name: "<FlowApiName>"
  flow_type: "<RecordTriggered|Screen|AutoLaunched|Scheduled>"
  run_as_user_id: "<user_id_placeholder>"
  log_records_queried: <integer or 0 for T0>
  redactions_applied:
    - field: "<FieldOrVariableName>"
      reason: "<record_id|user_id|variable_value|pii>"
  timestamp: "<ISO-8601-UTC>"
  org_type_verified: "<sandbox | production | N/A for T0>"
```

---

## Output Format

```yaml
verdict: "diagnosis-complete | needs-more-context | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

diagnosis:
  flow_api_name: "<FlowApiName or 'unknown'>"
  flow_type: "<type>"
  error_type: "<UNHANDLED_FAULT|NullPointerException|DML Exception|Governor Limit|Type Mismatch|Recursive|Other>"
  failing_element_name: "<ElementName or 'unknown'>"
  failing_element_type: "<Action|Decision|Assignment|Loop|GetRecords|UpdateRecords|Screen|Subflow>"
  root_cause_summary: "<1-2 sentence plain-language explanation>"
  root_cause_detail: "<technical detail with element names and variable context>"

fix_recommendation:
  immediate_fix: "<specific steps in Flow Builder>"
  fault_path_required: <true|false>
  fault_path_design: "<where to add it and where it should terminate>"
  null_guard_required: <true|false>
  null_guard_placement: "<before which element and what Decision logic>"
  data_type_fix: "<if type mismatch: what types, where to add intermediate variable>"
  governor_limit_fix: "<if governor limit: what to move outside the loop>"

fault_path_template:
  trigger: "<which element needs the fault connector>"
  fault_variable_capture: "<Fault Message variable assignment>"
  fault_path_ends_at: "<Screen|Custom Notification|Log to Custom Object|Email>"
  sample_fault_message_variable: "{!$Flow.FaultMessage}"

redaction_log:
  - "<description of what was masked>"

audit_envelope:
  <see Audit Envelope Schema>

escalation_triggers_fired:
  - "<trigger name or 'none'>"

missing_evidence:
  - "<what additional context would improve the diagnosis>"

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

Apply in order. Do not bypass for any reason.

1. **Salesforce Record IDs (15/18-char):** Replace with
   `<record_id_placeholder>` anywhere they appear in error messages or
   log entries.
2. **User IDs (OwnerId, CreatedById, RunningUserId in logs):** Replace
   with `<user_id_placeholder>`.
3. **Flow variable values** that may contain PII (email, phone, name,
   address): Replace with `<variable_value_redacted>`.
4. **Org IDs (18-char starting `00D`):** Replace with
   `<org_id_placeholder>`.
5. **OAuth tokens, session IDs:** Strip entirely — never include in output.
6. **Instance URLs:** Replace with `<org_instance_placeholder>`.
7. **Stack trace class paths containing customer namespace:** Preserve
   class names but redact any embedded record IDs or data values.

Document each redaction in `redaction_log`.

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| Fix requires Apex action code change | `salesforce-apex-log-analyzer-skill` |
| Fix requires Flow deployment to new org | `salesforce-deployment-validator-skill` |
| Flow passes but underlying permission is wrong | `salesforce-permission-model-review-skill` |
| Static quality review of the Flow design | `salesforce-flow-automation-review-skill` |
| Flow governs a business-critical process needing production change | `salesforce-live-guard-agent` |

---

## Stop Conditions

- Target org is production and T1 mode was requested without explicit
  production safety acknowledgment — stop and require acknowledgment.
- FlowInterviewLog object is not queryable (Flow Interview Logging not
  enabled in org) — stop T1 fetch, fall back to T0 mode with error message.
- Error message contains no element name and no variable context — request
  additional pasted context (full error text, Flow debug log) before
  proceeding.
- User requests to run a DML fix directly in T1 mode — refuse; route to
  human-approval path via `salesforce-live-guard-agent`.

---

## Security Notes

- **T1 read-only operational:** Queries `FlowInterviewLog` only. No DML,
  no metadata mutation, no Apex execution.
- **Sanitized output only:** All record IDs, user IDs, and variable values
  redacted before emission.
- **Sandbox-preferred for T1:** Production FlowInterviewLog access requires
  explicit user confirmation and applies stricter redaction.
- **Structured audit emitted:** Every T1 execution produces a complete audit
  envelope.
- **Least-privilege Run As account:** No Modify All Data, View All Data,
  View Encrypted Data, or Modify Metadata.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/flow-error-patterns.md` | Common Flow errors and their root causes |
| `references/fault-path-design.md` | When and how to add fault connectors |
| `references/interview-log-redaction.md` | Sanitizing FlowInterviewLog output |
