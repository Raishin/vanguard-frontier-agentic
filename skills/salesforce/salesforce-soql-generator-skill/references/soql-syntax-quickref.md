# SOQL Syntax Quick Reference

Quick-reference for generating correct SOQL. For full governor-limit context
see `governor-limits.md`. For pre-built patterns see `common-patterns.md`.

---

## Basic Query Shape

```soql
SELECT field1, field2, field3
FROM SObjectApiName
WHERE condition
ORDER BY field1 ASC NULLS LAST
LIMIT 200
OFFSET 0
```

All clauses except `SELECT` and `FROM` are optional. Order is fixed:
`SELECT … FROM … WHERE … WITH … GROUP BY … HAVING … ORDER BY … LIMIT … OFFSET … FOR …`

---

## SELECT

- Enumerate fields explicitly — `SELECT *` is not valid SOQL.
- Field API names are case-insensitive but use the documented casing for
  readability (`AccountId`, not `accountid`).
- Maximum 200 fields per query (API limit).
- Aggregate functions: `COUNT()`, `COUNT(field)`, `COUNT_DISTINCT(field)`,
  `SUM(field)`, `AVG(field)`, `MIN(field)`, `MAX(field)`.

### Parent field traversal (child-to-parent)

Access parent fields via dot notation. Up to 5 levels deep.

```soql
SELECT Id, Name, Account.Name, Account.Owner.Name
FROM Contact
WHERE Account.BillingCountry = 'US'
```

### Child subquery (parent-to-child)

One level of child subquery nesting. Use the child relationship name
(not the sObject API name).

```soql
SELECT Id, Name,
  (SELECT Id, StageName, Amount FROM Opportunities WHERE IsClosed = false)
FROM Account
WHERE Type = 'Customer'
LIMIT 100
```

### Polymorphic fields (TYPEOF)

Use `TYPEOF` for fields that can reference multiple sObject types.

```soql
SELECT Id, Subject,
  TYPEOF Who
    WHEN Contact THEN FirstName, LastName, Email
    WHEN Lead THEN FirstName, LastName, Company
  END
FROM Task
WHERE ActivityDate = TODAY
```

---

## FROM

Single sObject name. SOQL does not support multi-object joins — use a
subquery or two separate queries.

---

## WHERE

### Comparison operators

| Operator | Meaning |
|---|---|
| `=` | Equals |
| `!=` | Not equals |
| `<` `>` `<=` `>=` | Numeric / date comparison |
| `LIKE` | String pattern; `%` is wildcard, `_` is single char |
| `IN` | Value in list: `Status IN ('Open', 'Working')` |
| `NOT IN` | Value not in list |
| `INCLUDES` | Multi-select picklist includes value |
| `EXCLUDES` | Multi-select picklist excludes value |

### Logical operators

`AND`, `OR`, `NOT`. Use parentheses to control precedence.

### Date literals

Use these instead of hardcoded dates to respect user timezone.

| Literal | Meaning |
|---|---|
| `TODAY` | Current day |
| `YESTERDAY` | Previous day |
| `TOMORROW` | Next day |
| `THIS_WEEK` | Sun–Sat week containing today |
| `LAST_WEEK` | Previous Sun–Sat week |
| `NEXT_WEEK` | Next Sun–Sat week |
| `THIS_MONTH` | Calendar month of today |
| `LAST_MONTH` | Previous calendar month |
| `NEXT_MONTH` | Next calendar month |
| `THIS_QUARTER` | Current fiscal/calendar quarter |
| `LAST_QUARTER` | Previous quarter |
| `NEXT_QUARTER` | Next quarter |
| `THIS_YEAR` | Current year |
| `LAST_YEAR` | Previous year |
| `NEXT_YEAR` | Next year |
| `LAST_N_DAYS:n` | Last n days (0 = today) |
| `NEXT_N_DAYS:n` | Next n days |
| `LAST_N_WEEKS:n` | Last n weeks |
| `NEXT_N_WEEKS:n` | Next n weeks |
| `LAST_N_MONTHS:n` | Last n months |
| `NEXT_N_MONTHS:n` | Next n months |
| `LAST_N_QUARTERS:n` | Last n quarters |
| `NEXT_N_QUARTERS:n` | Next n quarters |
| `LAST_N_YEARS:n` | Last n years |
| `NEXT_N_YEARS:n` | Next n years |

