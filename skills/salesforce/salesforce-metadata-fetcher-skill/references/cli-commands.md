# CLI Commands Reference — salesforce-metadata-fetcher-skill

This reference documents every CLI command and REST path used by the metadata fetcher skill,
including privilege requirements, output shape, and known pitfalls.

---

## Command Index

| Command / Path | Privilege required | Mutation risk | Preferred |
|---|---|---|---|
| `sf org display` | Any authenticated session | None | Yes — always run first |
| `sf org list metadata --metadata-type <Type>` | View Setup and Configuration | None | Yes — enumerate before retrieve |
| `sf org list metadata-types` | View Setup and Configuration | None | Yes — discover available types |
| `sf sobject describe` | View Setup and Configuration + per-object Read | None | Yes — preferred for object schema |
| REST `/sobjects/<Object>/describe` | View Setup and Configuration + per-object Read | None | Yes — same as `sf sobject describe` |
| REST `/tooling/sobjects/Flow/<id>` | View Setup and Configuration | None | Yes — preferred for flow definitions |
| REST `/tooling/sobjects/ApexClass/<id>` | View Setup and Configuration | None | Yes — preferred for Apex class bodies |
| `sf project retrieve start` | View Setup and Configuration + **Modify Metadata on some orgs** | Writes local files | **Elevated path** — use only as fallback |

---

## `sf org display`

**Purpose:** Verify connectivity and confirm org type before any retrieval.

```bash
sf org display --target-org <alias>
```

**Key output fields:**

| Field | Meaning |
|---|---|
| `Status` | Must be `Connected` to proceed |
| `Instance URL` | Use to infer org type (`.sandbox.` in URL = sandbox) |
| `Access Token Expiry` | Confirm token is not expired |
| `Username` | Confirm Run As account identity |

**Org type inference from instance URL:**

- `https://<domain>.sandbox.my.salesforce.com` → sandbox
- `https://<domain>.scratch.my.salesforce.com` → scratch
- `https://<domain>.my.salesforce.com` (no `.sandbox.`) → likely production — treat as production

**No privilege beyond an active session required.** This command is safe to run on any org type.

---

## `sf org list metadata-types`

**Purpose:** List all metadata types supported in the target org. Run this when the user's
request does not map cleanly to a known metadata type.

```bash
sf org list metadata-types --target-org <alias>
```

**Output:** JSON array of metadata type descriptors (name, directoryName, suffix, etc.).

**Privilege required:** View Setup and Configuration (standard for T1 Run As account).

**Governor-limit note:** This is a single API call. Low footprint.

---

## `sf org list metadata --metadata-type <Type>`

**Purpose:** List available items of a specific metadata type in the org. Always run this
before retrieval to confirm items exist and to show the user what will be fetched.

```bash
sf org list metadata \
  --metadata-type <Type> \
  --target-org <alias> \
  --json
```

**Examples:**

```bash
# List all Flows
sf org list metadata --metadata-type Flow --target-org myorg --json

# List all PermissionSets
sf org list metadata --metadata-type PermissionSet --target-org myorg --json

# List all LWC bundles
sf org list metadata --metadata-type LightningComponentBundle --target-org myorg --json

# List all ApexClasses
sf org list metadata --metadata-type ApexClass --target-org myorg --json
```

**Output shape (--json):**

```json
{
  "status": 0,
  "result": [
    {
      "createdById": "<user_id>",
      "createdByName": "<name>",
      "createdDate": "2024-01-15T10:00:00.000Z",
      "fileName": "flows/MyFlow.flow",
      "fullName": "MyFlow",
      "id": "<metadata_id>",
      "lastModifiedById": "<user_id>",
      "lastModifiedDate": "2024-03-20T14:30:00.000Z",
      "manageableState": "unmanaged",
      "type": "Flow"
    }
  ]
}
```

**Sanitization required on output:** Redact `createdById`, `lastModifiedById`, and `id` fields.

**Privilege required:** View Setup and Configuration.

**Common pitfalls:**

