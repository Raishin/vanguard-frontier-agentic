---
name: salesforce-soql-generator-skill
description: "Generates SOQL queries from natural-language requirements without executing them. T0 generation skill — emits ready-to-paste SOQL with selectivity analysis, governor-limit guidance, and a 100-point quality score. Pairs with salesforce-soql-explorer-skill for live execution. TRIGGER when: user describes a query in plain English, asks for SOQL syntax help, needs an aggregate or relationship query, wants a pipeline hygiene query, needs report-equivalent SOQL. Trigger phrases: \"write SOQL for\", \"how do I query\", \"give me a SOQL query\", \"find all opportunities where\", \"count contacts with\", \"pipeline by stage SOQL\", \"stale leads query\". DO NOT TRIGGER when: the query needs to be executed live (use salesforce-soql-explorer-skill); when DML is required (T3 — escalate); when the user wants permission-set diagnostic SOQL (specialized form — use salesforce-permission-model-review-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: 0.1.0
  updated: 2026-05-21
  category: generation
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
---

# salesforce-soql-generator-skill

Pure generation skill — plain English to production-ready SOQL. This skill
is a **translator**, not an executor. It produces ready-to-paste queries with
selectivity analysis and governor-limit guidance, then hands off to
`salesforce-soql-explorer-skill` for live execution.

## When This Skill Owns the Task

Use `salesforce-soql-generator-skill` when the work requires **query authoring**
from a description, not live data retrieval:

- "Write SOQL to find all opportunities closing this quarter with no next step"
- "Give me a query to count contacts by account with no email"
- "How do I query stale leads not modified in 90 days?"
- "I need aggregate SOQL for pipeline by stage and owner"
- "Generate the SOQL equivalent of this report"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Query must be executed against a live org | `salesforce-soql-explorer-skill` |
| User needs permission-set or profile diagnostic SOQL | `salesforce-permission-model-review-skill` |
| Query result will drive DML or automation | Review with `salesforce-metadata-review-skill` first |
| Apex selector pattern needed (not raw SOQL) | `querying-soql` (sf-skills) |
| Report or dashboard conversion | Assess whether SOQL truly maps to Report; flag gap |

---

## Required Context to Gather First

Before generating any query, confirm or infer:

1. **Target sObject** — API name (e.g., `Opportunity`, `Lead`, `Contact`,
   `My_Custom_Object__c`). Is this a single-object or multi-level relationship
   query?
2. **Fields needed** — enumerate explicitly, or accept 'minimal' and choose
   the fewest informative fields.
3. **Filter criteria** — WHERE conditions, date literals, owner scope, record
   type, status values.
4. **Sort and pagination** — ORDER BY field, ASC/DESC, LIMIT, OFFSET if needed.
5. **Intended use** — display in UI, CSV export, automation trigger, Apex
   selector, or report equivalent. Affects LIMIT choice and field set.
6. **Expected result volume** — rough estimate. Affects selectivity guidance.
7. **Relationship depth** — parent lookup traversal, child subquery, or
   polymorphic relationship?

If any of these are ambiguous, make a conservative assumption, state it
explicitly, and offer an alternative.

---

## Recommended Workflow

### Step 1 — Confirm target sObject and primary purpose

Identify the root sObject and clarify whether the query is for:
- Record retrieval (display, export)
- Aggregate analysis (GROUP BY, COUNT, SUM)
- Relationship traversal (parent fields from child, child subquery from parent)
- Anti-join / semi-join (NOT IN, EXISTS equivalent)

State the confirmed interpretation before generating.

### Step 2 — Generate the simplest correct query

Apply these constraints from the start:
- Only enumerate required fields — no `SELECT *`
- Use the most selective WHERE clause available (indexed fields first)
- Apply LIMIT appropriate to the use case (default 200 for display, omit for
  aggregates that return few rows, use 50000-ceiling awareness for exports)
- Traverse relationships only as deep as necessary

Produce one canonical query first, then offer variants.

### Step 3 — Score selectivity

