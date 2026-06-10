# Language and Stack Boards

Language and stack boards are topical agent collections scoped to a language,
runtime, or functional domain rather than to a cloud provider. They live
alongside the provider boards (`aws`, `azure`, `gcp`, and others) and share the
same `provider` faceting axis: each shipped topical board has its own dedicated
`provider` enum value.

This document covers the current boards: `.NET`, `legal`, `hr`, `marketing`,
`salesforce`, `netsuite`, `accounting`, and `finance`. It also describes how
to use them for discovery and how to add a new board.

See [taxonomy.md](taxonomy.md) for the full provider and asset-type taxonomy
that governs all boards in this marketplace.

---

## What are language and stack boards?

A **language/stack board** is a dedicated directory pair — `agents/<name>/` and
`skills/<name>/` — plus a shared ID prefix (`<name>-*`) for all assets in that
collection. The board groups agents and skills that share a common subject area:
a language ecosystem (`.NET`), a professional function (`legal`, `hr`), or a
compliance domain (`marketing`).

Every board also gets an entry in `catalog/install-roles.json` under a named
install role so users can pull the full set with a single `--role` flag.

### How they differ from provider boards

| Dimension | Provider board (e.g. `aws`) | Language/stack board (e.g. `dotnet`) |
|-----------|----------------------------|--------------------------------------|
| `provider` field | `aws`, `azure`, `gcp`, … | dedicated board name (`dotnet`, `legal`, `hr`, `marketing`, `salesforce`, `netsuite`, `accounting`, `finance`) |
| Directory | `agents/aws/` | `agents/dotnet/`, `agents/legal/`, … |
| ID prefix | `aws-*` | `dotnet-*`, `legal-*`, `hr-*`, `marketing-*`, `salesforce-*`, `netsuite-*`, etc. |
| Subject scope | Cloud service surface | Language/runtime or professional function |
| Execution tier | Varies by agent | `static-review` (all language/stack boards) |
| Faceting axis | `provider` enum | `provider` enum (dedicated value) plus shared ID prefix |

Provider boards target infrastructure and cloud services. Language/stack boards
target code quality, legal posture, HR process risk, and compliance governance.
The two sets coexist in the same catalog and can be installed independently or
together.

---

## Provider values for topical boards

The `provider` field is a faceting axis. It started as a cloud/platform axis,
but it also carries non-cloud topical boards: each shipped board gets its own
dedicated `provider` enum value. `dotnet`, `hr`, `legal`, `marketing`,
`salesforce`, `netsuite`, `accounting`, and `finance` are all first-class
`provider` values, listed in `docs/taxonomy.md` under **Providers** and
accepted by the schema and catalog validators.

A dedicated `provider` value lets users filter the board directly — for
example `npx vfa-export-agents --platform claude-code --provider dotnet`
installs the entire `.NET` board. The shared ID prefix (`dotnet-*`, `hr-*`,
`legal-*`, `marketing-*`) remains the secondary discovery key and stays stable
even if the board's `provider` value ever changes.

A new topical board uses `provider: generic` only until it ships a coherent
agent/skill set; at that point it is promoted to its own `provider` value. The
`qa` board is the current pre-promotion example — its assets still declare
`provider: generic`. Promotion changes the `provider` field, the schema and
validator enums, and the catalog entries, but never the ID prefix.

---

## The boards

### .NET

The `.NET` board covers static review of C# applications and the surrounding
.NET ecosystem: runtime correctness, ASP.NET Core API architecture,
authentication and authorization, EF Core data access, test quality, CI/NuGet
supply-chain integrity, performance and Native AOT readiness, in-application
OpenTelemetry wiring, and .NET Aspire cloud-native posture.

| Property | Value |
|----------|-------|
| `provider` | `dotnet` |
| ID prefix | `dotnet-*` |
| Agent directory | `agents/dotnet/` |
| Skill directory | `skills/dotnet/` |
| Agents | 10 |
| Skills | 10 (1:1 companion skill per agent) |
| Install role | `dotnet-application-review-engineer` |
| Execution tier | `static-review` (all agents) |

**Agent directory layout**

