# Vanguard Frontier Agentic — Replication Contract for `netsuite` Provider

Reference precedent: `agents/salesforce/*` and `skills/salesforce/*`

---

## 1. SKILL.md Frontmatter Contract

Source: `/home/user/vanguard-frontier-agentic/schemas/skill.frontmatter.schema.json`

### Required fields (schema `"required"`)

| Field | Type | Constraints |
|-------|------|-------------|
| `name` | string | Kebab-case. Pattern: `^[a-z0-9][a-z0-9-]*$`. Must start with lowercase letter or digit. |
| `description` | string | 50–1500 characters. |
| `allowed-tools` | string OR array of strings | Space-separated string (minLength 1) OR YAML sequence (minItems 1, each item minLength 1). |
| `metadata` | object | Contains sub-fields; `author` and `version` are required inside it. |

### `metadata` sub-fields

| Field | Required? | Type | Constraints |
|-------|-----------|------|-------------|
| `author` | YES | string | e.g. `"github: Raishin"` |
| `version` | YES | string | Semver pattern `^\d+\.\d+\.\d+(-[\w.-]+)?$` |
| `updated` | optional | string | ISO 8601 `YYYY-MM-DD` |
| `category` | optional | string | Enum (see below) |
| `lifecycle` | optional | string | Enum: `experimental`, `beta`, `stable`, `deprecated` |

### `category` enum (full list)

`security`, `platform`, `data`, `finops`, `ai`, `delivery`, `observability`, `compliance`, `resilience`, `networking`, `storage`, `database`, `compute`, `architecture`, `messaging`, `serverless`, `cost-management`, `operational`, `generation`, `devsecops`, `finance`

### Optional top-level fields

| Field | Type | Purpose |
|-------|------|---------|
| `disable-model-invocation` | boolean | When true, suppresses direct model invocation; relies on tool calls only. |
| `license` | string | e.g. `MIT` (present in some skills, `additionalProperties: true` allows it) |
| `execution_tier` | string | Enum: `static-review`, `read-only-runtime`, `sandbox-mutating`, `mutating-runtime` (in `metadata.liveAgentFields` definition) |
| `mcp_servers` | array of strings | MCP server identifiers. Empty array means no MCP. |
| `oauth_scopes` | array of strings | OAuth scopes required. Empty = none needed. |
| `run_as_permissions` | object | `required: []` and `denied: []` arrays. |
| `required_egress` | array of strings | Named hostnames for runtime contact. |
| `requires_credentials` | array of strings | Environment variable names. Never echoed. |
| `output_attestation` | object | `schema` (string) and `signed_with` (`cosign-blob`, `in-toto`, or `none`). |
| `eval_fixtures` | string | Repo-relative path to golden fixture directory. |

### `allowed-tools` contract

The validator at `tests/validate-skill-allowed-tools.py` enforces:
- Every SKILL.md MUST declare `allowed-tools` in frontmatter.
- Token pattern: `^[A-Z][A-Za-z0-9]+(\([^)]+\))?$`
- Bare tool: `Read`, `Grep`, `Glob`, `Bash`, `Edit`, `Write`
- Constrained tool: `Bash(sf data query:*)`, `Bash(sf org list:*)`
- At least one token required.

For T0 static-review skills, the standard value is: `Read Grep Glob`

For T1 read-only-runtime skills with `sf` CLI, the value extends to include constrained Bash tokens, e.g.: `Bash(sf data query:*) Bash(sf org list:*) Bash(sf org display:*) Read Grep Glob`

### Verbatim real salesforce SKILL.md frontmatter example

From `/home/user/vanguard-frontier-agentic/skills/salesforce/salesforce-apex-generator-skill/SKILL.md`:

```yaml
---
name: salesforce-apex-generator-skill
description: "Generates production-grade Apex classes with Service-Selector-Domain layering, correct sharing models (with sharing / without sharing / inherited sharing per class type), async patterns (Queueable, Batchable, Schedulable), and governor-limit awareness. T0 static generation — no org connection required. TRIGGER when: user asks to write an Apex class or trigger, generate a service/selector/domain layer, create an async job, implement a REST resource, scaffold a .cls file, or port business logic to Apex. Trigger phrases: write apex class, generate apex trigger, create apex service, .cls file, implement queueable, create batch job. DO NOT TRIGGER when: live test execution needed (use salesforce-apex-test-runner-skill), generating test classes specifically (use salesforce-apex-test-generator-skill), debugging an existing class from log evidence (use salesforce-apex-log-analyzer-skill), or the user needs a live deployment (use salesforce-deployment-validator-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-21"
  category: generation
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---
```

Second example (T1 runtime skill) from `/home/user/vanguard-frontier-agentic/skills/salesforce/salesforce-soql-explorer-skill/SKILL.md`:

