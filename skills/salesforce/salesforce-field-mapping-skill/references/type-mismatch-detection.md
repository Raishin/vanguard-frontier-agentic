# Type Mismatch Detection Reference

## Risk Levels

| Risk | Meaning |
|---|---|
| HIGH | Import will fail or corrupt data without a transform step |
| MEDIUM | Import may succeed but data may be inaccurate or truncated |
| LOW | Import will succeed; minor precision or formatting concern |

---

## Common Type Mismatches

### String → Date (HIGH)

**Scenario:** Source contains date strings like `"01/15/2026"` or
`"Jan 15, 2026"` targeting a Salesforce Date field.

**Why it fails:** Salesforce Data Loader requires ISO 8601 date format
(`YYYY-MM-DD`). Non-ISO formats cause import failure or null values.

**Transform required:**
- Excel: Use formula `=TEXT(A1,"YYYY-MM-DD")`
- Python: `pd.to_datetime(df['col']).dt.strftime('%Y-%m-%d')`
- Salesforce Dataflows: `DATEVALUE(TEXT(date_field))`
- Common non-ISO formats to flag: `MM/DD/YYYY`, `DD/MM/YYYY`,
  `Month DD, YYYY`, `YYYY/MM/DD`

---

### String → Number / Currency (HIGH)

**Scenario:** Source contains values like `"$1,234.56"`, `"1 500"`,
`"1.5M"` targeting a Number or Currency field.

**Why it fails:** Data Loader cannot parse currency symbols, thousand
separators, or shorthand.

**Transform required:**
- Strip currency symbol: `$1,234.56` → `1234.56`
- Remove thousand separator: `1,234` → `1234`
- Expand shorthand: `1.5M` → `1500000`
- Set decimal precision — Salesforce Currency fields have a max 18 digits
  with up to 2 decimal places by default

---

### Multi-Value String → Single Picklist (HIGH)

**Scenario:** Source column contains `"Technology; Financial Services"` but
the target is a single-select picklist.

**Why it fails:** Single picklist accepts exactly one value. Multi-value
strings do not map automatically.

**Options:**
1. Use only the first value: `Technology`
2. Create a multi-select picklist field instead
3. Use the most specific/relevant value (requires business decision)

---

### String → Boolean / Checkbox (MEDIUM)

**Scenario:** Source contains `"Yes"`, `"No"`, `"1"`, `"0"`, `"TRUE"`,
`"FALSE"`, `"X"` targeting a Salesforce Checkbox.

**Data Loader behavior:** Accepts only `true` or `false` (case-insensitive).
Other values may be treated as `false`.

**Normalization map:**
| Source value | Map to |
|---|---|
| `Yes`, `yes`, `Y`, `y`, `1`, `TRUE`, `true`, `X` | `true` |
| `No`, `no`, `N`, `n`, `0`, `FALSE`, `false`, blank | `false` |

---

### String → Lookup (Record ID) (HIGH)

**Scenario:** Source contains a related record name or external ID but the
target Salesforce field requires an 18-character Salesforce ID.

**Why it fails:** Lookup fields store the related record's Salesforce ID,
not its name.

**Options:**
1. **Upsert by external ID:** Set an external ID field on the related object
   and use `RelationshipFieldName:ExternalIdField` syntax in Data Loader.
   Example: `Account:External_ID__c` to resolve AccountId.
2. **Pre-query IDs:** Run a SOQL query to build a name-to-ID mapping before
   import.
3. **Upsert by Email on Contact/Lead:** Salesforce supports
   `AccountId:Account:Name` for Account lookups when Name is unique.

---

### Integer → Currency (LOW)

**Scenario:** Source column contains integers (`1234`) targeting a
Currency field (`1234.00`).

**Risk:** Low — Salesforce accepts integers for currency fields and stores
as `1234.00`. Verify that the source column is not storing values in
whole-dollar amounts while the business expects cents.

---

### Mixed Case String → Picklist (MEDIUM)

**Scenario:** Source has `"closed won"` or `"CLOSED WON"` but Salesforce
picklist value is `"Closed Won"`.

**Why it fails:** Picklist values in Data Loader are **case-sensitive**.
`"closed won"` ≠ `"Closed Won"` and will import as a blank picklist value
or trigger a validation error.

**Transform required:** Normalize source values to exact Salesforce picklist
API value casing before import.

---

### DateTime → Date (MEDIUM)

**Scenario:** Source contains `"2026-01-15 14:30:00"` targeting a Date field
(not DateTime).

**Risk:** Data Loader will reject the value or strip the time component
inconsistently depending on locale settings.

**Transform required:** Strip time component: `2026-01-15 14:30:00` → `2026-01-15`

---

### String → Email (LOW)

**Scenario:** Source column is plain text but target is Email field type.

**Risk:** Low if values are already valid email format. Data Loader validates
email format — invalid emails will be rejected.

**Check:** Scan source column for values not matching `*@*.*` pattern.

---

### Long Text → Standard Text (MEDIUM)

**Scenario:** Source column contains values exceeding 255 characters targeting
a standard Text field (255 char limit).

**Risk:** Values exceeding the limit will be truncated or cause import failure.

**Fix:** Map to a LongTextArea or RichTextArea field, or add a pre-import
truncation step.

---

## Encrypted Field Handling

Fields with Salesforce Shield Platform Encryption (PMLE or Classic) have
additional import constraints:

- **Data Loader** can import into encrypted fields but requires the user
  running the import to have `View Encrypted Data` permission
- **PMLE-encrypted fields** cannot be used as upsert external ID keys
- **Classic encryption** masks values even for users with View access unless
  the user also has Manage Encryption Keys

Always flag encrypted target fields in the mapping output.

---

## Multi-Select Picklist Handling

Salesforce multi-select picklist fields use semicolon (`;`) as the value
separator in Data Loader imports.

| Source | Transform | Target |
|---|---|---|
| `"Value1,Value2"` | Replace `,` with `;` | `Value1;Value2` |
| `"Value1\|Value2"` | Replace `\|` with `;` | `Value1;Value2` |
| `"Value1"` (single) | No change | `Value1` |

**Warning:** Values must match the picklist exactly (case-sensitive).
Invalid values in multi-select imports are silently dropped in some
Data Loader versions.
