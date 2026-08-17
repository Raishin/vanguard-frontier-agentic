# ❄️ Snowflake Agents

## Overview

A **28-agent Snowflake board**: one router, 18 review specialists, six approval-gated live guards,
and three deprecated Azure-scoped predecessors kept installable for continuity. Every review agent
is read-only. Every mutation passes a human who has seen the blast radius and the rollback.

The board is not a set of Snowflake-themed prompts. It is a decision system with an opinion about
who owns which failure domain, what evidence a decision requires, who may veto an unsafe
assumption, when a human must approve execution, how a change is verified, how it is reversed, and
how the work creates measurable business value.

**Entry point:** `snowflake-maestro-agent`. It classifies the task, names the failure domains,
decides whether account-specific evidence is even required, and dispatches the narrowest specialist
or at most four in parallel. It never answers a Snowflake question itself.

Navigation compass: [`AGENTS.md`](AGENTS.md).

---

## 🧱 Board philosophy

**Four things that share vocabulary are kept separate.** A human persona ("our data engineer
needs…"), a Snowflake authorization role ("SECURITYADMIN can…"), an agent's responsibility (which
failure domain it owns), and an agent's runtime privilege (whether it may execute anything) are
independent. No agent's mandate is inferred from the name of a Snowflake RBAC role. That
`SECURITYADMIN` *can* manage grants does not mean a security agent should execute them; that
`ACCOUNTADMIN` *can* perform an operation never justifies assigning it to automation.

**Disagreement is a feature.** Performance wants a bigger warehouse; FinOps refuses unless the SLA
gain justifies the credits. Product wants outbound connectivity; security refuses unrestricted
egress. The architect wants Business Critical; the value strategist asks what outage risk that
premium actually removes. The maestro reports the point of disagreement, each side's evidence, the
business impact, the risk, the decision owner, and a recommended resolution — it never averages two
positions into a false consensus.

**An agent that cannot name the business loss it prevents does not belong on the board.** Every
specialist carries a Business Impact section stating the loss prevented, the outcome improved, and
the metrics that would show it.

---

## 🗂️ Agent categories

| Category | Agents |
|---|---|
| **Router** | `snowflake-maestro-agent` |
| **Architecture** | `snowflake-solution-architect-agent`, `snowflake-migration-modernization-agent` |
| **Administration** | `snowflake-platform-administrator-agent` |
| **Security** | `snowflake-identity-access-security-agent`, `snowflake-network-private-connectivity-agent` |
| **Governance** | `snowflake-governance-privacy-agent` |
| **Compliance** | `snowflake-compliance-evidence-auditor-agent` |
| **FinOps** | `snowflake-finops-cost-governor-agent` |
| **Performance** | `snowflake-query-performance-engineer-agent` |
| **Data engineering** | `snowflake-data-engineering-pipelines-agent` |
| **Streaming** | `snowflake-streaming-ingestion-reliability-agent` |
| **Analytics** | `snowflake-analytics-semantic-data-product-agent` |
| **ML** | `snowflake-data-science-ml-agent` |
| **AI / Cortex** | `snowflake-cortex-ai-agent-security-governor-agent` |
| **Native Apps / Marketplace** | `snowflake-native-app-marketplace-product-agent` |
| **BCDR** | `snowflake-bcdr-resilience-agent` |
| **DevOps / IaC** | `snowflake-devops-iac-release-agent` |
| **Business value** | `snowflake-business-value-adoption-strategist-agent` |
| **Live guards (6)** | RBAC grant · auth/network policy · warehouse & cost · data-protection policy · pipeline & streaming · failover promotion |

Per-agent load triggers, tiers, and mutation capability: [`AGENTS.md`](AGENTS.md).

---

## 🔍 Review versus live guard

| | Review specialist | Live guard |
|---|---|---|
| Execution tier | `static-review` | `mutating-runtime` |
| Reads | sanitized DDL, config, IaC, and account evidence extracts | the same, plus prior-state capture |
| Writes | nothing | exactly one approved mutation |
| Dispatch | by the maestro, freely | **never automatically** — only after explicit written human approval |
| Deliverable | findings with evidence labels, plus the exact proposed statement | the executed statement, its verification, its negative check, its attestation, and its rollback |

