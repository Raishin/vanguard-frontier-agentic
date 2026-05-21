# SOQL Governor Limits and Performance Guidance

Reference for generating governor-limit-safe SOQL. Every generated query must
be assessed against these limits before emission.

---

## Core Query Limits

| Limit | Value | Notes |
|---|---|---|
| Total SOQL queries per synchronous Apex transaction | 100 | Shared across all triggers, classes, and flows in the transaction |
| Total SOQL queries per asynchronous Apex (batch, future, queueable) | 200 | Higher ceiling for async contexts |
| Total rows returned across all queries in a synchronous transaction | 50,000 | Hard ceiling — query that would return > 50k rows fails |
| Total rows returned in an asynchronous Apex transaction | 50,000,000 | Batch Apex contexts only |
| Maximum rows returned by a single synchronous query | 50,000 | Applies per query; aggregate queries not affected |
| Maximum LIMIT value (synchronous API query) | 2,000 | Interactive/SOQL via API |
| Maximum LIMIT value (Apex query) | 50,000 | Apex can hold more; still constrained by 50k total |
| Maximum OFFSET value | 2,000 | Use keyset pagination beyond 2k rows |
| Maximum fields per SELECT | 200 | API-enforced |
| Maximum relationship traversal depth (parent) | 5 levels | e.g., `Contact.Account.Owner.Profile.Name` |
| Maximum subquery nesting (child) | 1 level | One child subquery per parent query |
| Query timeout | ~120 seconds | Varies; complex scans on large objects time out earlier |

---

## 50,000-Row Ceiling: Critical Guidance

The 50,000-row limit is the most commonly hit governor in SOQL-heavy
automation. Every generated query must account for this.

**Rules for generated queries:**

1. If the object could realistically hold > 50k records (Account, Contact,
   Opportunity, Lead, Task, Event, Case in large orgs), always include LIMIT.
2. If the user's stated purpose is "export all", warn about the ceiling and
   recommend `salesforce-bulk-data-ops-skill` for volumes > 2,000 records.
3. If the query is embedded in an Apex transaction (trigger, controller,
   flow-called Apex), assume the 50k total is shared — a conservative LIMIT
   of 200–1,000 per query is safer.
4. Use `COUNT()` queries to estimate volume before recommending a non-LIMIT
   query:

```soql
SELECT COUNT() FROM Account WHERE Type = 'Customer'
```

---

## 100-SOQL-per-Transaction Ceiling

In Apex (triggers, controllers, batch execute, queueable), every `[SOQL]`
call or `Database.query()` counts against the 100-query limit for synchronous
contexts.

**Implications for generation:**

- Do not generate patterns that imply a query per record (query-in-loop
  anti-pattern). Always note if the pattern must be bulkified.
- If the user describes a loop-based need, suggest a single query with an
  `IN` clause or a subquery instead:

```soql
-- WRONG (query-in-loop implication): one query per account
-- CORRECT: single query covering all accounts
SELECT Id, Name, AccountId
FROM Contact
WHERE AccountId IN :accountIds
```

---

## Large Data Volume (LDV) Considerations

For orgs with millions of records on a single object:

| Consideration | Guidance |
|---|---|
| Filter on indexed fields | Always use Id, Name, ExternalId, standard indexed lookups in WHERE first |
| Avoid leading wildcard LIKE | `WHERE Name LIKE '%Corp'` defeats the text index — use `LIKE 'Corp%'` or SOSL |
| Custom indexes | Request custom indexes from Salesforce Support for frequently-filtered non-standard fields |
| Skinny tables | Org admins can enable skinny tables for specific objects and field sets — reduces I/O for wide-table queries |
| Selective query threshold | Salesforce requires a WHERE filter to be selective (< 10% of rows) on objects with > 100k records; non-selective queries are throttled or rejected |

**Selectivity threshold rule of thumb:**

- Object has < 100k records: most queries are acceptable
- Object has 100k–1M records: WHERE must include at least one indexed field
- Object has > 1M records: WHERE must be highly selective (prefer Id or ExternalId filters, date range with indexed fields)

---

## Indexed Fields

Standard indexed fields (available by default, no admin action needed):

| Object | Indexed Fields |
|---|---|
| All objects | `Id`, `Name`, `OwnerId`, `CreatedDate`, `LastModifiedDate`, `SystemModstamp` |
| All objects with ExternalId | Custom fields marked as External ID |
| Account | `BillingCountry`, `ParentId`, `RecordTypeId` |
| Contact | `AccountId`, `Email`, `RecordTypeId` |
| Opportunity | `AccountId`, `CloseDate`, `IsClosed`, `StageName`, `RecordTypeId` |
| Lead | `Email`, `IsConverted`, `Status`, `OwnerId` |
| Case | `AccountId`, `ContactId`, `IsClosed`, `Status`, `RecordTypeId` |
| Task / Event | `WhatId`, `WhoId`, `OwnerId`, `ActivityDate` |

**Custom indexes** can be requested for non-standard fields via Salesforce
Support (for non-formula, non-encrypted, non-text-area fields). Use the
`suggested_index` field in the skill output to flag candidates.

**Formula field rule:** Formula fields are **never indexed**. Never place a
formula field as the sole filter in a WHERE clause on a high-volume object.
Instead, filter on the underlying source fields or use a custom index on a
stored (non-formula) equivalent field.

---

## Aggregate Queries and Limits

Aggregate queries (`GROUP BY`, `COUNT`, `SUM`, etc.) are **not subject to the
50,000-row-returned limit** in the same way as record queries — they return
aggregated results, not raw rows. However:

- The rows *scanned* during aggregation still count toward internal processing
  limits and can time out on very large objects without a selective WHERE.
- The result set of an aggregate query (number of groups) can still be large
  — apply a LIMIT or HAVING clause if the GROUP BY field has high cardinality.

---

## Query Timeout Signals

A query is at risk of timeout when:

- No indexed filter is present and the object has > 100k records
- A `LIKE '%pattern'` leading wildcard is used on a large text field
- A formula field appears in WHERE without a companion indexed filter
- A subquery result set is large (> 10k rows in the IN clause)
- Relationships are traversed more than 2 levels deep on a large object

Always flag these risks in `governor_limit_notes` in the skill output.

---

## Bulk Data Operations — When to Escalate

Generated SOQL is for synchronous interactive or Apex-transaction use. When
the user's need implies:

- Exporting > 2,000 records interactively
- Processing > 50,000 records in a single operation
- Scheduled batch extraction or load

Escalate to `salesforce-bulk-data-ops-skill`. The Bulk API 2.0 bypasses the
50,000-row governor for export and is the correct tool for large-volume work.

---

## Recommended Query Size Budget

Use this as a default budget when the user does not specify volume:

| Use case | Recommended LIMIT |
|---|---|
| UI display / developer inspection | 5–50 |
| Admin review / hygiene report | 200–500 |
| Export to CSV (small set) | 500–2,000 |
| Apex selector (per-transaction) | 200 |
| Aggregate (few groups) | Omit LIMIT or use 50–200 |
| Count only | No LIMIT needed (COUNT aggregate) |
