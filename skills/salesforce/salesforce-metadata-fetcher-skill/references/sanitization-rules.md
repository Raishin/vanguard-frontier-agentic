# Sanitization Rules Reference — salesforce-metadata-fetcher-skill

This reference documents precise sanitization rules that must be applied to all
metadata retrieved from a live Salesforce org before the data is passed to any
downstream review skill or surfaced to the user.

Sanitization is **mandatory and non-negotiable**. Output must not be emitted if
sanitization has not been completed and confirmed.

---

## Rule Classification

| Code | Severity | Action |
|---|---|---|
| S-STOP | Critical | Stop processing immediately; do not emit partial output |
| S-REDACT | High | Replace with placeholder; note in `skip_reasons` |
| S-FLAG | Medium | Flag for review; include in output with a caveat note |
| S-SKIP | High | Omit the field from output entirely; note in `encrypted_fields_skipped` |

---

## ID Redaction Rules

### Rule S-ID-01 — Org ID

**Pattern:** `00D[A-Za-z0-9]{12,15}`

**Description:** Salesforce organization ID. Always starts with `00D`. May appear in:
- API responses as `organizationId`
- SOQL filter conditions embedded in flow XML
- Formula field expressions
- Field default values

**Action:** S-REDACT — Replace with `<org_id_placeholder>`.

**Regex (Python-compatible):**

```python
import re
ORG_ID_PATTERN = re.compile(r'\b00D[A-Za-z0-9]{12,15}\b')

def redact_org_ids(text: str) -> str:
    return ORG_ID_PATTERN.sub('<org_id_placeholder>', text)
```

**Note:** Both 15-character and 18-character forms must be matched. The `{12,15}` suffix after
the `00D` prefix produces 15-to-18-character total IDs.

---

### Rule S-ID-02 — User ID

**Pattern:** `005[A-Za-z0-9]{12,15}`

**Description:** Salesforce user record ID. Always starts with `005`. May appear in:
- `createdById`, `lastModifiedById` fields in metadata list output
- Flow variable default values
- Profile/PermissionSet assignment records
- Apex class hardcoded values

**Action:** S-REDACT — Replace with `<user_id_placeholder>`.

**Regex (Python-compatible):**

```python
USER_ID_PATTERN = re.compile(r'\b005[A-Za-z0-9]{12,15}\b')

def redact_user_ids(text: str) -> str:
    return USER_ID_PATTERN.sub('<user_id_placeholder>', text)
```

---

### Rule S-ID-03 — Profile / Permission Set ID

**Pattern:** `00e[A-Za-z0-9]{12,15}`

**Description:** Salesforce profile or permission set record ID. May appear in:
- Profile metadata XML
- Permission set assignments
- Sharing rule criteria

**Action:** S-REDACT — Replace with `<profile_id_placeholder>`.

**Regex (Python-compatible):**

```python
PROFILE_ID_PATTERN = re.compile(r'\b00e[A-Za-z0-9]{12,15}\b')

def redact_profile_ids(text: str) -> str:
    return PROFILE_ID_PATTERN.sub('<profile_id_placeholder>', text)
```

---

### Rule S-ID-04 — General Salesforce Record ID (catch-all)

**Pattern:** `[a-zA-Z0-9]{15}` or `[a-zA-Z0-9]{18}` in a context where it appears to be a Salesforce ID

**Description:** Catch-all for Salesforce record IDs in field default values, formula expressions,
or flow variable defaults. These do not start with known prefixes and represent hardcoded record
references that would break between orgs.

**Action:** S-FLAG — Flag as a hardcoded ID finding. Include in output with a caveat. Do not repeat
the ID value itself; describe the location and pattern only.

**Detection heuristic:**

A string is likely a Salesforce record ID if:
- It is exactly 15 or 18 characters long
- It is alphanumeric (no special characters)
- It appears in a field default value, formula expression, or Apex string literal
- The surrounding context is a field default or assignment, not a human-readable string

**Note:** This catch-all intentionally has higher false-positive rate. Flag all matches for human review.

---

## Field Default Redaction Rules

### Rule S-FIELD-01 — Email Address in Field Default

**Description:** An email address in a field default value indicates hardcoded contact data
that will break between orgs or leak internal contact details.

**Detection regex:**

```python
EMAIL_PATTERN = re.compile(
    r'\b[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\b'
)
```

**Action:** S-REDACT — Replace with `<email_placeholder>`. Note field name and object in `skip_reasons`.

**Example:**

```
# Before sanitization
defaultValue: "support@mycompany.com"

# After sanitization
defaultValue: "<email_placeholder>"
# skip_reasons: ["Field 'Case.Email__c' default value contained an email address — redacted"]
```