```yaml
---
name: salesforce-soql-explorer-skill
description: "Executes read-only SOQL queries against a connected Salesforce org via the sf data query CLI under T1 least-privilege scope (api + refresh_token only, Run As service account with no ModifyAllData/ViewAllData/ViewEncryptedData). Returns sanitized JSON with a structured audit envelope. Live operational counterpart to the static-review skills. TRIGGER when: user asks to query records, run SOQL, fetch live data, inspect records by ID, count records, run aggregate queries, or check field values in a live org. Trigger phrases: query my org, run SOQL, show me records where, how many opportunities, what is the value of field X on record Y. DO NOT TRIGGER when: user pastes a metadata XML export for static review (use salesforce-metadata-review-skill); request requires DML — write, update, delete — those are T3 prohibited; bulk data operations needed (use salesforce-bulk-data-ops-skill); only schema metadata needed without data (use salesforce-metadata-fetcher-skill)."
license: MIT
allowed-tools: Bash(sf data query:*) Bash(sf org list:*) Bash(sf org display:*) Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-21"
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["View Setup and Configuration"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps"]
---
```

---

## 2. SKILL.md Body Section Structure

Read from three full salesforce SKILL.md files: `salesforce-apex-generator-skill`, `salesforce-zero-trust-maturity-skill`, and `salesforce-org-assessment-skill`.

### Standard section pattern (T0 static-review skill)

```
# <skill-name-human>

[One-sentence "forge" or "flashlight" characterization with T0/T1 note.]

## When This Skill Owns the Task
[Bullet list of use-case triggers, with delegation table showing what to use instead.]

## Required Context to Gather First
[Numbered list of required inputs before operating.]

## Recommended Workflow
### Step 1 — ...
### Step 2 — ...
[Numbered steps leading to output.]

## [Domain-Specific Reference Table or Rules Table]
[E.g. Class Type Reference, ZTA pillars table.]

## Rules
### Hard-Stop Constraints (Must Enforce)
[Table of constraints + rationale.]

## Quality Scoring Rubric (optional, if applicable)

## T0 Contract / T1 Contract
[Explicit statement of execution tier: no org connection, no OAuth, etc.]

## Refusal Triggers
[Numbered/bulleted list of stop conditions.]

## Output Format
[Structured YAML or JSON output shape.]

## Handoff Rules
[Table of "output → hand off to" for cross-skill routing.]

## Stop Conditions
[Numbered list of final stop conditions.]

## Security Notes
[Explicit security posture declaration.]

## Reference File Index (optional)
[Table of reference files under references/ and when to load them.]
```

### Key house-style rules for SKILL.md body

- Opening paragraph characterizes the skill as a "forge" (generation) or "flashlight" (read/query) pattern.
- "When This Skill Owns the Task" uses concrete quoted examples.
- "Delegate elsewhere when" is a 2-column table: Situation | Skill to use.
- Description field in frontmatter includes TRIGGER phrases and DO NOT TRIGGER cases inline.
- T0 contract explicitly states: No org connection, no OAuth required, output is draft for human review.
- T1 contract states OAuth scopes, Run As account constraints.
- `references/` subdirectory files are referenced but loaded on demand only — never dump into response.

---

## 3. Skill metadata.json Schema

Source: `/home/user/vanguard-frontier-agentic/schemas/skill.schema.json` (struct) and `catalog/skills.json` (catalog entries)

The validator at `tests/validate-catalog.py` requires these fields in `catalog/skills.json`:

```
id, name, type, provider, harnesses, summary, source_type, official_docs,
security_notes, last_verified, path, version, author
```

Note: `author` is required for skills in the catalog entry (see `update-catalog-new-agents.py` line: `CATALOG_FIELDS_SKILL = CATALOG_FIELDS_AGENT | {"author"}`).

### Full field list

| Field | Required | Notes |
|-------|----------|-------|
| `id` | YES | Kebab-case, matches `name` in SKILL.md frontmatter |
| `name` | YES | Human-readable name |
| `type` | YES | Always `"skill"` |
| `provider` | YES | Must be in ALLOWED_PROVIDERS (see below) |
| `harnesses` | YES | Non-empty array from: `codex`, `copilot`, `claude-code`, `cursor`, `gemini`, `kiro`, `other` |
| `summary` | YES | ≥ 20 characters |
| `source_type` | YES | `original`, `adapted`, or `reference-only` |
| `official_docs` | YES | Non-empty array of HTTPS URLs |
| `security_notes` | YES | ≥ 20 characters |
| `last_verified` | YES | `YYYY-MM-DD` format |
| `path` | YES | Repo-relative path (no trailing slash) e.g. `skills/salesforce/salesforce-apex-generator-skill` |
| `version` | YES | Semver `\d+\.\d+\.\d+` |
| `author` | YES | e.g. `"github: Raishin"` |
| `category` | optional | Same enum as frontmatter |
| `execution_tier` | optional | `static-review`, `read-only-runtime`, `sandbox-mutating`, `mutating-runtime` |
| `oauth_scopes` | optional | Array of strings |
| `mcp_servers` | optional | Array of strings |
| `run_as_permissions` | optional | Object with `required` and `denied` arrays |
| `sandbox_only` | optional | boolean |
| `production_allowed` | optional | boolean |
| `source_attribution` | optional | String (for `adapted` source type) |

### Verbatim real skill metadata.json

From `/home/user/vanguard-frontier-agentic/skills/salesforce/salesforce-apex-generator-skill/metadata.json`:

