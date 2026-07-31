---
name: salesforce-soql-explorer-skill
description: "Executes read-only SOQL queries against a connected Salesforce org via the sf data query CLI under T1 least-privilege scope (api + refresh_token only, Run As service account with no ModifyAllData/ViewAllData/ViewEncryptedData). Returns sanitized JSON with a structured audit envelope. Live operational counterpart to the static-review skills. TRIGGER when: user asks to query records, run SOQL, fetch live data, inspect records by ID, count records, run aggregate queries, or check field values in a live org. Trigger phrases: query my org, run SOQL, show me records where, how many opportunities, what is the value of field X on record Y. DO NOT TRIGGER when: user pastes a metadata XML export for static review (use salesforce-metadata-review-skill); request requires DML — write, update, delete — those are T3 prohibited; bulk data operations needed (use salesforce-bulk-data-ops-skill); only schema metadata needed without data (use salesforce-metadata-fetcher-skill)."
license: MIT
allowed-tools: Bash(sf data query:*) Bash(sf org list:*) Bash(sf org display:*) Read Grep Glob Bash(sf sobject describe:*) Bash(jq:*)
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
    required: ["View Setup and Configuration"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps"]
---

# salesforce-soql-explorer-skill

Read-only live SOQL execution against a connected Salesforce org under T1
least-privilege scope. This skill is a **flashlight**, not a filing cabinet.
It queries evidence; it does not write, deploy, or mutate anything.

## When This Skill Owns the Task

Use `salesforce-soql-explorer-skill` when the work requires **live record evidence**
from a connected org:

- "Show me the Opportunity pipeline for Q3 in sandbox"
- "How many Contacts have no email address?"
- "What is the value of Account.BillingCountry for record 001Xx000001ABC?"
- "Count open Cases by priority for the service team"
- "Verify the field value changed after yesterday's data load"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| User pastes metadata XML or deployment export for review | `salesforce-metadata-review-skill` |
| Schema or field definitions needed without record values | `salesforce-metadata-fetcher-skill` |
| Request requires INSERT, UPDATE, DELETE, MERGE, or UPSERT | T3 — requires human approval via `salesforce-live-guard-agent` |
| Large-volume export (> 2,000 records) or scheduled batch | `salesforce-bulk-data-ops-skill` |
| Static code review of a `.soql` file or Apex selector | `querying-soql` (sf-skills) |
| A change proposal emerges from findings | Escalate to `salesforce-live-guard-agent` |
| Permission topology findings surface | Hand off to `salesforce-permission-model-review-skill` |

---

## Required Context to Gather First

Before executing any query, confirm:

1. **Target org alias** — the `--target-org` value recognized by `sf org list`.
   Never accept a raw instance URL or session token.
2. **Org type** — production or sandbox. Flag if production; apply stricter
   scrutiny on field selection and result volume.
3. **Target sObject** — the API name (e.g., `Account`, `Opportunity`,
   `My_Custom_Object__c`).
4. **Fields needed** — enumerate explicitly; do not use SELECT *.
5. **Filter criteria** — WHERE clause conditions, date ranges, owner scope.
6. **Expected result volume** — inform LIMIT choice.
7. **Sensitivity classification** — does the query touch PII fields (email,
   phone, address, SSN, health, financial)? Regulated-vertical indicator
   (Health Cloud, Financial Services Cloud
)?
   Encrypted fields (Shield PE / PMLE)?

If any of these are missing, ask before proceeding.

---

## Recommended Workflow

### Step 1 — Confirm org alias and reachability

```bash
sf org display --target-org <alias>
```

Verify: org type (production vs. sandbox), username, instance URL, OAuth
Connected App. If the org type is production and the Connected App allowlist
does not explicitly authorize this alias, **stop** (see Stop Conditions).

### Step 2 — Confirm sObject exists and check FLS

```bash
sf sobject describe --sobject <SObjectName> --target-org <alias>
```

Confirm: the sObject exists, the Run As account has Read access, and the
requested fields are accessible (not restricted by FLS). If any field is
encrypted (Shield PE / PMLE), remove it from the query and note the redaction.

### Step 3 — Generate the simplest correct query

Apply these constraints:
- Enumerate only required fields — no `SELECT *`
- Apply a selective WHERE clause using indexed fields (Id, Name, ExternalId,
  lookup fields, standard indexed fields)
- Include LIMIT (default 200; reduce for PII-adjacent queries)
- Avoid formula fields or non-indexed fields in WHERE without a companion
  indexed filter

### Step 4 — Preview with LIMIT 5 first

```bash
sf data query \
  --query "SELECT <fields> FROM <SObject> WHERE <filter> LIMIT 5" \
  --target-org <alias> \
  --result-format json
```

Inspect output for: unexpected fields, encrypted placeholders, PII exposure,
schema surprises. Confirm the shape matches the intent before expanding volume.

### Step 5 — Execute with appropriate LIMIT

```bash
sf data query \
  --query "SELECT <fields> FROM <SObject> WHERE <filter> LIMIT <n>" \
  --target-org <alias> \
  --result-format json
```

Maximum interactive LIMIT: 2,000. For larger volumes, decline and route to
`salesforce-bulk-data-ops-skill`.

### Step 6 — Sanitize output

Apply redaction rules before emitting any output:

- Replace all 15/18-character Salesforce Ids with `<org_id_placeholder>` or
  `<record_id_placeholder>` as appropriate.
- Replace user IDs (User.Id references, OwnerId, CreatedById, LastModifiedById)
  with `<user_id_placeholder>`.
- Remove or mask any field that contains plaintext email, phone, SSN, or
  financial account numbers unless the user has explicitly acknowledged the
  PII scope and the org is not regulated.
- Skip any encrypted field (Shield PE / PMLE) entirely — do not include
  placeholder text that implies a value was present.
- Never emit OAuth tokens, refresh tokens, or session IDs.

Use `jq` for structured redaction:

```bash
sf data query \
  --query "SELECT Id, Name FROM Account LIMIT 5" \
  --target-org <alias> \
  --result-format json \
  | jq '.result.records[] | {Id: "<record_id_placeholder>", Name: .Name}'
```

### Step 7 — Emit audit envelope

Every execution must produce a structured audit envelope (see Audit Envelope
Schema below). Do not omit the envelope even if results are empty.

### Step 8 — Hand off to review skill if findings warrant

If query results surface access anomalies, unexpected field values, or policy
violations, hand off to the appropriate review skill with the sanitized results
and the audit envelope as the input payload. Do not escalate raw unsanitized
output.

---

## Quality Scoring Rubric (100-point)

Score the query design and execution quality before emitting results. Threshold:
80+ acceptable, 60–79 emit with caveat, below 60 reject and request revision.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Selectivity** | 30 | WHERE clause uses at least one indexed field; no full table scans on objects > 10k records |
| **Field minimality** | 20 | Only required fields enumerated; no SELECT *; no encrypted fields included |
| **Governor limit awareness** | 20 | LIMIT applied and appropriate to volume; no queries likely to exceed 50k row limit; aggregate used when count is the goal |
| **Redaction quality** | 15 | All Salesforce IDs, user IDs, PII fields redacted per rules; audit envelope populated |
| **Audit envelope completeness** | 15 | All required audit fields present; query text hash computed; timestamp accurate |

**Scoring penalties:**
- No WHERE clause on object with > 50k records: -20
- SELECT *: -15
- Missing LIMIT: -15
- Missing audit envelope: -20 (automatic caveat regardless of total score)
- Encrypted field included in output: immediate reject (score voided)

---

## T1 Least-Privilege Contract

This skill operates exclusively at T1 — read-only runtime. The contract is:

- **OAuth scopes used:** `api` and `refresh_token` only. No `full`, `web`,
  `sfap_api`, `cdp_query_api`, or any other scope.
- **Run As account profile:** System permissions: View Setup and Configuration
  only. Object permissions: Read only on objects in scope. FLS restricted to
  non-PII, non-encrypted fields by default.
- **Denied permissions (enforced at Connected App and profile level):**
  - Modify All Data
  - View All Data (system-level bypass)
  - View Encrypted Data
  - Modify Metadata Through Metadata API Functions
  - Author Apex
  - Customize Application
  - Manage Connected Apps
- **Org allowlist:** Enforced by Connected App IP restrictions and explicit
  org alias allowlist. Skill verifies via `sf org list` that the target alias
  is in the authorized set before executing any query.
- **No DML under any circumstances:** This skill will not construct or execute
  any statement containing INSERT, UPDATE, DELETE, MERGE, or UPSERT. Requests
  for DML must be refused and routed to the human approval path.
- **Revocation:** The least-privilege Run As account's refresh token can be
  rotated to instantly revoke all access without affecting other integrations.

---

## Refusal Triggers

Stop immediately and do not execute if:

- The target org appears to be production but the Connected App allowlist does
  not include it.
- The requested SOQL contains DML keywords: `INSERT`, `UPDATE`, `DELETE`,
  `MERGE`, `UPSERT` (case-insensitive).
- The query targets fields marked as encrypted (Shield PE / PMLE indicators
  in the `describe` output: `encrypted: true`).
- The audit envelope cannot be populated (matter_id missing, org alias
  unresolvable, run_as_user_id unavailable).
- The user requests that redaction be skipped or disabled.
- The org is identified as a regulated-vertical production org (Health Cloud,
  Financial Services Cloud
) and
  jurisdiction is unknown.
- The query result volume exceeds 2,000 records in interactive mode — route to
  bulk ops instead.
- The Run As account is missing the required `View Setup and Configuration`
  permission (verify via `sf org display` output).

---

## Audit Envelope Schema

Every execution emits an audit envelope. The envelope travels with the
sanitized output to any downstream review skill.

```yaml
audit_envelope:
  matter_id: "<caller-provided-or-generated-uuid>"
  skill_id: "salesforce-soql-explorer-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"             # never the raw org ID
  run_as_user_id: "<user_id_placeholder>" # placeholder; never real ID in output
  query_text_hash: "<sha256-of-query>"    # hash only if PII risk; else include query
  query_text: "<soql-string-or-redacted>" # omit if PII risk; include hash instead
  record_count: <integer>
  redactions_applied:
    - field: "<FieldApiName>"
      reason: "<pii|encrypted|org_id|user_id>"
  timestamp: "<ISO-8601-UTC>"
  org_type_verified: "sandbox | production"
  governor_limit_headroom: "<estimated-rows-vs-limit>"
```

---

## Output Format

```yaml
verdict: "acceptable | caveat | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

records:
  - <sanitized record objects>

metadata:
  record_count: <integer>
  sObject: "<SObjectApiName>"
  fields_queried: ["<field1>", "<field2>"]
  fls_notes: "<any FLS restrictions observed>"
  fields_skipped_encrypted: ["<field>"]   # if any

audit_envelope:
  <see Audit Envelope Schema>

escalation_triggers_fired:
  - "<trigger name or 'none'>"

missing_evidence:
  - "<what would strengthen the query or findings>"

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

Apply in order. Do not bypass for any reason.

1. **OAuth tokens, refresh tokens, session IDs:** Never include in any output,
   log, or audit envelope field. If they appear in CLI output, strip before
   emitting.
2. **Salesforce Org IDs (18-char starting with `00D`):** Replace with
   `<org_id_placeholder>`.
3. **Salesforce Record IDs (15/18-char):** Replace with
   `<record_id_placeholder>` in output records.
4. **User IDs (OwnerId, CreatedById, LastModifiedById, User.Id):** Replace
   with `<user_id_placeholder>`.
5. **Encrypted fields (Shield PE / PMLE):** Skip entirely — do not emit
   the field name or any placeholder that implies a value was retrieved.
   List skipped fields in `fields_skipped_encrypted`.
6. **PII fields (email, phone, SSN, health data, financial account numbers):**
   Mask or omit unless the user has explicitly acknowledged the PII scope,
   the org is non-production, and the matter classification permits. Document
   the acknowledgment in `assumptions`.
7. **Instance URLs and API endpoints:** Omit from output; reference only the
   org alias in the audit envelope.

---

## Handoff Rules

When findings from query results warrant further review, hand off to the
appropriate skill with the sanitized output and audit envelope as the payload:

| Finding type | Hand off to |
|---|---|
| Metadata anomalies (field config, object structure) | `salesforce-metadata-review-skill` |
| Access or permission findings (unexpected record visibility) | `salesforce-permission-model-review-skill` |
| A change proposal emerges from query evidence | `salesforce-live-guard-agent` |
| Org posture concern from query results | `salesforce-org-assessment-skill` |

Required handoff fields: `matter_id`, `audit_envelope`, `sanitized_records`
(summary — not full dump), `escalation_triggers_fired`, `missing_evidence`,
`assumptions`.

---

## Stop Conditions

Stop and do not continue if:

- Target org appears to be production but Connected App allowlist excludes it
  — stop, emit a refusal with reason, do not execute query.
- Query contains DML keywords (`INSERT`, `UPDATE`, `DELETE`, `MERGE`,
  `UPSERT`) — stop, emit a refusal, route to human approval path.
- Query targets encrypted fields (`encrypted: true` in describe output) —
  remove fields and warn, or stop if the user insists on including them.
- Audit envelope cannot be completed (missing matter_id or unresolvable org
  alias) — stop until resolved.
- Run As account is missing `View Setup and Configuration` — stop and escalate
  to org administrator.
- Result volume would exceed 2,000 records in interactive mode — stop and
  route to `salesforce-bulk-data-ops-skill`.
- The user requests redaction be disabled — stop and explain the policy.

---

## Security Notes

- **T1 read-only operational:** No DML, no metadata mutation, no Apex
  execution, no deployment.
- **Sanitized output only:** All Salesforce IDs, user IDs, and PII fields
  redacted before emission.
- **Org allowlist enforced:** Connected App restricts which orgs can be
  targeted; skill verifies before executing.
- **Structured audit emitted:** Every execution produces a complete audit
  envelope regardless of result count.
- **Revocable:** Rotating the Run As account's refresh token immediately
  revokes all access without affecting other integrations.
- **Least-privilege Run As account:** No Modify All Data, View All Data,
  View Encrypted Data, or any mutation permission.
- **No credential echo:** OAuth tokens, refresh tokens, and session IDs are
  never included in output or audit envelopes.
- **Regulated-vertical escalation:** Health Cloud and Financial Services Cloud
orgs trigger mandatory escalation
  to a qualified compliance specialist before results are shared externally.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/cli-commands.md` | sf CLI query commands, output formats, jq patterns, org introspection |
| `references/least-privilege-scope.md` | Connected App config, Run As profile design, denied permissions, token rotation |
| `references/safe-query-patterns.md` | Safe SOQL patterns, indexing rules, anti-patterns, annotated examples |
