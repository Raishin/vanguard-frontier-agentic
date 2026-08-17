# The Databricks Board

> **Status:** Cloud-neutral board covering 17 static-review agents across data engineering, analytics, AI/ML, platform governance, and reliability. All agents carry `execution_tier: static-review` and no execution tools. Mutation is reachable only through a pre-existing live guard (`databricks-live-unity-catalog-grant-guard-at-azure-agent`) for Unity Catalog grants, which is never auto-dispatched. Lifecycle: experimental.

---

## Why this board exists

Databricks has been the lakehouse platform of choice for enterprises seeking to consolidate analytics and AI operations on a single infrastructure. Yet choosing Databricks correctly—and using it safely—requires expertise across architecture, governance, data engineering, analytics, platform operations, and business value realization. The platform's scope (multi-workspace accounts, Unity Catalog, streaming, ML lifecycle, serverless compute, Delta Lake, AI agents) crosses multiple specialized domains; asking a single generalist to master all of them leads to either shallow reviews or unsafe assumptions.

The Databricks board exists to partition this surface into 17 specialist agents, each owning a critical decision boundary and none claiming expertise outside its scope. A maestro agent classifies incoming Databricks tasks and routes them to the narrowest specialist(s) that can answer; this prevents both bottlenecking on a single expert and the diffuse accountability of consensus-based review.

Three design principles guide the board:

1. **Static review only.** No agent mutates a workspace, executes a query, or invokes an API. All recommendations require explicit human judgment and approval before reaching production.

2. **Cloud-neutral scope.** The board owns platform design, governance, engineering, and operations on Databricks regardless of cloud. Azure-specific deployment detail (Entra ID federation, ADLS Gen2 wiring, Access Connector, VNet injection) belongs to hand-authored Azure agents; the board does not duplicate that expertise.

3. **Evidence-driven judgment.** Every finding rests on either the user's supplied configuration/data or on official Databricks documentation verified at the time of review. Industry benchmarks, vendor claims, and memory are never substituted for primary evidence.

---

## Cloud-neutral vs Azure-specific

The Databricks ecosystem has two agent sets: this cloud-neutral board (17 agents) and three hand-authored Azure-specific agents that predate it.

**Cloud-neutral agents** (this board):
- Own platform design, governance, data engineering, analytics, and operations on Databricks, independent of cloud.
- Examples: workspace topology, Unity Catalog privilege design, Lakeflow pipeline structure, SQL performance, MLflow model lifecycle, cost attribution, value realization.
- Serve AWS, Azure, GCP, and other clouds running Databricks.

**Azure-specific agents** (pre-existing):
- `databricks-unity-catalog-governance-at-azure-agent` — Azure Entra ID federation specifics, ADLS Gen2 external location wiring via managed identity, Access Connector configuration.
- `databricks-lakehouse-engineering-at-azure-agent` — Azure VNet integration, Private Link, cluster networking tied to Azure infrastructure.
- `databricks-live-unity-catalog-grant-guard-at-azure-agent` — Live-guard gate for production Unity Catalog GRANT/REVOKE operations on Azure, never auto-dispatched.

**The discriminator:** If the question involves Microsoft Entra ID federation, ADLS Gen2 storage-access wiring, Azure Access Connector managed identity, or VNet-level network isolation, route to the Azure agents. If the question is about the Databricks platform surface independent of Azure infrastructure, stay on this board.

This coexistence means that an Azure Databricks deployment can use both sets: the cloud-neutral board for architecture and governance design, the Azure agents for deployment-specific networking and identity wiring.

---

## The board