Evaluate the generated query against the selectivity rubric:

- Does the WHERE clause use at least one **indexed field**? (Id, Name, OwnerId,
  ExternalId__c, CreatedDate, standard lookup fields, custom external IDs)
- Is a LIMIT present (where appropriate)?
- Is the field count ≤ 10?
- Are date literals used correctly (TODAY, THIS_WEEK, LAST_N_DAYS:n)?
- Are formula fields or non-indexed fields used in WHERE without a companion
  indexed filter?

Document the selectivity assessment inline.

### Step 4 — Add governor-limit guidance

For every generated query, provide:
- Estimated row return range and relationship to the 50,000-row limit
- Whether a LIMIT is essential or advisory
- Whether the query is safe to embed in a transaction (≤ 100 SOQL queries per
  transaction ceiling awareness)
- Whether `USING SCOPE` or `FOR REFERENCE` / `FOR VIEW` / `FOR UPDATE` locking
  hints apply

### Step 5 — Provide variant queries

Emit at least two variants beyond the canonical query:
- **Filtered variant** — narrower scope (add date filter, status filter, or
  owner scope)
- **Aggregate variant** — GROUP BY with COUNT / SUM where applicable
- **Paginated variant** — with LIMIT + OFFSET or a keyset-pagination pattern
  using WHERE Id > :lastId when the result set could exceed one page

### Step 6 — Output ready-to-paste SOQL and handoff recommendation

Emit the structured output (see Output Format). Always include a
`handoff_recommendation` pointing to `salesforce-soql-explorer-skill` for
live execution, or to the appropriate review skill if the query surfaces a
governance concern.

---

## Quality Scoring Rubric (100-point)

Score every generated query before emitting. Threshold: 80+ acceptable for
paste-ready output; 60–79 emit with caveat; below 60 revise before emitting.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Selectivity** | 30 | WHERE uses at least one indexed field; no full-table-scan pattern on objects likely > 10k records |
| **Field minimality** | 20 | Only required fields; no `SELECT *`; field count ≤ 10 for display queries |
| **LIMIT present** | 15 | LIMIT applied where result set could be unbounded; aggregate queries exempt when GROUP BY guarantees few rows |
| **Syntax correctness** | 20 | Valid SOQL syntax; correct relationship traversal notation; correct date literal usage; no reserved word conflicts |
| **Governor-limit safety** | 15 | Row return estimate well below 50k ceiling; embeddable in transaction; no query-in-loop anti-pattern implied |

**Scoring penalties:**
- No WHERE clause on a high-volume object (> 50k records likely): −20
- `SELECT *` pattern: −15
- Missing LIMIT on unbounded query: −15
- Formula field in WHERE without indexed companion: −10
- Incorrect date literal syntax: −10

---

## Output Format

```yaml
generated_query: |
  SELECT <fields>
  FROM <SObject>
  WHERE <filter>
  ORDER BY <field> <ASC|DESC>
  LIMIT <n>

interpretation: "<what was understood from the natural-language request>"

alternate_queries:
  filtered: |
    SELECT <fields>
    FROM <SObject>
    WHERE <narrower filter>
    LIMIT <n>
  aggregate: |
    SELECT <grouping_field>, COUNT(Id) record_count
    FROM <SObject>
    WHERE <filter>
    GROUP BY <grouping_field>
  paginated: |
    SELECT <fields>
    FROM <SObject>
    WHERE Id > :lastId AND <filter>
    ORDER BY Id ASC
    LIMIT 200

selectivity_analysis:
  indexed_filters_used: ["<field1>", "<field2>"]
  non_indexed_filters: ["<field>"]
  full_table_scan_risk: "low | medium | high"
  notes: "<explanation>"

governor_limit_notes:
  estimated_row_range: "<low estimate> – <high estimate>"
  limit_advisory: "<why this LIMIT was chosen>"
  transaction_safe: true | false
  notes: "<any ceiling-proximity warnings>"

suggested_index:
  - field: "<FieldApiName>"
    reason: "<why an index on this field would help>"

score: <0-100>
score_breakdown:
  selectivity: <0-30>
  field_minimality: <0-20>
  limit_present: <0-15>
  syntax_correctness: <0-20>
  governor_limit_safety: <0-15>
score_notes: "<what drove the score; any penalties applied>"

handoff_recommendation:
  execute_live: "salesforce-soql-explorer-skill"
  governance_review: "<skill if a concern surfaced, else 'none'>"
  notes: "<when to execute, any pre-execution checks>"

assumptions:
  - "<explicit list of assumptions made where input was ambiguous>"
```

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| User wants to execute the generated query against a live org | `salesforce-soql-explorer-skill` |
| Query reveals a governance or access concern | `salesforce-permission-model-review-skill` |
| Query is a Report-equivalent but a Report likely meets the need | Note the gap; suggest the Report builder as an alternative |
| Permission diagnostic SOQL needed | `salesforce-permission-model-review-skill` |
| Apex selector pattern needed | `querying-soql` (sf-skills reference) |

