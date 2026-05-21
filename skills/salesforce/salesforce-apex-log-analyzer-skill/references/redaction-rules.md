# Log Redaction Rules Reference

## Purpose

Apex debug logs frequently contain sensitive data: record IDs, user IDs, field values
including PII, and occasionally session tokens that appear in `USER_DEBUG` output.
This reference defines the mandatory redaction patterns to apply before emitting any
log content in skill output.

---

## Redaction Priority Order

Apply in this order. Each rule is additive — apply all rules, not just the first match.

### Rule 1 — OAuth and Session Tokens

**Pattern:** Any string matching OAuth access token, refresh token, or session ID format.
These typically appear in USER_DEBUG lines when developers inadvertently log request headers
or authentication context.

**Action:** Strip entirely. Do not replace with a placeholder — omit the line or replace
the token value with `<token_redacted>`.

```bash
# Grep pattern to detect
grep -i "access_token\|refresh_token\|Bearer\|Authorization\|sessionId" apex-debug.log
```

---

### Rule 2 — Salesforce Org IDs

**Pattern:** 18-character alphanumeric strings starting with `00D` (Org ID prefix).

**Replacement:** `<org_id_placeholder>`

```bash
# Sed replacement
sed 's/00D[a-zA-Z0-9]\{15\}/<org_id_placeholder>/g'
```

---

### Rule 3 — Salesforce Record IDs

**Pattern:** 15-character or 18-character Salesforce IDs. Common prefixes:
- `001` — Account
- `003` — Contact
- `006` — Opportunity
- `00Q` — Lead
- `005` — User
- `00e` — Profile
- `01p` — ApexClass
- `707` — TestRunResult

**Replacement:** `<record_id_placeholder>`

```bash
# Match 18-char IDs (most common in logs)
sed 's/[0-9A-Za-z]\{18\}/<record_id_placeholder>/g'

# More targeted: match known prefix patterns
sed 's/\b00[0-9A-Za-z]\{13\}\b/<record_id_placeholder>/g'
```

**Note:** 18-char ID replacement is aggressive and may match non-ID strings. Use the
targeted prefix-based pattern when possible. When replacing, prefer targeted matching
and review the output before emitting.

---

### Rule 4 — User IDs

**Pattern:** IDs appearing in fields: `OwnerId`, `CreatedById`, `LastModifiedById`,
`RunningUserId`, `User.Id`, or following the text `Running User`.

**Replacement:** `<user_id_placeholder>`

This is in addition to Rule 3 (which catches all record IDs). Rule 4 ensures user IDs
are replaced even in descriptive text like `Running User: 005Xx000001ABCDEF`.

---

### Rule 5 — Email Addresses

**Pattern:** Strings matching `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`

**Action:** Apply in production log analysis or when sensitivity classification is HIGH.
Replace with `<email_placeholder>`.

**When to apply:**
- Always on regulated-vertical orgs (Health Cloud, Financial Services Cloud)
- Always on production org logs
- On sandbox logs unless the user has acknowledged PII scope

---

### Rule 6 — Phone Numbers

**Pattern:** Common formats: `+1-555-555-5555`, `(555) 555-5555`, `5555555555`

**Action:** Replace with `<phone_placeholder>` when appearing in USER_DEBUG output
or DML_BEGIN field values.

---

### Rule 7 — SSNs and Financial Data

**Pattern:** SSN: `\d{3}-\d{2}-\d{4}` or `\d{9}`. Financial account numbers are
context-dependent.

**Action:** Replace with `<pii_placeholder>`. Flag in `redactions_applied` with
`reason: pii`.

---

### Rule 8 — Instance URLs

**Pattern:** Strings matching `https://[a-zA-Z0-9-]+\.salesforce\.com` or
`https://[a-zA-Z0-9-]+\.my\.salesforce\.com`

**Action:** Replace with `<instance_url_placeholder>`. Reference only the org alias.

---

## jq Patterns for JSON Output Redaction

When log retrieval returns JSON (from `sf apex get log --result-format json`):

```bash
# Redact record IDs from log body content
sf apex get log --log-id <id> --target-org <alias> --result-format json \
  | jq '.result.log' \
  | sed 's/00[0-9A-Za-z]\{16\}/<record_id_placeholder>/g'

# Extract only log category lines, omitting USER_DEBUG (which may contain field values)
sf apex get log --log-id <id> --target-org <alias> \
  | grep -v "USER_DEBUG" \
  | grep "FATAL_ERROR\|LIMIT_USAGE\|SOQL_EXECUTE\|DML_BEGIN\|EXCEPTION_THROWN"
```

---

## What NOT to Redact

These items are needed for diagnosis and must NOT be redacted:

- Class names and method names in stack traces
- Line numbers
- Log categories (`SOQL_EXECUTE_BEGIN`, `DML_BEGIN`, etc.)
- Exception type names (`System.LimitException`, `System.QueryException`)
- Governor limit values (`101 out of 100`) — the numbers, not any IDs embedded
- SOQL query structure (but replace any literal ID bind values or string values)
- CPU time and heap values

---

## Audit Logging of Redactions

Every redaction applied must be recorded in the `redactions_applied` array in the
audit envelope:

```yaml
redactions_applied:
  - field_or_pattern: "record_ids (00D prefix)"
    reason: "org_id"
    count: 3
  - field_or_pattern: "record_ids (001/003 prefix)"
    reason: "record_id"
    count: 47
  - field_or_pattern: "email_address"
    reason: "pii"
    count: 2
  - field_or_pattern: "USER_DEBUG lines omitted"
    reason: "potential_pii"
    count: 12
```