| Agent | Skill Category | Owns (one line) |
|-------|---|---|
| **Control plane** | | |
| databricks-maestro-agent | architecture | Classifies Databricks task on seven axes; routes to single specialist, parallel team (max 4), unclassified request, or live-guard gate. Never answers directly. |
| **Platform & governance** | | |
| databricks-platform-architecture-agent | architecture | Workspace topology, metastore-per-region constraint, workspace segmentation ratios, serverless vs classic placement, cross-region access patterns, quota headroom. |
| databricks-unity-catalog-governance-agent | security | Three-level namespace, GRANT inheritance and privilege cascades, ownership design (single principal per securable), workspace-catalog binding ISOLATED mode, governed tags, storage credentials, audit completeness. |
| databricks-identity-network-security-agent | security | Account/workspace/metastore admin separation, SCIM and identity federation (10K user cap, 5K group cap), service-principal posture, OAuth vs PAT, token lifecycle and 90-day inactivity revocation, IP access lists, serverless network egress policies, secret scopes. |
| databricks-data-protection-privacy-agent | compliance | Row filters and column masks (UDF cost implications), ABAC policies, AI-driven data classification (backfill disabled by default), DELETE/MERGE/VACUUM/REORG coordination for GDPR erasure, Delta Sharing egress cost, data residency and Geo processing, customer-managed encryption (Enterprise tier only). |
| **Data engineering** | | |
| databricks-lakeflow-pipeline-engineering-agent | data | Lakeflow Spark Declarative Pipelines medallion layering, Lakeflow Jobs orchestration and task dependencies, Delta table layout (liquid clustering, deletion vectors DBR 15.4+, Predictive Optimization), Auto Loader choice and schema evolution, materialized view vs streaming table, backfill safety. |
| databricks-streaming-reliability-agent | data | Structured Streaming checkpoint compatibility, state-schema immutability (breaking changes on add/delete/type-change), watermark semantics and late-data thresholds, trigger selection (AvailableNow recommended, Once deprecated), exactly-once vs at-least-once sinks, foreachBatch idempotency, RocksDB state store, restart/backfill safety. |
| databricks-data-quality-observability-agent | observability | Lakeflow expectations and violation modes (warn/drop/fail), table constraints (NOT NULL/CHECK enforced; primary/foreign/unique informational), Lakehouse Monitoring profile/drift metrics, freshness anomaly detection and staleness model, pipeline event-log interrogation, quality SLA definition and alerting, downstream quality signaling. |
| **Analytics** | | |
| databricks-sql-performance-agent | data | SQL warehouse type and sizing for concurrency, Photon and Predictive I/O applicability (serverless/pro, not classic), three-tier caching (UI, remote result, local disk) and cache staleness, query-profile reading for skew/spill, data layout (liquid clustering preferred, data-skipping limits), materialized-view refresh semantics. |
| databricks-ai-bi-genie-agent | ai | Genie agent scoping (30-table limit), instructions and trusted-asset caching, metric-view correctness (measures, dimensions, parameters PUBLIC PREVIEW), dashboard limits (15 pages, 100 datasets, 100 widgets, 10K row rendering), benchmark design and LLM-judge confidence (88.1% ± 5.5%), 'Individual data' vs 'Share data' permission (critical: Share bypasses row-level security). |
| **AI / ML** | | |
| databricks-mlops-agent | ml | MLflow 3 with Unity Catalog as native registry, three-level model/feature namespace, alias-based promotion (Champion/Challenger), FeatureEngineeringClient for point-in-time correctness, Model Serving endpoint configuration (traffic splits, scale-to-zero), inference-table auto-logging (at-least-once), batch inference with `ai_query()`. |
| databricks-genai-agent-engineering-agent | ai | Mosaic AI Agent Framework and ResponsesAgent interface, Databricks AI Search index variant and sync-mode choice, retrieval and context engineering (chunking, grounding, context budget), Unity Catalog functions as governed tools, MCP server category (managed/external/custom) and trust boundaries, external model provider selection, Unity AI Gateway guardrails and traffic policy. |
| databricks-genai-evaluation-observability-agent | ai | MLflow Tracing instrumentation and span design, trace storage (experiment vs Unity Catalog Delta tables), `mlflow.genai.evaluate()` harness, built-in judge selection (ten single-turn, seven multi-turn), custom scorers, evaluation dataset construction, regression detection between releases, human feedback loops, cost/latency observability. |
| **Platform operations** | | |
| databricks-developer-platform-agent | architecture | Declarative Automation Bundles structure, deployment-mode design (development vs production), run-as identity boundaries and non-admin limitations, bundle variables (deployment-time-only resolution), CLI authentication (OAuth posture), Terraform vs direct deployment, Git folder flows, CI/CD gate design for safe promotion. |
| databricks-platform-reliability-agent | reliability | Operational evidence from system tables (compute, jobs, pipelines, billing, audit), job and pipeline run reliability (timeouts per retry, continuous jobs no-retry), cluster policies as guardrails, instance-pool minimum-idle cost floor, quota and rate-limit headroom (12K concurrent tasks, 10K jobs/hour), managed disaster recovery (RPO/RTO discipline), incident-evidence preservation. |
| databricks-finops-cost-agent | ml | Cost and billing via system.billing.usage/list_prices, custom-tag-based attribution with coverage reporting, DBU uptime semantics (charge by uptime not execution), serverless vs classic comparison validity, budgets and non-enforcing nature, compute policies and idle settings as cost controls, identifying expensive workloads. |
| **Business outcome** | | |
| databricks-value-realization-agent | ml | Measurability of claimed business outcomes: pain, executive owner, pre-change baseline, leading metric, lagging KPI, required data, attribution limits, measurement window, kill condition, post-measurement. Refuses to price any outcome without a baseline. |