---

## Common Pitfalls

These are the most frequent errors in generated SOQL. Check each before
emitting the final query.

### SELECT * pattern

SOQL does not support `SELECT *`. Always enumerate fields. When fields are
unknown, use a minimal set (`Id`, `Name`, one or two distinguishing fields)
and note that the user should expand once the schema is confirmed.

### Missing LIMIT

Any query on a high-volume object without a LIMIT risks hitting the 50,000-row
governor ceiling or causing a query timeout. Always include LIMIT unless:
- The query is a COUNT aggregate with GROUP BY that produces few groups
- The user has explicitly confirmed they need all records and the object has
  low cardinality

### Non-indexed filters

Filtering on formula fields, long-text-area fields, or custom fields without
an explicit custom index is a common performance trap. Always flag these in
`selectivity_analysis.non_indexed_filters` and recommend that the user check
with their Salesforce admin whether a custom index exists or is needed.

### Date literals

Use Salesforce date literal syntax precisely:
- `TODAY`, `YESTERDAY`, `TOMORROW`
- `THIS_WEEK`, `LAST_WEEK`, `NEXT_WEEK`
- `THIS_MONTH`, `LAST_MONTH`, `NEXT_MONTH`
- `THIS_QUARTER`, `LAST_QUARTER`, `NEXT_QUARTER`
- `THIS_YEAR`, `LAST_YEAR`, `NEXT_YEAR`
- `LAST_N_DAYS:n`, `NEXT_N_DAYS:n`
- `LAST_N_WEEKS:n`, `NEXT_N_WEEKS:n`
- `LAST_N_MONTHS:n`, `NEXT_N_MONTHS:n`

Do not mix date literals with datetime comparisons without explicit timezone
handling. Salesforce date literals respect the running user's timezone.

### Formula fields in WHERE

Formula fields are not indexed and cannot use standard indexes. Including a
formula field in a WHERE clause without a companion indexed filter forces a
full table scan. Flag this in `selectivity_analysis` and suggest an
alternative (e.g., use the underlying source fields instead of the formula).

### Polymorphic relationships

Querying the `Who` or `What` fields on `Task` / `Event`, or the `Owner`
field when it can be a User or Queue, requires a `TYPEOF` expression or a
`Type` filter. Incorrect assumptions produce incomplete results. Always
clarify whether the relationship is polymorphic and use `TYPEOF` where needed.

### Deep relationship traversal

SOQL allows up to 5 levels of parent traversal and up to 1 level of child
subquery nesting (for relationship queries). Beyond these limits, the query
will fail at runtime. Flag any traversal depth that approaches the limit.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/soql-syntax-quickref.md` | Syntax reference for SELECT, WHERE, GROUP BY, HAVING, ORDER BY, LIMIT, FOR clauses |
| `references/common-patterns.md` | Pre-built patterns for pipeline, stale records, ownership audit, junction objects |
| `references/governor-limits.md` | Row limits, query count limits, LDV considerations, index guidance |