```json
{
  "id": "salesforce-apex-generator-skill",
  "name": "Salesforce Apex Generator Skill",
  "type": "skill",
  "provider": "salesforce",
  "harnesses": ["claude-code", "codex", "cursor", "gemini", "kiro", "other"],
  "summary": "Generates production-grade Apex classes with Service-Selector-Domain layering, correct sharing models (with sharing / without sharing / inherited sharing per class type), async patterns (Queueable, Batchable, Schedulable), governor-limit awareness, and security defaults. T0 static generation — no org connection required. Emits .cls + .cls-meta.xml with a 100-point quality score and an explicit test class recommendation.",
  "source_type": "adapted",
  "source_attribution": "Adapted from forcedotcom/sf-skills generating-apex (Apache-2.0). Vanguard-specific additions: T0 tier declaration, 100-point scoring rubric, sharing model correctness gate, security-defaults enforcement, and handoff routing model.",
  "category": "generation",
  "execution_tier": "static-review",
  "oauth_scopes": [],
  "mcp_servers": [],
  "run_as_permissions": {},
  "sandbox_only": false,
  "production_allowed": true,
  "official_docs": [
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_intro.htm",
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_classes_sharing_with_sharing.htm",
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_gov_limits.htm",
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_classes_security_stripInaccessible.htm",
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_interface_queueable.htm",
    "https://developer.salesforce.com/docs/atlas.en-us.apexcode.meta/apexcode/apex_batch_interface.htm"
  ],
  "security_notes": "T0 static generation only. No org connection, no OAuth, no secrets. All generated Apex uses with sharing by default; without sharing only where required by class type and documented with justification. Generated code applies WITH USER_MODE and Security.stripInaccessible() for user-data-touching classes. No hardcoded credentials, org IDs, or session tokens are ever generated. Output is draft code requiring human review before deployment.",
  "last_verified": "2026-05-21",
  "path": "skills/salesforce/salesforce-apex-generator-skill",
  "author": "github: Raishin",
  "version": "0.1.0"
}
```

---

## 4. Agent metadata.json Schema

Source: `/home/user/vanguard-frontier-agentic/schemas/agent.schema.json`

### Required fields (schema `"required"`)

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | string | Pattern `^[a-z0-9][a-z0-9-]*$` |
| `name` | string | minLength 1 |
| `version` | string | Pattern `^\d+\.\d+\.\d+$` |
| `type` | const | Always `"agent"` |
| `provider` | string | Enum (see ALLOWED_PROVIDERS below) |
| `harnesses` | array | minItems 1; each item from harness enum |
| `summary` | string | minLength 20 |
| `source_type` | string | `original`, `adapted`, or `reference-only` |
| `official_docs` | array | minItems 1; each item a URI |
| `security_notes` | string | minLength 20 |
| `last_verified` | string | Pattern `^\d{4}-\d{2}-\d{2}$` |
| `path` | string | minLength 1 |

### Optional fields (schema)

| Field | Type | Notes |
|-------|------|-------|
| `companion_skills` | array of strings | Skill ids paired to this agent. `[]` declares intentional no-pair. Pattern per item: `^[a-z0-9][a-z0-9-]*$` |
| `execution_tier` | string | `static-review`, `read-only-runtime`, `mutating-runtime` |
| `lifecycle` | string | `experimental`, `beta`, `stable`, `deprecated` |
| `harness_variants` | object | Keys are harness ids; values are repo-relative file paths to the adapter files. Keys used in salesforce: `codex`, `copilot`, `claude-code`, `cursor`, `gemini`, `kiro-ide`, `kiro-cli` |
| `author` | string | Present in all salesforce examples (additionalProperties true) |

### ALLOWED_PROVIDERS enum (from `validate-catalog.py` + `agent.schema.json`)

Current list includes: `aws`, `azure`, `oracle`, `oci`, `gcp`, `alibaba`, `huawei`, `ovhcloud`, `ionos`, `scaleway`, `hetzner`, `contabo`, `kubernetes`, `terraform`, `multi-cloud`, `generic`, `dotnet`, `hr`, `legal`, `salesforce`, `accounting`, `finance`

**NOTE: `netsuite` is NOT in this list. The netsuite provider must be added to the schema enum before validation can pass.** Check `schemas/agent.schema.json` and `tests/validate-catalog.py` `ALLOWED_PROVIDERS` set.

### Harness enum values

`codex`, `copilot`, `claude-code`, `cursor`, `gemini`, `kiro`, `other`

Note: in `harnesses` array use `kiro`; in `harness_variants` object use `kiro-ide` and `kiro-cli` as keys.

### Verbatim maestro agent metadata.json

From `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-maestro-agent/metadata.json`:

