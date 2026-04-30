# 🚀 Vanguard Frontier Agentic

**A friendly, curated marketplace for cloud and zero-trust AI workflows.**

This repo collects reusable **skills**, **agents**, **rules**, **MCP references**,
and supporting assets for engineers working with AWS, Azure, OCI, GCP,
Kubernetes, Terraform, cloud security, and compliance-heavy architecture.

Think of it as a toolbox:

- 🧠 **Skills** = step-by-step workflows an AI assistant can follow.
- 🤖 **Agents** = reusable expert roles for review, architecture, and operations.
- 📏 **Rules** = durable instructions for a specific AI harness.
- 🔌 **MCP references** = trusted notes for connecting tools to real systems.
- 🗂️ **Catalogs** = machine-readable indexes so tools can discover everything.

The goal is simple: **make AI-assisted cloud work safer, reusable,
compliance-aware, and easier to understand.**

> 📦 **npm status (verified 2026-04-28):** `@raishin/vanguard-frontier-agentic`
> is **not published yet** on the public npm registry. Check live status with:
> `npm view @raishin/vanguard-frontier-agentic version`

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

| Framework / standard              | What it pushes us to remember                                                                                                                                     | Repo design implication                                                                                   |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 🔵 **SOC 2 Type 2**               | Controls must operate over a period of time, especially around security, availability, confidentiality, processing integrity, and privacy trust service criteria. | Workflows should leave evidence trails, not just one-time fixes.                                          |
| 💳 **PCI DSS**                    | Cardholder data environments need scoped controls, secure configuration, access control, monitoring, vulnerability management, and testing.                       | Workflows should reduce scope, avoid broad access, and flag payment-data risk.                            |
| 🇪🇺 **NIS2**                       | EU cybersecurity rules emphasize governance, risk management, incident reporting, supply-chain security, and management accountability.                           | Workflows should make ownership, reporting, and supplier/cloud dependencies explicit.                     |
| 🧭 **NIST CSF 2.0**               | Cybersecurity risk management spans Govern, Identify, Protect, Detect, Respond, and Recover.                                                                      | Assets should not stop at prevention; they should include detection, response, and recovery.              |
| 🏛️ **NIST SP 800-207 Zero Trust** | Access should be continuously evaluated and should not rely on implicit network trust.                                                                            | Agents and skills should challenge flat networks, permanent credentials, and unverified trust boundaries. |

Ruthless correction: **NIS2** is the European cybersecurity directive. **NIST**
is a U.S. standards body. If someone says “NIST2 European compliance,” they
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
| [`skills/`](skills/)       | Reusable workflows grouped by provider or domain | 🧠 “How do I do this task?”            |
| [`agents/`](agents/)       | Expert roles grouped by provider or domain       | 🤖 “Who should review this?”           |
| [`rules/`](rules/)         | Harness-specific instructions                    | 📏 “What behavior is always expected?” |
| [`mcp/`](mcp/)             | MCP server references and trust notes            | 🔌 “What can this connect to?”         |
| [`catalog/`](catalog/)     | JSON indexes for marketplace discovery           | 🗂️ “What assets exist?”                |
| [`schemas/`](schemas/)     | Metadata validation contracts                    | ✅ “What fields are required?”         |
| [`templates/`](templates/) | Starter templates for new assets                 | 🧱 “How do I add one?”                 |
| [`docs/`](docs/)           | Quality rules, taxonomy, and marketplace notes   | 📚 “How should this repo work?”        |
| [`assets/`](assets/)       | Logos and visual assets                          | 🎨 “What images can docs use?”         |

---

## 📦 Consumer install and export selected agents

This repository is designed so consumers can install **selected marketplace
agents** into their own project instead of copying everything by hand.

### Current package status

As of **2026-04-28**, the public npm package:

```bash
@raishin/vanguard-frontier-agentic
```

was verified as **not yet published** on npm.

Live check:

```bash
npm view @raishin/vanguard-frontier-agentic version
```

If that command returns `404 Not Found`, the package is still unpublished.

### Use it today from GitHub

Until npm publishing is live, install from GitHub:

```bash
npm install github:Raishin/vanguard-frontier-agentic
```

### Export selected agents into a consumer repository

After installation, this package ships a CLI:

```bash
vfa-export-agents
```

It copies selected agent harness files from this marketplace into the correct
runtime folders in a consumer repository.

List available agent IDs:

```bash
npx vfa-export-agents --list
```

Export one agent to **Claude Code**:

```bash
npx vfa-export-agents \
  --platform claude-code \
  --agents azure-cosmosdb-platform-operator-agent \
  --repo /path/to/consumer-repo
```

Export one agent to **GitHub Copilot**:

```bash
npx vfa-export-agents \
  --platform copilot \
  --agents azure-cosmosdb-platform-operator-agent \
  --repo /path/to/consumer-repo
```

Export one agent to **Kiro** (both IDE + CLI adapters):

```bash
npx vfa-export-agents \
  --platform kiro \
  --agents azure-cosmosdb-platform-operator-agent \
  --repo /path/to/consumer-repo
```

Export **all** agents for a platform:

```bash
npx vfa-export-agents --platform codex --all --repo /path/to/consumer-repo
```

Overwrite existing exported files intentionally:

```bash
npx vfa-export-agents --platform copilot --all --repo /path/to/consumer-repo --force
```

### Platform destination folders

The exporter writes into the destination repository using platform-native
runtime paths:

| Platform | Destination path(s) |
| -------- | ------------------- |
| Codex | `.codex/agents/` |
| Claude Code | `.claude/agents/` |
| GitHub Copilot | `.github/agents/` |
| Cursor | `.cursor/agents/` |
| Gemini CLI | `.gemini/agents/` |
| Kiro IDE | `.kiro/agents/` |
| Kiro CLI | `.kiro/agents/` |

### Important limitation

This exporter installs **custom agent files**, not full repo-level guidance.

If the consumer also wants the repository-level instruction layer, they should
use the matching project entrypoints in their own repo as appropriate:

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `.github/copilot-instructions.md`

See [`docs/normalized-platform-matrix.md`](docs/normalized-platform-matrix.md)
for the exact distinction between repo guidance and custom agent installation.

---

## 🧠 Skills

Skills are practical workflows. They should help an engineer complete a task
with less guesswork.

Current provider layout:

```text
skills/
├── aws/
├── azure/
├── gcp/
├── kubernetes/
├── oci/
├── security/
└── terraform/
```

Examples:

- 🔐 [`skills/aws/aws-iam-least-privilege-review`](skills/aws/aws-iam-least-privilege-review/)  
  Review AWS IAM policies and reduce unnecessary access.

- 🟦 [`skills/azure/azure-rbac-review`](skills/azure/azure-rbac-review/)  
  Review Azure RBAC assignments, scopes, and custom roles.

- 🟥 [`skills/oci/oci-autonomous-database-architect`](skills/oci/oci-autonomous-database-architect/)  
  Design and review Oracle Autonomous Database across OCI and multicloud options.

- 🏗️ [`skills/oci/oci-exadata-database-architect`](skills/oci/oci-exadata-database-architect/)  
  Stress-test Exadata architecture, migration, HA/DR, and deployment choices.

- 🔌 [`skills/oci/oracle-oci-mcp-grounded-advisor`](skills/oci/oracle-oci-mcp-grounded-advisor/)  
  Ground Oracle/OCI MCP and cloud advice in official sources.

Rule of thumb: if the asset teaches **how to do a repeatable task**, it is
probably a skill.

---

## 🤖 Agents

Agents are reusable expert roles. They are organized by cloud provider or domain
so engineers can find the right reviewer quickly.

```text
agents/
├── aws/
├── azure/
├── gcp/
├── oci/
├── multi-cloud/
├── security/
└── terraform/
```

Example:

- 🧱 [`agents/terraform/terraform-reviewer-agent`](agents/terraform/terraform-reviewer-agent/)  
  Review Terraform modules, plans, provider usage, and state assumptions.

Use an agent when you need a **role with judgment**, not just a checklist.

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

## 🎨 Logos and assets

Visual assets are organized for future documentation, marketplace cards, and
generated outputs.

```text
assets/logos/
├── cloud/
│   └── oci/
└── vendors/
    └── oracle/
```

Use:

- `cloud/<provider>/` for platform or service logos.
- `vendors/<vendor>/` for company/vendor logos.
- SVG first, PNG fallback when needed.

See [`assets/logos/README.md`](assets/logos/README.md).

---

## ✅ Quality bar

This repo is **not** a prompt junk drawer.

Every cataloged asset should be:

- 🔎 **Traceable** — includes official docs or clear provenance.
- 🔐 **Security-aware** — explains access, risk, and least-privilege concerns.
- 🧪 **Validated** — passes repo checks before being shared.
- 🧭 **Scoped** — clearly says which provider, domain, and harness it supports.
- 🧯 **Safe by default** — read-only discovery before mutation; approval before
  dangerous actions.

Hard no:

- ❌ Secrets or credentials.
- ❌ Vague “do everything” prompts.
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

MCP references also need:

- official project/source URL
- vendor
- auth model
- install/config example
- unofficial/community warning when relevant

---

## 🔏 Skill integrity manifests

