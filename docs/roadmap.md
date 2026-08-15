---
layout: default
title: "Roadmap"
permalink: /docs/roadmap/
---

# 🗺️ Roadmap

Current state, recent milestones, and planned work areas.

---

## 📊 Current State: v{{ site.data.catalog.version }}

| Metric | Value |
|--------|-------|
| Skills | {{ site.data.catalog.skills }} |
| Agents | {{ site.data.catalog.agents }} |
| Providers | {{ site.data.catalog.providers }} |
| MCP References | {{ site.data.catalog.mcp_references }} |
| Rules | {{ site.data.catalog.rules }} |
| Supported Harnesses | 6 (Claude Code, Codex, Copilot, Cursor, Gemini CLI, Kiro) |
| Validation Gates | {{ site.data.catalog.validation_gates }} |
| Maestro Routing Scenarios | {{ site.data.catalog.maestro_scenarios }} |

---

## 🚀 Recent Milestones

### v2.6.0

- FinOps price fixture validation
- README count automation
- QA cluster evaluation gate
- Multi-harness marketplace validation

### v2.5.0

- Kiro Powers integration
- Codex marketplace support
- Cross-harness consistency validation

### v2.4.0

- Maestro routing validation (357 scenarios)
- NVIDIA promotion gatekeeper
- Plugin manifest validation

### Earlier

- OIDC trusted publishing (eliminated NPM_TOKEN)
- SLSA Build L3 attestations
- OpenSSF Scorecard integration
- Property-based fuzz testing

---

## 🎯 Planned Work Areas

### More Providers

Current: {{ site.data.catalog.providers }} providers covering AWS, Azure, Kubernetes, Terraform, NetSuite, Salesforce, and others.

Potential additions: [NEEDS OWNER INPUT]
- GCP-specific agents and skills
- Datadog integration agents
- PagerDuty operational agents
- HashiCorp Vault security agents

### Deeper MCP Integration

Current: {{ site.data.catalog.mcp_references }} MCP references.

Planned:
- [NEEDS OWNER INPUT] Expand MCP reference catalog
- [NEEDS OWNER INPUT] MCP server discovery and registration protocol
- [NEEDS OWNER INPUT] Bidirectional MCP tool invocation from skills

### FinOps Expansion

Current: FinOps agents with price fixture validation.

Planned:
- [NEEDS OWNER INPUT] Multi-cloud cost comparison agents
- [NEEDS OWNER INPUT] Budget alert integration skills
- [NEEDS OWNER INPUT] Reservation and savings plan analysis

### More Harness Adapters

Current: 6 harnesses supported.

Potential: [NEEDS OWNER INPUT]
- Amazon Q Developer
- JetBrains AI
- Windsurf (Codeium)
- Tabnine

### Documentation

- API reference generation from schemas [NEEDS OWNER INPUT]
- Interactive skill browser [NEEDS OWNER INPUT]
- Video walkthroughs [NEEDS OWNER INPUT]

### Testing and Quality

- Expand Maestro routing scenarios beyond 357
- Add integration tests for each harness adapter
- [NEEDS OWNER INPUT] Performance benchmarking for large catalogs
- [NEEDS OWNER INPUT] Chaos testing for degraded environments

---

## 🚫 What Is NOT Planned

To set expectations clearly:

- **Runtime execution engine** - This is a catalog, not a runtime. Skills provide guidance, not code execution.
- **Self-hosted registry** - npm is the distribution channel. No custom registry.
- **GUI/dashboard** - The interface is AI harnesses, not a web UI.
- **Multi-tenant hosting** - Each consumer gets their own catalog installation.

---

## 🤝 How to Propose a New Direction

1. **Check existing ADRs** - Your idea may already be discussed in `docs/adr/`
2. **Open a GitHub Issue** - Use the "Skill or Agent Proposal" template for new assets, or a general issue for architectural proposals
3. **Write an ADR** - For significant changes, create `docs/adr/NNNN-your-proposal.md` with status "Proposed"
4. **Open a PR** - Include the ADR and any prototype implementation
5. **Discuss** - Maintainer review and feedback happens on the PR

### What makes a good proposal

- [ ] Clear problem statement (what is missing or broken)
- [ ] Evidence of demand (not speculative)
- [ ] Compatibility analysis (does it break existing users?)
- [ ] Maintenance burden assessment (who maintains it long-term?)
- [ ] Security impact (new trust boundaries? New dependencies?)

---

## Versioning Policy

This project uses semantic versioning via semantic-release:

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| New skills/agents (non-breaking) | Minor | 2.6.0 -> 2.7.0 |
| Bug fix in validation/schema | Patch | 2.6.0 -> 2.6.1 |
| Breaking schema change | Major | 2.6.0 -> 3.0.0 |
| New provider (non-breaking) | Minor | 2.6.0 -> 2.7.0 |
| Documentation only | No release | - |

---

## 🏛️ Enterprise Reviewer Notes

- Items marked `[NEEDS OWNER INPUT]` are speculative and require maintainer direction
- The roadmap is intentionally conservative; this project favors stability over feature velocity
- No timeline commitments are made for planned items
- The "What is NOT planned" section sets clear boundaries for scope discussions
