<!-- Parent: salesforce-soql-explorer-skill/SKILL.md -->
# Salesforce CLI Commands — SOQL Explorer Reference

> **verify-before-merge:2026-05-21** — Salesforce CLI command signatures
> and flag names drift across minor releases. Verify against
> https://developer.salesforce.com/docs/atlas.en-us.sfdx_cli_reference.meta/sfdx_cli_reference/cli_reference_data_commands_unified.htm
> before publishing. Minimum required version: sf CLI v2.x (unified CLI).
> Do not use the legacy `sfdx` entrypoint.

---

## Org Introspection

### Confirm connectivity and org type

```bash
sf org display --target-org <alias>
```

Output includes: username, instance URL, org type (Scratch Org / Sandbox /
Production), OAuth client ID, access token expiry. Use this to verify
the org is reachable and that the alias resolves to the expected org type
before running any query.

**T1 check:** If `orgType` or `isDevHub` indicates production and the
Connected App allowlist does not explicitly authorize this alias, stop.

### List authorized orgs

```bash
sf org list --connected
```

Returns all orgs with active authentication. Use to enumerate valid aliases
before accepting a user-provided alias. Cross-reference against the
org allowlist maintained in the Connected App configuration.

```bash
# JSON output for scripting / allowlist verification
sf org list --connected --json
```

---

## Schema Introspection

### Describe an sObject (field list, FLS, encryption flags)

```bash
sf sobject describe --sobject Account --target-org <alias>
```

Read the output before building a query:
- `fields[].name` — API names of available fields
- `fields[].encrypted` — `true` if Shield PE / PMLE encrypted; **skip these**
- `fields[].nillable` / `fields[].type` — inform filter design
- `queryable: true` — confirms the object supports SOQL

```bash
# JSON output for jq processing
sf sobject describe --sobject Account --target-org <alias> --json \
  | jq '[.result.fields[] | {name: .name, type: .type, encrypted: .encrypted}]'
```

**Encrypted field detection:**

```bash
sf sobject describe --sobject Contact --target-org <alias> --json \
  | jq '[.result.fields[] | select(.encrypted == true) | .name]'
```

Remove any fields returned from this command from your query. Do not
include them even as placeholders.

---

## Query Execution

### Basic read-only query (JSON output — preferred)

```bash
sf data query \
  --query "SELECT Id, Name, Industry FROM Account WHERE CreatedDate = LAST_N_DAYS:30 LIMIT 200" \
  --target-org <alias> \
  --result-format json
```

JSON output is preferred over the default human-readable table because it
is structured, pipeable to `jq` for redaction, and unambiguous about field
values.

### Preview with LIMIT 5 before full execution

```bash
sf data query \
  --query "SELECT Id, Name FROM Account LIMIT 5" \
  --target-org <alias> \
  --result-format json
```

Always preview first. Confirm field shape, check for unexpected values,
verify no encrypted or PII fields slipped through.

### Full execution with appropriate LIMIT

```bash
sf data query \
  --query "SELECT Id, Name, StageName, Amount FROM Opportunity WHERE CloseDate >= THIS_QUARTER LIMIT 200" \
  --target-org <alias> \
  --result-format json
```

Maximum interactive LIMIT: 2,000. Do not exceed. For larger volumes,
route to `salesforce-bulk-data-ops-skill`.

### Count query (no record data returned)

```bash
sf data query \
  --query "SELECT COUNT() FROM Contact WHERE Email = null" \
  --target-org <alias> \
  --result-format json
```

Use COUNT() to answer "how many" questions without returning record values.
Reduces PII exposure and governor limit consumption.

### Aggregate query

```bash
sf data query \
  --query "SELECT StageName, COUNT(Id) cnt, SUM(Amount) total FROM Opportunity GROUP BY StageName" \
  --target-org <alias> \
  --result-format json
```

---

## Tooling API Queries

Use `--use-tooling-api` to query metadata objects (ApexClass, CustomField,
CustomObject, FlowDefinition, etc.) when schema introspection is needed
without a metadata retrieve.