```json
{
  "id": "salesforce-maestro-agent",
  "name": "Salesforce Maestro Agent",
  "type": "agent",
  "provider": "salesforce",
  "harnesses": [
    "codex",
    "copilot",
    "claude-code",
    "cursor",
    "gemini",
    "kiro"
  ],
  "harness_variants": {
    "codex": "agents/salesforce/salesforce-maestro-agent/harnesses/codex.toml",
    "copilot": "agents/salesforce/salesforce-maestro-agent/harnesses/copilot.agent.md",
    "claude-code": "agents/salesforce/salesforce-maestro-agent/harnesses/claude-code.agent.md",
    "cursor": "agents/salesforce/salesforce-maestro-agent/harnesses/cursor.agent.md",
    "gemini": "agents/salesforce/salesforce-maestro-agent/harnesses/gemini.agent.md",
    "kiro-ide": "agents/salesforce/salesforce-maestro-agent/harnesses/kiro-ide.agent.md",
    "kiro-cli": "agents/salesforce/salesforce-maestro-agent/harnesses/kiro-cli.agent.json"
  },
  "summary": "Routes Salesforce matters to the right specialist agent and coordinates cross-functional review using the Salesforce routing protocol, case capsule, and risk taxonomy. Classification and routing only — never executes changes or mutates a Salesforce org.",
  "source_type": "original",
  "official_docs": [
    "https://help.salesforce.com/",
    "https://trailhead.salesforce.com/credentials/administrator",
    "https://developer.salesforce.com/docs"
  ],
  "security_notes": "Classification and routing only — works from sanitized signals and never requests org credentials, session tokens, client secrets, or PII. Never executes or recommends execution of live-org mutations; routes all live-org matters to salesforce-live-guard-agent with a named human decision owner and a structured case capsule.",
  "last_verified": "2026-05-20",
  "path": "agents/salesforce/salesforce-maestro-agent/",
  "companion_skills": [],
  "execution_tier": "static-review",
  "lifecycle": "experimental",
  "author": "github: Raishin",
  "version": "0.1.0"
}
```

### Verbatim specialist agent metadata.json

From `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-adaptive-access-agent/metadata.json`:

```json
{
  "id": "salesforce-adaptive-access-agent",
  "name": "Salesforce Adaptive Access Agent",
  "type": "agent",
  "provider": "salesforce",
  "harnesses": ["codex","copilot","claude-code","cursor","gemini","kiro"],
  "harness_variants": {
    "codex": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/codex.toml",
    "copilot": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/copilot.agent.md",
    "claude-code": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/claude-code.agent.md",
    "cursor": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/cursor.agent.md",
    "gemini": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/gemini.agent.md",
    "kiro-ide": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/kiro-ide.agent.md",
    "kiro-cli": "agents/salesforce/salesforce-adaptive-access-agent/harnesses/kiro-cli.agent.json"
  },
  "summary": "Reviews contextual and risk-based access controls in Salesforce — Transaction Security Policies, Shield real-time event monitoring, Dynamic Forms conditions, permission set policies, Context-Aware Access, anomaly scoring, high-assurance session enforcement, and Einstein Trust Layer boundaries — against zero-trust principles; static review only, never mutates any org.",
  "source_type": "original",
  "official_docs": [
    "https://help.salesforce.com/s/articleView?id=sf.transaction_security_policy_events.htm",
    "https://help.salesforce.com/s/articleView?id=sf.shield_event_monitoring_intro.htm"
  ],
  "security_notes": "Static review only — works from sanitized configuration excerpts and never requests org credentials, API keys, or user PII. Does not approve, deploy, or mutate any org.",
  "last_verified": "2026-05-21",
  "path": "agents/salesforce/salesforce-adaptive-access-agent/",
  "companion_skills": ["salesforce-zero-trust-maturity-skill"],
  "execution_tier": "static-review",
  "lifecycle": "experimental",
  "author": "github: Raishin",
  "version": "0.1.0"
}
```

---

## 5. AGENT.md Section Structure and House Style

Source: `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-adaptive-access-agent/AGENT.md` (specialist) and `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-maestro-agent/AGENT.md` (maestro)

### AGENT.md frontmatter (required by `schemas/agent.frontmatter.schema.json`)

```yaml
---
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---
```

Only `metadata.author` and `metadata.version` are required. `additionalProperties: true` so harness-specific fields like `name`, `description`, `allowed-tools` are permitted but not enforced.

### Specialist AGENT.md headings (verbatim from salesforce-adaptive-access-agent)

```
# <Agent Display Name>

> Agent for `<agent-id>`. <one-line summary>

## Canonical Contract

# <Agent Display Name>

Use this canonical agent only for `<agent-id>` work.

## Required Skill
Before answering, read and follow:
- `skills/<provider>/<companion-skill-id>/SKILL.md`

## Mission

## Scope Owned

## Out of Scope

## Salesforce Role / Certification Inspiration

## Required Inputs

## Operating Rules

## Evidence Requirements

## Refusal Triggers

## Escalation Triggers

## Permission / Tooling Posture

## Output Format

## Companion Skill

## Validation Plan

## Safe Next Actions
```

### Maestro AGENT.md headings (verbatim from salesforce-maestro-agent)

```
# <Maestro Name>

> Maestro agent for the <Domain> domain. <one-line summary>

## Canonical Contract

# <Maestro Name>

Use this agent only for `<maestro-id>` routing and coordination work.

## Required Skills
Before answering, read and follow:
- `skills/cross-functional/<routing-protocol>/SKILL.md`
- `skills/cross-functional/<case-capsule>/SKILL.md`
- `skills/cross-functional/<risk-taxonomy>/SKILL.md`

## Focus

## Operating Rules

## Response Shape
1. ...
2. ...
```

