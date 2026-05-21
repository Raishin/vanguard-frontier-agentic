# Privacy Redaction Rules

Agentforce-specific redaction policy for `salesforce-agentforce-stdm-observer-skill`.

This skill operates on aggregate metrics only. Session text content — user
messages, agent responses, LLM prompt/response pairs — is never emitted. This
is a structural design decision, not a configurable option.

**Verify-before-merge:** Agentforce session data handling requirements and
applicable data residency rules vary by Salesforce product edition, org region,
and customer data classification. Validate against your organization's data
governance policy and the Salesforce Data Processing Addendum before deploying
this skill in a regulated environment.
<!-- verify-before-merge:2026-05-21 -->

---

## Why Session Content Is Never Emitted

Agentforce sessions capture live conversational exchanges between users and the
agent. That content may include:

- User names, contact information, account details
- Health, financial, or HR data depending on agent scope
- Customer complaints, support issues, or sensitive business context
- Internal record IDs that could be combined with other data to identify individuals
- In regulated verticals (Health Cloud, Financial Services Cloud): PHI, PII, or
  regulated financial information

Even a single session that appears to contain only "generic" content cannot be
safely classified as PII-free without examining it — which creates a circular
dependency. The aggregate-only policy sidesteps this problem entirely: the skill
never touches session content, so there is no PII surface to manage in output.

If raw session content is genuinely required (e.g., a developer must debug a
specific hallucination in a production session), that access must be:

1. Requested through `salesforce-live-guard-agent` for human-in-the-loop
   confirmation.
2. Performed by a human operator with appropriate data access permissions,
   not by an automated agent.
3. Handled according to the org's data classification and retention policy.

See the Human-in-the-Loop Path section for the routing procedure.

---

## Redaction Rules (Apply in Order)

### Rule 1: Session text content — absolute prohibition

**Never** include any of the following in output, intermediate files, audit
envelopes, or log entries:

- `ssot__ContentText__c` (AiAgentInteractionMessage — raw user/agent messages)
- `ssot__InputValueText__c` (AiAgentInteractionStep — action input data)
- `ssot__OutputValueText__c` (AiAgentInteractionStep — action output data)
- `ssot__PreStepVariableText__c` / `ssot__PostStepVariableText__c` (variable snapshots)
- `prompt` / `llm_response` from `getLlmStepDetails()` return values
- `request_summary` / `response_summary` from `getMomentInsights()` return values
  (LLM-synthesized paraphrases — still potentially sensitive)
- `quality_reasoning` from moment insights (LLM-generated explanation referencing
  session content)

**What to emit instead:** numeric scores, counts, timestamps, and metric names only.

### Rule 2: OAuth tokens, refresh tokens, session IDs

Never include in any output, log, or audit envelope field. If they appear in
CLI output, strip before emitting. This applies to:

- OAuth access tokens
- Refresh tokens
- `sf` CLI auth credentials
- Salesforce session cookies

### Rule 3: Salesforce Org IDs

Salesforce Org IDs are 18-character strings starting with `00D`. Replace with
`<org_id_placeholder>` in all output and audit envelopes.

### Rule 4: Salesforce Record IDs (general)

15-character or 18-character Salesforce record IDs appearing in query results,
step data, or metadata must not be echoed. Replace with `<record_id_placeholder>`.

**Exception for action invocation deduplication:** When counting unique records
accessed by action invocations (e.g., to detect an agent repeatedly querying
the same record), hash the raw record ID to a deterministic 6-character token
(e.g., `rec_a3f2`) using SHA-256 truncated. This allows duplicate detection
without exposing the raw ID. Never emit the source ID alongside the hash.

### Rule 5: Session participant IDs and user IDs

Session participant IDs, user IDs (`005...`), and any other identifiers that
link a session to a specific Salesforce user must be replaced with
`<user_id_placeholder>`.

This applies to:
- `ssot__ParticipantId__c` on `AiAgentSessionParticipant__dlm`
- `OwnerId`, `CreatedById`, `LastModifiedById` on any queried object
- `UserId` in session metadata

### Rule 6: Customer names, email addresses, and contact data

Any field that may contain customer-facing PII — even if the field name does
not explicitly indicate PII — must be treated as sensitive:

- Email addresses: `<email_redacted>`
- Phone numbers: `<phone_redacted>`
- Customer names appearing in intent summaries: `<name_redacted>`
- Mailing or billing addresses: `<address_redacted>`
- Social Security Numbers or national ID numbers: `<national_id_redacted>`

If a field's content is ambiguous, redact it and note the redaction in the
audit envelope.

### Rule 7: Instance URLs and API endpoints

Do not include Salesforce instance URLs (e.g., `https://myorg.my.salesforce.com`),
Data Cloud endpoints, or any URL that reveals org-specific infrastructure.
Reference only the org alias in the audit envelope.

