# Vanguard Frontier Agentic

<div align="center">
  <p><strong>A curated marketplace for cloud and zero-trust AI workflows.</strong></p>

  <p>
    <a href="#get-started">Get Started</a> &nbsp;·&nbsp;
    <a href="#skills">Skills</a> &nbsp;·&nbsp;
    <a href="#agents">Agents</a> &nbsp;·&nbsp;
    <a href="#cli-commands">Commands</a> &nbsp;·&nbsp;
    <a href="https://github.com/Raishin/vanguard-frontier-agentic/issues">Issues</a> &nbsp;·&nbsp;
    <a href="#faq">FAQ</a> &nbsp;·&nbsp;
    <a href="#feedback">Feedback</a>
  </p>
</div>

---

This repo collects reusable **skills**, **agents**, **rules**, **MCP references**,
and supporting assets for engineers working with AWS, Azure, OCI, GCP,
Kubernetes, Terraform, cloud security, and compliance-heavy architecture.

- 🧠 **Skills** = step-by-step workflows an AI assistant can follow.
- 🤖 **Agents** = reusable expert roles for review, architecture, and operations.
- 📏 **Rules** = durable instructions for a specific AI harness.
- 🔌 **MCP references** = trusted notes for connecting tools to real systems.
- 🗂️ **Catalogs** = machine-readable indexes so tools can discover everything.

