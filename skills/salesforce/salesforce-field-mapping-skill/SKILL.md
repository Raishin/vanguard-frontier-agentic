---
name: salesforce-field-mapping-skill
description: "Maps CSV or spreadsheet column headers to Salesforce field API names with type mismatch detection, missing-field flagging, picklist value normalization, and API name collision detection. Used during data migration from HubSpot, Pipedrive, Excel exports, and legacy CRMs. TRIGGER when: user says map these columns to salesforce fields, field mapping for migration, convert csv headers to api names, import field mapping, migrate data from hubspot/pipedrive/excel, column mapping for data loader. DO NOT TRIGGER when: live data migration execution (use salesforce-bulk-data-ops-skill), schema design or new field creation (use salesforce-data-architecture-agent), field-level security review (use salesforce-permission-model-review-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-21"
  category: generation
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# salesforce-field-mapping-skill

Pure-generation T0 skill that maps CSV/spreadsheet column headers to
Salesforce field API names with type mismatch detection. The daily-driver
for admins migrating from HubSpot, Pipedrive, Excel exports, and legacy CRMs.
Outputs a complete mapping table, type mismatch report, missing-field list,
and a Data Loader–ready column header row.

## When This Skill Owns the Task

Use `salesforce-field-mapping-skill` when the work is to **design or
validate a field mapping** for a data import:

- "I'm migrating from HubSpot — map these column headers to Salesforce Contact fields"
- "Here's my Excel export. What are the Salesforce API names?"
- "I need a Data Loader field mapping CSV for importing Opportunities from Pipedrive"
- "Flag any type mismatches in my column list before I import"
- "Which of my CSV columns don't have matching Salesforce fields?"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Actually executing the data migration with records | `salesforce-bulk-data-ops-skill` |
| Designing new custom fields to receive imported data | `salesforce-data-architecture-agent` |
| Reviewing FLS or who can see imported fields | `salesforce-permission-model-review-skill` |
| Fetching field metadata from a live org to compare | `salesforce-metadata-fetcher-skill` |

---

## Required Context to Gather First

Before generating a mapping, confirm:

1. **Target Salesforce object** — standard (Contact, Lead, Account,
   Opportunity, Case) or custom (`Custom_Object__c`).
2. **Source system** — HubSpot, Pipedrive, Salesforce Classic, Excel/CSV
   export, Marketo, or other. Source system determines default field label
   conventions.
3. **Column header list** — the complete list of CSV/spreadsheet column
   headers, either as a comma-separated list or pasted table.
4. **Custom fields in scope** — list any known custom fields on the target
   object that may receive data (e.g., `HubSpot_Contact_ID__c`).
5. **Operation type** — insert, update, or upsert. Affects whether `Id` or
   an external ID field is required in the mapping.
6. **External ID field** — for upsert: which field is the external ID key?
   (e.g., `HubSpot_Contact_ID__c`, `Email`)
7. **Multi-value fields** — any source columns that contain comma-separated
   multi-values intended for multi-select picklists.

If the column header list is not provided, ask before proceeding.

---

## Recommended Workflow

### Step 1 — Normalize source column headers

Apply label normalization to make headers comparable:
- Strip leading/trailing whitespace
- Collapse multiple spaces to single space
- Remove non-alphanumeric prefix/suffix characters
- Flag headers containing special characters that may corrupt Data Loader
  (e.g., `#`, `$`, `%`, `(`, `)`, `/`)

### Step 2 — Derive candidate API names from labels

For each normalized column header, apply the API name derivation rules
(see `references/api-name-normalization.md`):
1. Replace spaces with underscores
2. Remove characters not in `[A-Za-z0-9_]`
3. Strip leading digits (Salesforce API names must start with a letter)
4. Collapse consecutive underscores to single underscore
5. Truncate to 40 characters (API name limit)
6. Append `__c` for custom fields
7. Check against reserved words list

### Step 3 — Match against known standard field API names

Apply fuzzy matching against the standard object's well-known fields:

For **Contact**: `FirstName`, `LastName`, `Email`, `Phone`, `MobilePhone`,
`Title`, `Department`, `AccountId`, `MailingStreet`, `MailingCity`,
`MailingState`, `MailingPostalCode`, `MailingCountry`, `LeadSource`,
`Salutation`, `Birthdate`, `Description`, `DoNotCall`, `HasOptedOutOfEmail`,
`OwnerId`, `ReportsToId`

For **Lead**: `FirstName`, `LastName`, `Company`, `Email`, `Phone`,
`MobilePhone`, `Status`, `LeadSource`, `Title`, `Industry`, `AnnualRevenue`,
`NumberOfEmployees`, `Rating`, `Street`, `City`, `State`, `PostalCode`,
`Country`, `Description`, `HasOptedOutOfEmail`, `DoNotCall`

