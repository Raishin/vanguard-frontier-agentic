---
name: salesforce-metadata-fetcher-skill
description: "Fetches Salesforce metadata (objects, fields, flows, validation rules, permission sets, profiles, Apex classes/triggers, Lightning components) live from a connected org under T1 least-privilege scope (api + refresh_token only, no ModifyMetadata grant — uses sf org list metadata and REST describe endpoints requiring only View Setup and Configuration). Sanitizes output (redacts org IDs, user IDs, hardcoded values) and feeds downstream review skills: salesforce-metadata-review-skill, salesforce-flow-automation-review-skill, salesforce-permission-model-review-skill, salesforce-apex-lwc-code-review-skill. TRIGGER when: user asks to fetch metadata live, retrieve object schema, list flows, list permission sets, retrieve Apex classes, fetch validation rules, or wants live org schema rather than pasting XML. Trigger phrases: fetch metadata, retrieve from org, show me the schema, list my flows, get object describe, pull validation rules from prod. DO NOT TRIGGER when: only data records are needed (use salesforce-soql-explorer-skill); user has already pasted metadata XML (use salesforce-metadata-review-skill directly); request requires deploying metadata (T2/T3 — use salesforce-deployment-validator-skill)."
license: MIT
allowed-tools: Bash(sf org list metadata:*) Bash(sf project retrieve:*) Bash(sf sobject describe:*) Bash(sf org display:*) Read Grep Glob
metadata:
  author: "github: Raishin"
  version: 0.1.0
  updated: 2026-05-21
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["View Setup and Configuration"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps", "Customize Application"]
---

# salesforce-metadata-fetcher-skill

Fetches Salesforce metadata live from a connected org under T1 least-privilege scope,
sanitizes output, and routes sanitized payloads to the appropriate downstream review skill.
This skill **eliminates the hand-paste requirement** across the entire Vanguard Salesforce
review portfolio — admins no longer need to manually export XML and paste it into review skills.

---

## When This Skill Owns the Task

This skill owns the task when the user wants **live metadata from a connected org** rather
than supplying a pre-exported file.

### Explicit ownership signals

- "Fetch the Account object schema from my org"
- "List all flows in production"
- "Show me the PermissionSet called Sales_Rep"
- "Retrieve the ContactTrigger Apex class"
- "Get the validation rules on Opportunity"
- "Pull the LWC bundle for accountCard from dev org"

### Explicit delegation routing (do not own these tasks)

| Request type | Delegate to |
|---|---|
| Live SOQL record queries | `salesforce-soql-explorer-skill` |
| Static review of already-pasted XML | `salesforce-metadata-review-skill` |
| Deploy or validate a deployment | `salesforce-deployment-validator-skill` (T2) |
| Full org posture assessment | `salesforce-org-assessment-skill` |
| Permission topology across profiles + PSGs | `salesforce-permission-model-review-skill` |
| Flow logic review (user has flow XML) | `salesforce-flow-automation-review-skill` |
| Apex code review (user has class code) | `salesforce-apex-lwc-code-review-skill` |
| Certificate lifecycle / Connected App review | `salesforce-integration-agent` + `salesforce-certificate-lifecycle-agent` |

---

## Required Context to Gather First

Before executing any CLI command, confirm all of the following:

1. **Target org alias** — the authenticated org alias (`sf org list` to verify connectivity). Never default to the defaultusername if the user has not confirmed the target.
2. **Metadata type(s) requested** — must be from the allowed list below. If the user's request maps to an unlisted type, surface the gap and ask to confirm before proceeding.
3. **Sensitivity classification** — is this a production org? sandbox? regulated vertical (healthcare, financial services, government)? Regulated-vertical production orgs require explicit human confirmation before any retrieval.
4. **Downstream review skill (if known)** — if the user already knows what review they want, declare the handoff destination before fetching so the routing is explicit.

### Allowed metadata types (T1 scope)

The following types can be fetched without requiring `ModifyMetadata` permission via the REST describe or list paths:

- `CustomObject` and `CustomField` — via `/sobjects/<Object>/describe`
- `Flow` — via Tooling API `/tooling/sobjects/Flow`
- `ValidationRule` — via Tooling API
- `PermissionSet` — via `sf org list metadata --metadata-type PermissionSet`
- `Profile` — via `sf org list metadata --metadata-type Profile` (list only; full retrieval is elevated — see note)
- `ApexClass` — via `sf org list metadata --metadata-type ApexClass` + `sf project retrieve start` (Tooling API preferred)
- `ApexTrigger` — via Tooling API `/tooling/sobjects/ApexTrigger`
- `LightningComponentBundle` (LWC) — via `sf org list metadata --metadata-type LightningComponentBundle`
- `AuraDefinitionBundle` (Aura) — via `sf org list metadata --metadata-type AuraDefinitionBundle`
- `CustomMetadata` — via REST describe (type definitions only, not record values)
- `CustomSetting` — via REST describe (type definitions only)

**Profile full retrieval note:** Full profile XML retrieval via `sf project retrieve start` requires `Customize Application` or elevated permissions on some orgs. Prefer listing profiles and confirming with the user before full retrieval.

---

## Recommended Workflow

### Step 1 — Verify org connectivity

```bash
sf org display --target-org <alias>
```

- Confirm `Status: Connected` and that the `Access Token Expiry` is not expired.
- Record the org type (production vs. sandbox) from the instance URL pattern.
- If org type is ambiguous, check whether the URL contains `.sandbox.` or `.scratch.`.
- **Stop if:** connectivity fails. Do not proceed without a confirmed connected org.

### Step 2 — Confirm metadata type is in the allowed list

Before issuing any retrieval command:

1. Map the user's request to a concrete metadata type from the allowed list.
2. If the type is not on the allowed list, explain which type is closest and ask the user to confirm.
3. If the type is on the elevated list (e.g., full Profile XML, Connected App), surface the elevated requirement before proceeding and require explicit user confirmation.
4. Record the confirmed metadata type in the session context.

### Step 3 — List available metadata

Prefer listing before retrieving. This step has the lowest privilege footprint and confirms what is actually in the org.

```bash
sf org list metadata --metadata-type <Type> --target-org <alias>
```

- For object fields: use `/services/data/vXX.X/sobjects/<Object>/describe` via REST.
- Capture the list output. Do not proceed to retrieval if the list is empty (nothing to fetch).
- If the list is large (> 50 results), surface a summary and ask the user to confirm which specific items to retrieve.

### Step 4 — Prefer REST describe over `sf project retrieve start`

`sf project retrieve start` writes files to disk and may require `Modify Metadata` on some orgs. Prefer REST describe paths wherever possible:

**Object describe (no `ModifyMetadata` required):**
```bash
sf sobject describe --sobject <ObjectApiName> --target-org <alias>
```

**Flow via Tooling API (requires only View Setup and Configuration):**
```bash
# List flows
sf org list metadata --metadata-type Flow --target-org <alias>

# Retrieve specific flow definition via REST
# GET /services/data/v62.0/tooling/sobjects/Flow/<flowId>
```

**Apex class via Tooling API:**
```bash
# GET /services/data/v62.0/tooling/sobjects/ApexClass/<classId>
```

Only fall back to `sf project retrieve start` when no REST equivalent exists, and always declare the elevated path in the audit envelope.

### Step 5 — Sanitize output

Apply all sanitization rules from `references/sanitization-rules.md` before passing output to any downstream skill or surfacing it to the user:

1. Redact org IDs (15/18-char Salesforce ID format starting with `00D`).
2. Redact user IDs (15/18-char format starting with `005`).
3. Redact profile/permission set IDs (starting with `00e`).
4. Scan field defaults for email addresses, phone numbers, and account-name-like strings — flag and redact.
5. Scan for URL credentials (`https://user:pass@`) — reject the entire payload and stop.
6. Scan for Named Credential headers — reject and stop.
7. Identify encrypted field markers (Shield Platform Encryption, PMLE) — skip those fields entirely; do not attempt to read their values.
8. Scan Apex class body for hardcoded session ID literals — flag as Critical, escalate, do not pass to downstream skill until resolved.

Sanitization must complete before any output is emitted. If sanitization cannot be confirmed (e.g., the payload is too large to inspect), surface this as a gap and do not proceed.

### Step 6 — Emit audit envelope

Every retrieval operation must emit a structured audit envelope before handing off to a downstream skill. See **Audit Envelope Schema** below. The envelope must include:

- `operation` — the CLI command or REST path used
- `metadata_type` — the type retrieved
- `org_type` — production or sandbox
- `org_id_placeholder` — `<org_id_placeholder>` (never the real org ID)
- `run_as_user_id_placeholder` — `<user_id_placeholder>`
- `items_retrieved` — count
- `sanitization_applied` — boolean
- `timestamp` — ISO 8601
- `elevated_path_used` — boolean (true if `sf project retrieve start` was used instead of REST)
- `escalation_triggers_fired` — list of any stop conditions that fired

### Step 7 — Hand off sanitized output to the appropriate downstream skill

Use the delegation routing table in `references/delegation-routing.md` to determine the correct downstream skill. Emit the required handoff fields for that route.

Every handoff must include:
- The sanitized metadata payload (structured YAML/JSON, not raw XML unless the downstream skill specifically requires XML)
- The audit envelope
- The `downstream_skill_recommendation` field identifying which skill to invoke next

### Step 8 — Stop conditions

Stop and do not proceed if any of the following are true:

- The retrieved payload contains what appears to be encrypted field data (Shield PE or PMLE markers present).
- A field default or formula contains what appears to be a real secret, token, or API key (pattern matches token-like strings).
- The org is a regulated-vertical production org and no compliance review skill is declared in the session context.
- Sanitization cannot be fully applied to the payload.
- URL-embedded credentials are found anywhere in the payload.
- A Named Credential header value is exposed in the retrieval output.
- The Apex class body contains a hardcoded `UserInfo.getSessionId` call being stored in a field or sent externally.

---

## Quality Scoring Rubric (100-point)

Score the output of this skill before handing off. Outputs scoring below 60 must be rejected. Outputs scoring 60–79 may proceed with a caveat flag. Outputs scoring 80+ are acceptable.

| Dimension | Max points | Scoring guidance |
|---|---|---|
| **Completeness** | 25 | All requested metadata types retrieved and present in output. Partial retrieval: deduct 5 per missing type. Empty retrieval: 0. |
| **Sanitization quality** | 35 | All org IDs, user IDs, and profile IDs redacted: 20 pts. Field default scan complete: 8 pts. Encrypted field markers identified and skipped: 7 pts. Any unredacted Salesforce ID found: deduct 20. Any real credential found unredacted: 0 pts for entire dimension + stop. |
| **Audit envelope** | 15 | All required envelope fields present: 15 pts. Missing 1–2 fields: 8 pts. Missing > 2 fields or envelope absent: 0 pts. |
| **Proper delegation routing** | 15 | Downstream skill correctly identified and declared: 15 pts. Wrong skill recommended: 5 pts. No recommendation made: 0 pts. |
| **Governor-limit awareness** | 10 | Large payloads (> 50 items) summarized before full retrieval; user confirmation obtained: 10 pts. Large retrieval without confirmation: 0 pts. |

---

## T1 Least-Privilege Contract

This skill operates under the T1 read-only-runtime tier as defined in `docs/salesforce-wave-4-plan.md`.

### OAuth scopes

| Scope | Required | Rationale |
|---|---|---|
| `api` | Yes | Enables REST API and Tooling API calls |
| `refresh_token` | Yes | Allows token refresh without re-authentication |
| `full` | **Denied** | Excessive; grants admin-level access |
| `web` | **Denied** | Not required for CLI/API operations |
| `sfap_api` | **Denied** | Agentforce platform scope; out of T1 scope |
| `cdp_query_api` | **Denied** | Data Cloud scope; out of T1 scope |

### Run As service account permissions

| Permission | Status |
|---|---|
| View Setup and Configuration | **Required** |
| Per-object Read FLS (on target objects) | Required |
| ModifyAllData | **Denied** |
| ViewAllData (system-level) | **Denied** |
| ViewEncryptedData | **Denied** |
| ModifyMetadata | **Denied** (REST describe paths avoid this) |
| AuthorApex | **Denied** |
| ManageConnectedApps | **Denied** |
| Customize Application | **Denied** (flag as elevated if full Profile XML is needed) |

### CLI command scope

Only the following CLI command families are pre-approved:

- `sf org display` — connectivity check only
- `sf org list metadata` — enumeration only, no retrieval
- `sf sobject describe` — REST describe, read-only
- `sf project retrieve start` — **elevated path**, must be declared in audit envelope; only used when no REST equivalent exists

Commands that are explicitly out of scope:
- `sf project deploy` (any form) — T2/T3
- `sf data` (DML) — T2/T3
- `sf apex run` — T2/T3
- Any command with `--full` flag — excessive scope

---

## Refusal Triggers

Stop immediately and do not emit output (except a structured refusal message) if:

1. The user requests retrieval of all metadata types at once (`*` wildcard or "dump everything").
2. The target org appears to be a production org in a regulated vertical (healthcare, financial services, government) and the user has not confirmed compliance review coverage.
3. The metadata type requested is `EncryptionKey`, `TenantSecret`, `ManagedContentType`, or any type that directly exposes encryption configuration.
4. The retrieval output contains URL-embedded credentials.
5. The retrieval output contains Named Credential header values.
6. A hardcoded session ID literal is found in an Apex class body being passed externally.
7. The org alias is `production`, `prod`, `PROD`, or any variant that suggests a production org and the metadata type includes Profile or PermissionSet full retrieval — require explicit confirmation.

**Refusal message format:**

```yaml
refusal:
  trigger: [which refusal condition fired]
  reason: [plain-language explanation]
  recommended_action: [what the user should do instead]
  escalation_required: [true/false]
```

---

## Audit Envelope Schema

Every retrieval operation must emit this envelope. Fields marked `required` must be present for the output to be accepted by downstream skills.

```yaml
audit_envelope:
  skill_id: salesforce-metadata-fetcher-skill          # required
  skill_version: "0.1.0"                              # required
  operation: "<sf command or REST path used>"          # required
  metadata_type: "<Type>"                              # required
  org_type: "production | sandbox | scratch | unknown" # required
  org_id_placeholder: "<org_id_placeholder>"           # required; never the real org ID
  run_as_user_id_placeholder: "<user_id_placeholder>"  # required; never the real user ID
  items_retrieved: <integer>                            # required
  sanitization_applied: true                           # required; false triggers rejection
  elevated_path_used: <boolean>                        # required; true if sf project retrieve start was used
  timestamp: "<ISO 8601>"                              # required
  escalation_triggers_fired: []                        # required; empty list if none
  quality_score: <integer 0-100>                       # required
```

---

## Output Format

All output from this skill is YAML. Raw XML from `sf project retrieve start` must be converted to structured YAML before emission.

```yaml
salesforce_metadata_fetch:
  sanitized_metadata:
    metadata_type: "<Type>"
    items:
      - name: "<ApiName>"
        label: "<Label>"
        # Type-specific fields follow — see delegation-routing.md for required fields per type

  metadata_summary:
    total_items: <integer>
    types_retrieved: [<list of types>]
    items_skipped: <integer>
    skip_reasons: [<list of reasons — e.g., "encrypted field skipped", "field default redacted">]

  fls_notes:
    # For object/field retrieval: list fields where FLS was not readable
    inaccessible_fields: [<list>]
    encrypted_fields_skipped: [<list>]

  audit_envelope:
    # Full audit envelope as defined above

  downstream_skill_recommendation:
    skill_id: "<downstream-skill-id>"
    rationale: "<why this skill was chosen>"
    required_handoff_fields:
      # Type-specific handoff fields as defined in references/delegation-routing.md

  escalation_triggers_fired: []  # empty if none
  missing_evidence: []           # gaps that would improve coverage
  assumptions: []                # any assumptions made during retrieval
```

---

## Redaction Rules

Precise redaction rules are documented in `references/sanitization-rules.md`. Summary:

| Pattern | Action |
|---|---|
| Org ID: `00D[A-Za-z0-9]{12,15}` | Replace with `<org_id_placeholder>` |
| User ID: `005[A-Za-z0-9]{12,15}` | Replace with `<user_id_placeholder>` |
| Profile/PermSet ID: `00e[A-Za-z0-9]{12,15}` | Replace with `<profile_id_placeholder>` |
| Email address in field default | Flag and redact; note in `skip_reasons` |
| Phone number in field default | Flag and redact; note in `skip_reasons` |
| URL with embedded credentials | **Reject entire payload; stop** |
| Named Credential header value | **Reject entire payload; stop** |
| Shield PE / PMLE encrypted field | Skip field entirely; do not read value |
| Hardcoded session ID in Apex | Flag as Critical; escalate before handoff |
| Token-like string (> 20 chars, high entropy) in field default | Flag and redact; note in `skip_reasons` |

Hardcoded ID values must never be repeated verbatim in output. Describe the pattern and location only.

---

## Handoff Rules

Full routing map is in `references/delegation-routing.md`. Summary:

| Metadata type retrieved | Downstream skill | Key handoff fields |
|---|---|---|
| `CustomObject`, `CustomField` (ObjectDescribe) | `salesforce-metadata-review-skill` | `object_summary`, `field_inventory`, `validation_rules_summary` |
| `Flow` | `salesforce-flow-automation-review-skill` | `flow_xml_sanitized`, `fault_path_present`, `automation_mix_summary` |
| `PermissionSet`, `Profile` | `salesforce-permission-model-review-skill` | `permission_set_summary`, `system_perms_granted`, `object_perms_summary`, `fls_summary` |
| `ApexClass`, `ApexTrigger` | `salesforce-apex-lwc-code-review-skill` | `class_name`, `with_sharing_status`, `soql_count`, `complexity_indicators` |
| `LightningComponentBundle`, `AuraDefinitionBundle` | `salesforce-apex-lwc-code-review-skill` | `component_name`, `js_imports`, `apex_calls`, `lwc_security_concerns` |
| `ConnectedApp` | `salesforce-integration-agent` + `salesforce-certificate-lifecycle-agent` | `oauth_scopes`, `ip_relaxation`, `certificate_thumbprint` |

---

## Stop Conditions

Stop processing and emit a structured stop message (not a refusal — stop conditions fire mid-execution, refusal triggers fire pre-execution) when:

1. Retrieved payload contains Shield PE or PMLE encrypted field markers.
2. Field default or Apex class body contains a suspected secret, token, or API key.
3. Org is a regulated-vertical production org with no compliance review skill declared.
4. Sanitization cannot be fully applied to the payload.
5. URL-embedded credentials found in retrieval output.
6. Named Credential header value exposed in retrieval output.
7. Apex class body contains `UserInfo.getSessionId` result being stored externally or in a field default.

**Stop message format:**

```yaml
stop:
  condition_fired: [which condition]
  mid_execution: true
  items_sanitized_before_stop: <integer>
  items_not_sanitized: <integer>
  recommended_action: [what the user or operator should do]
  escalation_required: true
  audit_envelope:
    # Partial audit envelope up to point of stop
```

---

## Security Notes

- This skill is **read-only at runtime**. It never writes to the org, never deploys metadata, and never executes Apex.
- The Run As service account must not hold `ModifyMetadata`, `AuthorApex`, or `ManageConnectedApps`. If the org CLI session has these permissions, surface a warning before executing — the T1 contract is violated.
- Org IDs and user IDs are never repeated verbatim in any output. Placeholders are always used.
- REST describe paths are preferred over `sf project retrieve start` because they have a lower permission footprint. When `sf project retrieve start` is used, it is declared in the audit envelope as an elevated path.
- Downstream review skills receive sanitized YAML, not raw XML, minimizing the risk of accidentally propagating unsanitized values.
- This skill does not store, cache, or persist any org metadata beyond the active session context.
- For regulated-vertical orgs (healthcare, financial services, government), explicit operator confirmation is required before any production retrieval. Sandbox retrievals from regulated orgs are permitted without additional confirmation.