```
agents/dotnet/
  dotnet-maestro-agent/
  dotnet-csharp-runtime-review-agent/
  dotnet-aspnetcore-api-review-agent/
  dotnet-aspnetcore-identity-authz-review-agent/
  dotnet-efcore-data-access-review-agent/
  dotnet-testing-quality-review-agent/
  dotnet-supply-chain-review-agent/
  dotnet-performance-aot-review-agent/
  dotnet-observability-otel-review-agent/
  dotnet-aspire-cloud-native-review-agent/
```

**Example agents**

| Agent | Scope |
|-------|-------|
| `dotnet-maestro-agent` | Router; classifies a `.NET` task and dispatches the narrowest specialist or a parallel team of up to four. Never answers `.NET` questions itself. |
| `dotnet-efcore-data-access-review-agent` | EF Core: DbContext lifetime, N+1 patterns, raw SQL safety, concurrency tokens, migrations, multi-tenant query filters. |
| `dotnet-supply-chain-review-agent` | CI/CD and NuGet supply chain: SDK pinning, package locking, feed trust, fork-PR secret exposure, vulnerability scanning. |
| `dotnet-aspire-cloud-native-review-agent` | .NET Aspire AppHost and service-defaults review for cloud-native readiness. |

Each agent reads source and sanitized configuration only. No agent runs
`dotnet build`, `dotnet test`, `dotnet ef`, contacts a live system, or edits
project files.

---

### Legal

The `legal` board covers adversarial, evidence-grounded triage of legal and
compliance risk: contracts, privacy, regulatory obligations, litigation holds,
IP, vendor procurement, ethics investigations, policy governance, and public
disclosure. It is scoped to the enterprise legal and compliance function across
the US, EU, UK, Singapore, and Australia.

| Property | Value |
|----------|-------|
| `provider` | `legal` |
| ID prefix | `legal-*` |
| Agent directory | `agents/legal/` |
| Skill directory | `skills/legal/` |
| Agents | 13 |
| Skills | 1 board-specific + 3 cross-functional (shared with `hr`) |
| Install role | `legal-hr-risk-reviewer` (shared with the `hr` board) |
| Execution tier | `static-review` (all agents) |

**Agent directory layout**

```
agents/legal/
  legal-maestro-agent/
  legal-counsel-review-agent/
  legal-contract-review-agent/
  legal-privacy-data-protection-agent/
  legal-employment-law-risk-agent/
  legal-litigation-discovery-hold-agent/
  legal-regulatory-compliance-agent/
  legal-ip-open-source-agent/
  legal-vendor-procurement-risk-agent/
  legal-ethics-investigations-agent/
  legal-policy-governance-agent/
  legal-public-disclosure-agent/
  legal-knowledge-management-agent/

skills/legal/
  legal-counsel-review/

skills/cross-functional/
  legal-hr-case-capsule/      # shared with hr board
  legal-hr-routing-protocol/
  legal-hr-risk-taxonomy/
```

**Example agents**

| Agent | Scope |
|-------|-------|
| `legal-maestro-agent` | Router; classifies a legal matter and routes it to the right specialist or coordinates multi-agent review. |
| `legal-contract-review-agent` | Contract clauses: indemnity, liability, termination, renewal, warranties, audit rights, governing law. |
| `legal-privacy-data-protection-agent` | Data protection posture, retention, cross-border transfer, DPIA readiness, vendor DPAs, employee-data processing. |
| `legal-ethics-investigations-agent` | Whistleblower, conflict of interest, anti-bribery, sanctions, and executive-misconduct intake triage. |

The `legal` board is paired with the `hr` board under a single install role
because cross-domain matters — employment disputes, investigations,
terminations — regularly require both. Cross-domain hand-off uses the
`legal-hr-case-capsule` skill; see
`docs/architecture/legal-hr-agent-routing.md`.

These agents give no legal advice and form no attorney-client relationship. All
outputs are risk-structured analysis for review by qualified counsel.

---

### HR

The `hr` board covers HR, employment-risk, and People-function review: employee
relations, workplace investigations, performance management, termination
readiness, leave and accommodation, recruiting, compensation equity, benefits,
workforce planning, HR analytics, culture and DEI, and HRIS process controls.