Date literals evaluate at query execution time in the running user's timezone.
Do not mix date literals with hardcoded ISO datetime strings.

### NULL handling

```soql
WHERE Email = null        -- no email
WHERE Email != null       -- has email
WHERE LastActivityDate = null  -- never had activity
```

---

## WITH

Optional security and data-category clauses. Most commonly:

```soql
WITH SECURITY_ENFORCED  -- respects FLS and object permissions at query time
```

Use `WITH SECURITY_ENFORCED` in Apex contexts where FLS must be explicitly
enforced server-side. Not required for API queries (FLS is enforced by default).

---

## GROUP BY and HAVING

Use with aggregate functions. All non-aggregate SELECT fields must appear
in GROUP BY.

```soql
SELECT StageName, COUNT(Id) deal_count, SUM(Amount) total_amount
FROM Opportunity
WHERE IsClosed = false
GROUP BY StageName
HAVING COUNT(Id) > 5
ORDER BY SUM(Amount) DESC
```

`ROLLUP` and `CUBE` extensions are available for subtotal aggregation:

```soql
SELECT Type, BillingCountry, COUNT(Id)
FROM Account
GROUP BY ROLLUP(Type, BillingCountry)
```

---

## ORDER BY

```soql
ORDER BY FieldName ASC NULLS FIRST   -- nulls sort first
ORDER BY FieldName DESC NULLS LAST   -- nulls sort last (default)
```

Multiple fields: `ORDER BY LastModifiedDate DESC, Name ASC`

---

## LIMIT and OFFSET

```soql
LIMIT 200          -- max rows returned
OFFSET 0           -- skip first n rows (zero-indexed)
```

Ceiling: LIMIT max is 2,000 for synchronous queries (API). Governor limit:
50,000 rows total per transaction. OFFSET max is 2,000.

Prefer keyset pagination (WHERE Id > :lastId ORDER BY Id ASC LIMIT 200)
over OFFSET for large datasets to avoid the 2,000 OFFSET ceiling.

---

## FOR Clauses

```soql
FOR REFERENCE   -- updates LastReferencedDate on records
FOR VIEW        -- updates LastViewedDate on records
FOR UPDATE      -- locks records for DML (Apex only, not usable in read contexts)
```

Use `FOR REFERENCE` when the query is part of a "recently viewed" tracking
workflow. Avoid `FOR UPDATE` in generation contexts — that is a T3 concern.

---

## USING SCOPE

Filter by scope without additional WHERE conditions:

```soql
SELECT Id, Name FROM Account USING SCOPE mine       -- only records owned by running user
SELECT Id, Name FROM Account USING SCOPE team       -- team-owned records
SELECT Id, Name FROM Account USING SCOPE allRecords -- include archived/deleted (Recycle Bin)
```

---

## SOSL (Salesforce Object Search Language)

Use SOSL when searching text across multiple objects or when LIKE-based
full-text search would be too broad.

```sosl
FIND {Acme Corp} IN ALL FIELDS
RETURNING Account(Id, Name), Contact(Id, FirstName, LastName, Email)
LIMIT 50
```

SOSL uses the search index. It is faster than SOQL LIKE for full-text search
but cannot be used for structured field comparisons or aggregates.

---

## Anti-Patterns to Avoid

- `SELECT *` — not valid; enumerate fields
- No WHERE on a high-volume object — risks 50k row governor hit
- Formula field in WHERE — not indexed; causes full table scan
- Leading wildcard in LIKE: `WHERE Name LIKE '%Corp'` — defeats the text index
- Querying inside a loop in Apex context — SOQL-in-loop governor trap
- Hardcoded record IDs in WHERE — use a bind variable or dynamic query instead
- Mixing date literals with hardcoded ISO strings — timezone ambiguity
