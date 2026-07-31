---
name: salesforce-agentforce-stdm-observer-skill
description: "Queries Salesforce Telemetry & Data Management (STDM) and Data Cloud for live Agentforce session traces, faithfulness scores, answer relevance scores, action invocation telemetry, and quality metrics under T1 least-privilege scope (api + refresh_token + cdp_query_api). Answers the production observability question: \"is my Agentforce agent working correctly right now?\" Operational counterpart to the static-review salesforce-agentforce-risk-review-skill. TRIGGER when: user asks for Agentforce session metrics, faithfulness scores, answer relevance, AI Evaluation results, action telemetry, STDM queries, agent performance KPIs, hallucination rates. Trigger phrases: \"how is my agent performing\", \"show STDM data\", \"agent observability\", \"agent telemetry\", \"AiAgentTagAssociation\", \"AiEvaluationDefinition results\", \"agentforce production metrics\". DO NOT TRIGGER when: user wants static configuration review (use salesforce-agentforce-risk-review-skill); when modifying agent configurations (T3 — escalate to salesforce-live-guard-agent); when Agent Script authoring is needed (out of Wave 4 scope, route to forcedotcom/sf-skills developing-agentforce reference)."
license: MIT
allowed-tools: Bash(sf data query:*) Bash(sf agent test:*) Bash(sf org display:*) Read Grep Glob Bash(sf api request:*) Bash(sf apex run:*)
metadata:
  author: "github: VincentChuWaiChow"
  version: 0.1.0
  updated: 2026-05-21
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token", "cdp_query_api"]
  run_as_permissions:
    required: ["View Setup and Configuration", "Data Cloud Query API access"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps", "Customize Application", "Manage Agentforce"]
---

# salesforce-agentforce-stdm-observer-skill

Production observability for Agentforce agents via STDM and Data Cloud. This skill
is a **live evidence reader**, not a configuration reviewer. It queries session
telemetry, quality scores, and action traces to answer: "Is my agent working
correctly right now?" It does not modify agents, configurations, or any org data.

**Adaptation note:** Query mechanics in this skill are adapted from the
`observing-agentforce` skill published by Salesforce in the
`forcedotcom/sf-skills` repository (Apache-2.0). Vanguard-specific additions
include the T1 least-privilege contract, structured audit envelope, explicit
aggregate-only output policy, and the handoff routing model.

**Verify-before-merge notice:** All Agentforce, STDM, Data Cloud, and Einstein
AI feature names evolve rapidly. Validate all product references, DMO field
names, and API structures against current official Salesforce documentation
before use in production.

---

## When This Skill Owns the Task

Use `salesforce-agentforce-stdm-observer-skill` when the goal is **live
production observability** for an Agentforce agent:

- "How is my Order Service agent performing over the last 7 days?"
- "Show me faithfulness and answer relevance scores for the Support agent"
- "Are there action invocation errors for the Case Routing agent?"
- "What is the session abandonment rate for the HR Self-Service agent?"
- "Show me STDM session telemetry for the last 48 hours"
- "Is the agent hallucinating? What do the quality scores say?"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Static review of agent configuration (topics, actions, instructions) | `salesforce-agentforce-risk-review-skill` |
| Agent is misconfigured and must be changed | T3 — requires human approval via `salesforce-live-guard-agent` |
| Compliance/privacy review of session data handling | `salesforce-compliance-privacy-agent` |
| General SOQL record queries unrelated to Agentforce | `salesforce-soql-explorer-skill` |
| Metadata export and schema inspection | `salesforce-metadata-fetcher-skill` |
| Authoring or editing `.agent` files | `developing-agentforce` (forcedotcom/sf-skills) |
| Agent performance degrades and a fix must be deployed | Route through `salesforce-live-guard-agent` for human approval |

---

## Required Context to Gather First

Before executing any STDM query, confirm all of the following. Ask if missing.

1. **Target org alias** — the `--target-org` value recognized by `sf org list`.
   Never accept a raw instance URL or session token.
2. **Org type** — production or sandbox. Flag production orgs; apply stricter
   output scrutiny and confirm the Connected App allowlist authorizes this alias.
3. **Agent name(s) to observe** — the agent's display label (`MasterLabel`) or
   API name. Resolve against the org before querying (see Step 2).
4. **Time window** — ISO 8601 start and end timestamps (UTC). Default to last
   7 days if not specified.
5. **Specific metrics requested** — sessions summary, faithfulness, answer
   relevance, action telemetry, or all. Prefer `getAggregatedMetrics` as the
   first call to bound scope.
6. **Sensitivity classification** — does the org serve Health Cloud,
   Financial Services Cloud, or other regulated verticals? If yes, extra
   redaction scrutiny applies; route any anomalies involving session content
   to `salesforce-compliance-privacy-agent` before sharing externally.
7. **Data Cloud provisioned?** — STDM requires Data Cloud and the Agentforce
   Activity data stream enabled. Confirm before executing (see Step 1).

---

## Recommended Workflow

### Step 1 — Verify org connectivity and Data Cloud availability

Confirm the org alias is reachable and Data Cloud is provisioned:

```bash
sf org display --target-org <alias>
```

Then probe the Data Cloud data spaces endpoint to confirm STDM DMOs are
available:

```bash
sf api request rest "/services/data/v63.0/ssot/data-spaces" \
  --target-org <alias>
```

Note: `sf api request rest` is a beta command — do not add `--json` (that
flag is unsupported and causes an error in this command).

**Decision logic:**
- If the command fails (404 or permission error), assume `DATA_SPACE=default`
  and log it as an assumption.
- Filter to only `status: "Active"` data spaces.
- If exactly one active data space exists, use it automatically.
- If multiple exist, show the list and ask the user which to use.
- Store the selected `name` as `DATA_SPACE` for all subsequent steps.

If Data Cloud is unavailable, stop and inform the user:
> STDM requires Data Cloud with "Agentforce Activity" data stream active.
> Navigate to Setup → Data Cloud → Data Streams to verify. This skill
> cannot proceed without STDM. For local trace analysis without Data Cloud,
> see the `observing-agentforce` skill from forcedotcom/sf-skills.

### Step 2 — Resolve target agent and confirm AiAgentTag

Resolve the user-provided agent name to the exact `MasterLabel` used by STDM.
Field names and exact object names are drift-prone — run this query and use
the returned values, not the user-provided string:

```bash
sf data query \
  --query "SELECT Id, MasterLabel, DeveloperName FROM GenAiPlannerDefinition WHERE MasterLabel LIKE '%<user-provided-name>%' OR DeveloperName LIKE '%<user-provided-name>%'" \
  --target-org <alias> \
  --result-format json
```

Store:
- `AGENT_MASTER_LABEL` — for STDM `findSessions` agent filter
- `PLANNER_ID` — the Salesforce record ID for this agent (redact in output)

**If the query returns no results:** The agent does not exist in this org.
Show the full list of agents and ask the user to identify the target.

### Step 3 — Query AiAgentTagAssociation for tagging context

Retrieve tagging metadata to understand what quality evaluation definitions
are configured. This confirms the org has quality scoring enabled before
querying scores:

```bash
sf data query \
  --query "SELECT Id, AiAgentTagId, EntityId, EntityType FROM AiAgentTagAssociation LIMIT 10" \
  --target-org <alias> \
  --result-format json
```

If AiAgentTagAssociation returns no rows, quality scores may not be configured.
Note this in the output and proceed with session-level metrics only.

**Note:** `AiAgentTag`, `AiAgentTagDefinition`, and `AiAgentTagAssociation`
are Tooling API objects. Use `--use-tooling-api` if the standard SOQL path
returns an "object not found" error:

```bash
sf data query \
  --query "SELECT Id, AiAgentTagId, EntityId, EntityType FROM AiAgentTagAssociation LIMIT 10" \
  --target-org <alias> \
  --use-tooling-api \
  --result-format json
```


### Step 4 — Query STDM sessions via aggregated metrics

Start with `getAggregatedMetrics` via the `AgentforceOptimizeService` Apex
helper class to get the health dashboard before drilling into individual
sessions. This is the most efficient first call and avoids fetching session
content.

Full Apex service deployment steps and invocation patterns are documented
in `references/stdm-queries.md` (adapted from forcedotcom/sf-skills
`observing-agentforce`).

```apex
String result = AgentforceOptimizeService.getAggregatedMetrics(
    '<DATA_SPACE>',
    '<START_ISO>',
    '<END_ISO>',
    50,
    '<AGENT_MASTER_LABEL>'
);
System.debug('STDM_RESULT:' + result);
```

```bash
sf apex run --json --file /tmp/stdm_metrics.apex --target-org <alias>
```

Parse the result using the `DEBUG|STDM_RESULT:` pattern (see
`references/stdm-queries.md`). The aggregated metrics return:
- `total_sessions`, `total_turns`, `avg_quality_score`
- `avg_faithfulness`, `avg_answer_relevance`, `avg_context_precision`
- `abandonment_rate`, `deflection_rate`, `escalation_rate`
- `end_type_counts`, `quality_distribution`, `top_intents`
- `unavailable_dmos` — list of DMOs that could not be queried

**If `findSessions` returns empty:** No production sessions exist in this
date window. Check that the date range is correct and that the agent is
actively receiving traffic. Consider widening the window.

### Step 5 — Drill into faithfulness and answer relevance (if warranted)

If `avg_faithfulness` or `avg_answer_relevance` falls below the thresholds
defined in `references/observability-rubric.md`, run targeted observability
queries:

```apex
AgentforceOptimizeService.ObservabilityInput inp = new AgentforceOptimizeService.ObservabilityInput;
inp.queryType = 'Hallucination';
inp.agentApiName = '<AGENT_MASTER_LABEL>';
inp.lookbackDays = 7;

List<AgentforceOptimizeService.ObservabilityOutput> results =
    AgentforceOptimizeService.runObservabilityQuery(
        new List<AgentforceOptimizeService.ObservabilityInput>{ inp }
    );
System.debug('STDM_RESULT:' + results[0].resultJson);
```

Available query types: `KnowledgeGap`, `Hallucination`, `RetrievalQuality`,
`AnswerRelevancy`, `Leaderboard` — see `references/stdm-queries.md` for the
full table.

**Do NOT** use `getMultipleConversationDetails` or `getLlmStepDetails`
in this skill. Those methods return raw session content (user messages, agent
responses) which may contain PII. This skill operates on aggregate metrics
only. See Redaction Rules below and `references/privacy-redaction.md`.

### Step 6 — Sanitize output

Before emitting any result, apply all redaction rules from the Redaction
Rules section and `references/privacy-redaction.md`. Specifically:

- **Never include session text content** (user messages, agent responses).
  Aggregate metrics and structured scores only.
- Replace all Salesforce record IDs with `<record_id_placeholder>`.
- Replace all user IDs (session participants) with `<user_id_placeholder>`.
- Mask raw record IDs in action invocation steps — hash them to detect
  duplicates but never echo raw.
- If the org is a regulated vertical (Health Cloud, Financial Services Cloud
), apply the compliance-vertical
  flag and route any anomalies through `salesforce-compliance-privacy-agent`.

### Step 7 — Emit audit envelope

Every execution must produce a complete audit envelope (see Audit Envelope
Schema). Emit it unconditionally — even if the result set is empty or an
error occurred.

### Step 8 — Hand off if anomalies warrant

Compare results against the rubric in `references/observability-rubric.md`:
- Faithfulness below threshold → hand off to `salesforce-agentforce-risk-review-skill`
- Action error rate above threshold → hand off to `salesforce-agentforce-risk-review-skill`
- A configuration change is required → route through `salesforce-live-guard-agent`
- Regulated-vertical session anomalies → `salesforce-compliance-privacy-agent`

See Handoff Rules for the full escalation matrix.

---

## Quality Scoring Rubric (100-point)

Score the observability execution quality before emitting results. Threshold:
80+ acceptable, 60–79 emit with caveat, below 60 reject and request revision.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Query selectivity** | 25 | Time-window applied; agent filter set; no full-DMO scans; aggregate-first approach used |
| **Sanitization** | 30 | No session content in output; all IDs redacted; regulated-vertical flag applied if applicable; audit envelope populated |
| **Metric completeness** | 20 | Sessions count, avg_faithfulness, avg_answer_relevance, action_invocation_count, error_rate all reported (or explicitly noted as unavailable) |
| **Audit envelope** | 15 | All required audit fields present; timestamp accurate; org_type_verified correct |
| **Proper delegation** | 10 | Anomalies routed to the correct downstream skill; no configuration changes attempted |

**Scoring penalties:**
- Session text content emitted in output: immediate reject (score voided, refusal issued)
- Missing audit envelope: -20 (automatic caveat regardless of total score)
- No time-window filter on STDM query: -15
- Encrypted or PII fields echoed: immediate reject
- Configuration mutation attempted: immediate stop, route to Live Guard

---

## T1 Least-Privilege Contract

This skill operates exclusively at T1 — read-only runtime. The contract is:

- **OAuth scopes used:** `api`, `refresh_token`, and `cdp_query_api` only.
  The `cdp_query_api` scope is required for Data Cloud SQL queries via the
  `ConnectApi.CdpQuery` namespace. No `full`, `web`, `sfap_api`, or any
  other scope.
- **Run As account profile:** System permissions: View Setup and Configuration
  and Data Cloud Query API access. Object permissions: Read only on objects
  in scope. FLS restricted to non-PII, non-encrypted fields by default.
- **Denied permissions (enforced at Connected App and profile level):**
  - Modify All Data
  - View All Data (system-level bypass)
  - View Encrypted Data
  - Modify Metadata Through Metadata API Functions
  - Author Apex
  - Customize Application
  - Manage Connected Apps
  - Manage Agentforce (
— permission
    API name subject to change)
- **Org allowlist:** Enforced by Connected App IP restrictions and explicit
  org alias allowlist. Skill verifies via `sf org display` that the target
  alias is in the authorized set before any query.
- **No DML under any circumstances:** This skill will not construct or execute
  any statement containing INSERT, UPDATE, DELETE, MERGE, or UPSERT.
- **No agent mutation:** This skill does not call `sf agent publish`,
  `sf agent activate`, `sf project deploy start`, or any command that
  modifies agent configuration or org state.
- **Aggregate metrics only:** This skill does not retrieve or emit raw
  session content, individual message texts, or LLM prompt/response pairs.
  Those are available in the `observing-agentforce` sf-skills pattern for
  use by human operators with appropriate data handling controls.
- **Revocation:** The least-privilege Run As account's refresh token can be
  rotated to instantly revoke all access without affecting other integrations.

---

## Refusal Triggers

Stop immediately and do not execute if any of the following apply:

- The target org appears to be a production org but the Connected App
  allowlist does not include it.
- The user requests session text content (user messages, agent responses,
  LLM prompt/response pairs). This skill emits aggregate metrics only; raw
  content requires explicit human-in-the-loop confirmation via
  `salesforce-live-guard-agent`.
- The user requests cross-tenant queries (sessions from other orgs or
  Data Cloud tenants not owned by the connected org).
- The Run As account has `Manage Agentforce` permission present — this skill
  requires that permission to be explicitly denied.
- The Run As account is missing `View Setup and Configuration` — stop and
  escalate to org administrator.
- The org is a regulated-vertical production org (Health Cloud, Financial
  Services Cloud
) and jurisdiction
  or data classification is unknown.
- The audit envelope cannot be completed (missing matter_id, unresolvable
  org alias, or run_as_user_id unavailable).
- The user requests that redaction be skipped or disabled.
- Any mutation command is requested: `sf project deploy start`, `sf agent
  publish`, Apex DML, or equivalent.

---

## Audit Envelope Schema

Every execution emits an audit envelope. The envelope travels with the
sanitized output to any downstream skill.

```yaml
audit_envelope:
  matter_id: "<caller-provided-or-generated-uuid>"
  skill_id: "salesforce-agentforce-stdm-observer-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"              # never the raw org ID
  run_as_user_id: "<user_id_placeholder>"  # placeholder; never real ID in output
  agent_master_label: "<label>"            # display name used for STDM filter
  data_space: "<data_space_name>"          # resolved Data Cloud data space
  query_types_executed: ["getAggregatedMetrics", "runObservabilityQuery"]
  time_window_start: "<ISO-8601-UTC>"
  time_window_end: "<ISO-8601-UTC>"
  redactions_applied:
    - type: "<session_content|user_id|record_id|pii>"
      reason: "<aggregate-only-policy|pii-risk|encrypted>"
  timestamp: "<ISO-8601-UTC>"
  org_type_verified: "sandbox | production"
  regulated_vertical_flag: true | false
  downstream_skill_recommended: "<skill-id or null>"
```

---

## Output Format

All output is in YAML. Emit this structure for every execution.

```yaml
verdict: "acceptable | caveat | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

aggregate_metrics:
  sessions_count: <integer>
  total_turns: <integer>
  avg_quality_score: <float>           # 1.0-5.0 scale
  avg_faithfulness: <float>            # 0.0-1.0; null if unavailable
  avg_answer_relevance: <float>        # 0.0-1.0; null if unavailable
  avg_context_precision: <float>       # 0.0-1.0; null if unavailable
  action_invocation_count: <integer>   # total across all sessions
  action_error_count: <integer>
  error_rate: <float>                  # action_error_count / action_invocation_count
  abandonment_rate: <float>
  deflection_rate: <float>
  escalation_rate: <float>
  end_type_counts:
    USER_ENDED: <integer>
    AGENT_ENDED: <integer>
    UNKNOWN: <integer>
  quality_distribution:
    "5": <integer>
    "4": <integer>
    "3": <integer>
    "2": <integer>
    "1": <integer>
  top_intents:
    "<intent summary>": <count>
  unavailable_dmos: []

anomalies_detected:
  - dimension: "<faithfulness|relevance|error_rate|abandonment>"
    observed_value: <float>
    threshold: <float>
    severity: "low | medium | high | critical"
    interpretation: "<human-readable explanation>"

sanitized_sample_sessions: null
# Always null in this skill. Session content is never emitted.
# If per-session debugging is genuinely required, route through
# salesforce-live-guard-agent for human-in-the-loop confirmation.

audit_envelope:
  # See Audit Envelope Schema above

downstream_skill_recommendation: "<skill-id or null>"
downstream_routing_reason: "<why this skill was chosen>"

missing_evidence:
  - "<what additional data would improve confidence>"

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

Apply in order. Do not bypass for any reason.

1. **Session text content (user messages, agent responses, LLM prompts):**
   Never include in any output. This is not a limitation — it is the
   intentional aggregate-only policy of this skill. Route content access
   through `salesforce-live-guard-agent`.
2. **OAuth tokens, refresh tokens, session IDs:** Never include in any output,
   log, or audit envelope field.
3. **Salesforce Org IDs (18-char starting with `00D`):** Replace with
   `<org_id_placeholder>`.
4. **Salesforce Record IDs (15/18-char) in action invocations:** Hash to a
   deterministic short token (e.g., `rec_a3f2`) to detect duplicates without
   echoing the raw ID. Never emit raw record IDs.
5. **User IDs (session participants, OwnerId, UserId):** Replace with
   `<user_id_placeholder>`.
6. **Customer names, email addresses, phone numbers in any field:**
   Replace with `<pii_redacted>`.
7. **Instance URLs and API endpoints:** Omit from output; reference only the
   org alias in the audit envelope.
8. **Agent version suffix on DeveloperName** (e.g., `_v9`): Omit from
   output — it reveals internal versioning structure.
9. **Encrypted fields (Shield PE / PMLE):** Skip entirely. Do not emit the
   field name or any placeholder that implies a value was retrieved.

---

## Handoff Rules

When metrics cross the thresholds in `references/observability-rubric.md`,
hand off to the appropriate skill with the sanitized output and audit
envelope as the payload.

| Finding | Hand off to | Payload required |
|---|---|---|
| Faithfulness drops below threshold | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics, anomalies_detected |
| Answer relevance below threshold | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics, anomalies_detected |
| Action error rate > 5% | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics, error breakdown |
| A configuration change is proposed | `salesforce-live-guard-agent` | audit_envelope, change_proposal, anomalies_detected |
| Regulated-vertical session anomalies | `salesforce-compliance-privacy-agent` | audit_envelope, anomalies_detected, vertical_flag |
| General SOQL follow-up needed | `salesforce-soql-explorer-skill` | audit_envelope, specific query request |

Required handoff fields: `matter_id`, `audit_envelope`, `aggregate_metrics`
(summary — not raw session data), `anomalies_detected`, `assumptions`.

---

## Stop Conditions

Stop and do not continue if:

- Target org appears to be production but the Connected App allowlist excludes
  it — stop, emit a refusal with reason, do not execute any query.
- Data Cloud is unavailable in the target org — stop, inform the user,
  reference Setup → Data Cloud → Data Streams.
- The Run As account has `Manage Agentforce` permission granted — this skill
  requires it to be denied; stop and escalate to the org administrator.
- Session text content is requested by the user — stop, explain the
  aggregate-only policy, offer to route through `salesforce-live-guard-agent`.
- Cross-tenant queries are requested — stop and refuse.
- The audit envelope cannot be completed — stop until resolved.
- The user requests that redaction be disabled — stop and explain the policy.
- A mutation command is requested (deploy, publish, DML, configuration change)
  — stop immediately and route to `salesforce-live-guard-agent` for human
  approval.

---

## Security Notes

- **T1 read-only operational:** No DML, no metadata mutation, no Apex
  authoring, no agent publishing or activation.
- **Aggregate-only output:** Session text content is never emitted under any
  circumstance. This is a structural policy, not a configurable option.
- **Additional OAuth scope (`cdp_query_api`):** Required for Data Cloud SQL
  queries. This scope does not grant write access to Data Cloud; it permits
  read-only queries against the `cdp_query_api` endpoint only.
- **Sanitized output only:** All Salesforce IDs, user IDs, and PII fields
  redacted before emission. Record IDs in action invocations are hashed.
- **Org allowlist enforced:** Connected App restricts which orgs can be
  targeted; skill verifies before executing.
- **Structured audit emitted:** Every execution produces a complete audit
  envelope regardless of result count.
- **Revocable:** Rotating the Run As account's refresh token immediately
  revokes all access without affecting other integrations.
- **No credential echo:** OAuth tokens, refresh tokens, and session IDs are
  never included in output or audit envelopes.
- **Regulated-vertical escalation:** Health Cloud and Financial Services Cloud
orgs trigger mandatory escalation
  to `salesforce-compliance-privacy-agent` before results are shared
  externally.
- **Manage Agentforce permission must be denied:** If the Run As account has
  this permission, this skill refuses to operate. Read-only telemetry access
  does not require management permissions.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/stdm-queries.md` | STDM query patterns, SOQL/SQL examples, Apex service methods, Data Cloud `cdp_query_api` scope, anti-patterns |
| `references/observability-rubric.md` | Thresholds for faithfulness, relevance, error rate, abandonment; escalation matrix |
| `references/privacy-redaction.md` | Agentforce-specific redaction rules, session content policy, human-in-the-loop path |