### House-style rules (evidence labels and refusal patterns)

From `agents/salesforce/AGENTS.md`:

- Rate every finding `Critical / High / Medium / Low / Unknown`. Unknown is mandatory when org type, product identity, or material facts are absent.
- Evidence is labeled: `[FACT]`, `[ASSUMPTION]`, `[INFERENCE]` (implied by "Separate facts from inference" pattern).
- Refusal triggers are explicit checklist items — must list each category of refused request.
- Escalation triggers are explicit conditions that fire routing or escalation.
- "Never accept verbal or summary assertions as a substitute for configuration excerpts or screenshots."
- T3 production mutation is PROHIBITED for all agents — only humans operate via live-guard.
- T1/T2 agents must declare OAuth scopes as `api refresh_token` only — never `full`, `web`, `chatbot_api`, or `sfap_api`.
- Codex adapter: keep `harnesses/codex.toml` flat; no leading indentation on top-level keys; use TOML multiline strings for `developer_instructions`.
- Markdown adapters: flush-left after frontmatter; do not indent body or create accidental code blocks.

---

## 6. LEAST-PRIVILEGES.md Structure

Source: `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-adaptive-access-agent/LEAST-PRIVILEGES.md`

Required per the `agents/salesforce/AGENTS.md` rule: "LEAST-PRIVILEGES.md is the agent's least-privilege Salesforce posture — execution tier, OAuth scopes, Run As account requirements, MCP server binding, blast-radius bound, refusal triggers, escalation path. Required for every agent in this folder."

### Standard section headings

```markdown
# Least-privilege <Provider> posture for <Agent Display Name>

## Execution tier

**T0 — Static Review** (or T1/T2/T3)

Rationale: `execution_tier: "<tier>"` declared in `metadata.json`. <justification>

## Identity model

[What identity is required; "No live identity required" for T0]

## Run As account requirements

[Not applicable for T0. For T1: Connected App scope, service account constraints.]

## MCP server binding

[None for T0. For T1+: list of permitted MCP servers.]

## Blast-radius bound

[Explicit statement of what this agent cannot do even if fully compromised.]

## Refusal triggers

- [ ] <condition>
- [ ] <condition>

## Escalation path

[Where to route all mutation requests; named agent.]

---

References: [Execution tiers](<path to docs/execution-tiers.md>) | [<Provider> agents README](<path to README>)

## Validation checklist

Before submitting configuration excerpts for review by this agent:

- [ ] <checklist item>

## Companion skill

`<companion-skill-id>` — <one-line description of the skill's role>
```

---

## 7. The 7 Harness Adapter Files — Format Specification

### CRITICAL: Generator Discovery

**NO generator exists for salesforce-style harness adapters.** The generators in `scripts/` are:

- `scripts/gen_azure_live_guards.py` — generates Azure live-guard agent + skill pairs including all 7 harness files (this is a model to follow when writing a netsuite generator).
- `scripts/gen_oci_live_guards.py` — similar for OCI.

The salesforce agents do NOT have a generator script — their harness files were authored individually. For the netsuite build (25 agents × 7 harnesses = 175 files), the recommended approach is to write a Python generator in the style of `scripts/gen_azure_live_guards.py`. The Azure generator is the exact pattern to replicate.

### Generator command to write (does not exist yet)

```bash
python3 scripts/gen_netsuite_agents.py
```

This must be created. Model it on `/home/user/vanguard-frontier-agentic/scripts/gen_azure_live_guards.py`.

### Harness file formats (verbatim from salesforce-maestro-agent and salesforce-adaptive-access-agent)

#### 1. `harnesses/codex.toml`

Required fields (validated by `tests/validate-catalog.py` → `validate_codex_harness_adapters()`):
- `name`
- `description`
- `developer_instructions`
- Top-level keys must NOT be indented (4-space indentation fails validation).

Verbatim from `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-adaptive-access-agent/harnesses/codex.toml`:

```toml
name = "salesforce_adaptive_access_agent"
description = "Reviews contextual and risk-based access controls in Salesforce — Transaction Security Policies, Shield real-time event monitoring, Dynamic Forms conditions, permission set policies, Context-Aware Access, anomaly scoring, high-assurance session enforcement, and Einstein Trust Layer boundaries — against zero-trust principles; static review only, never mutates any org."
model = "gpt-5.5"
model_reasoning_effort = "high"
sandbox_mode = "read-only"

developer_instructions = """
Load and follow the bound `salesforce-zero-trust-maturity-skill` skill first.

Token discipline:
- Read only SKILL.md first; load references only when the task requires them.
- Keep answers compact: verdict, brutal assessment, facts, assumptions, findings, adversarial stress test, risk table, safe next actions, escalation trigger, open questions.

Role focus: Review contextual and risk-based access controls in Salesforce — Transaction Security Policy coverage and enforcement actions, Shield real-time event monitoring posture, Dynamic Forms access conditions, permission set assignment policies, Context-Aware Access policies, anomaly scoring, high-assurance session enforcement before sensitive operations, and Einstein Trust Layer access boundaries — against zero-trust principles.

Safety contract:
- Static review only; never invokes Salesforce APIs, sf CLI, or org credentials.
- Work from sanitized configuration excerpts; never request org credentials, API keys, or user PII.
- Does not approve, deploy, or mutate any org.
- Rate every finding Critical / High / Medium / Low / Unknown.
- Flag uncovered Transaction Security event types, notify-only enforcement on high-risk events, and privileged permission sets without high-assurance session requirements as priority findings.
"""

[metadata]
author = "github: Raishin"
version = "0.1.0"

[[skills.config]]
path = "skills/salesforce/salesforce-zero-trust-maturity-skill/SKILL.md"
enabled = true
```