| Property | Value |
|----------|-------|
| `provider` | `hr` |
| ID prefix | `hr-*` |
| Agent directory | `agents/hr/` |
| Skill directory | `skills/hr/` |
| Agents | 15 |
| Skills | 1 board-specific + 3 cross-functional (shared with `legal`) |
| Install role | `legal-hr-risk-reviewer` (shared with the `legal` board) |
| Execution tier | `static-review` (all agents) |

**Agent directory layout**

```
agents/hr/
  hr-maestro-agent/
  hr-risk-triage-review-agent/
  hr-employee-relations-agent/
  hr-workplace-investigations-agent/
  hr-performance-management-agent/
  hr-termination-readiness-agent/
  hr-leave-accommodation-agent/
  hr-recruiting-selection-agent/
  hr-compensation-equity-agent/
  hr-benefits-payroll-agent/
  hr-workforce-planning-rif-agent/
  hr-learning-policy-agent/
  hr-analytics-people-data-agent/
  hr-culture-dei-agent/
  hr-hris-process-controls-agent/

skills/hr/
  hr-risk-triage-review/
```

**Example agents**

| Agent | Scope |
|-------|-------|
| `hr-maestro-agent` | Router; classifies an HR matter and routes it to the right specialist or coordinates cross-functional review. |
| `hr-risk-triage-review-agent` | Triage terminations, discipline, accommodations, wage/hour, discrimination, harassment, retaliation, and layoff risk for employment-law exposure. |
| `hr-workplace-investigations-agent` | Investigation planning, evidence mapping, witness sequencing, neutrality and confidentiality controls. |
| `hr-compensation-equity-agent` | Compensation, promotion, leveling, pay equity, incentives, calibration, and adverse-impact risk. |

No agent terminates, disciplines, denies leave, or sends an employee
communication. Every adverse or irreversible action routes to a named human
owner. These agents give no legal or HR advice and form no attorney-client
relationship.

---

### Marketing

The `marketing` board covers the marketing-technology compliance and security
surface: consent and data-collection posture (GDPR/ePrivacy/CCPA), advertising
pixel personal-data leakage, GPC signal handling, email sender authentication,
programmatic supply-chain integrity, AI advertising fairness, EU AI Act
applicability, lookalike audience upload compliance, email list retention,
influencer disclosure, and dark-pattern conversion flow review.

| Property | Value |
|----------|-------|
| `provider` | `marketing` |
| ID prefix | `marketing-*` (most agents); some use domain-specific prefixes |
| Agent directory | `agents/marketing/` |
| Skill directory | `skills/marketing/` |
| Agents | 14 |
| Skills | 14 (1:1 companion skill per agent) |
| Install role | `marketing-governance-reviewer` |
| Execution tier | `static-review` (13 agents); `read-only-runtime` (maestro) |

**Agent directory layout**

```
agents/marketing/
  marketing-maestro-agent/
  marketing-consent-data-collection-review-agent/
  marketing-pixel-data-leakage-review-agent/
  martech-access-governance-review-agent/
  marketing-gpc-signal-honoring-review-agent/
  email-sender-authentication-review-agent/
  programmatic-supply-chain-integrity-review-agent/
  ai-advertising-targeting-fairness-review-agent/
  eu-ai-act-marketing-system-review-agent/
  lookalike-audience-upload-compliance-review-agent/
  marketing-email-list-retention-review-agent/
  influencer-disclosure-compliance-review-agent/
  marketing-conversion-flow-dark-pattern-review-agent/
  analytics-data-minimization-review-agent/
```

**Example agents**

| Agent | Scope |
|-------|-------|
| `marketing-maestro-agent` | Router; classifies a marketing-governance task and dispatches the appropriate specialist. |
| `marketing-pixel-data-leakage-review-agent` | Advertising pixels and conversion event tracking: PII leakage to ad networks, form-field auto-capture, pixel placement on sensitive pages. |
| `eu-ai-act-marketing-system-review-agent` | AI-powered marketing systems: EU AI Act risk-classification and prohibited-practice exposure. |
| `martech-access-governance-review-agent` | OAuth connected apps, API keys, CRM and marketing-automation roles, and integration scopes: least-privilege violations and stale credentials. |

