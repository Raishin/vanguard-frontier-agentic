# AGENTS.md

## Purpose
- Cloud + zero-trust agentic marketplace for skills, agents, rules, MCP references, compliance-aware docs, and npm distribution.
- Optimize for evidence-backed security workflows: least privilege, source grounding, manifests, validation, and safe automation.

## Stack Map
- `.code-review-graph/` → generated local graph cache; do not edit.
- `.git/` → Git internals; do not edit.
- `agents/` → Markdown/JSON agent definitions; provider/domain layout.
- `assets/` → curated logos and visual assets.
- `catalog/` → JSON marketplace indexes, skill integrity manifest, and role taxonomy.
- `docs/` → Markdown governance, taxonomy, release, compatibility, evidence output spec, and CI/CD enforcement patterns.
- `mcp/` → Markdown/JSON MCP references.
- `rules/` → Markdown/JSON harness rules.
- `schemas/` → JSON Schema metadata contracts.
- `skills/` → Markdown/JSON skill packages with reference files.
- `templates/` → starter Markdown/JSON asset templates.
- `tests/` → Python validation scripts.
- `package.json` → npm package metadata and validation scripts.

## Workflows
- `npm run validate` → catalog + skill manifest + offline link validation.
- `npm run manifest:write` → refresh `catalog/skill-manifest.json` after intentional skill edits.
- `python3 tests/validate-links.py` → online link validation before release.
- `npm pack --dry-run` → inspect npm package contents before publish.
- `vfa-export-agents --list-roles` → list available role IDs with agent counts.
- `vfa-export-agents --platform <p> --role <role-id> --repo <path>` → install all agents for a role.
- `vfa-export-agents --platform <p> --role <role-id> --provider <provider> --repo <path>` → install role agents for one provider.

## Change Rules
- Update catalog JSON when adding, moving, or removing cataloged assets.
- Regenerate skill manifest after any intentional change under cataloged `skills/**`.
- For every `skills/**/SKILL.md`, keep skill frontmatter metadata under `metadata`, including `metadata.version` and `metadata.author`; use the GitHub-style author value (for example `github: Raishin`) and do not use top-level `version` or `author` keys in skill frontmatter.
- For cross-platform agent work, keep `author` and `version` truth in the canonical contract plus adjacent `metadata.json` unless a harness's official docs explicitly verify executable metadata support.
- Do not create partial metadata truth across harnesses: if `author` and `version` are not both doc-verified in an executable adapter, keep both out of that adapter.
- Keep README human-friendly; keep this file agent-focused and compressed.
- For provider-specific `README.md` files, add the matching cloud-provider logo near the top using a repo-local asset from `assets/logos/cloud/<provider>/`.
- Do not add a provider logo to neutral or multi-cloud READMEs unless one provider is clearly the primary subject.
- Do not add secrets, credentials, tokens, wallets, tenant IDs, or customer data.
- Prefer official docs and live evidence over memory for cloud/compliance claims.
- Treat broad permissions, destructive automation, and MCP mutation paths as high-risk.
- When adding new agents, update `catalog/install-roles.json` if the agent belongs to one or more roles. Roles are: `cloud-security-engineer`, `cloud-platform-engineer`, `cloud-dba`, `cloud-finops-analyst`, `cloud-solutions-architect`, `cloud-devops-engineer`. An agent may appear in multiple roles.
- All live-guard and review agents must produce the five required evidence fields defined in `docs/evidence-output-spec.md`: `verdict`, `evidence_level`, `blockers`, `safe_next_actions`, `open_questions`.

## Role-Based Pattern

`catalog/install-roles.json` defines six cross-provider roles. Each role is a curated list of agent (and skill) IDs that practitioners in that function need, across all supported cloud providers.

| Role ID | Who uses it |
|---------|------------|
| `cloud-security-engineer` | IAM reviewers, security posture teams, compliance engineers |
| `cloud-platform-engineer` | Infrastructure/SRE, IaC owners, Kubernetes platform teams |
| `cloud-dba` | Database administrators, data platform engineers |
| `cloud-finops-analyst` | FinOps leads, cost governance teams |
| `cloud-solutions-architect` | Cloud architects, migration leads, AI/generative engineers |
| `cloud-devops-engineer` | CI/CD engineers, release managers, SRE ops |

Roles overlap intentionally — an agent useful to both a security engineer and a platform engineer appears in both lists.

The `--role` flag in `vfa-export-agents` resolves the role's agent list and exports them in one command. Use `--provider` to further filter by cloud provider (e.g., `--provider azure`).

Pipeline enforcement — pushing role installs into CI/CD so guardrails run without developer opt-in — is documented in `docs/ci-cd-enforcement-pattern.md`.

Evidence output — how the structured verdict response from every live-guard and review agent satisfies SOC 2, PCI DSS, NIS2, NIST CSF, and ISO 27001 controls — is documented in `docs/evidence-output-spec.md`.

## Load When
- editing `agents/` → `agents/AGENTS.md`
- editing `catalog/` → `catalog/AGENTS.md`
- editing `docs/` → `docs/AGENTS.md`
- editing `mcp/` → `mcp/AGENTS.md`
- editing `rules/` → `rules/AGENTS.md`
- editing `schemas/` → `schemas/AGENTS.md`
- editing `skills/` → `skills/AGENTS.md`
- editing `templates/` → `templates/AGENTS.md`
- editing `tests/` or validation scripts → `tests/AGENTS.md`
