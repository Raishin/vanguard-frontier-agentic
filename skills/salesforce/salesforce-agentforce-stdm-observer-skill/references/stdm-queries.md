# STDM Query Reference

Query patterns for Salesforce Telemetry and Data Management (STDM) and Data Cloud
used by `salesforce-agentforce-stdm-observer-skill`.

**Attribution:** Query mechanics in this file are adapted from the
`observing-agentforce` skill published by Salesforce in the
`forcedotcom/sf-skills` repository (Apache-2.0 license). Vanguard adaptations
include the aggregate-only restriction, T1 least-privilege annotations, and the
privacy-safe redaction notes.

**Verify-before-merge:** All Agentforce/STDM DMO names, field names, and API
structures are subject to rapid change across Salesforce releases. Validate
against current official documentation before use in production.

---

## Prerequisites

### Required Setup

Before any STDM query can succeed:

1. **Data Cloud is provisioned** in the target org.
2. **Agentforce Activity data stream is active:** Setup → Data Cloud → Data Streams
   → confirm "Agentforce Activity" stream shows `Active`.
3. **Run As account has `Data Cloud Query API access`** system permission.
4. **OAuth scope `cdp_query_api`** is granted on the Connected App for the
   Run As service account.

### Deploy AgentforceOptimizeService (once per org)

The `AgentforceOptimizeService` Apex helper class wraps STDM DMO queries and
returns clean JSON. It must be deployed once to the org before use.

**This class is published in the `forcedotcom/sf-skills` repository under
`skills/observing-agentforce/apex/`. Deploy from that source — do not
reproduce the full class here.**

Check if already deployed:

```bash
sf data query \
  --query "SELECT Id, Name FROM ApexClass WHERE Name = 'AgentforceOptimizeService'" \
  --target-org <alias> \
  --result-format json
```

If not deployed, copy from the sf-skills install and deploy:

```bash
mkdir -p <project-root>/force-app/main/default/classes
cp ~/.claude/skills/observing-agentforce/apex/AgentforceOptimizeService.cls \
   <project-root>/force-app/main/default/classes/
cp ~/.claude/skills/observing-agentforce/apex/AgentforceOptimizeService.cls-meta.xml \
   <project-root>/force-app/main/default/classes/

sf project deploy start \
  --metadata ApexClass:AgentforceOptimizeService \
  --target-org <alias> \
  --result-format json
```

If the deploy fails with a compile error, verify that Data Cloud is enabled
in the org — the `ConnectApi.CdpQuery` namespace requires it.

**Note on Apex deployment in T1 context:** Deploying the helper class is a
one-time, bounded operation. The class is read-only in function (all methods
are queries). Confirm deployment with the org administrator if the Run As
account does not have `AuthorApex` permission; in that case, a separate
privileged account must deploy the class first.

---

## Resolve Data Space

Before any STDM query, resolve the correct Data Cloud Data Space API name:

```bash
sf api request rest "/services/data/v63.0/ssot/data-spaces" \
  --target-org <alias>
```

Note: `sf api request rest` is a beta command — do not add `--json`.

Response shape:

```json
{
  "dataSpaces": [
    {
      "id": "0vhKh000000g3DjIAI",
      "label": "default",
      "name": "default",
      "status": "Active"
    }
  ],
  "totalSize": 1
}
```

- Filter to `status: "Active"` entries only.
- If exactly one active data space: use it, confirm to the user.
- If multiple active data spaces: show labels and ask the user to choose.
- If the endpoint fails (404 or permission error): assume `DATA_SPACE=default`
  and log as an assumption.

Store the resolved `name` as `DATA_SPACE` for all subsequent calls.

---

## Resolve Agent Name

Before querying STDM, resolve the user-provided agent name to the exact
`MasterLabel` that STDM uses as its filter key:

```bash
sf data query \
  --query "SELECT Id, MasterLabel, DeveloperName FROM GenAiPlannerDefinition WHERE MasterLabel LIKE '%<user-provided-name>%' OR DeveloperName LIKE '%<user-provided-name>%'" \
  --target-org <alias> \
  --result-format json
```

Store:
- `AGENT_MASTER_LABEL` — for `findSessions` and `getAggregatedMetrics` agent filter
- `PLANNER_ID` — the Salesforce record ID (redact in all output)

If no results: show the full agent list and ask the user to identify the target:

```bash
sf data query \
  --query "SELECT Id, MasterLabel, DeveloperName FROM GenAiPlannerDefinition" \
  --target-org <alias> \
  --result-format json
```

---

## SOQL Patterns for Tooling API Objects

These queries use Salesforce Tooling API objects related to Agentforce
evaluation configuration. Use `--use-tooling-api` when the standard SOQL
path returns "object not found."

### Locate agent tag definitions

```bash
sf data query \
  --query "SELECT Id, Name, MasterLabel FROM AiAgentTagDefinition LIMIT 50" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json
```

### Retrieve AiAgentTagAssociation for an agent

```bash
sf data query \
  --query "SELECT AiAgentTagId, EntityId, EntityType FROM AiAgentTagAssociation WHERE AiAgentTagId = '<tag_id>'" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json
```