Note: several agents in this board use non-`marketing-` prefixes
(`martech-*`, `ai-*`, `analytics-*`, `email-*`, `programmatic-*`,
`influencer-*`, `lookalike-*`, `eu-ai-act-*`). This reflects the
subject-area specificity of those agents. All live under `agents/marketing/`
and declare `provider: marketing` in `metadata.json`.

---

## How to use language/stack boards

### Discovery via install roles

The primary entry point for users is the named install role in
`catalog/install-roles.json`. Each role bundles the minimal agent and skill
set for a given function.

| Role | Boards covered | Agents | Skills |
|------|---------------|--------|--------|
| `dotnet-application-review-engineer` | `.NET` | 10 | 10 |
| `legal-hr-risk-reviewer` | `legal` + `hr` | 28 | 5 (2 board-specific + 3 cross-functional) |
| `marketing-governance-reviewer` | `marketing` | 14 | 14 |

Install a role with the export CLI:

```bash
npx vfa-export-agents --platform claude-code --role dotnet-application-review-engineer --repo .
npx vfa-export-agents --platform claude-code --role legal-hr-risk-reviewer --repo .
npx vfa-export-agents --platform claude-code --role marketing-governance-reviewer --repo .
```

### Routing

Each board includes a maestro router agent (`dotnet-maestro-agent`,
`legal-maestro-agent`, `hr-maestro-agent`, `marketing-maestro-agent`). Address
the maestro with the task; it classifies the work and dispatches the narrowest
specialist or a small parallel team. Do not reach past the maestro and invoke a
specialist directly unless you already know which specialist applies.

### Invocation

Agents install as harness-native adapter files. On Claude Code:

```
@dotnet-maestro-agent Review the EF Core migrations in /src/Data/
@legal-counsel-review-agent Here is the indemnity clause from our SaaS agreement: …
@hr-risk-triage-review-agent The manager wants to terminate an employee who returned from FMLA last week.
@marketing-pixel-data-leakage-review-agent Here is our Meta Pixel configuration and the pages it fires on.
```

On other harnesses (Copilot, Cursor, Codex, Gemini) the same agent IDs are
available via the harness's agent-selection UI or `@mention` syntax. See
[compatibility.md](compatibility.md) for per-harness adapter details.

### Individual agent install

If you need only one agent from a board rather than the full role:

```bash
npx vfa-export-agents --platform claude-code \
  --agents dotnet-efcore-data-access-review-agent --repo .
```

---

## Adding a new language/stack board

Follow this sequence to introduce a new board. Use `agents/dotnet/` as the
reference implementation — it is the most complete example with the taxonomy
note, tier table, and 1:1 agent-to-skill pairing.

### 1. Create the directory pair

```
agents/<board-name>/
  README.md
skills/<board-name>/
```

The `README.md` should state the board's subject scope, tier table, and
taxonomy note. Copy the note from `agents/dotnet/README.md` and adapt it to
clarify the board's `provider` value: a shipped board uses its own dedicated
`provider` value (added to the schema and validator enums); a board still
under construction uses `provider: generic` until it is promoted.

### 2. Define agents

For each agent, create:

```
agents/<board-name>/<board>-<role>-agent/
  AGENT.md
  metadata.json
  harnesses/
    claude-code.agent.md
    codex.toml
    copilot.agent.md
    cursor.agent.md
    gemini.agent.md
    kiro-ide.agent.md
    kiro-cli.agent.json
```

Key `metadata.json` fields for a language/stack board agent:

| Field | Value |
|-------|-------|
| `id` | `<board>-<role>-agent` |
| `type` | `"agent"` |
| `provider` | `"<board-name>"` (dedicated value) or `"generic"` until promoted |
| `execution_tier` | `"static-review"` |
| `companion_skills` | Array of companion skill IDs, or `[]` |
| `lifecycle` | `"experimental"` until the board stabilizes |

### 3. Define companion skills

For each agent, create a matching skill:

```
skills/<board-name>/<board>-<role>/
  SKILL.md          # YAML frontmatter + skill body
  metadata.json
```