### Rule 8: Agent version suffix on DeveloperName

The `_vN` version suffix on `DeveloperName` (e.g., `OrderServiceAgent_v9`)
reveals internal versioning cadence. Omit the suffix from all output; use the
base API name only.

### Rule 9: Encrypted fields (Shield PE / PMLE)

If a query inadvertently touches a field marked `encrypted: true` in the
sObject describe response, skip the field entirely. Do not emit the field
name or any placeholder that implies a value was retrieved. List skipped
fields in the audit envelope under `redactions_applied`.

---

## Regulated Vertical Handling

When the target org is identified as serving a regulated vertical — Health Cloud,
Financial Services Cloud, Government Cloud, or any other edition that may host
protected data — apply the following additional controls:

1. **Flag the org** in the audit envelope: `regulated_vertical_flag: true`.
2. **Apply all standard redaction rules** with zero exceptions.
3. **Do not share output externally** without routing through
   `salesforce-compliance-privacy-agent` first.
4. **If any anomaly is detected**, the downstream recommendation must include
   `salesforce-compliance-privacy-agent` regardless of anomaly severity.
5. **Retain no session metadata** beyond the current conversation turn. Do not
   write session IDs, participant IDs, or any session-linked identifier to files
   or intermediate storage.

**Regulated vertical detection signals:**
- Org alias includes terms: `hc`, `health`, `fsc`, `financial`, `gov`, `govt`
- `sf org display` output includes industry-specific features
- User states the org is used for Health Cloud, Financial Services Cloud,
  Government Cloud, or equivalent
- Org metadata includes custom objects with obvious regulated-data names
  (e.g., `Patient__c`, `MedicalRecord__c`, `LoanApplication__c`)

If there is any doubt, treat the org as regulated.

---

## Audit Envelope Redaction Record

Every redaction applied must be recorded in the audit envelope under
`redactions_applied`. Each entry includes:

```yaml
redactions_applied:
  - type: "session_content"
    reason: "aggregate-only policy — session text content never emitted"
  - type: "user_id"
    reason: "participant IDs replaced with placeholder per redaction rule 5"
  - type: "record_id"
    reason: "record IDs hashed for dedup tracking; raw IDs not emitted"
  - type: "pii"
    field: "<FieldApiName>"
    reason: "field may contain customer contact data"
```

The audit envelope redaction record allows downstream reviewers to understand
what was removed and why, without the redacted content being present.

---

## Human-in-the-Loop Path for Session Content Access

If a human operator or security reviewer has a legitimate need to access raw
session content for debugging a specific production issue (e.g., investigating
a confirmed hallucination in a specific session):

1. The requester must initiate the request through `salesforce-live-guard-agent`.
2. `salesforce-live-guard-agent` routes for explicit human approval from an
   authorized data steward.
3. The approved requester accesses session content directly via Salesforce
   Setup or the STDM API with their own authenticated credentials — not
   through this skill.
4. The access event must be logged in the org's Event Monitoring stream
   (<!-- verify-before-merge:2026-05-21 → Event Monitoring API name
   for STDM access subject to change -->).
5. This skill is not involved in the content access — it stops at the
   referral to `salesforce-live-guard-agent`.

**This path exists for genuine debugging needs only.** It must not be used
as a routine observability mechanism. Production session content access
requires human authorization every time.

---

## What This Skill Is Permitted to Emit

Summary of what IS and IS NOT emitted:

| Data type | Permitted | Redaction applied |
|---|---|---|
| Session count (integer) | Yes | None |
| Avg faithfulness score (float) | Yes | None |
| Avg answer relevance score (float) | Yes | None |
| Avg quality score (float, 1-5) | Yes | None |
| Quality distribution (counts per score) | Yes | None |
| Action invocation count (integer) | Yes | None |
| Action error count (integer) | Yes | None |
| Abandonment rate (float) | Yes | None |
| End type counts (USER_ENDED, AGENT_ENDED, UNKNOWN) | Yes | None |
| Top intents (intent label + count) | Yes | Intent label may be paraphrased by LLM — treat as potentially sensitive; emit without modification |
| Subagent API names (for score breakdown) | Yes | Omit version suffix |
| Session IDs | No | Replaced with `<session_id_placeholder>` |
| User messages | No | Never fetched |
| Agent responses | No | Never fetched |
| LLM prompts | No | Never fetched |
| LLM responses | No | Never fetched |
| Request/response summaries (LLM paraphrases) | No | Never emitted |
| Quality reasoning text | No | Never emitted |
| Record IDs (raw) | No | Replaced with placeholder or hash |
| User IDs / participant IDs | No | Replaced with placeholder |
| Org IDs | No | Replaced with placeholder |
| OAuth tokens | No | Never fetched or emitted |
| Instance URLs | No | Omitted; org alias used instead |
