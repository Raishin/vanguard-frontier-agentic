---
layout: default
title: "FAQ"
permalink: /docs/faq/
---

# Frequently Asked Questions

---

## What is Vanguard Frontier Agentic?

A curated catalog of AI agent skills and agent definitions for enterprise cloud operations. It provides:

- **404 skills** - Structured guidance documents that AI coding assistants follow
- **426 agents** - Named agents with defined capabilities and role assignments
- **32 providers** - Cloud platforms and infrastructure tools covered
- **6 harness adapters** - Works with Claude Code, Codex, GitHub Copilot, Cursor, Gemini CLI, and Kiro

It is not a runtime, not a framework, and not a SaaS product. It is an npm package (`@raishin/vanguard-frontier-agentic`) containing validated content that AI coding tools consume.

---

## How is this different from just writing prompts?

| Aspect | Ad-hoc prompts | This catalog |
|--------|---------------|--------------|
| Validation | None | 17 automated gates |
| Schema contracts | None | JSON Schema enforcement |
| Multi-harness | Copy-paste per tool | Single source, 6 adapters |
| Supply chain | No integrity checks | SLSA L3, provenance, SBOM |
| Routing | Manual selection | Maestro router (357 scenarios) |
| Versioning | None | Semantic versioning, CHANGELOG |
| Safety | Hope for the best | Refusal-by-default model |

The key difference: this is a validated, versioned, multi-harness catalog with supply chain integrity, not a collection of prompt files.

---

## Is it production-ready?

Evidence-based answer:

**Yes**, with caveats:

### Evidence for production readiness

- **17 validation gates** enforced in CI on every PR (`npm run validate`)
- **357 Maestro routing scenarios** tested per release
- **Property-based fuzz testing** via fast-check (`tests/fuzz-properties.test.mjs`)
- **SLSA Build Level 3** attestations on every release
- **npm provenance** with Sigstore verification
- **SPDX SBOM** on every release
- **OpenSSF Scorecard** + Best Practices badges
- **CodeQL** static analysis
- **No lifecycle scripts** (zero code execution on install)
- **Asset integrity validation** via SHA-256 hashes
- **OIDC trusted publishing** (no stored credentials)
- **Automated releases** via semantic-release (no manual intervention)
- **Security policy** with defined SLA (`SECURITY.md`)

### Caveats

- Single maintainer (@Raishin) - bus factor of 1
- Content quality depends on AI harness capabilities (garbage in, garbage out)
- No runtime guarantees - this provides guidance, not execution sandboxing
- [NEEDS OWNER INPUT] No formal SLA for uptime or availability of the npm package itself

---

## What license is this under?

Apache License 2.0 (Apache-2.0).

- Commercial use: permitted
- Modification: permitted
- Distribution: permitted
- Patent use: explicitly granted
- Private use: permitted
- Trademark: not granted

Full text: `LICENSE` file in repository root.

---

## How do I get support?

| Channel | Purpose |
|---------|---------|
| [GitHub Issues](https://github.com/Raishin/vanguard-frontier-agentic/issues) | Bug reports, feature requests |
| [Security Advisories](https://github.com/Raishin/vanguard-frontier-agentic/security/advisories/new) | Vulnerability reports (private) |
| Pull Requests | Code contributions, doc fixes |

There is no commercial support offering, chat channel, or email list at this time.

---

## What is the relationship between skills, agents, rules, and MCP references?

### Skills

- Location: `skills/<provider>/<skill-id>.md`
- Purpose: Step-by-step guidance for a specific task
- Format: Markdown with YAML frontmatter
- Schema: `schemas/skill.frontmatter.schema.json`
- Example: "How to audit S3 bucket policies"

### Agents

- Location: `agents/<provider>/<agent-id>/metadata.json`
- Purpose: Named entity with a role, capabilities, and skill references
- Format: JSON metadata
- Schema: `schemas/agent.frontmatter.schema.json`
- Example: "AWS Security Auditor agent with IAM and S3 skills"

### Rules

- Location: `rules/`
- Purpose: Constraints on agent behavior (what agents must/must not do)
- Schema: `schemas/rule.schema.json`
- Current count: 1

### MCP References

- Location: `mcp/`
- Purpose: Define external tool integrations via Model Context Protocol
- Schema: `schemas/mcp-reference.schema.json`
- Current count: 3
- Trust: Validated against MCP trust matrix

### How they relate

```
Agent
  ├── references Skills (provides guidance)
  ├── constrained by Rules (limits behavior)
  └── can invoke MCP References (external tools)

Maestro Router
  └── routes Intent → Agent (by provider + role)
```

---

## Can I use this with my own AI coding tool?

Yes. The catalog is plain Markdown and JSON. Any tool that can read files can consume it.

For structured integration:
1. Use `npx vfa-export-agents --platform <your-platform> --all` to export in a standard format
2. Or read `catalog/agents.json` and `catalog/skills.json` directly
3. Skill content is standalone Markdown - no runtime dependencies

For a first-class adapter (included in the multi-harness validation), see [Contributing](../contributing/) for how to propose a new harness.

---

## How often are new versions released?

Releases are triggered by merging to master with conventional commits:

- `fix:` commits trigger a patch release
- `feat:` commits trigger a minor release
- Breaking changes trigger a major release
- `chore:` and `docs:` commits do not trigger releases

In practice, this means releases happen as frequently as PRs merge. There is no fixed release cadence.

---

## Do I need all 404 skills?

No. Use the CLI to filter:

```bash
# Only AWS skills for security engineers
npx vfa-export-agents --provider aws --role cloud-security-engineer

# Only Kubernetes skills
npx vfa-export-agents --provider kubernetes --all

# Metadata only (no skill content)
npx vfa-export-agents --all --no-skills
```

The install roles allow scoped installation:
- `cloud-security-engineer`
- `cloud-platform-engineer`
- `cloud-dba`
- `cloud-finops-analyst`
- `cloud-solutions-architect`
- `cloud-devops-engineer`

---

## What happens if a skill contains bad advice?

The refusal-by-default safety model means:

1. Skills only activate when Maestro routing matches a known intent
2. Skills are validated against schema contracts (no arbitrary content)
3. Code review via CODEOWNERS ensures human review
4. Fuzz testing checks that malformed inputs do not cause crashes

However, the content quality of individual skills is ultimately a human judgment call during code review. This catalog provides structure and validation, not a guarantee of correctness for every piece of advice.

---

## How to Verify This Works

```bash
# Confirm the package installs cleanly
npm install @raishin/vanguard-frontier-agentic

# Verify supply chain
npm audit signatures

# Export and inspect agents
npx vfa-export-agents --all --no-skills | head -50
```