A live guard is **not** a smarter review agent. It is a narrowly scoped execution boundary, and it
is never selected merely because a requested change *could* eventually be executed. A mutation
request routes to the review specialist first, in `live-guard-gate` mode; the guard is reached only
after a human has read the blast radius and the rollback and approved in writing.

**Urgency raises the gate rather than lowering it.** "Production is down, fail over now" is exactly
the circumstance in which promoting without dependency readiness converts a regional incident into
a longer, harder, multi-region one.

Even so, no harness adapter grants any agent on this board an execution tool. A live guard's output
is the approved, preflighted statement plus its attestation and rollback; a named human operator
runs it.

---

## 🛡️ Live-system safety

Every live guard requires, before a statement is composed: an exact target; the exact account and
environment; the exact mutation; explicit written human approval; current-state capture; a
deterministic preflight and dry run; a blast-radius analysis; a least-privilege executor identity;
idempotency protection; an audit record; verification; a rollback or failback path; and post-change
evidence.

Standing permission posture, stated in each guard's `PERMISSIONS.md`:

- `ACCOUNTADMIN` — **forbidden without exception.**
- `SECURITYADMIN` / `SYSADMIN` — forbidden unless technically unavoidable and justified in writing
  before first run.
- Executor identity — `TYPE = SERVICE` (or `SERVICE_AGENT`), authenticating by key-pair or workload
  identity federation. Password authentication for a non-human identity is a hard stop.
- Privileges — scoped to the single target the approval names, and returned when a bounded exercise
  closes.

Two guards carry an additional refusal that no approval can override:

- **auth/network policy guard** — will not tighten reachability until a *surviving administrative
  path* is demonstrated from login history: a named principal, a location it has actually connected
  from, and the privilege to execute the inverse. Without that, the rollback would require the
  access the change removes.
- **failover promotion guard** — will not promote without a declared incident or drill, a named
  accountable owner, a data-loss window computed from replication refresh history, dependency
  readiness confirmed by each owning team, and a stated failback strategy.

---

## 🔬 Evidence model

Every material claim carries one label, and the labels are not interchangeable:

| Label | Means |
|---|---|
| `LIVE-EVIDENCE` | Observed in this account — SHOW output, `ACCOUNT_USAGE`, `ORGANIZATION_USAGE`, `INFORMATION_SCHEMA`, Trust Center |
| `REPOSITORY-EVIDENCE` | Read from committed artifacts — DDL, Terraform, connector config. Proves intent, not deployed state |
| `DOCUMENTATION-BASED` | Snowflake documentation establishes platform behaviour. Proves what is supported, never what is configured |
| `STANDARD-BASED` | An external standard establishes the requirement (CIS, NIST, OWASP, FinOps Foundation, Iceberg spec, regulatory text) |
| `INFERENCE` | Reasoned from the above, with the reasoning shown |
| `ESTIMATE` | A number with a stated method and stated error bars |
| `UNKNOWN` | The evidence does not establish it — a valid, expected answer that is never replaced by a confident guess |

The distinction that does the most work: **documentation proves supported platform behaviour; it
never proves configured account behaviour.** An account's edition, region, enabled bundles,
authentication enforcement state, and policy attachments are all `UNKNOWN` until account evidence
establishes them.

---

## 📚 Skill system

Each agent has a dedicated skill under `skills/snowflake/<skill-id>/` using progressive disclosure:
a lean `SKILL.md` carrying purpose, trigger conditions, **when NOT to use**, operating rules, the
evidence model, the decision workflow, escalation, and the response contract — with deep material
in lazy-loaded `references/` files that are read only when a task needs them.

References are domain-specific decision knowledge, not bibliographies. Each carries the claims that
change a decision, the evidence queries that establish them, a time-sensitive table for volatile
facts with verification dates, and a sources section in which every URL states *what that page
proves* and what it does not.

`when NOT to use` is mandatory on every skill. Agent systems degrade because each skill explains
when it applies and none explains when it stops applying.

---

## 📖 Source policy

1. **Tier 1 — primary.** Snowflake documentation, release notes, SQL reference, official Snowflake
   GitHub repositories including the Terraform provider, official SDKs and connectors, and
   cloud-provider documentation where Snowflake depends on that cloud's networking or identity.