Redact `AiAgentTagId` and `EntityId` before emitting output. These are
Salesforce record IDs — replace with `<record_id_placeholder>`.

### Query AiEvaluationDefinition runs

```bash
sf data query \
  --query "SELECT Id, Name, Status, CreatedDate FROM AiEvaluationDefinition WHERE CreatedDate = LAST_N_DAYS:7 ORDER BY CreatedDate DESC LIMIT 20" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json
```


---

## Aggregated Metrics (Recommended First Step)

Get the health dashboard before drilling into individual sessions. This call
is the most efficient first query and avoids fetching session content.

Write `/tmp/stdm_metrics.apex` and run:

```apex
String result = AgentforceOptimizeService.getAggregatedMetrics(
    '<DATA_SPACE>',
    '<START_ISO>',    // e.g. '2026-05-14T00:00:00Z'
    '<END_ISO>',      // e.g. '2026-05-21T23:59:59Z'
    50,               // maxRows — number of sessions to include in aggregation
    '<AGENT_MASTER_LABEL>'
);
System.debug('STDM_RESULT:' + result);
```

```bash
sf apex run --json --file /tmp/stdm_metrics.apex --target-org <alias>
```

Parse the result — search for `DEBUG|STDM_RESULT:` in the debug log output:

```bash
python3 -c "
import json, sys
logs = json.load(sys.stdin)['result']['logs']
idx = logs.find('DEBUG|STDM_RESULT:')
print(logs[idx + len('DEBUG|STDM_RESULT:'):].split('\n')[0].strip)
" < /tmp/apex_result.json
```

Expected response shape:

```json
{
  "total_sessions": 36,
  "total_moments": 32,
  "total_turns": 101,
  "avg_quality_score": 4.34,
  "avg_session_duration_sec": 45.2,
  "end_type_counts": { "USER_ENDED": 5, "AGENT_ENDED": 10, "UNKNOWN": 21 },
  "quality_distribution": { "5": 20, "4": 6, "3": 4, "2": 1, "1": 1 },
  "abandonment_rate": 0.14,
  "deflection_rate": 0.28,
  "escalation_rate": 0.0,
  "top_intents": { "Check order status": 3, "Get account details": 2 },
  "avg_faithfulness": 0.85,
  "avg_answer_relevance": 0.72,
  "avg_context_precision": 0.91,
  "unavailable_dmos": []
}
```

**Key interpretation signals** (see `observability-rubric.md` for thresholds):
- `avg_quality_score` < 4.0 → investigate low-scoring moments
- `quality_distribution` skewed toward 1-3 → systemic quality issue
- `abandonment_rate` > 0.3 → users giving up; check for dead-ends
- Low `avg_faithfulness` → RAG retrieval issues
- Low `avg_answer_relevance` → topic routing or grounding issues
- `unavailable_dmos` non-empty → log as missing evidence; some scores will be null

---

## Find Sessions (for Session-Level Context Only)

Use only to obtain session IDs for further aggregation — do **not** use the
returned session metadata to fetch session content. Session content is out of
scope for this skill.

Write `/tmp/stdm_find.apex` and run:

```apex
String result = AgentforceOptimizeService.findSessions(
    '<DATA_SPACE>',
    '<START_ISO>',
    '<END_ISO>',
    20,
    '<AGENT_MASTER_LABEL>'
);
System.debug('STDM_RESULT:' + result);
```

The result is a JSON array of `SessionSummary` objects:

```json
[
  {
    "session_id": "...",
    "start_time": "...", "end_time": "...",
    "channel": "...", "duration_ms": 12345,
    "end_type": "USER_ENDED"
  }
]
```

**Privacy note:** Redact `session_id` values in output — replace with
`<session_id_placeholder>`. The session IDs are Salesforce record IDs
that could be used to retrieve session content if the caller has additional
permissions; do not echo them.

**If `findSessions` returns empty:** No production sessions with conversation
turns exist in this date window. The agent may not be receiving traffic, or
the date range may need widening.

---

## Observability Queries (RAG Deep-Dive)

For targeted RAG and faithfulness analysis when aggregated metrics flag
issues. These queries operate on Data Lake objects (`*__dll`) directly.

Available `queryType` values:

| queryType | What it returns |
|---|---|
| `KnowledgeGap` | Avg context precision + answer relevancy by subagent/agent (lowest first) |
| `Hallucination` | Subagents with avg faithfulness < threshold (see `observability-rubric.md`) |
| `RetrievalQuality` | Avg context precision by retriever/subagent/agent |
| `AnswerRelevancy` | Subagents with avg answer relevancy below threshold |
| `Leaderboard` | Combined precision, relevancy, and faithfulness by subagent/agent |

Write `/tmp/observability_query.apex` and run:

