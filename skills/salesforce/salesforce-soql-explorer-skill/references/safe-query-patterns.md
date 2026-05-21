<!-- Parent: salesforce-soql-explorer-skill/SKILL.md -->
# Safe SOQL Query Patterns — SOQL Explorer Reference

> **verify-before-merge:2026-05-21** — SOQL syntax and governor limits are
> stable across most releases, but indexed field behavior and query optimizer
> heuristics change with each Salesforce release. Verify governor limits
> against https://developer.salesforce.com/docs/atlas.en-us.salesforce_app_limits_cheatsheet.meta/salesforce_app_limits_cheatsheet/
> before production use.

---

## Non-Negotiable Query Rules

These apply to every query executed by this skill. No exceptions.

1. **Always include LIMIT.** Default: `LIMIT 200`. Maximum interactive: `LIMIT 2000`.
2. **Never use SELECT *.** Enumerate every field explicitly.
3. **Always use a selective WHERE clause** when querying objects with > 10,000 records.
4. **Never include encrypted fields** (`encrypted: true` in describe output).
5. **Never include DML keywords** (`INSERT`, `UPDATE`, `DELETE`, `MERGE`, `UPSERT`).
6. **Use COUNT() for aggregate counts** — do not load records just to count them.

---

## Indexed Fields — Use These in WHERE Clauses

Salesforce automatically indexes these fields on all standard objects:

| Field | API Name | Notes |
|---|---|---|
| Record ID | `Id` | Always selective; use for record lookup |
| Record Name | `Name` | Case-insensitive; not always unique |
| Owner | `OwnerId` | Indexed; filter by team or user scope |
| Created Date | `CreatedDate` | Indexed; use date literals |
| Last Modified Date | `LastModifiedDate` | Indexed |
| External ID fields | varies | Custom external ID fields are indexed |
| Lookup / master-detail fields | varies | Foreign key fields are indexed |
| Standard indexed fields | `Email` on Contact/Lead, `RecordTypeId` | Varies by object |

Non-indexed fields (formula fields, multi-select picklists, long text areas)
in a WHERE clause without a companion indexed filter will trigger a full table
scan. On objects > 50,000 records, non-selective queries will fail with
`QUERY_TIMEOUT` or `NUMBER_OF_QUERY_ROWS_EXCEEDED`.

---

## Safe Pattern Library

### Pattern 1 — Record lookup by ID

Fastest possible query. Always selective.

```soql
SELECT Id, Name, BillingCity, Phone
FROM Account
WHERE Id = '001Xx000001ABCxxx'
LIMIT 1
```

### Pattern 2 — Pipeline query by date range and stage

Uses indexed `StageName` (picklist, standard indexed) and `CloseDate` (indexed).

```soql
SELECT Id, Name, StageName, Amount, CloseDate
FROM Opportunity
WHERE StageName NOT IN ('Closed Won', 'Closed Lost')
  AND CloseDate >= THIS_QUARTER
  AND CloseDate <= NEXT_QUARTER
LIMIT 200
```

### Pattern 3 — Account by owner (team scope)

```soql
SELECT Id, Name, Industry, AnnualRevenue
FROM Account
WHERE OwnerId IN (
  SELECT Id FROM User WHERE UserRole.Name = 'Account Executive'
)
LIMIT 200
```

### Pattern 4 — Contact by domain (email suffix)

> **Note:** `LIKE` with a leading wildcard (`%@example.com`) defeats
> indexes. Use only when the result set is bounded by an additional indexed
> filter like `AccountId` or `CreatedDate`.

```soql
SELECT Id, FirstName, LastName, Email
FROM Contact
WHERE AccountId = '001Xx000001ABCxxx'
  AND Email LIKE '%@example.com'
LIMIT 200
```

### Pattern 5 — Count by category (aggregate — no record data)

```soql
SELECT Priority, COUNT(Id) cnt
FROM Case
WHERE IsClosed = false
  AND CreatedDate = LAST_N_DAYS:30
GROUP BY Priority
```

No record IDs or PII fields returned. Use this for "how many" questions.

### Pattern 6 — Open cases without an assigned owner

Uses boolean filter on indexed `OwnerId` combined with `IsClosed`.

```soql
SELECT Id, CaseNumber, Subject, CreatedDate
FROM Case
WHERE OwnerId = null
  AND IsClosed = false
LIMIT 200
```

### Pattern 7 — Child-to-parent relationship query

Traverses one level. Keep relationship depth to one level unless the
extra level is required.

```soql
SELECT Id, Name, Account.Name, Account.BillingCity
FROM Contact
WHERE Account.Industry = 'Technology'
  AND CreatedDate = THIS_YEAR
LIMIT 200
```

### Pattern 8 — Parent-to-child subquery

Keep subquery result sets small. Subqueries are subject to 200-record
inner-query limits <!-- verify-before-merge:2026-05-21 -->.

```soql
SELECT Id, Name,
  (SELECT Id, Subject, Status FROM Cases WHERE IsClosed = false LIMIT 5)
FROM Account
WHERE AnnualRevenue > 1000000
LIMIT 50
```

### Pattern 9 — Opportunity pipeline aggregate by owner

```soql
SELECT OwnerId, StageName, COUNT(Id) cnt, SUM(Amount) total
FROM Opportunity
WHERE CloseDate >= THIS_QUARTER
  AND IsClosed = false
GROUP BY OwnerId, StageName
ORDER BY total DESC
LIMIT 200
```

> **Redaction reminder:** `OwnerId` in aggregate results must be replaced
> with `<user_id_placeholder>` before emission.