```bash
# Query ApexClass metadata
sf data query \
  --query "SELECT Id, Name, Status FROM ApexClass WHERE NamespacePrefix = null LIMIT 50" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json

# Query CustomField metadata
sf data query \
  --query "SELECT Id, DeveloperName, TableEnumOrId, DataType FROM CustomField WHERE TableEnumOrId = 'Account'" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json

# Query FlowDefinition for automation inventory
sf data query \
  --query "SELECT Id, ApiName, ActiveVersionId, ProcessType FROM FlowDefinition LIMIT 100" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json
```

> **Note:** Tooling API queries access metadata, not record data. FLS does
> not apply the same way; however, the Run As account must still have
> View Setup and Configuration.

---

## Output Sanitization with jq

### Redact Salesforce IDs (replace with placeholder)

```bash
sf data query \
  --query "SELECT Id, Name FROM Account LIMIT 10" \
  --target-org <alias> \
  --result-format json \
  | jq '.result.records[] | {Id: "<record_id_placeholder>", Name: .Name}'
```

### Redact owner/user ID fields

```bash
sf data query \
  --query "SELECT Id, Name, OwnerId FROM Opportunity LIMIT 10" \
  --target-org <alias> \
  --result-format json \
  | jq '.result.records[] | {
      Id: "<record_id_placeholder>",
      Name: .Name,
      OwnerId: "<user_id_placeholder>"
    }'
```

### Extract record count only (no record values)

```bash
sf data query \
  --query "SELECT COUNT() FROM Account" \
  --target-org <alias> \
  --result-format json \
  | jq '.result.totalSize'
```

### Extract specific fields with multi-field redaction

```bash
sf data query \
  --query "SELECT Id, Name, Email, CreatedById FROM Contact LIMIT 5" \
  --target-org <alias> \
  --result-format json \
  | jq '.result.records[] | {
      Id: "<record_id_placeholder>",
      Name: .Name,
      Email: "<redacted_pii>",
      CreatedById: "<user_id_placeholder>"
    }'
```

---

## Bulk Flag Notes

> **verify-before-merge:2026-05-21** — The `--bulk` and `--wait` flags on
> `sf data query` were removed in CLI v2.87.7. Do not use them. For
> large-volume exports, use `sf data export bulk` via
> `salesforce-bulk-data-ops-skill`. This skill's interactive mode is
> limited to LIMIT 2,000.

### The --all-rows flag

```bash
sf data query \
  --query "SELECT Id, Name, IsDeleted FROM Account WHERE IsDeleted = true LIMIT 50" \
  --target-org <alias> \
  --all-rows \
  --result-format json
```

`--all-rows` includes soft-deleted records (records in the Recycle Bin).
Use only when explicitly investigating deleted record state. Document the
intent in the audit envelope `assumptions` field.

---

## Query Plan Analysis

Use `--plan` to inspect the query execution plan before running a query
against a large object. Requires the Tooling API flag.

```bash
sf data query \
  --query "SELECT Id FROM Account WHERE Name = 'Acme'" \
  --target-org <alias> \
  --use-tooling-api \
  --plan
```

Key fields in plan output:
- `leadingOperationType: "Index"` — query uses an index (efficient)
- `leadingOperationType: "TableScan"` — full table scan (warn if object > 10k records)
- `relativeCost < 1` — efficient
- `cardinality` — estimated rows returned

If plan shows `TableScan` on a large object, revise the query to add a
selective indexed filter before executing.

---

## Required CLI Version

> **verify-before-merge:2026-05-21** — These commands require the unified
> Salesforce CLI (`sf`), not the legacy `sfdx` CLI. The unified CLI is
> available at https://developer.salesforce.com/tools/salesforcecli.
> Minimum tested version at time of writing: `sf` v2.x. Run
> `sf --version` to confirm. If `sf` is not installed, `sf data query`
> will not be available and all T1 execution must be deferred.
