# Plan Review, Pipeline, and Rollback

What to read in a plan, how to bound the deployment identity, and what a Snowflake rollback can and cannot do. Load before approving any change or designing a pipeline.

## Reading a plan for safety, not for syntax

- **Destroy** — an object is dropped. State what depended on it: grants pointing at it, views selecting from it, tasks referencing it, shares including it. The plan does not show these; a dependency query does.
- **Replace** — the resource is destroyed and re-created. Between those two moments the object does not exist, and anything holding a grant on it loses it. For grants and roles this window is an access outage; for tables it can be data loss.
- **Grant and ownership changes** — a revoke followed by a grant is not idempotent from the perspective of anyone querying in between, and an ownership transfer changes who can grant on the object afterwards.
- **In-place updates that are secretly replaces** — some attribute changes force recreation. The plan says so; readers skim past it.
- For every destroy and replace on a production object, state the gap window and what happens in it. That statement is the deliverable of a plan review.
- Never request or paste raw plan output with values or a state file: both can contain sensitive values. Review the operations and resource addresses.

## The deployment identity

- This identity can change everything the pipeline manages, from any environment the pipeline can reach, on any commit that passes review. It is the highest-value credential in the estate.
- It should be `TYPE = SERVICE` with key-pair or, better, workload identity federation so no Snowflake credential is stored at all. Password authentication for it is a hard finding.
- Its `DEFAULT_ROLE` should be the narrow deployment role, never a system role. Published CI/CD tutorials commonly show `DEFAULT_ROLE = ACCOUNTADMIN`; copying that makes every pipeline compromise an account compromise.
- Where workload identity federation is used, the subject binding is the authorization boundary. A subject scoped to a whole repository or organization rather than a specific branch, environment, or service connection is over-broad in exactly the way a leaked static credential would be.
- Compare privileges held against privileges exercised, from query history under that identity. The difference is removable, and removing it is the cheapest blast-radius reduction available in the estate.

## Rollback for changes that have no inverse

- Reverting a commit produces a new plan, not the previous state. For anything destroyed, the revert is a re-create with a gap; for anything renamed or transferred, it may not be reachable at all.
- Classify each change in the plan: cleanly reversible (a parameter, a warehouse size), reversible with a gap (a grant, a role, a view), and not reversible by the pipeline (a dropped table with data, an ownership transfer, a consumed migration).
- For the third class, the real control is the gate before the apply, not the rollback after it. Say so explicitly rather than writing a rollback section that cannot work.
- Behaviour-change bundles have their own rollback semantics and their own default date. Assign an owner, test in a lower environment while the bundle can still be toggled, and record the decision before the date passes.
- Environment promotion is only a control if the environments are comparable on the dimensions the change touches. Test that claim rather than asserting it; edition, parameters, and attached policies are the usual divergences.

## Evidence queries

Bound the deployment identity — compare what it holds to what it has actually exercised.

```sql
-- What it holds.
SHOW GRANTS TO ROLE deployment_role;

-- What it actually did, which is usually a much shorter list.
SELECT query_type,
       COUNT(*)            AS executions,
       MAX(start_time)     AS last_run
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_HISTORY
 WHERE user_name = 'DEPLOYMENT_SERVICE_USER'
   AND start_time >= DATEADD(day, -90, CURRENT_TIMESTAMP())
 GROUP BY query_type
 ORDER BY executions DESC;
-- The gap between the two is removable blast radius. It is also the cheapest
-- security improvement available in most Snowflake estates.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/release-notes/behavior-changes — That behaviour-change bundles exist, can be enabled and disabled for testing during a window, and become default on a schedule
- https://docs.snowflake.com/en/developer-guide/snowflake-cli/index — The Snowflake CLI's automation surface and its authentication options for non-interactive use