Key conventions:
- `name` uses `snake_case` (underscores, not hyphens).
- `model = "gpt-5.5"` and `model_reasoning_effort = "high"` are standard.
- `sandbox_mode = "read-only"` for T0/T1 static-review agents; `"workspace-write"` only for guarded live operators.
- `[[skills.config]]` section declares the companion skill path.
- Validation also checks `developer_instructions` contains the safety contract prose.

#### 2. `harnesses/copilot.agent.md`

The copilot format has a frontmatter with `name`, `description`, and a `tools` list (Copilot-specific). Verbatim from the salesforce-adaptive-access-agent:

```markdown
---
name: "salesforce-adaptive-access-agent"
description: "Reviews contextual and risk-based access controls in Salesforce — Transaction Security Policies, Shield real-time event monitoring, Dynamic Forms conditions, permission set policies, Context-Aware Access, anomaly scoring, high-assurance session enforcement, and Einstein Trust Layer boundaries — against zero-trust principles; static review only, never mutates any org."
---

# Salesforce Adaptive Access Agent

Use this agent only for `salesforce-adaptive-access-agent` work.

## Required Skill
Before answering, read and follow:
- `skills/salesforce/salesforce-zero-trust-maturity-skill/SKILL.md`

## Mission
...

## Response Shape
1. ...
```

Note: The Azure generator uses an extended copilot frontmatter with explicit `tools:` list including `"read"`, `"search"`, `"execute/runInTerminal"`, etc. The salesforce agents use minimal frontmatter (name + description only). Follow the salesforce pattern for netsuite.

#### 3. `harnesses/claude-code.agent.md`

Identical content to copilot.agent.md for salesforce agents. Verbatim from maestro:

```markdown
---
name: "Salesforce Maestro Agent"
description: "Routes Salesforce matters to the right Salesforce specialist agent..."
---

# Salesforce Maestro Agent

Use this agent only for `salesforce-maestro` routing and coordination work.

## Required Skills
Before answering, read and follow:
- `skills/cross-functional/salesforce-routing-protocol/SKILL.md`
- ...

## Focus
...

## Operating Rules
...

## Response Shape
1. ...
```

#### 4. `harnesses/cursor.agent.md`

Identical content to claude-code.agent.md for salesforce agents. Same frontmatter + body.

#### 5. `harnesses/gemini.agent.md`

Identical content to claude-code.agent.md for salesforce agents. Same frontmatter + body.

#### 6. `harnesses/kiro-ide.agent.md`

Identical content to claude-code.agent.md for salesforce agents. Same frontmatter + body.

#### 7. `harnesses/kiro-cli.agent.json`

JSON format with three keys: `name`, `description`, `prompt`. The `prompt` is the full agent body as a single escaped JSON string (newlines as `\n`).

Verbatim from `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-adaptive-access-agent/harnesses/kiro-cli.agent.json`:

```json
{
  "name": "salesforce-adaptive-access-agent",
  "description": "Reviews contextual and risk-based access controls in Salesforce — ...",
  "prompt": "You are the Salesforce Adaptive Access Agent. Load and follow the bound skill at skills/salesforce/salesforce-zero-trust-maturity-skill/SKILL.md before answering.\n\nMission: ...\n\nScope: ...\n\nOut of Scope: ...\n\nOperating Rules: ...\n\nRefusal Triggers: ...\n\nEscalation Triggers: ...\n\nPermission posture: Static review only. Never invokes Salesforce APIs, sf CLI, or org credentials. Does not approve, deploy, or mutate any org.\n\nRespond with: 1) Verdict ..., 2) ..., 10) Open questions."
}
```

Key: `prompt` is the agent body flattened to one JSON string. All sections from the AGENT.md body are concatenated with `\n\n` between sections, markdown headings preserved, content on single lines. The `name` field uses kebab-case (hyphens), not snake_case.

#### Summary: which files differ vs. which are identical

| File | Unique format? |
|------|---------------|
| `codex.toml` | YES — TOML, unique format |
| `copilot.agent.md` | Minimal frontmatter + body (same body as others) |
| `claude-code.agent.md` | Identical to copilot content for salesforce agents |
| `cursor.agent.md` | Identical to copilot content for salesforce agents |
| `gemini.agent.md` | Identical to copilot content for salesforce agents |
| `kiro-ide.agent.md` | Identical to copilot content for salesforce agents |
| `kiro-cli.agent.json` | YES — JSON, unique format, body flattened to prompt string |

---

## 8. Maestro Agent Specifics

