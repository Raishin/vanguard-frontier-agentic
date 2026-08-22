# Visibility Prediction and Consumption Paths

How to state what each role class will see, and how to find the paths where protection does not follow. Load during preflight — these two artifacts are what the approver approves.

## The per-role-class visibility prediction

- For each role class, state one of three outcomes: sees the full value, sees the masked value, or sees fewer rows — and for a row-access change, state approximately how many fewer, from a count comparison rather than from the policy text.
- Enumerate the classes explicitly and individually: each human role that reads the object, each service account, the BI tool's identity, the replication path, and any agent identity. The generic phrase 'service accounts' is how one gets missed.
- Derive the class list from access history, not from the role model. The roles that actually read the object are the ones affected, and they are frequently not the roles anyone expected.
- Test the prediction before execution — in a non-production environment or against a test object carrying the same policy — and record the result. An untested prediction is a hypothesis and this guard does not execute on hypotheses.
- Mark any class that could not be tested as `UNKNOWN` rather than assuming it behaves like a tested one. Service and agent identities frequently differ from human roles in exactly the way that matters.

## Paths where protection stops

- **Views** — a consumer reading through a view may or may not inherit the base object's protection. Trace it; this is the single most common way a masking programme silently fails.
- **Clones** — cloning produces a sibling whose policy references must be checked independently.
- **Shares and listings** — data leaving the account is a separate exposure decision with its own consumer-side visibility.
- **Replicas** — a secondary region is a second copy with its own policy state; whether the protection replicates is a fact to verify with the BCDR agent, not to assume.
- **Materialized copies and exports** — any pipeline reading the protected column and writing it elsewhere has created an unprotected copy unless the target is protected too.
- **Semi-structured payloads** — sensitive values inside JSON or free-text columns are not reached by column-level masking on a sibling column in the way people assume. State this rather than reporting the object as protected.
- Enumerate the paths before attaching, and state explicitly which ones the protection will not reach. That list, not the attachment, is what determines whether the data is actually protected.

## Evidence queries

Establish prior attachment state and check for a conflicting tag-based attachment.

```sql
SELECT policy_kind, policy_name,
       ref_database_name || '.' || ref_schema_name || '.' || ref_entity_name AS ref_object,
       ref_column_name
  FROM SNOWFLAKE.ACCOUNT_USAGE.POLICY_REFERENCES
 WHERE ref_database_name = '<DB>'
   AND ref_schema_name   = '<SCHEMA>'
   AND ref_entity_name   = '<OBJECT>';

SELECT tag_name, tag_value, column_name
  FROM SNOWFLAKE.ACCOUNT_USAGE.TAG_REFERENCES
 WHERE object_database = '<DB>'
   AND object_schema   = '<SCHEMA>'
   AND object_name     = '<OBJECT>';
-- A tag-based attachment plus a direct one is a state nobody can reason
-- about later. Reconcile it with the data owner before proceeding.
```

Derive the affected role classes from what actually reads the object.

```sql
SELECT ah.role_name,
       ah.user_name,
       COUNT(*)                 AS accesses,
       MAX(ah.query_start_time) AS last_access
  FROM SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY ah,
       LATERAL FLATTEN(input => ah.base_objects_accessed) f
 WHERE ah.query_start_time >= DATEADD(day, -90, CURRENT_TIMESTAMP())
   AND f.value:objectName::string = '<DB>.<SCHEMA>.<OBJECT>'
 GROUP BY 1, 2
 ORDER BY accesses DESC;
-- Every distinct role here needs a line in the visibility prediction.
-- Ninety days to catch monthly consumers.
```

Find the consumption paths the protection may not follow.

```sql
SELECT referencing_database || '.' || referencing_schema || '.' || referencing_object_name AS dependent_object,
       referencing_object_domain,
       referenced_object_name
  FROM SNOWFLAKE.ACCOUNT_USAGE.OBJECT_DEPENDENCIES
 WHERE referenced_database = '<DB>'
   AND referenced_schema   = '<SCHEMA>'
   AND referenced_object_name = '<OBJECT>';
-- Each dependent object is a path to check. Lineage stops at exports and
-- external copies, so state that boundary as UNKNOWN rather than clear.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/security-column-intro — Masking policy semantics and that the result depends on the querying role — the basis for a per-role-class prediction
- https://docs.snowflake.com/en/sql-reference/account-usage/policy_references — How policy attachment is recorded, and therefore how attachment is distinguished from policy existence