---

## Routing

The maestro classifies on seven axes:

1. **User intent** — Is this a design decision, a diagnosis, a review, a cost question, or a request to mutate production state?
2. **Business context** — Platform-wide strategy, workload-specific optimization, incident response, or compliance/audit?
3. **Artifact type** — The specific thing available (SQL, notebook, `databricks.yml`, job JSON, query profile, system-table output, dashboard, model code).
4. **Blast radius / risk** — Affects one table, one workspace, one account, or the entire organization? Can fail silently or fail loudly?
5. **Required evidence** — What data does the specialist need to answer this? (Job definition, schema, query profile, cluster policy, system-table snapshot.)
6. **Implied runtime authority** — T0 (static review), T1 (design approval), T2 (configuration change, test), T3 (production mutation).
7. **Specialist ownership** — Which 15 domain specialists actually owns this boundary?

Any task implying T2+ authority leaves the routing table and enters the live-guard gate, requiring explicit written human approval naming the exact target, principal, operation, and rollback owner.

**Worked example 1:** "Our Databricks bill spiked last month. What happened?"
- Intent: Diagnosis
- Context: Workload (cost spike is specific to this account, not a platform-wide issue)
- Artifact: None yet (ask for system.billing.usage slice or a time window)
- Blast radius: Account
- Evidence: system.billing.usage, job and cluster inventory, compute policies
- Authority: T0 (static review of billing and config)
- Route: **Single** → `databricks-finops-cost-agent` (owns cost attribution and system-table evidence). If the spike traces to warehouse idle time, follow-on route to `databricks-platform-reliability-agent` for cluster-policy review; if it traces to query volume, follow-on route to `databricks-sql-performance-agent` for optimization.

**Worked example 2:** "Give the analytics team SELECT on all production data in the `prod` catalog."
- Intent: Access-control change (mutation)
- Context: Governance (privilege is a boundary)
- Artifact: Privilege scope (broad GRANT on catalog, not narrowed to schema or table)
- Blast radius: Catalog-level (affects all tables and future tables)
- Evidence: Current privilege assignments, desired analyst list, data-sensitivity classification
- Authority: T3 (mutation to production registry)
- Route: **Parallel (2), conflict named** → (1) `databricks-unity-catalog-governance-agent` for privilege-design review and recommendation (likely narrower than catalog-wide), (2) `databricks-data-protection-privacy-agent` for whether the data classification justifies such broad access. The specialists resolve the conflict (governance says "too broad," privacy says "these columns are public"). No execution without their consensus and explicit written approval naming the exact GRANT, principal, and rollback owner. The actual GRANT goes through the live-guard gate only.

---

## Role bundles

Each role curates a subset of the 17 agents suited to a specific practitioner function. Roles are not "install the whole board" — they are designed for a focused career path.