- Do not omit `--metadata-type`. Running without it attempts org-wide enumeration (slow, broad).
- The `--json` flag is required for structured output that can be sanitized reliably.
- Large result sets (> 50 items): surface a summary to the user before retrieving all items.

---

## `sf sobject describe`

**Purpose:** Retrieve the full schema of a Salesforce object — fields, relationships, picklist
values, metadata. **Preferred path for object schema.** Does not require `ModifyMetadata`.

```bash
sf sobject describe \
  --sobject <ObjectApiName> \
  --target-org <alias> \
  --json
```

**Examples:**

```bash
# Describe the Account object
sf sobject describe --sobject Account --target-org myorg --json

# Describe a custom object
sf sobject describe --sobject My_Custom_Object__c --target-org myorg --json
```

**Equivalent REST path:**

```
GET /services/data/v62.0/sobjects/<ObjectApiName>/describe
Authorization: Bearer <access_token>
```

**Key output sections:**

| Section | Contains |
|---|---|
| `fields` | API names, types, labels, default values, formula expressions, FLS booleans |
| `recordTypeInfos` | Record type names and IDs |
| `childRelationships` | Related objects |
| `actionOverrides` | Custom actions |
| `searchLayoutable` | Whether the object appears in search |

**Sanitization required on output:**

- `defaultValue` fields: scan for email, phone, hardcoded IDs, token-like strings.
- `formula` fields: scan for hardcoded Salesforce IDs.
- `calculatedFormula` fields: same as formula.
- `referenceTo` arrays: no redaction needed (these are object API names, not IDs).

**FLS awareness:** Fields where `accessible: false` indicate FLS restrictions on the Run As account. List these in `fls_notes.inaccessible_fields` in the output envelope.

**Privilege required:** View Setup and Configuration + Read FLS on the target object.

---

## REST Tooling API — Flow

**Purpose:** Retrieve Flow metadata (definition, version, status) for a specific flow.
**Preferred path** — does not require `ModifyMetadata`.

### List active flow versions

```
GET /services/data/v62.0/tooling/query?q=SELECT+Id,MasterLabel,ApiName,ProcessType,Status,LastModifiedDate+FROM+Flow+WHERE+Status='Active'
Authorization: Bearer <access_token>
```

### Retrieve flow definition body

```
GET /services/data/v62.0/tooling/sobjects/Flow/<flowId>
Authorization: Bearer <access_token>
```

**Key output fields:**

| Field | Contains |
|---|---|
| `Id` | Flow version ID — **redact** |
| `MasterLabel` | Human name |
| `ApiName` | API name |
| `ProcessType` | Flow, AutoLaunchedFlow, Workflow, etc. |
| `Status` | Active, Draft, Obsolete |
| `Metadata` | Full flow XML definition as JSON structure |

**Sanitization required:** Redact `Id` (Salesforce ID). Scan `Metadata.variables[].value` and `Metadata.formulas[].expression` for hardcoded IDs or sensitive defaults.

**Note on fault paths:** Check `Metadata.faultConnectors` and each element for `faultConnector`. Flows without fault paths on subflow or action elements are a review finding.

---

## REST Tooling API — ApexClass

**Purpose:** Retrieve Apex class body and metadata. **Preferred path** over
`sf project retrieve start` because it does not require `ModifyMetadata`.

### List Apex classes

```
GET /services/data/v62.0/tooling/query?q=SELECT+Id,Name,Status,ApiVersion,LastModifiedDate+FROM+ApexClass+ORDER+BY+Name
Authorization: Bearer <access_token>
```

### Retrieve class body

```
GET /services/data/v62.0/tooling/sobjects/ApexClass/<classId>
Authorization: Bearer <access_token>
```

**Key output fields:**

| Field | Contains |
|---|---|
| `Id` | Class ID — **redact** |
| `Name` | Class API name |
| `Body` | Full Apex source code |
| `ApiVersion` | API version declared in class |
| `Status` | Active, Inactive |

**Sanitization required on `Body`:**