---

### Rule S-FIELD-02 — Phone Number in Field Default

**Description:** A phone number in a field default value indicates hardcoded contact data.

**Detection patterns (covers US, international, common formats):**

```python
PHONE_PATTERN = re.compile(
    r'(\+?\d[\d\s\-.]{7,}\d)'
)
```

**Action:** S-REDACT — Replace with `<phone_placeholder>`. Note in `skip_reasons`.

**Note:** This pattern is broad. Apply it only to fields of type `Phone` or field API names
containing `Phone`, `Tel`, `Fax`, or `Mobile`. Applying it to all fields would produce excessive
false positives for numeric IDs and version numbers.

---

### Rule S-FIELD-03 — Account Name or Organization Name in Field Default

**Description:** A field default containing what appears to be a specific company or organization
name indicates hardcoded tenant-specific data.

**Detection:** This cannot be reliably detected with a regex. Apply contextual judgment:
- If a `Text` field on a related object (e.g., `Account.Name__c`) has a default value that is a
  proper-noun string (e.g., `"Acme Corporation"`, `"Global Services LLC"`), flag it.
- Proper noun detection: string begins with a capital letter, is not a known Salesforce system value,
  and appears to be a company or person name.

**Action:** S-FLAG — Flag for human review. Do not redact automatically (too many false positives).
Note in `assumptions`.

---

## Credential and Secret Redaction Rules

### Rule S-CRED-01 — URL with Embedded Credentials

**Pattern:** `https?://[^:@\s]+:[^@\s]+@[^\s]+`

**Description:** A URL containing `username:password@hostname` form. Indicates credentials were
embedded in a field default, Named Credential configuration, or Apex string literal.

**Action:** S-STOP — **Reject the entire payload immediately.** Do not emit any partial output.
Emit a stop message with `condition_fired: "url_embedded_credentials"` and `escalation_required: true`.

**Detection regex:**

```python
URL_CRED_PATTERN = re.compile(r'https?://[^\s:@]+:[^\s@]+@[^\s]+')
```

**Note:** This pattern fires on any URL with a colon in the authority component before the `@`.
Legitimate colon-separated port numbers (e.g., `https://host:8080/`) do not match because they
lack the `@` after the port.

---

### Rule S-CRED-02 — Named Credential Header Value

**Description:** Named Credential definitions may contain authorization header values
(e.g., `Authorization: Bearer <token>`, `Api-Key: <secret>`). These must never appear in output.

**Detection:** Named Credential metadata type (`NamedCredential`) is not in the T1 allowed list
and should not be retrieved at all. However, if a Named Credential header value appears in the
body of another metadata type (e.g., embedded in a flow variable default), it must be caught.

**Detection heuristic:** Look for strings matching:
- `Bearer [A-Za-z0-9\-._~+/]+=*` (JWT or opaque bearer token)
- `Basic [A-Za-z0-9+/]+=*` (Base64 encoded credentials)
- Key-value patterns in Apex class string literals where the key contains `Authorization`,
  `Api-Key`, `X-Api-Key`, `Secret`, or `Token`.

**Action:** S-STOP — **Reject the entire payload immediately.**

---

### Rule S-CRED-03 — High-Entropy Token-Like String in Field Default

**Description:** A field default value containing a high-entropy string (> 20 characters,
mixture of uppercase, lowercase, digits, and/or special characters) that does not match
a known safe pattern. Likely an API key, refresh token, or webhook secret that was hardcoded.

**Detection heuristic:**

```python
import math, re

def estimate_entropy(s: str) -> float:
    """Shannon entropy in bits per character."""
    if not s:
        return 0.0
    freq = {}
    for c in s:
        freq[c] = freq.get(c, 0) + 1
    length = len(s)
    return -sum((f/length) * math.log2(f/length) for f in freq.values)

TOKEN_CANDIDATE_PATTERN = re.compile(r'[A-Za-z0-9\-_+/=]{20,}')

def is_likely_token(s: str) -> bool:
    return len(s) >= 20 and estimate_entropy(s) > 3.5
```

**Action:** S-REDACT — Replace with `<token_placeholder>`. Note the field name and object in `skip_reasons`.

---

## Encrypted Field Rules

### Rule S-ENC-01 — Shield Platform Encryption (SPE) Field

**Description:** Fields protected by Salesforce Shield Platform Encryption have
`encryptionScheme` set to a non-`None` value in their describe output (e.g., `"AES128"`,
`"AES256"`, `"DETERMINISTIC"`, `"PROBABILISTIC"`).