Source: `/home/user/vanguard-frontier-agentic/agents/salesforce/salesforce-maestro-agent/README.md`

### Maestro README.md structure (required for the maestro agent)

The maestro is the only agent that has a `README.md` file. Specialist agents do NOT have README.md files (only AGENT.md, LEAST-PRIVILEGES.md, metadata.json, and harnesses/).

The maestro README.md contains:

```
# <Provider> Maestro Agent

[2-paragraph description of the maestro as entry point, what it does, what it refuses.]

---

## Quick start (5 minutes)

Choose the harness that matches your tooling. All adapter files live at
`agents/<provider>/<maestro-id>/harnesses/`.

### Claude Code
[Install path, copy command, invocation, minimal prompt]

### Cursor
[Install path, copy command, invocation, minimal prompt]

### GitHub Copilot
[Install path, copy command, invocation, minimal prompt]

### Gemini
[Install path, copy command, invocation, minimal prompt]

### Kiro IDE
[Install path, copy command, invocation, minimal prompt]

### Kiro CLI
[Install path, copy command, invocation, minimal prompt]

### Codex
[Install path, copy command, invocation, minimal prompt]

---

## How routing works

### Required skills
[List of cross-functional skills loaded by the maestro]

### Case capsule
[Table: field name | description for every case capsule field]

### Routing modes
[single / parallel / escalate / unclassified definitions]

### Escalation gates
[Bulleted list of escalation-grade categories]

### Taxonomy reference
[Reference to tests/fixtures/<provider>-maestro-routing/taxonomy.json]

---

## The <N>-domain taxonomy

| Domain | Primary agent | Typical signals |
|---|---|---|
...

[Two structural roles not in routing table:]
| Role | Agent | Function |
|---|---|---|
| Maestro | ... | ... |
| Live Guard | ... | ... |

---

## Worked examples (step by step)

### Example 1 — Static review (T0)
[Request, Step 1 Classification, Step 2 Risk rating, Step 3 Case capsule excerpt, Step 4 Human action]

### Example 2 — Read-only runtime (T1)
[Same structure]

### Example 3 — Production mutation request (T3 — refused)
[Same structure]

---

## What the maestro will refuse

[Bulleted list of refusal categories]

---

## Least-privilege posture

[Reference to LEAST-PRIVILEGES.md and docs/execution-tiers.md]

---

## Troubleshooting

### Ambiguous routing — multiple keyword clusters match
### Low-confidence routing
### Request falls outside the <N>-domain taxonomy
### Specialist agents disagree

---

## Eval coverage

[Reference to tests/fixtures/<provider>-maestro-routing/ eval structure]
[Command: npm run validate:maestro-routing]

---

## Versioning

[Reference to metadata.json and package.json sync via scripts/release-prepare.mjs]

---

## Related docs

[Links to docs/execution-tiers.md, agents/<provider>/README.md, docs/release-versioning.md, docs/compatibility.md, tests/fixtures/]

---

Part of the [Vanguard Frontier Agentic](...) <Provider> portfolio.
```

### Routing table format (from the maestro README.md)

The taxonomy table uses this 3-column format:

```markdown
| Domain | Primary agent | Typical signals |
|---|---|---|
| `<domain-key>` | `<provider>-<domain>-agent` | keyword1, keyword2, keyword3 |
```

The maestro README explicitly lists all domains in the taxonomy, with two entries excluded from the keyword routing table shown separately as "structural roles" (Maestro itself + Live Guard).

---

## 9. Catalog and Post-Build Commands

After creating all agent and skill files, these commands must be run:

```bash
# Add new entries to catalog/agents.json and catalog/skills.json
python3 scripts/update-catalog-new-agents.py

# Refresh skill-manifest.json (SHA256 hashes of all skill files)
npm run manifest:write

# Update README inline count markers
npm run readme-counts:write

# Update Jekyll docs/_data/catalog.yml
npm run docs-data:write

# Sync .claude-plugin version + agents
npm run plugin-manifest:write

# Sync .cursor-plugin version + agents
npm run cursor-plugin:write

# Regenerate Kiro Powers for all providers
npm run kiro-powers:write

# Refresh SHA256 hashes of tracked assets
python3 tests/validate-asset-integrity.py --write

# Or all-in-one:
npm run manifest:write:all

# Final validation (all 19+ gates)
npm run validate
```

NOTE: `netsuite` as a provider must be added to:
1. `schemas/agent.schema.json` — `provider` enum
2. `tests/validate-catalog.py` — `ALLOWED_PROVIDERS` set

---

## 10. Provider Registration: Mandatory Schema Changes

The `netsuite` provider does not currently exist in the repo's allow-lists. Two files MUST be updated before any netsuite asset can pass `npm run validate`:

### `schemas/agent.schema.json` — add `"netsuite"` to `provider` enum

Current provider enum includes: `aws`, `azure`, `oracle`, `oci`, `gcp`, `alibaba`, `huawei`, `ovhcloud`, `ionos`, `scaleway`, `hetzner`, `contabo`, `kubernetes`, `terraform`, `multi-cloud`, `generic`, `dotnet`, `hr`, `legal`, `salesforce`, `accounting`, `finance`

Add `"netsuite"` to this list.