| Role ID | Label | Agents | Focus |
|---|---|---|---|
| `databricks-platform-engineer` | Databricks Platform Engineer | 6 | Account and workspace topology, identity and network boundary, Declarative Automation Bundles and CI/CD, operational reliability from system tables, platform cost control. |
| `databricks-solution-architect` | Databricks Solution Architect | 6 | End-to-end solution design: platform topology and segmentation, Unity Catalog governance model, Lakeflow pipeline and medallion design, ML and model lifecycle, value case with baseline, named owner, attribution limits, kill condition. |
| `databricks-data-engineer` | Databricks Data Engineer | 5 | Lakeflow Spark Declarative Pipelines, Delta table layout, Structured Streaming correctness and recovery, data-quality expectations and monitoring, SQL performance of produced tables. |
| `databricks-analytics-engineer` | Databricks Analytics Engineer | 5 | SQL warehouse performance, AI/BI dashboards and Genie agents grounded in metric views, data-quality evidence, pipeline design for analytics serving tables. |
| `databricks-governance-security-engineer` | Databricks Governance and Security Engineer | 5 | Unity Catalog namespace and privilege design, identity federation and admin separation, network and secret boundaries, row filters and column masks, data classification and sharing egress, residency and platform topology. |
| `databricks-ml-engineer` | Databricks ML Engineer | 5 | MLflow 3 and models in Unity Catalog, alias-based promotion, feature engineering and point-in-time correctness, Model Serving endpoints, evaluation and tracing, bundle-driven promotion, data-quality dependencies. |
| `databricks-genai-engineer` | Databricks GenAI Engineer | 5 | Mosaic AI Agent Framework authoring, Databricks AI Search and retrieval, Unity Catalog functions and MCP tools, Unity AI Gateway policy, MLflow Tracing and GenAI evaluation, natural-language BI over governed data. |
| `databricks-sre` | Databricks Site Reliability Engineer | 5 | System-table operational evidence, job and pipeline run reliability, cluster policies and compute guardrails, quota headroom, managed disaster recovery and failover discipline, streaming recovery semantics, deployment safety, warehouse performance under load. |
| `databricks-finops-engineer` | Databricks FinOps Engineer | 5 | Billing system-table evidence and correct price joins, tag attribution coverage, DBU uptime semantics, compute policies and cost controls, query/workload tuning for cost, business outcome measurement. |

---

## Safety posture

All 17 agents carry `execution_tier: static-review` and declare no execution tools. This is enforced by `tests/validate-agent-tool-tiers.py`: any agent in the Databricks board attempting to declare a mutation tool, SQL execution capability, or API invocation is rejected at validation time.

Mutation is reachable only through a pre-existing live guard:

- **`databricks-live-unity-catalog-grant-guard-at-azure-agent`** — The only live agent on the Databricks board, and only for Unity Catalog GRANT/REVOKE operations on Azure. This agent is never auto-dispatched and never appears in a routing table outcome. It is reachable only through the live-guard gate, which requires:
  1. A written request naming the exact target securable (catalog, schema, table, column), the exact principal (user, group, service principal), and the exact privilege (SELECT, MANAGE, etc.).
  2. Explicit human approval by a named approver with account-admin authority.
  3. A documented rollback owner and rollback procedure (the reverse REVOKE statement or the restore point).
  4. A post-execution verification step (confirmation that the GRANT took effect and downstream systems still function).

The live-guard gate is validated by `tests/validate-maestro-routing.py`: any non-gate routing mode that attempts to dispatch a live-guard agent fails validation and blocks release.

---

## Evidence discipline

Every claim in this board is grounded in one of four sources, listed in order of preference:

1. **Official Databricks documentation** — Current at the time of review. Service docs describe feature availability; library docs pin call signatures. Both were consulted for claims about Databricks API, CLI, platform behaviour, and Terraform provider support.

2. **User-supplied configuration** — The user's workspace topology, bundle YAML, cluster policy, monitoring configuration, query profile, cost data from `system.billing.usage`, or job definition. These are treated as primary evidence and never assumed.