**Action:** S-SKIP — Do not attempt to read the field value. Do not include field value in output.
Include the field API name in `encrypted_fields_skipped` with the encryption scheme noted.

**Detection in sobject describe output:**

```json
{
  "name": "SSN__c",
  "type": "EncryptedText",
  "encryptionScheme": "PROBABILISTIC_RANDOM"
}
```

The `encryptionScheme` field is present and non-null, or the field `type` is `EncryptedText`.

---

### Rule S-ENC-02 — Platform Masking and Late Encryption (PMLE)

**Description:** Fields protected by PMLE (Platform Masking, field-level encryption via Data Mask
or compliance rules) may not always be identifiable from the describe output alone.

**Detection heuristic:** If a field has `type: "EncryptedText"` or `encryptionScheme` != `"NONE"`,
apply S-SKIP. Additionally, if the field API name contains `SSN`, `TaxId`, `CreditCard`,
`BankAccount`, `Password`, or `Secret` (case-insensitive), apply extra caution and note in output.

**Action:** S-SKIP — Same as S-ENC-01.

---

## Apex-Specific Rules

### Rule S-APEX-01 — Hardcoded Session ID in Apex Class

**Description:** An Apex class body that calls `UserInfo.getSessionId` and passes the result
to an external callout, stores it in a field, or logs it represents a Critical security finding.

**Detection:**

```python
SESSION_ID_EXFIL_PATTERN = re.compile(
    r'UserInfo\.getSessionId\(\).*?(?:HttpRequest|Callout|insert|update|Database\.insert|System\.debug)',
    re.DOTALL
)
```

**Action:** S-STOP (treat as Critical) — Flag the finding, note the class name and approximate
line context (not the literal value), and escalate before passing to downstream skill.

**Emit:**

```yaml
stop:
  condition_fired: "apex_session_id_exfiltration"
  mid_execution: true
  class_name: "<ClassName>"
  finding: "UserInfo.getSessionId result appears to be passed to an external callout or stored in a field"
  escalation_required: true
```

---

### Rule S-APEX-02 — Hardcoded Salesforce ID in Apex String Literal

**Description:** A hardcoded 15- or 18-character Salesforce record ID in an Apex string literal
(e.g., `Id myId = '0018000001abc123abc';`). This will break in any org other than the one it was
written for.

**Action:** S-FLAG — Do not repeat the ID value in output. Describe the location (class name,
approximate context) and pattern. Include in the downstream skill's `complexity_indicators` or
`hardcoded_id_count` field.

---

## Output Documentation Rules

### What must appear in output when redaction fires

Every redaction or skip event must be documented in the output envelope:

```yaml
fls_notes:
  inaccessible_fields:
    - "Account.Revenue__c"   # FLS blocked — not included in output
  encrypted_fields_skipped:
    - field: "Contact.SSN__c"
      encryption_scheme: "PROBABILISTIC_RANDOM"
      action: "skipped — value not read"

metadata_summary:
  items_skipped: 2
  skip_reasons:
    - "Field 'Case.SupportEmail__c' default value contained an email address — redacted"
    - "Field 'Contact.APIKey__c' default value matched high-entropy token pattern — redacted"
```

### What must NEVER appear in output

- Literal Salesforce org IDs (15 or 18 characters starting with `00D`)
- Literal user IDs (starting with `005`)
- Literal profile or permission set IDs (starting with `00e`)
- Hardcoded ID values from field defaults or formula expressions — describe pattern and location only
- Email addresses from field defaults
- Phone numbers from field defaults
- URL-embedded credentials (stop condition — no partial output)
- Named Credential header values (stop condition — no partial output)
- API keys, tokens, or high-entropy secrets from field defaults
- Apex source code lines containing session ID exfiltration patterns (escalate before sharing)

---

## Sanitization Verification Checklist

Before emitting any output, confirm:

- [ ] All org ID patterns (`00D...`) have been replaced with `<org_id_placeholder>`
- [ ] All user ID patterns (`005...`) have been replaced with `<user_id_placeholder>`
- [ ] All profile/permission set ID patterns (`00e...`) have been replaced with `<profile_id_placeholder>`
- [ ] All field default values have been scanned for email, phone, and token patterns
- [ ] All URL-embedded credential patterns have been checked — if found, stop and do not emit
- [ ] All Named Credential header value patterns have been checked — if found, stop and do not emit
- [ ] Encrypted field markers (Shield PE, PMLE) have been identified and those fields skipped
- [ ] Apex class bodies (if retrieved) have been scanned for session ID exfiltration patterns
- [ ] All redaction and skip events have been documented in `skip_reasons` or `encrypted_fields_skipped`
- [ ] `sanitization_applied: true` is set in the audit envelope