### `tests/validate-catalog.py` — add `"netsuite"` to `ALLOWED_PROVIDERS` set

Current set (line ~22–59 of validate-catalog.py):
```python
ALLOWED_PROVIDERS = {
    "aws",
    "azure",
    ...
    "salesforce",
    "accounting",
    "finance",
}
```

Add `"netsuite"` to this set.

---

## TEMPLATE CHECKLIST

### Per Skill (1 skill = 3 files + optional references/)

Ordered creation sequence:

1. `skills/netsuite/<skill-id>/SKILL.md` — YAML frontmatter + body
2. `skills/netsuite/<skill-id>/metadata.json` — full skill metadata
3. `skills/netsuite/<skill-id>/references/` — optional reference markdown files (e.g., `references/field-mappings.md`)

After all skills created:
4. Run `python3 scripts/update-catalog-new-agents.py` to add to `catalog/skills.json`
5. Run `npm run manifest:write` to regenerate `catalog/skill-manifest.json`

### Per Agent (1 agent = 4 required files + 7 harness files = 11 files)

Ordered creation sequence:

1. `agents/netsuite/<agent-id>/metadata.json` — full agent metadata (with `harness_variants` map)
2. `agents/netsuite/<agent-id>/AGENT.md` — frontmatter + canonical contract body
3. `agents/netsuite/<agent-id>/LEAST-PRIVILEGES.md` — execution tier, identity model, blast-radius, refusal triggers, escalation path
4. `agents/netsuite/<agent-id>/harnesses/codex.toml` — TOML adapter
5. `agents/netsuite/<agent-id>/harnesses/copilot.agent.md` — Markdown adapter (name + description frontmatter)
6. `agents/netsuite/<agent-id>/harnesses/claude-code.agent.md` — Markdown adapter (same body as copilot)
7. `agents/netsuite/<agent-id>/harnesses/cursor.agent.md` — Markdown adapter (same body as copilot)
8. `agents/netsuite/<agent-id>/harnesses/gemini.agent.md` — Markdown adapter (same body as copilot)
9. `agents/netsuite/<agent-id>/harnesses/kiro-ide.agent.md` — Markdown adapter (same body as copilot)
10. `agents/netsuite/<agent-id>/harnesses/kiro-cli.agent.json` — JSON adapter with flattened prompt string
11. `agents/netsuite/<agent-id>/README.md` — ONLY for the maestro agent (not for specialist agents)

After all agents created:
12. Run `python3 scripts/update-catalog-new-agents.py` to add to `catalog/agents.json`

### Per Provider (one-time setup)

Before building any asset:

A. Edit `schemas/agent.schema.json` — add `"netsuite"` to `provider` enum
B. Edit `tests/validate-catalog.py` — add `"netsuite"` to `ALLOWED_PROVIDERS` set
C. Create `agents/netsuite/AGENTS.md` — operating rules for the netsuite portfolio
D. Create `agents/netsuite/README.md` — portfolio overview (all N agents, descriptions)
E. Create `skills/netsuite/README.md` — skills portfolio overview

After all assets built:
F. Run `npm run manifest:write:all` — refreshes all generated catalogs and manifests
G. Run `python3 tests/validate-asset-integrity.py --write` — regenerates SHA256 hashes
H. Run `git add catalog/asset-integrity.json` — stage the new hashes
I. Run `npm run validate` — must pass all 19+ gates

### Recommended build order for 25 agents + 25 skills

1. Schema changes (A, B above) — unblock the provider
2. Write Python generator: `scripts/gen_netsuite_agents.py` — modeled on `scripts/gen_azure_live_guards.py` — generates all agent + skill file structures from a data definition dict
3. Run the generator
4. Review output, adjust descriptions and domain-specific content
5. Add provider directories and README files (C, D, E)
6. Run post-build catalog commands (F, G, H)
7. Run `npm run validate` and fix any remaining errors

### File count summary for 25 agents + 25 skills

| Category | Files per unit | Units | Total |
|----------|---------------|-------|-------|
| Skill SKILL.md | 1 | 25 | 25 |
| Skill metadata.json | 1 | 25 | 25 |
| Skill references/ files | ~1–4 each | 25 | ~25–100 |
| Agent metadata.json | 1 | 25 | 25 |
| Agent AGENT.md | 1 | 25 | 25 |
| Agent LEAST-PRIVILEGES.md | 1 | 25 | 25 |
| Agent harness codex.toml | 1 | 25 | 25 |
| Agent harness copilot.agent.md | 1 | 25 | 25 |
| Agent harness claude-code.agent.md | 1 | 25 | 25 |
| Agent harness cursor.agent.md | 1 | 25 | 25 |
| Agent harness gemini.agent.md | 1 | 25 | 25 |
| Agent harness kiro-ide.agent.md | 1 | 25 | 25 |
| Agent harness kiro-cli.agent.json | 1 | 25 | 25 |
| Maestro README.md | 1 | 1 | 1 |
| Provider AGENTS.md | 1 | 1 | 1 |
| Provider agent README.md | 1 | 1 | 1 |
| Provider skill README.md | 1 | 1 | 1 |
| **Minimum total (no references)** | | | **~328** |