Skills are executable guidance. Treat them like supply-chain artifacts.

This repo uses:

```text
catalog/skill-manifest.json
```

to record SHA-256 hashes for every file under every cataloged skill directory.

Why it matters:

- 🧾 proves what changed between releases,
- 🚨 catches accidental or unauthorized edits,
- 📦 gives npm consumers a package-level integrity map,
- 🧪 makes skill integrity testable in CI,
- 🔍 supports audit evidence for security-conscious teams.

After intentional skill edits, regenerate the manifest:

```bash
npm run manifest:write
```

Before release or review, check it:

```bash
npm run manifest:check
```

Ruthless truth: a manifest proves file integrity. It does **not** prove a skill
is safe, correct, compliant, or officially endorsed. You still need review.

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

Before a release, also run the online link check and npm package preview:

```bash
python tests/validate-links.py
npm pack --dry-run
```

If validation fails, fix that first. A broken catalog makes the marketplace
harder to trust.

---

## 📦 npm publishing and semantic versioning

This repository is npm-ready through [`package.json`](package.json).

Use SemVer: `MAJOR.MINOR.PATCH`.

| Version bump | Use when                                                                                                                                 | Example           |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------- |
| 🩹 `PATCH`   | Safe fixes: typos, metadata corrections, non-behavioral doc cleanup, manifest refresh after safe edits.                                  | `0.1.0` → `0.1.1` |
| ✨ `MINOR`   | Backwards-compatible additions: new skills, new agents, new provider folders, new optional metadata.                                     | `0.1.0` → `0.2.0` |
| 💥 `MAJOR`   | Breaking changes: removed/renamed IDs, moved paths without aliases, schema-required-field changes, incompatible manifest format changes. | `1.4.2` → `2.0.0` |

While the package is below `1.0.0`, be extra explicit in release notes because
minor bumps may still include early breaking design changes.

Read the full policy in [`docs/release-versioning.md`](docs/release-versioning.md).

---

## 📚 Source anchors

Use official sources when writing security or compliance-sensitive assets:

- 🏛️ [NIST SP 800-207 Zero Trust Architecture](https://csrc.nist.gov/pubs/sp/800/207/final)
- 🧭 [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- 🇪🇺 [European Commission: NIS2 Directive](https://digital-strategy.ec.europa.eu/en/policies/nis2-directive)
- 🇪🇺 [ENISA: NIS Directive 2](https://www.enisa.europa.eu/topics/state-of-cybersecurity-in-the-eu/cybersecurity-policies/nis-directive-2)
- 💳 [PCI Security Standards Council Document Library](https://www.pcisecuritystandards.org/document_library)
- 🔵 [AICPA SOC 2 Trust Services Criteria](https://www.aicpa-cima.com/topic/audit-assurance/audit-and-assurance-greater-than-soc-2)

Prefer these over blog posts. Blog posts can help explain, but they are not the
source of truth.

---

## 🧑‍💻 How to add a new asset

1. 🧭 Pick the right folder.
   - Cloud workflow? Put it under `skills/<provider>/`.
   - Expert role? Put it under `agents/<provider-or-domain>/`.
   - Harness instruction? Put it under `rules/<harness>/`.
   - MCP info? Put it under `mcp/official/` or `mcp/community/`.

2. 🧱 Start from a template.
   - Use [`templates/skill-template`](templates/skill-template/) for skills.
   - Use [`templates/agent-template`](templates/agent-template/) for agents.

3. 🗂️ Add or update catalog metadata.
   - Skills go in [`catalog/skills.json`](catalog/skills.json).
   - Agents go in [`catalog/agents.json`](catalog/agents.json).
   - Rules go in [`catalog/rules.json`](catalog/rules.json).
   - MCP references go in [`catalog/mcp-references.json`](catalog/mcp-references.json).

4. ✅ Run validation.

5. 🧯 Check safety.
   - No secrets.
   - No broad permissions unless justified.
   - No destructive actions without approval gates and rollback notes.

---

## 🛡️ Contribution stance

The default answer to low-trust contributions is **no**.

That is intentional. Cloud automation can break real systems.

Good contributions are:

- useful,
- specific,
- auditable,
- source-backed,
- safe by default,
- friendly for engineers of any seniority.

See:

- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`docs/taxonomy.md`](docs/taxonomy.md)
- [`docs/compatibility.md`](docs/compatibility.md)
- [`docs/marketplace-model.md`](docs/marketplace-model.md)

---

## 🧠 Remember this

```text
Skills = workflows 🧠
Agents = expert roles 🤖
Rules = always-on behavior 📏
MCP = real tool connections 🔌
Catalog = searchable index 🗂️
Validation = trust ✅
```
