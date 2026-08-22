# Agent Access and Effective Reach

The Snowflake privileges that decide what an AI system can touch, and how to compute its real reach. Load first in every AI review.

## The privileges that matter

- `SNOWFLAKE.CORTEX_USER` is a database role that grants the ability to use Cortex capability broadly. `SNOWFLAKE.CORTEX_AGENT_USER` is the narrower role documented for Cortex Agent access. Where both would work, the narrower one is the correct grant.
- Snowflake's own agent deployment guidance includes revoking agent access from `PUBLIC` and granting it to a specific role. That instruction exists because a broad grant is a real state that occurs in practice — check for it explicitly rather than assuming it away.
- Cortex AI functions are documented as requiring the `USE AI FUNCTIONS` account-level privilege together with either the `CORTEX_USER` or `AI_FUNCTIONS_USER` database role. Enumerate who holds each of these; they decide who can run inference over your data.
- Agent-object privileges are separate again: USAGE (invoke it), MODIFY (change its instructions and tools — effectively rewriting what it does), MONITOR (observe it), and OWNERSHIP (all of the above plus granting). MODIFY on a production agent is the privilege most often granted casually and least often reviewed.
- The agent's *own* role is the one that matters most: what the agent's actions execute as. Compute its transitive closure the same way any other principal's is computed, and remember that ownership edges and future grants apply to it too.

## Reach versus requirement

- The finding is the gap. State what the use case requires the agent to read (usually a handful of schemas), state what its identity can actually read, and report the difference as the primary risk.
- Verify the computed reach against reality: what did the agent identity actually access, from access history? A reach that is theoretically broad and practically narrow is still broad — an injection uses the theoretical reach.
- An agent identity should be purpose-built, not reused. Sharing an identity with an ETL service or a BI tool imports every one of that service's privileges into the AI surface.
- Where an agent identity is being created, `TYPE = SERVICE_AGENT` exists as a documented user type for automated AI agents, and `TYPE = SERVICE` for automated applications generally. Neither should authenticate with a password; prefer key-pair or workload identity federation, and route the identity design to the identity agent.
- Attribution matters as much as reach: if every agent action appears under one service identity with no link to the requesting human, an incident cannot be investigated and a per-user authorization model cannot be enforced.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Cortex Agent access is governed through Snowflake RBAC, with `SNOWFLAKE.CORTEX_AGENT_USER` documented as the database role granted for agent access, distinct from the broader `SNOWFLAKE.CORTEX_USER`. | Current documented behaviour | 2026-08-17 via Context7 `/websites/snowflake_en` (cortex-agents-setup) | That a narrower agent-specific grant exists and should be preferred over the broad Cortex role | What this account has actually granted, or to whom — that requires grant evidence |
| Snowflake agent deployment guidance includes revoking agent access from the PUBLIC role and granting it to a specific role instead. | Current documented guidance | 2026-08-17 via Context7 `/websites/snowflake_en` (deploy-agents) | That a PUBLIC grant of agent access is a real state worth checking for explicitly | That any given account currently has or lacks that grant |
| Using Cortex AI functions requires the `USE AI FUNCTIONS` account-level privilege and either the `CORTEX_USER` or `AI_FUNCTIONS_USER` database role. | Current documented requirement — re-verify, AI privilege surfaces move quickly | 2026-08-17 via Context7 `/websites/snowflake_en` (aisql) | That AI function invocation is gated by both an account privilege and a database role, giving two places to check and two places to over-grant | The full current set of AI-related privileges, which changes as capabilities are added |
| Agent monitoring uses the MONITOR privilege on the agent together with a Cortex database role. | Current documented behaviour | 2026-08-17 via Context7 `/websites/snowflake_en` (cortex-agents-monitor) | That observability of an agent is itself a granted privilege that must be assigned to someone | That anyone in this account actually holds it or watches the output |

## Evidence queries

Inventory who holds AI capability — the query that most often produces the headline finding.

```sql
SELECT grantee_name AS role_name,
       name         AS granted_role,
       created_on,
       deleted_on
  FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
 WHERE granted_on = 'DATABASE_ROLE'
   AND name IN ('CORTEX_USER', 'CORTEX_AGENT_USER', 'AI_FUNCTIONS_USER')
   AND deleted_on IS NULL
 ORDER BY name, role_name;
-- A row where role_name = 'PUBLIC' means every identity in the account,
-- present and future, holds that AI capability. Report it first.
```

Establish who can invoke, change, or observe a specific agent.

```sql
SHOW AGENTS;
SHOW GRANTS ON AGENT my_db.my_schema.my_agent;

-- MODIFY on a production agent is the privilege to rewrite what it does,
-- including which tools it may call. Review it as carefully as OWNERSHIP.
```

Verify computed reach against what the agent identity actually read.

```sql
SELECT f.value:objectName::string AS object_accessed,
       COUNT(*)                   AS accesses,
       MIN(query_start_time)      AS first_access,
       MAX(query_start_time)      AS last_access
  FROM SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY ah,
       LATERAL FLATTEN(input => ah.base_objects_accessed) f
 WHERE ah.user_name = 'MY_AGENT_SERVICE_USER'
   AND ah.query_start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY 1
 ORDER BY accesses DESC;
-- Narrow observed access does not narrow the risk: an injection uses the
-- theoretical reach, not the historical one.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-agents-setup — That CORTEX_AGENT_USER is the documented database role for agent access, granted to a custom role
- https://docs.snowflake.com/en/user-guide/snowflake-cortex/aisql — That AI functions require the USE AI FUNCTIONS account privilege plus CORTEX_USER or AI_FUNCTIONS_USER
- https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-agents-monitor — That agent monitoring is gated by the MONITOR privilege on the agent together with a Cortex database role
- https://docs.snowflake.com/en/sql-reference/sql/alter-user — That SERVICE_AGENT exists as a user type intended for automated AI agents