**Works with:**
[Claude Code](https://docs.anthropic.com/en/docs/claude-code) &nbsp;·&nbsp;
[Codex](https://github.com/openai/codex) &nbsp;·&nbsp;
[GitHub Copilot](https://github.com/features/copilot) &nbsp;·&nbsp;
[Cursor](https://www.cursor.com/) &nbsp;·&nbsp;
[Gemini CLI](https://github.com/google-gemini/gemini-cli) &nbsp;·&nbsp;
[Kiro](https://kiro.dev/) &nbsp;·&nbsp;
*and any other coding agent.*

> 📦 **Available on npm:** `@raishin/vanguard-frontier-agentic` is published on the public npm registry.

---

## Get Started

**Prerequisites:** [Node.js](https://nodejs.org/) 18+ (for the exporter CLI).

### 1. Install from npm

```bash
npm install @raishin/vanguard-frontier-agentic
```

Or pin to the latest release:

```bash
npm install @raishin/vanguard-frontier-agentic@latest
```

### 2. Open your coding agent

Launch [Claude Code](https://docs.anthropic.com/en/docs/claude-code), [Gemini CLI](https://github.com/google-gemini/gemini-cli), [Codex](https://github.com/openai/codex), or any coding agent you prefer.

### 3. Export agents into your repository

List available agent IDs:

```bash
npx vfa-export-agents --list
```

Export by role (install all agents for your job function at once):

```bash
# Install all cloud-security-engineer agents for Claude Code
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --repo /path/to/your-repo

# Install only OCI agents for a cloud-platform-engineer
npx vfa-export-agents --platform codex --role cloud-platform-engineer --provider oci --repo /path/to/your-repo
```

Or export specific agents:

```bash
# Claude Code
npx vfa-export-agents --platform claude-code --agents azure-live-aks-rollout-guard-agent --repo /path/to/your-repo

# GitHub Copilot
npx vfa-export-agents --platform copilot --agents azure-live-aks-rollout-guard-agent --repo /path/to/your-repo

# Kiro (writes both IDE + CLI adapters)
npx vfa-export-agents --platform kiro --agents azure-live-aks-rollout-guard-agent --repo /path/to/your-repo

# Export everything for a platform
npx vfa-export-agents --platform codex --all --repo /path/to/your-repo
```

### 4. Use the skill or agent

Inside your coding agent session, reference the skill directly or let the exported agent guide you:

```text
Use the azure-live-aks-rollout-guard skill to audit my deployment rollout before I proceed.
```

---

## Skills

**115 skills** across AWS, Azure, OCI, Kubernetes, Terraform, and more.

| Domain | Count | What they cover |
|--------|------:|----------------|
| AWS | 43 | IAM, EKS, ECS, Lambda, RDS, S3, Cost, DevOps, Bedrock, Security, Live Guards |
| Azure | 32 | AKS, App Service, ARM/Bicep, Key Vault, PIM, Cost, Entra ID, CosmosDB, Live Guards |
| OCI | 37 | ADB, OKE, IAM, Vault, Resource Manager, Cost, Networking, Live Guards |
| Kubernetes | 2 | RBAC review, live RBAC mutation guard |
| Terraform | 1 | IaC review and plan safety |

### Live Guard skills (high-risk cloud mutations)

Live-guard skills enforce approval gates and rollback posture for irreversible operations:

**Azure (7):**
- `azure-live-aks-rollout-guard` — PDB audit, rollout pause/undo, post-rollout health
- `azure-live-arm-deployment-stack-guard` — what-if evidence, denySettings, PIM-gated delete
- `azure-live-app-service-slot-swap-guard` — sticky-setting audit, traffic shifting, swap-back path
- `azure-live-keyvault-rotation-purge-guard` — rotation policy, soft-delete/purge-protection, PIM gate
- `azure-live-pim-jit-activation-guard` — eligible assignment audit, MFA gate, JIT revocation
- `azure-live-cost-budget-action-guard` — budget mutation, GPU SKU policy, quota read-only
- `azure-live-entra-role-assignment-guard` — permanent role assignment scope/principal audit, PIM-preference enforcement, Guest principal blocking

**OCI (7):**
- `oci-live-autonomous-db-lifecycle-guard` — ADB scale/stop/clone/terminate with tag enforcement
- `oci-live-oke-rollout-guard` — DevOps pipeline approval, PDB audit, rollout pause/undo
- `oci-live-resource-manager-stack-guard` — plan-before-apply, drift detection, job-lock enforcement
- `oci-live-vault-key-destruction-guard` — rotation vs. destruction separation, 7–30 day deletion window
- `oci-live-iam-policy-compartment-guard` — MFA break-glass, dual-approval for tenancy-root changes
- `oci-live-cost-budget-runaway-guard` — 3-tier budget management, GPU shape gate, ONS alert routing
- `oci-live-network-security-rule-guard` — Security List/NSG rule capture, 0.0.0.0/0 detection, DB-subnet criticality, Path Analyzer gate

**Kubernetes (1):**
- `kubernetes-live-rbac-mutation-guard` — escalate/bind/impersonate verb detection, wildcard blocking, pre-mutation state capture, rollback via YAML backup

### Sample skills

- 🔐 [`skills/aws/aws-iam-least-privilege-review`](skills/aws/aws-iam-least-privilege-review/) — Review AWS IAM policies and reduce unnecessary access.
- 🟦 [`skills/azure/azure-rbac-review`](skills/azure/azure-rbac-review/) — Review Azure RBAC assignments, scopes, and custom roles.
- 🟥 [`skills/oci/oci-autonomous-database-architect`](skills/oci/oci-autonomous-database-architect/) — Design and review Oracle Autonomous Database across OCI and multicloud options.
- 💰 [`skills/finops/finops-cloud-price-advisor`](skills/finops/finops-cloud-price-advisor/) — Fetch live prices from AWS, Azure, and OCI public pricing APIs; estimate costs for live environments or prototypes.

Rule of thumb: if the asset teaches **how to do a repeatable task**, it is a skill.

---

## Agents

**115 agents** matching the skill catalog — each agent ships 7 harness adapters and a hardened permission model.

| Provider | Count | Specialisations |
|----------|------:|----------------|
| AWS | 43 | advisory, execution, live-guard operators |
| Azure | 32 | advisory, live-guard operators |
| OCI | 35 | advisory, live-guard operators |
| Kubernetes | 2 | RBAC review, live RBAC mutation guard |
| Multi-cloud | 1 | FinOps Cloud Price Advisor |
| Terraform | 2 | IaC review, maestro |

Every agent ships:
- `AGENT.md` — harness-neutral contract with guarded response shape
- `metadata.json` — schema-validated catalog entry
- 7 harness adapters — claude-code, codex, copilot, cursor, gemini, kiro-ide, kiro-cli

```text
agents/
├── aws/          (43 agents)
├── azure/        (32 agents)
├── finops/       (1 agent — cross-cloud price advisor)
├── kubernetes/   (2 agents)
├── oci/          (35 agents)
└── terraform/    (2 agents)
```

Example:

- 🧱 [`agents/terraform/terraform-reviewer`](agents/terraform/terraform-reviewer/) — Review Terraform modules, plans, provider usage, and state assumptions.

Use an agent when you need a **role with judgment**, not just a checklist.

---

## CLI Commands

The `vfa-export-agents` CLI ships with this package.

| Command | What it does |
|---------|-------------|
| `vfa-export-agents --list` | List all available agent IDs |
| `vfa-export-agents --list-roles` | List available role IDs with agent counts |
| `vfa-export-agents --platform <p> --agents <id> --repo <path>` | Export one agent to a platform |
| `vfa-export-agents --platform <p> --role <role> --repo <path>` | Export all agents for a role |
| `vfa-export-agents --platform <p> --role <role> --provider <p> --repo <path>` | Export role agents filtered to one provider |
| `vfa-export-agents --platform <p> --all --repo <path>` | Export all agents for a platform |
| `vfa-export-agents --platform <p> --all --repo <path> --force` | Overwrite existing exported files |

<details>
<summary>Supported platforms and destination paths</summary>

| Platform flag | Destination in consumer repo |
|---------------|------------------------------|
| `codex` | `.codex/agents/` |
| `claude-code` | `.claude/agents/` |
| `copilot` | `.github/agents/` |
| `cursor` | `.cursor/agents/` |
| `gemini` | `.gemini/agents/` |
| `kiro` | `.kiro/agents/` |

</details>

**Important:** the exporter installs custom agent files only — not repo-level guidance layers (`AGENTS.md`, `CLAUDE.md`, `.github/copilot-instructions.md`, etc.). See [`docs/normalized-platform-matrix.md`](docs/normalized-platform-matrix.md) for the distinction.

---

## Role-Based Install

`catalog/install-roles.json` maps six engineering roles to the agents they need, across all supported cloud providers.

| Role ID | Label | Coverage |
|---------|-------|---------|
| `cloud-security-engineer` | Cloud Security Engineer | IAM/RBAC review, secrets lifecycle, identity governance, live guards for access mutations |
| `cloud-platform-engineer` | Cloud Platform Engineer | IaC safety review, container platforms, networking, landing zones, live deployment guards |
| `cloud-dba` | Cloud Database Administrator | RDS/Aurora, DynamoDB, CosmosDB, OCI Autonomous/Exadata/MySQL, live DB lifecycle guards |
| `cloud-finops-analyst` | Cloud FinOps Analyst | Cost optimization, anomaly watch, budget runaway guards, capacity planning |
| `cloud-solutions-architect` | Cloud Solutions Architect | Solution architecture, migration cutover, resilience/BCDR, event-driven, AI/generative |
| `cloud-devops-engineer` | Cloud DevOps Engineer | CI/CD, pipeline approval gates, live rollout guards, serverless, observability |

### Install by role

```bash
# Export all cloud-security-engineer agents for Claude Code
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --repo .

# Export only Azure agents for a cloud-platform-engineer
npx vfa-export-agents --platform codex --role cloud-platform-engineer --provider azure --repo .

# List what roles are available
npx vfa-export-agents --list-roles
```

### Pipeline enforcement

Install by role at the CI/CD layer to enforce guardrails without developer opt-in.
See [`docs/ci-cd-enforcement-pattern.md`](docs/ci-cd-enforcement-pattern.md) for GitHub Actions, Azure DevOps, and OCI DevOps templates.

---

## 🌍 Vision

**Build a practical AI workflow marketplace for secure cloud engineering.**

This repository exists for teams that need to design, review, and operate cloud
systems where security and compliance are not optional extras.

The north star:

> 🛡️ **Cloud architecture should be zero-trust by default, evidence-backed by
> design, and understandable by engineers of any seniority.**

That means every serious workflow should help engineers answer:

- 👤 **Who is accessing what?**
- 🔐 **Why are they allowed?**
- 🧾 **Where is the evidence?**
- 🚨 **How do we detect abuse or drift?**
- 🧯 **How do we respond and recover?**
- 📋 **Which compliance obligation does this support?**

---

## 🧬 Philosophy

This repo is opinionated. That is a feature, not a bug.

### 1. 🛡️ Zero trust beats implicit trust

Do not trust a network, cloud account, CI runner, agent, workload, or human just
because it is "inside" something.

Good assets should push for:

- strong identity,
- least privilege,
- explicit authorization,
- segmentation,
- continuous verification,
- logging and detection,
- short-lived credentials where possible,
- safe rollback paths.

### 2. 🧾 Compliance needs evidence, not vibes

SOC 2 Type 2, PCI DSS, NIS2, and NIST-style control frameworks are not passed by
good intentions. They require repeatable controls and evidence over time.

Good assets should produce or point to evidence:

- policy decisions,
- access reviews,
- architecture diagrams,
- ticket approvals,
- logs and alerts,
- backup and restore tests,
- vulnerability and patch records,
- incident response records,
- change history.

### 3. 🔐 Least privilege is the default

If a workflow recommends broad admin access, it must explain why.

If it cannot explain why, it should not recommend it.

### 4. 🧪 Every claim needs a source or a validation path

Cloud behavior changes. Compliance expectations evolve. Vendor services drift.

So assets should clearly separate:

- ✅ verified facts,
- 🧠 engineering judgment,
- ⚠️ assumptions,
- ❓ unknowns.

### 5. 🧯 Automation must have brakes

AI-assisted automation should not become a fast path to production damage.

Dangerous actions need:

- read-only discovery first,
- explicit approval,
- scoped credentials,
- dry-run or plan mode where possible,
- rollback notes,
- post-change validation.

---

## 📋 Compliance compass

This repository is not a compliance product and does not replace auditors,
QSAs, legal counsel, or official standards.

It is a **control-aware engineering toolbox**. The assets should help teams
design and collect evidence for common security expectations across frameworks.

Every live-guard and review agent produces a **structured verdict response** (`verdict`, `evidence_level`, `blockers`, `safe_next_actions`, `open_questions`) that maps directly to SOC 2 CC6.1, PCI DSS Req 7, NIS2 Article 21, NIST CSF PR.AC-4, and ISO 27001 A.9.1.1 — no post-processing required. See [`docs/evidence-output-spec.md`](docs/evidence-output-spec.md) for the full control mapping and evidence retention guidance.

| Framework / standard              | What it pushes us to remember                                                                                                                                     | Repo design implication                                                                                   |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 🔵 **SOC 2 Type 2**               | Controls must operate over a period of time, especially around security, availability, confidentiality, processing integrity, and privacy trust service criteria. | Workflows should leave evidence trails, not just one-time fixes.                                          |
| 💳 **PCI DSS**                    | Cardholder data environments need scoped controls, secure configuration, access control, monitoring, vulnerability management, and testing.                       | Workflows should reduce scope, avoid broad access, and flag payment-data risk.                            |
| 🇪🇺 **NIS2**                       | EU cybersecurity rules emphasize governance, risk management, incident reporting, supply-chain security, and management accountability.                           | Workflows should make ownership, reporting, and supplier/cloud dependencies explicit.                     |
| 🧭 **NIST CSF 2.0**               | Cybersecurity risk management spans Govern, Identify, Protect, Detect, Respond, and Recover.                                                                      | Assets should not stop at prevention; they should include detection, response, and recovery.              |
| 🏛️ **NIST SP 800-207 Zero Trust** | Access should be continuously evaluated and should not rely on implicit network trust.                                                                            | Agents and skills should challenge flat networks, permanent credentials, and unverified trust boundaries. |

Ruthless correction: **NIS2** is the European cybersecurity directive. **NIST**
is a U.S. standards body. If someone says "NIST2 European compliance," they
probably mean **NIS2** or they are mixing two different things.

---

## 🏗️ Architecture principles

Use these principles when creating or reviewing assets:

| Principle                   | What good looks like                                                                  |
| --------------------------- | ------------------------------------------------------------------------------------- |
| 👤 Identity-first           | Humans, workloads, agents, and CI/CD jobs have explicit identities.                   |
| 🔐 Least privilege          | Permissions are narrow, justified, and reviewable.                                    |
| 🧱 Segmented blast radius   | Network, account, project, subscription, tenancy, and data boundaries are deliberate. |
| 🧾 Evidence by design       | The workflow naturally produces logs, approvals, diffs, plans, or reports.            |
| 🔎 Continuous monitoring    | Detection is part of the design, not an afterthought.                                 |
| 🧯 Recoverability           | Backups, restore tests, rollback, and incident response are considered upfront.       |
| 🧭 Source-grounded guidance | Official docs and live state beat memory and assumptions.                             |
| 🤝 Human accountability     | AI can assist, but owners still approve risk.                                         |

---

## 🧭 Quick map

| Folder                     | What lives here                                  | Easy memory hook                       |
| -------------------------- | ------------------------------------------------ | -------------------------------------- |
| [`skills/`](skills/)       | Reusable workflows grouped by provider or domain | 🧠 "How do I do this task?"            |
| [`agents/`](agents/)       | Expert roles grouped by provider or domain       | 🤖 "Who should review this?"           |
| [`rules/`](rules/)         | Harness-specific instructions                    | 📏 "What behavior is always expected?" |
| [`mcp/`](mcp/)             | MCP server references and trust notes            | 🔌 "What can this connect to?"         |
| [`catalog/`](catalog/)     | JSON indexes for marketplace discovery           | 🗂️ "What assets exist?"                |
| [`schemas/`](schemas/)     | Metadata validation contracts                    | ✅ "What fields are required?"         |
| [`templates/`](templates/) | Starter templates for new assets                 | 🧱 "How do I add one?"                 |
| [`docs/`](docs/)           | Quality rules, taxonomy, compliance evidence spec, CI/CD enforcement patterns | 📚 "How should this repo work?"        |
| [`assets/`](assets/)       | Logos and visual assets                          | 🎨 "What images can docs use?"         |

---

## 🔌 MCP references

MCP references describe tool/server integrations and their trust boundaries.

Examples:

- 🟧 [`mcp/official/aws-mcp-servers.md`](mcp/official/aws-mcp-servers.md)
- 🟦 [`mcp/official/azure-mcp-server.md`](mcp/official/azure-mcp-server.md)
- 🟥 [`mcp/official/oracle-mcp-servers.md`](mcp/official/oracle-mcp-servers.md)

Important: MCP tools may read or mutate real infrastructure. Treat them like
production access, not like harmless documentation links.

---

## ✅ Quality bar

This repo is **not** a prompt junk drawer.

Every cataloged asset should be:

- 🔎 **Traceable** — includes official docs or clear provenance.
- 🔐 **Security-aware** — explains access, risk, and least-privilege concerns.
- 🧪 **Validated** — passes repo checks before being shared.
- 🧭 **Scoped** — clearly says which provider, domain, and harness it supports.
- 🧯 **Safe by default** — read-only discovery before mutation; approval before dangerous actions.

Hard no:

- ❌ Secrets or credentials.
- ❌ Vague "do everything" prompts.
- ❌ Unsafe production mutation recipes.
- ❌ Cloud claims with no source or verification path.

For the detailed standard, read [`docs/quality-bar.md`](docs/quality-bar.md).

---

## 🗂️ Metadata contract

Every cataloged asset needs metadata so people and tools can understand it.

Required common fields:

- `id`
- `name`
- `type`: `skill`, `agent`, `rule`, or `mcp-reference`
- `provider`: `aws`, `azure`, `oracle`, `oci`, `gcp`, `kubernetes`,
  `terraform`, `multi-cloud`, or `generic`
- `harnesses`: one or more of `codex`, `copilot`, `claude-code`, `cursor`,
  `gemini`, `kiro`, `other`
- `summary`
- `source_type`: `original`, `adapted`, or `reference-only`
- `official_docs`
- `security_notes`
- `last_verified`
- `path`

---

## 🔏 Skill integrity manifests

Skills are executable guidance. Treat them like supply-chain artifacts.

This repo uses `catalog/skill-manifest.json` to record SHA-256 hashes for every
file under every cataloged skill directory.

After intentional skill edits, regenerate the manifest:

```bash
npm run manifest:write
```

Before release or review, check it:

```bash
npm run manifest:check
```

---

## 🧪 Validate your changes

Before contributing or sharing changes, run:

```bash
npm run validate
```

Equivalent manual commands:

```bash
python tests/validate-catalog.py
python tests/validate-skill-manifest.py
python tests/validate-links.py --offline
```

If validation fails, fix that first. A broken catalog makes the marketplace
harder to trust.

---

## 📦 npm publishing and semantic versioning

Use SemVer: `MAJOR.MINOR.PATCH`.

| Version bump | Use when | Example |
| ------------ | -------- | ------- |
| 🩹 `PATCH`   | Typos, metadata corrections, manifest refresh | `0.1.0` → `0.1.1` |
| ✨ `MINOR`   | New skills, agents, provider folders, optional metadata | `0.1.0` → `0.2.0` |
| 💥 `MAJOR`   | Removed/renamed IDs, moved paths, breaking schema changes | `1.4.2` → `2.0.0` |

Read the full policy in [`docs/release-versioning.md`](docs/release-versioning.md).

---

## 🧑‍💻 How to add a new asset

1. 🧭 Pick the right folder — `skills/<provider>/`, `agents/<provider>/`, `rules/<harness>/`, or `mcp/official/`.
2. 🧱 Start from a template — [`templates/skill-template`](templates/skill-template/) or [`templates/agent-template`](templates/agent-template/).
3. 🗂️ Add or update catalog metadata in the matching `catalog/*.json` file.
4. ✅ Run `npm run validate`.
5. 🧯 Check safety — no secrets, no broad permissions without justification, no destructive actions without approval gates.

---

## ❓ FAQ

**Skills vs agents — what's the difference?**<br>
A **skill** teaches your coding agent *how to do a task* (step-by-step workflow, CLI commands, reference material). An **agent** gives your coding agent a *role with judgment* — it loads the skill and adds a guarded response shape, approval gates, and a hardened permission model.

**Do I need a cloud account to use these?**<br>
For reviewing architecture, writing IaC, or planning — no. For live-guard agents that execute against a real environment — yes, and they will ask you to confirm subscription/tenancy/principal before any mutation.

**Can I use a skill or agent without the exporter CLI?**<br>
Yes. Copy the harness file for your platform from `agents/<provider>/<id>/harnesses/` directly into your repo's agent folder. The CLI just automates that copy.

**What is a "live guard" agent?**<br>
A live-guard agent operates against a real cloud environment. It enforces approval gates before any mutation, requires preflight evidence (what-if/plan/status output), and treats missing rollback design as a stop condition. Live guards are refusal-by-default — if target identity, approval state, or rollback posture is ambiguous, they stop and say so.

**What does the FinOps price advisor actually do?**<br>
It fetches live on-demand prices from AWS Price List API, Azure Retail Prices API, and OCI public pricing API — all public, unauthenticated endpoints. It never needs billing credentials. Currency defaults to USD; other currencies are available via Azure's native `currencyCode` parameter or public exchange rate APIs for AWS/OCI.

**Can I contribute new skills or agents?**<br>
Yes — see [Contributing](#contributing). The baseline requirement: the asset must be specific, source-backed, security-aware, and validated by `npm run validate`.

---

## 📚 Source anchors

Use official sources when writing security or compliance-sensitive assets:

- 🏛️ [NIST SP 800-207 Zero Trust Architecture](https://csrc.nist.gov/pubs/sp/800/207/final)
- 🧭 [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- 🇪🇺 [European Commission: NIS2 Directive](https://digital-strategy.ec.europa.eu/en/policies/nis2-directive)
- 🇪🇺 [ENISA: NIS Directive 2](https://www.enisa.europa.eu/topics/state-of-cybersecurity-in-the-eu/cybersecurity-policies/nis-directive-2)
- 💳 [PCI Security Standards Council Document Library](https://www.pcisecuritystandards.org/document_library)
- 🔵 [AICPA SOC 2 Trust Services Criteria](https://www.aicpa-cima.com/topic/audit-assurance/audit-and-assurance-greater-than-soc-2)

Prefer these over blog posts. Blog posts can help explain, but they are not the source of truth.

---

## 💬 Feedback

We value your input — it helps improve this marketplace for the whole community.

- **Bugs & feature requests:** [open an issue](https://github.com/Raishin/vanguard-frontier-agentic/issues/new) — 👍 the ones you want prioritized.
- **New skill or agent ideas:** describe the use case in an issue and we will review.
- **Security concerns:** see [`SECURITY.md`](SECURITY.md) for responsible disclosure.

---

## 🛡️ Contributing

The default answer to low-trust contributions is **no**. That is intentional — cloud automation can break real systems.

Good contributions are: useful, specific, auditable, source-backed, safe by default, and friendly for engineers of any seniority.

See:

- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`docs/taxonomy.md`](docs/taxonomy.md)
- [`docs/compatibility.md`](docs/compatibility.md)
- [`docs/marketplace-model.md`](docs/marketplace-model.md)

---

```text
Skills  = workflows        🧠   115 across AWS · Azure · OCI · Kubernetes · Terraform
Agents  = expert roles     🤖   115 with 7 harness adapters each
Rules   = always-on        📏   harness-specific operating guidance
MCP     = real connections 🔌   AWS · Azure · Oracle official servers
Catalog = searchable index 🗂️   machine-readable, hash-verified
```
