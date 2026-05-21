# Interview Log Redaction Reference

## What Is a FlowInterviewLog

`FlowInterviewLog` is a Salesforce object that records details of each Flow
execution (interview). It is available when Flow Interview Logging is enabled
in Setup → Process Automation Settings.

**Key fields on FlowInterviewLog:**

| Field | Contains | Redaction required? |
|---|---|---|
| `Id` | 18-char Salesforce ID | Yes — `<log_id_placeholder>` |
| `FlowApiName` | Developer name of the Flow | No |
| `FlowVersionRunNumber` | Integer version number | No |
| `InterviewLabel` | Display label of the interview | No |
| `CurrentElement` | Element where the interview is at | No |
| `ErrorCode` | Error type string | No |
| `ErrorMessage` | Full error message text | Yes — sanitize embedded IDs and values |
| `StartTime` | ISO datetime | No |
| `EndTime` | ISO datetime | No |
| `Status` | Fault, Finished, etc. | No |
| `RunningUserId` | 18-char User ID | Yes — `<user_id_placeholder>` |

**Key fields on FlowInterviewLogEntry (child object):**

| Field | Contains | Redaction required? |
|---|---|---|
| `InterviewId` | Parent FlowInterviewLog ID | Yes — `<log_id_placeholder>` |
| `ElementApiName` | Element API name | No |
| `ElementLabel` | Element display label | No |
| `Input` | Input variables serialized as JSON | Yes — mask all variable values |
| `Output` | Output variables serialized as JSON | Yes — mask all variable values |
| `DurationMillis` | Execution time | No |

---

## Redaction Steps for FlowInterviewLog Output

### Step 1 — Strip Record IDs

Replace all 15-character and 18-character Salesforce record IDs with placeholders.

Patterns to match:
- 18-character: `[0-9a-zA-Z]{18}` where the prefix identifies the object type
  (e.g., `00D` for Org, `005` for User, `001` for Account, `006` for Opportunity)
- 15-character: Shorter version of the same

Common prefixes to watch for in error messages:

| Prefix | Object |
|---|---|
| `00D` | Organization ID |
| `005` | User |
| `001` | Account |
| `006` | Opportunity |
| `003` | Contact |
| `00Q` | Lead |
| `500` | Case |
| `a00` or similar | Custom objects |

### Step 2 — Mask Variable Values in Input/Output Fields

The `Input` and `Output` JSON fields in `FlowInterviewLogEntry` contain
serialized variable values. These can include:

- Email addresses
- Phone numbers
- Names and company names
- Record IDs
- Picklist values that may indicate sensitive categories

**Masking approach:**

```bash
# Using jq to mask all values while preserving keys
echo '<log_json>' | jq '
  .result.records[] |
  {
    ElementApiName: .ElementApiName,
    ElementLabel: .ElementLabel,
    Input: (.Input | fromjson | with_entries(.value = "<redacted>") | tojson),
    Output: (.Output | fromjson | with_entries(.value = "<redacted>") | tojson),
    DurationMillis: .DurationMillis
  }
'
```

For debugging purposes, preserve the variable **keys** (names) but mask
the **values**. This allows root cause analysis without exposing data.

### Step 3 — Sanitize ErrorMessage Field

The `ErrorMessage` field often contains embedded record IDs and sometimes
field values in the error context. Apply a regex pass:

```python
import re

def redact_sf_ids(text):
    # Replace 18-char Salesforce IDs
    text = re.sub(r'\b[0-9a-zA-Z]{18}\b', '<record_id_placeholder>', text)
    # Replace 15-char Salesforce IDs
    text = re.sub(r'\b[0-9a-zA-Z]{15}\b', '<record_id_placeholder>', text)
    return text
```

### Step 4 — Aggregate Failure Counts Instead of Listing Individual Records

When multiple records fail with the same error pattern, aggregate rather
than listing each failing record ID:

**Before redaction:**
```
Record 001Xx000001ABCDEF failed: Required field missing
Record 001Xx000001GHIJKL failed: Required field missing
Record 001Xx000001MNOPQR failed: Required field missing
```

**After aggregation and redaction:**
```
3 records failed with: Required field missing
Failing element: Update Records (UpdateOpportunity)
Sample error: Required field missing: [CloseDate]
```

---

## Minimum Safe Output for Debugging

The minimum information needed for diagnosis without exposing sensitive data:

```yaml
flow_interview_summary:
  flow_api_name: "<FlowApiName>"
  flow_version: "<integer>"
  error_code: "<ErrorCode>"
  error_message_sanitized: "<sanitized error — IDs replaced with placeholder>"
  failing_element_api_name: "<ElementApiName>"
  failing_element_label: "<ElementLabel>"
  status: "Fault"
  duration_ms: <integer>

variable_context:
  # Keys preserved, values masked
  variables_in_scope:
    - name: "<variableName>"
      type: "<Text|Number|Record|Boolean>"
      value: "<redacted>"
      was_null: <true|false>
```

The `was_null` field can be inferred from the error type (NullPointerException
or similar) without revealing the actual value.

---

## Enabling Flow Interview Logging

If `FlowInterviewLog` is not queryable, it must be enabled first:

1. Setup → Process Automation Settings
2. Enable "Flow Interview Logging"
3. Choose log level: "Minimal", "Standard", or "Detailed"
4. Note: Detailed logging can consume significant storage — enable during
   debugging only, then disable after resolution

**Fallback when logging is not enabled:**
Use the Flow debug run in Setup → Flows → Debug (sandbox only) to
generate a step-by-step debug log. Paste the debug output into
T0 mode of this skill for analysis.