For **Account**: `Name`, `Type`, `Industry`, `AnnualRevenue`,
`NumberOfEmployees`, `BillingStreet`, `BillingCity`, `BillingState`,
`BillingPostalCode`, `BillingCountry`, `Phone`, `Website`, `Description`,
`OwnerId`, `ParentId`

For **Opportunity**: `Name`, `StageName`, `CloseDate`, `Amount`,
`Probability`, `AccountId`, `OwnerId`, `Type`, `LeadSource`, `Description`,
`ForecastCategory`, `ForecastCategoryName`, `CampaignId`, `NextStep`

### Step 4 — Detect type mismatches

For each mapped pair, compare source data type to Salesforce field type.
Flag mismatches (see `references/type-mismatch-detection.md`):

| Source type | Target type | Risk | Action |
|---|---|---|---|
| String | Date | HIGH | Requires DATEVALUE() transform; verify ISO format |
| String | Number/Currency | HIGH | Requires VALUE() or numeric parse; check for non-numeric chars |
| Multi-value string | Single picklist | HIGH | Must pick one value or use transformation script |
| String | Boolean/Checkbox | MEDIUM | Map "true"/"1"/"yes" → true; all else → false |
| String | Lookup (Id) | HIGH | Requires upsert by external ID or pre-lookup step |
| Integer | Currency | LOW | Implicit conversion; verify decimal precision |
| Mixed case | Picklist | MEDIUM | Picklist values are case-sensitive in Data Loader |

### Step 5 — Flag missing and unmappable fields

Produce two lists:
1. **Unmapped source columns** — columns with no clear Salesforce target
2. **Required fields missing from source** — Salesforce required fields
   (e.g., `LastName` on Contact, `LastName + Company` on Lead, `Name +
   StageName + CloseDate` on Opportunity, `Name` on Account) not present
   in the source column list

### Step 6 — Normalize picklist values

For any column mapping to a standard picklist field, list the expected
Salesforce API values and flag mismatches:

- `LeadSource`: Cold Call, Web, Phone Inquiry, Partner Referral, Purchased
  List, Other, Word of mouth, Employee, Internal, Partner
- `StageName`: Prospecting, Qualification, Needs Analysis, Value Proposition,
  Id. Decision Makers, Perception Analysis, Proposal/Price Quote, Negotiation/Review,
  Closed Won, Closed Lost
- `Industry`: Agriculture, Apparel, Banking, Biotechnology, Chemicals,
  Communications, Construction, Consulting, Education, Electronics, Energy,
  Engineering, Entertainment, Environmental, Finance, Food & Beverage,
  Government, Healthcare, Hospitality, Insurance, Machinery, Manufacturing,
  Media, Not For Profit, Recreation, Retail, Shipping, Technology,
  Telecommunications, Transportation, Utilities, Other

### Step 7 — Detect special character and collision risks

- Flag API names that would collide (two source columns normalizing to
  the same API name)
- Flag reserved words: `Id`, `Name`, `OwnerId`, `CreatedDate`,
  `LastModifiedDate`, `SystemModstamp`, `IsDeleted` — these cannot be
  used as custom field API names
- Flag API names exceeding 40 characters after normalization

### Step 8 — Emit structured output

Produce the full output block per the Output Format section below,
including a Data Loader–ready header row.

---

## Quality Scoring Rubric (100-point)

Score every output before emitting. Threshold: 85+ ship, 70–84 ship with
caveat, below 70 reject and revise.

| Dimension | Points | What earns full marks |
|---|---|---|
| **API name accuracy** | 30 | Every standard field mapped to correct API name; custom fields flagged with `__c`; no typos or reversed lookups |
| **Type mismatch detection** | 25 | All HIGH and MEDIUM risk mismatches flagged with recommended transforms; no silently ignored mismatches |
| **Missing-field flagging** | 15 | All required Salesforce fields checked; clearly listed if absent from source; import will fail without them |
| **Picklist value normalization** | 15 | Case-sensitive picklist values verified; multi-value vs. single-value conflict flagged; API values used not display labels |
| **Special char / collision detection** | 15 | Column header special chars flagged; API name collisions detected; reserved word conflicts identified |

**Scoring penalties:**
- Missing a HIGH-risk type mismatch: -15 per missed mismatch
- Missing a required field from target object: -10
- Incorrect standard API name: -15 per error
- Multi-select picklist mapped to single picklist without flag: -20
- Lookup/relationship field mapped without upsert key guidance: -10

---

## T0 Contract

This skill is **static-review (T0)** only:

- No Salesforce org connection required or permitted.
- No CLI commands are executed.
- No live field metadata is fetched.
- The mapping is generated from known standard field sets and the user's
  provided column list.