- Scan for hardcoded Salesforce IDs (15/18-char patterns).
- Scan for `UserInfo.getSessionId` return value being stored, logged, or sent externally — **Critical escalation** if found.
- Scan for hardcoded credentials, tokens, or API keys in string literals.
- Scan for `with sharing` vs. `without sharing` keyword (note in handoff fields).

---

## `sf project retrieve start` — Elevated Path

**Purpose:** Retrieve metadata to local filesystem as XML. Covers types not accessible via
REST describe or Tooling API (e.g., certain layout types, full profile XML on some orgs).

**Privilege note:** This command **may require `Modify Metadata`** on some org configurations,
even for retrieve (not deploy). This is not universally enforced but is org-dependent.
Always declare `elevated_path_used: true` in the audit envelope when this command is used.

```bash
sf project retrieve start \
  --metadata "<Type>:<ApiName>" \
  --target-org <alias> \
  --output-dir ./retrieved \
  --json
```

**Examples:**

```bash
# Retrieve a single permission set
sf project retrieve start \
  --metadata "PermissionSet:Sales_Rep" \
  --target-org myorg \
  --output-dir ./retrieved \
  --json

# Retrieve a single custom object
sf project retrieve start \
  --metadata "CustomObject:My_Object__c" \
  --target-org myorg \
  --output-dir ./retrieved \
  --json
```

**NEVER use:**

```bash
# Prohibited — org-wide dump, excessively broad
sf project retrieve start --metadata "*" --target-org myorg

# Prohibited — multi-type wildcard
sf project retrieve start --metadata "CustomObject:*" --target-org myorg

# Prohibited — full org snapshot
sf project retrieve start --manifest package.xml --target-org myorg  # only if package.xml scope is known
```

**Output sanitization:** Retrieved XML files must be parsed and sanitized before passing to downstream skills. Use `jq` on any JSON intermediaries. For XML: parse to a structured representation and apply redaction rules.

---

## `jq` Sanitization Examples

**Redact Salesforce IDs from `sf org list metadata` output:**

```bash
sf org list metadata --metadata-type Flow --target-org myorg --json | \
  jq '.result | map({fullName, fileName, type, lastModifiedDate, manageableState})'
```

This projection drops `id`, `createdById`, `lastModifiedById` — the fields containing Salesforce IDs.

**Redact IDs from sobject describe output (field array):**

```bash
sf sobject describe --sobject Account --target-org myorg --json | \
  jq '.result.fields | map({name, label, type, length, custom, updateable, createable, defaultValue, formula: .calculatedFormula})'
```

This projection retains schema-level fields and drops controller/OwnerId references.

**Extract Flow metadata without version ID:**

```bash
# Via Tooling API response stored in flow.json
jq '{masterLabel: .MasterLabel, apiName: .ApiName, processType: .ProcessType, status: .Status, metadata: .Metadata}' flow.json
```

---

## Common Mistakes

| Mistake | Why it is a problem | Correct approach |
|---|---|---|
| Using `--full` flag with `sf org list metadata` | Fetches org-wide metadata types; excessively broad; slow | Use `--metadata-type <Type>` to scope |
| Fetching `*` wildcard types | Downloads entire org metadata; includes encrypted fields, secrets in Named Credentials | Always specify exact type and name |
| Skipping `sf org display` | May retrieve from wrong org if alias is ambiguous | Always verify connectivity first |
| Not using `--json` flag | Human-readable output is hard to sanitize reliably | Always use `--json` for machine-parseable output |
| Using `sf project retrieve start` for all types | May require elevated permissions; writes files to disk | Prefer REST describe / Tooling API paths |
| Retrieving large lists without user confirmation | May time out or hit governor limits; user may not want all items | Summarize list first; require confirmation for > 50 items |
| Not checking FLS on Run As account | Fields with `accessible: false` indicate FLS gaps; should be noted | Check FLS booleans in describe output and note in `fls_notes` |
| Retrieving full Profile XML without elevated check | Full profile XML may require `Customize Application` | Confirm with user before full profile retrieval |