```apex
AgentforceOptimizeService.ObservabilityInput inp = new AgentforceOptimizeService.ObservabilityInput;
inp.queryType = 'Hallucination';      // or KnowledgeGap, RetrievalQuality, AnswerRelevancy, Leaderboard
inp.agentApiName = '<AGENT_MASTER_LABEL>';
inp.topicApiName = null;              // optional subagent filter
inp.lookbackDays = 7;                 // default 90

List<AgentforceOptimizeService.ObservabilityOutput> results =
    AgentforceOptimizeService.runObservabilityQuery(
        new List<AgentforceOptimizeService.ObservabilityInput>{ inp }
    );
System.debug('STDM_RESULT:' + results[0].summaryText);
System.debug('STDM_RESULT:' + results[0].resultJson);
```

```bash
sf apex run --json --file /tmp/observability_query.apex --target-org <alias>
```

---

## Data Cloud SQL (cdp_query_api Scope)

Direct Data Cloud SQL queries require the `cdp_query_api` OAuth scope. These
queries bypass the Apex helper class and call the Data Cloud query endpoint
directly. Use this path when `AgentforceOptimizeService` is not available or
when a custom DMO query is needed.

**Scope requirement:** The Connected App must grant `cdp_query_api` scope to
the Run As service account. This is separate from the standard `api` scope.

Example — count sessions in a time window (aggregate only, no content):

```sql
SELECT COUNT(ssot__Id__c) AS session_count,
       AVG(ssot__Duration__c) AS avg_duration_ms
FROM "ssot__AiAgentSession__dlm"
WHERE ssot__StartTimestamp__c >= '2026-05-14T00:00:00Z'
  AND ssot__StartTimestamp__c <= '2026-05-21T23:59:59Z'
```

Via Apex (no session content):

```apex
ConnectApi.CdpQueryInput qi = new ConnectApi.CdpQueryInput;
qi.sql = 'SELECT COUNT(ssot__Id__c) AS session_count FROM "ssot__AiAgentSession__dlm" WHERE ssot__StartTimestamp__c >= \'<START>\' LIMIT 1';
ConnectApi.CdpQueryOutputV2 out = ConnectApi.CdpQuery.queryAnsiSqlV2(qi, '<DATA_SPACE>');
System.debug('COUNT:' + out.data);
```

**Privacy rule:** Never construct a Data Cloud SQL query that selects
`ssot__ContentText__c` (message content), `ssot__InputValueText__c`,
`ssot__OutputValueText__c`, or any field containing session text. Aggregate
functions (`COUNT`, `AVG`, `SUM`, `MIN`, `MAX`) only.

---

## Moment Insights (Aggregate Summaries)

`getMomentInsights` returns per-moment quality scores and LLM-generated
summaries. The summaries (`request_summary`, `response_summary`) are
LLM-synthesized descriptions of intent — they are not raw session content.
Even so, they may contain paraphrased user data.

**Policy for this skill:** Treat `request_summary` and `response_summary`
as potentially sensitive and do NOT emit them in output. Emit only the
numeric quality scores and counts.

```apex
String result = AgentforceOptimizeService.getMomentInsights(
    '<DATA_SPACE>',
    new List<String>{ '<SESSION_ID_1>', '<SESSION_ID_2>' }
);
System.debug('STDM_RESULT:' + result);
```

From the returned `SessionInsights` array, extract only:
- `avg_quality_score` per session
- `moment_count` and `turn_count` (efficiency ratio)
- `action_error_count`
- `quality_score` per moment (numeric)

Do not echo `request_summary`, `response_summary`, or `quality_reasoning`.

---

## Anti-Patterns

These query patterns are explicitly prohibited in this skill:

| Anti-pattern | Why prohibited |
|---|---|
| `SELECT ssot__ContentText__c FROM ssot__AiAgentInteractionMessage__dlm` | Selects raw user message text — PII risk |
| `SELECT ssot__InputValueText__c FROM ssot__AiAgentInteractionStep__dlm` | Selects raw action input data — may contain PII |
| `SELECT ssot__OutputValueText__c FROM ssot__AiAgentInteractionStep__dlm` | Selects raw action output — may contain record data |
| `getMultipleConversationDetails` without content stripping | Returns full turn-by-turn messages — content out of scope |
| `getLlmStepDetails` | Returns LLM prompt and response text — content out of scope |
| SOQL/SQL with no time-window WHERE clause on STDM DMOs | Full-table scans on large session tables; governor limit risk |
| `SELECT * FROM ...` on any STDM DMO | Never use SELECT * — enumerate fields explicitly |
| Storing session IDs in output without redaction | Session IDs can be used to retrieve content if permissions change |

---

## Time-Windowed Query Pattern

Always apply a time filter to bound STDM queries. STDM session tables grow
continuously in active orgs; unbounded queries will hit Data Cloud query
limits.

Recommended window: 7 days for routine observability checks.
Maximum window for aggregate queries: 90 days (per `runObservabilityQuery`
default).

ISO 8601 UTC format required:

```
START_ISO = '2026-05-14T00:00:00Z'
END_ISO   = '2026-05-21T23:59:59Z'
```

Use Python to compute windows:

```bash
python3 -c "
from datetime import datetime, timezone, timedelta
now = datetime.now(timezone.utc)
start = now - timedelta(days=7)
print('START:', start.strftime('%Y-%m-%dT%H:%M:%SZ'))
print('END:  ', now.strftime('%Y-%m-%dT%H:%M:%SZ'))
"
```