The `SKILL.md` frontmatter must declare `allowed-tools` at the least-privilege
baseline. Language/stack board skills are read-only; a minimal baseline is
`Read Grep Glob`. See `schemas/skill.frontmatter.schema.json` for the full
contract and `skills/dotnet/dotnet-maestro/SKILL.md` for a worked example.

### 4. Add a maestro router

Include a `<board>-maestro-agent` and a paired `<board>-maestro` skill. The
maestro classifies tasks and dispatches specialists; it never answers domain
questions itself.

### 5. Add a board-level README

Document the board's scope, the tier table (router vs. review agents, default
access, live-execution policy), and a brief operating note for each agent. See
`agents/dotnet/README.md` and `agents/legal/README.md` as templates.

### 6. Register the provider value

A shipped board gets its own dedicated `provider` value. Add the board name to
every provider enum so the schema and catalog validators accept it:

- `schemas/agent.schema.json` — `provider.enum`
- `schemas/skill.schema.json` — `provider.enum`
- `tests/validate-catalog.py` — `ALLOWED_PROVIDERS`
- `docs/taxonomy.md` — the **Providers** list

Then set `provider: "<board-name>"` in every agent and skill `metadata.json`
for the board. A board still under construction may skip this step and use
`provider: generic` until it is promoted.

### 7. Update the catalog

Add each new agent to `catalog/agents.json` and each new skill to
`catalog/skills.json`, using the board's `provider` value. After adding
skills, regenerate the manifest:

```bash
npm run manifest:write
```

### 8. Wire an install role

Add an entry to `catalog/install-roles.json`:

```json
"<board>-<function>-reviewer": {
  "label": "<Human-readable label>",
  "description": "<One- or two-sentence description of scope and static-review posture>",
  "agents": ["<board>-maestro-agent", "<board>-specialist-a-agent", "…"],
  "skills": ["<board>-maestro", "<board>-specialist-a", "…"]
}
```

If the new board shares natural install-role scope with an existing board
(as `legal` and `hr` do), add the new agents and skills to the existing role
entry rather than creating a second one.

### 9. Run validation

```bash
npm run validate
```

All seven gates must pass before opening a pull request. The key gates for a
new board are `validate:catalog` (all entries reference real paths and satisfy
schemas), `validate:skill-schema`, `validate:agent-schema`,
`validate:allowed-tools`, and `validate:links`. If skills changed, confirm
`manifest:check` also passes.

---

## Trust posture

All language/stack board agents are scoped to `static-review` or an equivalent
read-only tier. This is a design constraint, not a default.

| Board | Tier | Constraint |
|-------|------|------------|
| `.NET` | `static-review` | Reads source and sanitized configuration; never runs `dotnet build`, `dotnet test`, `dotnet ef`, or contacts a live system |
| `legal` | `static-review` | Reads sanitized excerpts; never contacts regulators, triggers legal systems, or makes binding legal determinations |
| `hr` | `static-review` | Reads sanitized excerpts; never terminates, disciplines, denies leave or accommodation, or sends employee communications |
| `marketing` | `static-review` (specialists) / `read-only-runtime` (maestro) | Reads sanitized configuration and evidence; never mutates CMP, tag-manager, or ad-platform state |

New boards contributed to this repository must follow the same posture. An agent
that builds, runs, mutates, or contacts a live system is not a language/stack
board agent — it belongs on a provider board with an appropriate
`execution_tier` and per-session opt-in controls.

No language/stack board agent:

- Runs, compiles, or deploys code
- Contacts an external API, database, or live service
- Makes a binding legal or HR determination
- Stores or echoes secrets, credentials, tokens, or personal data

---

## Cross-references

- [taxonomy.md](taxonomy.md) — provider enum, asset types, skill categories,
  trust levels, and lifecycle values; includes the canonical taxonomy note for
  language/stack boards
- [marketplace-model.md](marketplace-model.md) — how the catalog, schemas, and
  validation gates fit together
- [compatibility.md](compatibility.md) — harness support contract and adapter
  format requirements
- `CONTRIBUTING.md` — step-by-step guide for adding agents and skills, including
  required metadata fields and catalog refresh commands
- `agents/dotnet/README.md` — reference implementation for the board README
  format and tier table
- `docs/architecture/legal-hr-agent-routing.md` — cross-domain routing between
  the `legal` and `hr` boards