### Pattern 10 — Records modified in last N hours (incident triage)

```soql
SELECT Id, Name, LastModifiedById, LastModifiedDate
FROM Account
WHERE LastModifiedDate = LAST_N_HOURS:24
LIMIT 200
```

> **Redaction reminder:** `LastModifiedById` must be replaced with
> `<user_id_placeholder>`.

---

## OFFSET Usage

OFFSET is supported but has a hard limit of 2,000 records
<!-- verify-before-merge:2026-05-21 -->. Use sparingly and only for
small paginated result sets where ORDER BY is defined:

```soql
SELECT Id, Name FROM Account ORDER BY CreatedDate DESC LIMIT 50 OFFSET 50
```

Do not use OFFSET to paginate through large objects. Use `sf data export bulk`
via `salesforce-bulk-data-ops-skill` for large-volume pagination.

---

## Date Literals Reference

Salesforce date literals avoid timezone ambiguity and use server-side
indexed date values:

| Literal | Meaning |
|---|---|
| `TODAY` | Current day (midnight to midnight in org timezone) |
| `THIS_WEEK` | Current week |
| `THIS_MONTH` | Current calendar month |
| `THIS_QUARTER` | Current fiscal or calendar quarter |
| `THIS_YEAR` | Current year |
| `LAST_N_DAYS:n` | Previous n days (e.g., `LAST_N_DAYS:30`) |
| `LAST_N_HOURS:n` | Previous n hours |
| `NEXT_N_DAYS:n` | Next n days |
| `LAST_QUARTER` | Previous quarter |
| `NEXT_QUARTER` | Following quarter |

Date literals are always safe to use in WHERE clauses on date/datetime fields
and will use the index.

---

## Bind Variables

Bind variables (`:variableName`) are not supported in the `sf data query`
CLI. Use literal values in WHERE clauses when executing from the CLI.

```soql
-- This works in Apex but NOT in sf CLI
SELECT Id FROM Account WHERE Name = :accountName  ← not valid in CLI

-- Use literal values in CLI
SELECT Id FROM Account WHERE Name = 'Acme Corp'   ← valid
```

If a query requires dynamic values that would come from a variable, construct
the literal string carefully and audit it for injection risk (a concern even
in read-only contexts if the query is constructed from user input).

---

## Anti-Patterns — Do Not Use

### Anti-pattern 1 — SELECT * equivalent (enumerating all fields without selection)

```soql
-- WRONG: querying every field without need
SELECT Id, Name, Phone, Fax, Email, Website, BillingStreet, BillingCity,
       BillingState, BillingPostalCode, BillingCountry, ShippingStreet, ...
FROM Account LIMIT 200
```

Query only the fields the task actually requires.

### Anti-pattern 2 — No LIMIT on a large object

```soql
-- WRONG: no LIMIT on a high-volume object
SELECT Id, Name FROM Contact WHERE LastName = 'Smith'
```

Always include LIMIT. Use COUNT() if the goal is a count, not records.

### Anti-pattern 3 — Non-indexed WHERE without companion indexed filter

```soql
-- WRONG: formula field in WHERE with no indexed companion filter
SELECT Id, Name FROM Account WHERE My_Formula_Field__c = 'Active'
```

Formula fields are not indexed. This causes a full table scan. Add an
indexed field filter (e.g., `CreatedDate = THIS_YEAR`) as a companion.

### Anti-pattern 4 — Leading wildcard LIKE on unfiltered large object

```soql
-- WRONG: leading wildcard defeats index on large object
SELECT Id, Name FROM Contact WHERE Email LIKE '%@gmail.com'
```

Add a companion indexed filter (AccountId, OwnerId, CreatedDate) to bound
the scan before using LIKE.

### Anti-pattern 5 — OFFSET without ORDER BY

```soql
-- WRONG: OFFSET without ORDER BY produces non-deterministic pages
SELECT Id FROM Account LIMIT 50 OFFSET 50
```

Always pair OFFSET with ORDER BY.

### Anti-pattern 6 — Querying encrypted fields

```soql
-- WRONG: querying an encrypted field
SELECT Id, Name, SSN__c FROM Contact LIMIT 10
```

If `SSN__c` has `encrypted: true` in the describe output, remove it.
The Run As account does not have View Encrypted Data permission and the
query will either fail or return masked values — either way the result
is unusable and the attempt must be logged.

### Anti-pattern 7 — DML in a query context (not applicable to CLI but listed for awareness)

```soql
-- NOT POSSIBLE in sf data query but refuse if attempted
UPDATE Account SET Name = 'New Name' WHERE Id = '001...'  ← DML, not SOQL SELECT
```

The `sf data query` command only accepts SELECT statements. Any request
to execute DML must be refused and routed to the human approval path.

---

## Governor Limit Awareness

| Limit | Value | Notes |
|---|---|---|
| Synchronous query rows | 50,000 per transaction | Applies to interactive `sf data query` |
| Aggregate query rows | 2,000 | GROUP BY result rows |
| SOQL query depth (subqueries) | 1 level of child subquery | One subquery per query |
| OFFSET maximum | 2,000 | Hard limit |
| Bulk API batch size | 10,000 rows | Handled by bulk ops skill |

> **verify-before-merge:2026-05-21** — Governor limit values are stable
> across most releases but confirm at
> https://developer.salesforce.com/docs/atlas.en-us.salesforce_app_limits_cheatsheet.meta/salesforce_app_limits_cheatsheet/
> before production use.

If a query would approach the 50,000-row synchronous limit, add a tighter
LIMIT or date range filter, or route to `salesforce-bulk-data-ops-skill`.
