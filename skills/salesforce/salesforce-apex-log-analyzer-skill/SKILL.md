---
name: salesforce-apex-log-analyzer-skill
description: "Retrieves and analyzes Apex debug logs from a connected Salesforce org to identify governor-limit hits, SOQL N+1 patterns, unhandled exceptions, and async job failures. T1 read-only runtime — retrieves logs only, never executes code or mutates data. TRIGGER when: user asks to analyze an Apex log, debug a trigger failure, diagnose a governor limit hit, interpret a stack trace from a Salesforce org, or review a DEBUG log for performance issues. Trigger phrases: analyze apex log, debug this trigger, why is my trigger failing, governor limit hit, DEBUG log analysis, check my log file. DO NOT TRIGGER when: user wants to run live tests (use salesforce-apex-test-runner-skill), static code review without logs (use salesforce-apex-lwc-code-review-skill), generating new Apex code (use salesforce-apex-generator-skill), or Agentforce session telemetry (use salesforce-agentforce-stdm-observer-skill)."
license: MIT
allowed-tools: Bash(sf apex get log:*) Bash(sf apex tail log:*) Bash(sf data query:*) Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-21"
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["View Setup and Configuration"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps"]
---

# salesforce-apex-log-analyzer-skill

T1 read-only runtime skill for Apex debug log retrieval and analysis. This skill is a
**diagnostic flashlight** — it retrieves log content, identifies governor-limit hits,
surfaces SOQL N+1 patterns, traces exceptions, and produces a prioritized finding report.
It does not execute code, mutate data, or deploy anything.

## When This Skill Owns the Task

Use `salesforce-apex-log-analyzer-skill` when the work requires **log-based diagnosis**:

- "Why is my Account trigger hitting a governor limit?"
- "Analyze the debug log from my sandbox — it shows a heap size limit"
- "This trigger is slow — can you read the log and find the SOQL bottleneck?"
- "I got an unhandled exception in my batch job — here is the log"
- "Check if my Queueable is completing successfully in the async log"
- "Retrieve the latest log for user jsmith@myorg.sandbox and diagnose it"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| User wants to run Apex tests (not read logs) | `salesforce-apex-test-runner-skill` |
| Static code review without log evidence | `salesforce-apex-lwc-code-review-skill` |
| Generate a fix for the identified code problem | `salesforce-apex-generator-skill` |
| Agentforce session traces / parquet telemetry | `salesforce-agentforce-stdm-observer-skill` |
| SOQL query performance analysis without logs | `salesforce-soql-explorer-skill` |

---

## Required Context to Gather First

Before retrieving or analyzing any log, confirm:

1. **Target org alias** — the `--target-org` value from `sf org list`.
2. **Log identifier** — log ID, log file path, user/transaction context, or timeframe.
   Accept a pasted log file if the user provides one directly.
3. **Transaction context** — which trigger, class, batch job, or user action generated the log.
4. **Goal** — governor-limit diagnosis, exception trace, performance analysis, or all.
5. **Sensitivity level** — does the log likely contain PII (email, phone, record data from
   regulated objects)? Apply stricter redaction if so.

---

## Recommended Workflow

### Step 1 — Verify org alias and reachability

```bash
sf org display --target-org <alias>
```

Confirm the org is reachable. Note org type (production vs sandbox) for the audit envelope.
Apply stricter scrutiny for production org logs — log content may contain PII.

### Step 2 — List available logs

```bash
sf apex get log --list --target-org <alias>
```

Identify the relevant log(s) by user, timestamp, and log type. If the user provides a log ID,
skip this step and proceed to Step 3.

### Step 3 — Retrieve the log

**Option A — specific log by ID:**

```bash
sf apex get log \
  --log-id <logId> \
  --target-org <alias>
```

**Option B — latest log for current user:**

```bash
sf apex get log \
  --number 1 \
  --target-org <alias>
```

**Option C — tail live logs (non-blocking diagnostic):**

```bash
sf apex tail log \
  --target-org <alias> \
  --color
```

**Option D — user has pasted log content directly:**
Accept the pasted content and proceed to Step 4.

### Step 4 — Parse the log in order

Analyze in this sequence per `references/log-format-reference.md`:

1. **Entry point** — identify transaction type (Apex class invocation, trigger, REST callout, batch execute)
2. **Fatal errors and unhandled exceptions** — `FATAL_ERROR` and `EXCEPTION_THROWN` lines
3. **Governor limit hits** — `LIMIT_USAGE_FOR_NS` entries; compare to limits in `references/governor-limit-signatures.md`
4. **SOQL patterns** — `SOQL_EXECUTE_BEGIN` / `SOQL_EXECUTE_END` pairs; look for repeated queries in loop context
5. **DML patterns** — `DML_BEGIN` / `DML_END` pairs; check for loop context
6. **CPU hotspots** — high cumulative CPU entries; identify top consumers
7. **Heap usage** — `HEAP_ALLOCATE` patterns; identify large collection accumulation
8. **Async job indicators** — `ENTERING_MANAGED_PKG`, `CALLOUT_REQUEST`, `FUTURE_CALL_PROCESS`

### Step 5 — Classify findings by severity

| Severity | Criteria |
|---|---|
| **Critical** | Runtime failure, hard limit hit, unhandled exception, data corruption risk |
| **Warning** | Near-limit (>75% of a governor limit), non-selective SOQL, slow DML path |
| **Info** | Optimization opportunity, hygiene issue, async pattern note |

### Step 6 — Apply redaction

Apply all redaction rules (see Redaction Rules section) before emitting any log excerpt
in output. Never echo raw log lines containing record IDs, user IDs, or PII field values.

### Step 7 — Emit findings with audit envelope

Score findings against the quality rubric (see below). Emit the full audit envelope.
Propose the smallest correct fix for each Critical and Warning finding.

### Step 8 — Route to repair skill

If a code fix is needed, hand off to `salesforce-apex-generator-skill` with the
specific finding as context. If a deeper test run is needed, route to
`salesforce-apex-test-runner-skill`.

---

## Quality Scoring Rubric (100-point)

Score the analysis quality before presenting. Threshold: 80+ acceptable.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Governor-limit identification** | 25 | All governor limit hits identified with limit name, current value, and limit ceiling; near-limit warnings included |
| **Root cause clarity** | 25 | Each finding traces to a specific class, method, and line number; not just "SOQL limit hit" but "AccountSelector.getByOwner line 45 in loop at TriggerHandler line 12" |
| **SOQL N+1 detection** | 15 | Repeated `SOQL_EXECUTE_BEGIN` entries in loop context identified and the responsible pattern named |
| **Sanitization quality** | 15 | All record IDs, user IDs, and PII values redacted in output; no raw log lines echoed; audit envelope populated |
| **Handoff routing** | 10 | Each Critical/Warning finding includes a specific next action and the skill to route to |
| **Audit envelope** | 10 | All required fields present; log ID hashed if PII risk; timestamp accurate |

**Scoring penalties:**
- Raw log lines with record IDs echoed: -20 (immediate caveat)
- Governor limit hit identified but not traced to source: -15
- SOQL in loop not flagged: -15
- Missing audit envelope: -15
- PII field values in output: score voided (immediate reject)

---

## T1 Least-Privilege Contract

This skill operates at T1 — read-only runtime.

- **OAuth scopes:** `api` and `refresh_token` only.
- **Run As account permissions:**
  - REQUIRED: `View Setup and Configuration`
  - DENIED: `ModifyAllData`, `ViewAllData`, `ViewEncryptedData`, `ModifyMetadata`,
    `AuthorApex`, `ManageConnectedApps`
- **No View All Data:** Unlike the test runner skill, this skill does NOT require
  `View All Data`. Log retrieval uses `sf apex get log` which operates under the
  `View Setup and Configuration` permission scope.
- **Read-only:** Retrieves log content only. No code execution, no DML, no metadata change.
- **Log content sensitivity:** Debug logs may contain field values from records processed
  during the transaction. Apply PII redaction rules strictly.

---

## Audit Envelope Schema

```yaml
audit_envelope:
  matter_id: "<caller-provided-or-generated-uuid>"
  skill_id: "salesforce-apex-log-analyzer-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"
  run_as_user_id: "<user_id_placeholder>"
  org_type_verified: "sandbox | production | unknown"
  log_id: "<log-id-or-hash-if-pii-risk>"
  log_size_bytes: <integer>
  transaction_entry_point: "<class or trigger name>"
  timestamp: "<ISO-8601-UTC>"
  pii_risk_assessed: true | false
  redactions_applied:
    - field_or_pattern: "<pattern>"
      reason: "<record_id|user_id|pii|session_token>"
  findings_count:
    critical: <integer>
    warning: <integer>
    info: <integer>
```

---

## Output Format

```yaml
verdict: "acceptable | caveat | reject"
quality_score: <0-100>
quality_notes: "<scoring rationale>"

transaction_summary:
  entry_point: "<class/trigger/batch>"
  transaction_type: "<trigger|class|batch|queueable|schedulable|rest>"
  org_alias: "<alias>"
  org_type: "sandbox | production | unknown"

findings:
  critical:
    - finding: "<title>"
      location: "<ClassName.method line N>"
      evidence: "<redacted log excerpt>"
      governor_limit: "<limit name if applicable>"
      current_value: <integer>
      limit_ceiling: <integer>
      suggested_fix: "<specific remediation>"
      route_to: "<skill or action>"
  warning:
    - finding: "<title>"
      location: "<ClassName.method line N>"
      evidence: "<redacted log excerpt>"
      suggested_fix: "<specific remediation>"
  info:
    - finding: "<title>"
      notes: "<optimization or hygiene note>"

soql_analysis:
  total_queries: <integer>
  governor_limit: 100
  n_plus_1_patterns:
    - query_excerpt: "<SELECT ... FROM ...>"
      invocation_count: <integer>
      loop_context: "<loop location>"
      suggested_fix: "<move SOQL outside loop; use Map pattern>"
  non_selective_queries: ["<query excerpt>"]

cpu_analysis:
  total_cpu_ms: <integer>
  cpu_limit_ms: 10000
  hotspots: ["<ClassName.method: N ms>"]

heap_analysis:
  peak_heap_bytes: <integer>
  heap_limit_bytes: 6291456
  large_allocations: ["<description>"]

audit_envelope:
  <see Audit Envelope Schema>

next_steps:
  - "<Critical/Warning: route to salesforce-apex-generator-skill for fix>"
  - "<if tests needed after fix: salesforce-apex-test-runner-skill>"

assumptions:
  - "<explicit list>"

missing_evidence:
  - "<what additional log context would help>"
```

---

## Redaction Rules

Apply in order before emitting any output. Never bypass for any reason.

1. **OAuth tokens, refresh tokens, session IDs:** Never include. Strip from any CLI output.
2. **Salesforce Org IDs (18-char starting with `00D`):** Replace with `<org_id_placeholder>`.
3. **Salesforce Record IDs (15/18-char alphanumeric):** Replace with `<record_id_placeholder>`
   in all log excerpts. Record IDs appear frequently in `USER_DEBUG`, `DML_BEGIN`, and
   `SOQL_EXECUTE_BEGIN` lines.
4. **User IDs (OwnerId, CreatedById, Running User ID, User.Id):** Replace with `<user_id_placeholder>`.
5. **Email addresses in log output:** Replace with `<email_placeholder>` unless the user
   explicitly acknowledges PII scope and the org is non-production.
6. **Phone numbers, SSNs, financial account numbers in log output:** Replace with
   `<pii_placeholder>`. Flag in `redactions_applied`.
7. **Session tokens and access tokens in USER_DEBUG lines:** Strip entirely. Do not include
   a placeholder that implies a token value was present.
8. **Instance URLs:** Replace with org alias in output. Do not emit raw instance URLs.
9. **Stack traces:** Retain class names and line numbers — required for diagnosis. Replace
   any record IDs embedded in exception messages.

---

## Handoff Rules

| Finding | Hand off to |
|---|---|
| Code fix needed (SOQL in loop, DML pattern, exception handling) | `salesforce-apex-generator-skill` |
| Test coverage gap surfaced by log | `salesforce-apex-test-generator-skill` |
| Verification run needed after fix | `salesforce-apex-test-runner-skill` |
| Permission or FLS finding in log | `salesforce-permission-model-review-skill` |
| Agentforce / Einstein AI trace in log | `salesforce-agentforce-stdm-observer-skill` |
| Critical finding needing deployment review | `salesforce-deployment-validator-skill` |

Required handoff fields: `matter_id`, `audit_envelope`, `findings` (sanitized), `next_steps`.

---

## Stop Conditions

Stop and do not continue if:

- Log retrieval fails and the user cannot provide log content directly — stop and explain
  that a log ID, user context, or pasted log content is required.
- Log content contains fields identified as encrypted (Shield PE / PMLE) — skip those
  entries, note the redaction, and analyze the remainder.
- The audit envelope cannot be completed — stop until matter_id or org alias is resolved.
- The user requests redaction to be disabled — stop and explain the policy.
- Log content is from a production org and contains PII fields — apply maximum redaction;
  if PII is pervasive and the matter classification does not permit, stop and escalate to
  the compliance specialist.

---

## Security Notes

- **T1 read-only runtime:** No code execution, no DML, no metadata mutation.
- **No View All Data required:** Log retrieval operates under `View Setup and Configuration`
  only. This distinguishes this skill from the test runner and reduces its permission footprint.
- **Log content sensitivity:** Apex debug logs may capture field values, user data, and API
  payloads. PII and encrypted field redaction is mandatory.
- **Sanitized output only:** All record IDs, user IDs, session tokens, and PII values
  redacted before emission. Raw log lines are never echoed.
- **Structured audit:** Every execution produces a complete audit envelope including
  `pii_risk_assessed` and `redactions_applied` fields.
- **Revocable:** Rotating the Run As account's refresh token immediately revokes all access.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/log-format-reference.md` | Apex log levels, log line categories (USER_DEBUG, METHOD_ENTRY, SOQL_EXECUTE_BEGIN, LIMIT_USAGE_FOR_NS, FATAL_ERROR, etc.) |
| `references/governor-limit-signatures.md` | Common governor limit hit patterns, limit ceilings, and remediation strategies |
| `references/redaction-rules.md` | Detailed redaction patterns for record IDs, user IDs, session tokens, PII field values; jq and grep patterns for automated stripping |