3. **Primary claim verification** — Four naming claims were verified verbatim against official sources because product marketing and community blogs frequently conflate names and features:
   - "Databricks AI Search" was formerly marketed as "Vector Search" (name confirmed current as of 2025).
   - "Lakeflow Spark Declarative Pipelines" was formerly marketed as "Delta Live Tables" (DLT); the product is the same, the name changed in 2025 (name confirmed current).
   - "Unity AI Gateway" is the correct name for what was sometimes called "AI Gateway" in early documentation (name confirmed current).
   - "Lakeflow Jobs" is the current name for what was "Workflows" in legacy documentation (name confirmed current).

4. **API library documentation** — MLflow client library, Databricks SDK, and Terraform provider documentation were cross-checked against official Databricks service docs when there was potential divergence between feature descriptions (service docs) and call signatures (library docs). MLflow client API signatures were additionally verified via Context7 because library docs are the authoritative source for SDK behaviour and service docs sometimes describe the feature at a higher level.

All 17 agents carry a documented security_notes field stating that they never request credentials, customer data, or workspace URLs; never execute SQL or API calls; and never access live systems. Every agent defaults to static review and escalates any mutation request to the live-guard gate or to the appropriate specialist.

---

## Regenerating the board

When the agent generator (`scripts/gen_databricks_agents.py`) produces new agents or modifies existing ones, the following sequence brings the catalog, fixtures, and documentation into alignment:

```bash
python3 scripts/gen_databricks_agents.py
python3 scripts/update-catalog-new-agents.py --provider databricks
npm run manifest:write:all
npm run docs-data:write
npm run model-policy:apply
npm run asset-integrity:write
npm run validate
```

**Step-by-step:**

1. **`python3 scripts/gen_databricks_agents.py`** — Regenerates agent metadata files from the 17 data files in `scripts/databricks_data/agents/`. Never touches the three pre-existing Azure agents.

2. **`python3 scripts/update-catalog-new-agents.py --provider databricks`** — Adds newly generated agents to `catalog/agents.json`, updating the full catalog metadata.

3. **`npm run manifest:write:all`** — Regenerates all derived manifests in parallel: `catalog/agent-manifest.json`, `catalog/skill-manifest.json`, `.claude-plugin/plugin.json`, `.cursor-plugin/plugin.json`, etc.

4. **`npm run docs-data:write`** — Regenerates `docs/_data/catalog.yml` with updated agent, skill, and provider counts (used by Liquid templates in Jekyll).

5. **`npm run model-policy:apply`** — Projects model and reasoning-effort policy from `catalog/model-policy.json` into agent `.agent.md` frontmatter (model field) and Codex `codex.toml` files (if applicable).

6. **`npm run asset-integrity:write`** — Refreshes SHA256 hashes in `catalog/asset-integrity.json` for all agents, plugins, and root files. **This must run last and on its own** because the earlier steps write new files and the manifest must be stable before hashing.

7. **`npm run validate`** — Runs 20+ validation gates (schema, catalog consistency, asset integrity, model policy, maestro routing, marketplace consistency). Zero failures required before merge.

**Important caveat:** `npm run maestro-routing:write` regenerates fixtures for ALL provider maestros, not just Databricks. After running it, scope the commit back to `tests/fixtures/databricks-maestro-routing/` only, to avoid polluting the PR with unintended changes to AWS, Azure, or GCP routing fixtures:

```bash
# Run the full fixture regeneration (this touches all providers)
npm run maestro-routing:write

# Revert changes to other providers, keep only databricks changes
git checkout tests/fixtures/aws-maestro-routing/
git checkout tests/fixtures/azure-maestro-routing/
git checkout tests/fixtures/gcp-maestro-routing/
# (etc. for other providers)

# Now the staged changes are only databricks-maestro-routing
git add tests/fixtures/databricks-maestro-routing/
git commit -m "chore(databricks-board): regenerate maestro routing fixtures"
```

After regeneration, always run `npm run validate` to catch any schema violations, routing inconsistencies, or policy mismatches before pushing.
