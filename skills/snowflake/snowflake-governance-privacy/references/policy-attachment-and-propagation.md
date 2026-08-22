# Policy Attachment and Propagation

The difference between a policy existing, applying, and behaving — and the paths where protection silently stops. Load for any coverage claim.

## Three distinct claims

- **Existence** — the policy object is defined. Evidence: `SHOW MASKING POLICIES` / `SHOW ROW ACCESS POLICIES`. This proves almost nothing on its own.
- **Attachment** — the policy is bound to a specific column or table. Evidence: `POLICY_REFERENCES`. This is the claim most 'coverage' reports actually need and rarely make.
- **Behaviour** — a given role, querying through a given path, sees the intended result. Evidence: evaluating the policy logic per role class, and testing before production. This is the only claim that maps to the business outcome.
- Report all three separately. Collapsing them is how a governance programme reports success while the data is exposed.

## Where protection stops

- **Views.** A consumer reading a view over a protected base table may or may not inherit the protection depending on how the view and policy are defined. Trace it; do not assume it.
- **Clones.** Cloning is a common way a protected dataset acquires an unprotected sibling. Check what policy references the clone carries.
- **Shares and listings.** Data leaving the account through a share is a separate exposure decision with its own consumer-side visibility.
- **Replicas.** A secondary region is a second copy with its own policy state. Whether the protection replicates with the object is a fact to verify, not to assume — and it is a joint finding with the BCDR agent.
- **Materialized copies and exports.** Any pipeline that reads a protected column and writes it elsewhere has created an unprotected copy unless the target is protected too. Lineage is how you find these; incomplete lineage is why you miss them.
- **Unstructured and semi-structured payloads.** Sensitive values inside JSON, free text, or concatenated fields are not reached by column-level masking on the containing column in the way people assume. State this explicitly rather than reporting the column as protected.

## Tag-based attachment

- Attaching policies through tags is what makes protection scale to objects that do not exist yet — and it is what makes coverage depend entirely on tagging discipline.
- Tag inheritance follows the object hierarchy. Determine which assignments in `TAG_REFERENCES` are direct and which are inherited, because removing a parent tag removes the inherited protection everywhere below it.
- A tag taxonomy with values nobody maps to a policy is an index, not a control. For each tag value, state which policy it attaches or explicitly state that it attaches none.

## Evidence queries

Establish attachment rather than existence — which policies are actually bound to which columns and tables.

```sql
SELECT policy_kind,
       policy_name,
       ref_entity_domain,
       ref_database_name || '.' || ref_schema_name || '.' || ref_entity_name AS ref_object,
       ref_column_name
  FROM SNOWFLAKE.ACCOUNT_USAGE.POLICY_REFERENCES
 ORDER BY policy_kind, policy_name, ref_object;
```

Find the gap that matters — columns identified or tagged as sensitive with no policy attached.

```sql
WITH tagged AS (
  SELECT object_database || '.' || object_schema || '.' || object_name AS obj,
         column_name,
         tag_name,
         tag_value
    FROM SNOWFLAKE.ACCOUNT_USAGE.TAG_REFERENCES
   WHERE column_name IS NOT NULL
),
protected AS (
  SELECT ref_database_name || '.' || ref_schema_name || '.' || ref_entity_name AS obj,
         ref_column_name AS column_name
    FROM SNOWFLAKE.ACCOUNT_USAGE.POLICY_REFERENCES
   WHERE policy_kind = 'MASKING_POLICY'
)
SELECT t.obj, t.column_name, t.tag_name, t.tag_value
  FROM tagged t
  LEFT JOIN protected p
         ON p.obj = t.obj AND p.column_name = t.column_name
 WHERE p.column_name IS NULL
 ORDER BY t.obj, t.column_name;
-- This is the 'tagged but unprotected' metric. Report it separately from
-- 'untagged' — a programme can score well on one while failing the other.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/security-column-intro — Masking policy semantics, including conditional masking and how a policy is evaluated for the querying role
- https://docs.snowflake.com/en/user-guide/security-row-intro — Row-access policy semantics and their interaction with mapping tables and session context
- https://docs.snowflake.com/en/user-guide/object-tagging — Tag inheritance through the object hierarchy — the basis for distinguishing direct from inherited assignment