2. **Tier 2 — independent standards.** CIS, NIST, OWASP, the FinOps Foundation, the Apache Iceberg
   specification, and applicable regulatory text — used where a standard, not a vendor, defines the
   requirement.
3. **Tier 3 — secondary.** Blogs, forums, and conference material may suggest an operational
   hypothesis. They never establish Snowflake behaviour where primary documentation exists, and any
   conclusion drawn from them is labelled as secondary-sourced.

Volatile claims recorded in a reference carry the claim, its status or constraint, a verification
date, what the source proves, and what it does **not** prove.

---

## 🚦 Routing examples

| Request | Route |
|---|---|
| "This dashboard query takes 30 seconds — make the warehouse 4XL" | `query-performance-engineer` + `finops-cost-governor`, in parallel and expected to disagree |
| "Our bill doubled" | `finops-cost-governor`, decomposing warehouse / idle / serverless / AI / storage / transfer / volume growth before proposing anything |
| "New Cortex agent over customer data" | `cortex-ai-agent-security-governor` + `identity-access-security`, with governance and business value joining where scope demands |
| "Snowpipe Streaming for a new project" | `streaming-ingestion-reliability`, which re-verifies current lifecycle guidance before recommending an architecture |
| "Give our automation ACCOUNTADMIN" | `identity-access-security`, which refuses and returns the least-privilege alternative. No live guard executes |
| "Block all public access now" | `network-private-connectivity` review first; the auth/network guard only after a surviving admin path is proven |
| "Production is down, fail over immediately" | `bcdr-resilience` review first; the failover guard only after a declaration, a computed data-loss window, and confirmed dependency readiness |
| "Replace Databricks entirely with Snowflake" | `migration-modernization` + `business-value-adoption-strategist` — and the board may conclude *do not migrate this workload* |
| "Build it because Snowflake has the feature" | `business-value-adoption-strategist`, which may return **NO-GO: technically valid, economically unjustified** |
| "Can you look at our setup and see if anything's off?" | `unclassified` — the maestro asks for the smallest sufficient evidence set rather than guessing |

Executable form: `tests/fixtures/snowflake-maestro-routing/` (27 scenarios — red team, negative
routing, cross-agent conflict, live-guard gate). Run `npm run validate:maestro-routing`.

---

## ⚠️ Snowflake volatility disclaimer

Snowflake changes quickly, and several facts this board reasons about are volatile by nature:
GA and preview status, deprecations and behaviour-change bundles, SQL syntax, account parameters,
service limits, pricing behaviour, edition/cloud/region availability, Terraform provider resource
stability, driver minimums, authentication enforcement, Cortex capability and model availability,
connector availability, streaming lifecycle guidance, and catalog/interoperability recommendations.

Reference material here records a verification date against a primary source and states what that
source does not prove. **Re-verify any volatile fact against current Snowflake documentation before
encoding it in a long-lived recommendation.** Where sources conflict or appear stale, the board's
instruction is to report `Status: unresolved` with the action to verify — never to supply a
plausible date or status that no primary source establishes.

---

## 📦 Install

```bash
# Whole board
npx vfa-export-agents --platform claude-code --provider snowflake --repo .

# Narrower, role-scoped installs
npx vfa-export-agents --list-roles
npx vfa-export-agents --platform claude-code --role snowflake-security-governance-engineer --repo .
```

Roles: `snowflake-platform-architect`, `snowflake-security-governance-engineer`,
`snowflake-finops-performance-engineer`, `snowflake-data-engineer`,
`snowflake-ai-analytics-engineer`, `snowflake-delivery-resilience-engineer`,
`snowflake-data-product-manager`. The legacy `azure-snowflake-platform-engineer` role installs the
three deprecated Azure-scoped agents.

---

## 🔄 Regenerating this board

Agents and skills here are generated from committed data files; behaviour changes only when the
data changes, and the generator never consults the wall clock.

```bash
python3 scripts/gen_snowflake_agents.py
python3 scripts/update-catalog-new-agents.py --provider snowflake
npm run manifest:write:all && npm run docs-data:write && npm run model-policy:apply
npm run asset-integrity:write      # last, on its own
npm run validate
```

Edit `scripts/snowflake_data/agents/*.json` — never the generated output.