- If the target object has custom fields that the user has not listed,
  mappings to those custom fields cannot be generated here — use
  `salesforce-metadata-fetcher-skill` to fetch them first.

---

## Refusal Triggers

Stop and decline if:

- The request is to execute a live data migration (route to
  `salesforce-bulk-data-ops-skill`).
- The target object is not specified after two clarification attempts.
- The column header list is not provided after one prompt.
- The operation involves permanent hard-deletes (T3 prohibited — route
  to `salesforce-live-guard-agent`).

---

## Output Format

```yaml
verdict: "ship | ship-with-caveat | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

mapping_summary:
  source_system: "<HubSpot | Pipedrive | Excel | Salesforce Classic | Other>"
  target_object: "<SObjectApiName>"
  operation: "<insert | update | upsert>"
  external_id_field: "<FieldApiName | none>"
  total_source_columns: <integer>
  mapped_columns: <integer>
  unmapped_columns: <integer>
  type_mismatch_count: <integer>
  required_fields_missing: <integer>

field_mapping:
  - source_column: "<source header>"
    salesforce_api_name: "<FieldApiName>"
    salesforce_field_type: "<Text|Date|Number|Currency|Picklist|Lookup|Checkbox|etc>"
    confidence: "<high|medium|low>"
    type_mismatch: "<none|HIGH|MEDIUM|LOW>"
    mismatch_detail: "<description of mismatch if present>"
    transform_recommended: "<none|DATEVALUE()|VALUE()|Picklist normalization|External ID lookup>"
    notes: "<any special handling>"

unmapped_source_columns:
  - column: "<source header>"
    reason: "<no standard match | ambiguous | custom field needed>"
    suggested_custom_api_name: "<Suggested_Name__c>"

required_fields_missing_from_source:
  - field: "<FieldApiName>"
    required_for: "<insert|update|upsert>"
    impact: "<import will fail | data quality risk>"

picklist_value_mismatches:
  - source_column: "<source header>"
    salesforce_field: "<FieldApiName>"
    source_values_found: ["<val1>", "<val2>"]
    valid_salesforce_values: ["<val1>", "<val2>"]
    mismatched_values: ["<val1>"]
    normalization_recommended: "<map 'val1' → 'Correct Value'"

collision_and_special_char_warnings:
  - column: "<source header>"
    issue: "<special chars | api name collision | reserved word | length overflow>"
    recommendation: "<how to resolve>"

data_loader_header_row: "<Comma-separated list of Salesforce API names in source column order>"

handoff_suggestions:
  - "<e.g., fetch custom fields from org with salesforce-metadata-fetcher-skill>"
  - "<e.g., run data migration with salesforce-bulk-data-ops-skill>"

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

This is a T0 generation skill with no live org connection. Apply:

1. Never emit actual production org IDs, user IDs, or credentials even if
   the user includes them in their prompt.
2. If the user pastes sample data rows alongside headers, do not include
   actual record values (email addresses, names, phone numbers) in the output.
3. Sanitize pasted column lists that contain Org IDs (18-char starting `00D`)
   before incorporating into output.

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| User wants to execute the migration | `salesforce-bulk-data-ops-skill` |
| User needs to fetch custom field API names from org | `salesforce-metadata-fetcher-skill` |
| User wants to create new fields to receive import data | `salesforce-data-architecture-agent` |
| FLS or sharing concerns on import fields | `salesforce-permission-model-review-skill` |

---

## Stop Conditions

- Import will fail because required fields (`LastName`, `Company`,
  `StageName + CloseDate + Name`) are entirely missing from the source —
  stop, surface the list, require resolution before producing a mapping.
- Every source column maps to the same single Salesforce field (e.g.,
  all columns appear to be "Name") — stop and request clarification.
- Requested mapping includes hard-delete or permanent-destroy operations —
  stop and route to `salesforce-live-guard-agent`.

---

## Security Notes

- **T0 static generation only:** No org connection, no CLI execution, no
  MCP calls.
- **No credential handling:** Skill never requests or emits OAuth tokens,
  session IDs, or Connected App credentials.
- **PII in sample data:** If user pastes real data rows, do not include
  field values containing PII in the output mapping table. Process headers
  only.
- **Upsert external ID guidance:** External ID fields used in upsert
  operations must have "External ID" checked on the field definition in
  Salesforce Setup — flag this requirement explicitly.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/api-name-normalization.md` | How to derive API name from label, reserved words, length limits, collision rules |
| `references/type-mismatch-detection.md` | Common type mismatches, normalization strategies, transform patterns |
| `references/picklist-value-mapping.md` | Picklist value conversion, case sensitivity, multi-select handling |
