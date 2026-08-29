---
layout: default
title: "ADR-0001: Three-Layer Maestro Architecture"
permalink: /docs/adr/0001-initial-architecture/
---

# 📐 ADR-0001: Three-Layer Maestro Architecture

## Status

Accepted

## Date

2024-01-15

## Context

Enterprise AI agent ecosystems face a coordination problem: hundreds of skills across dozens of cloud providers must be routed safely to the correct specialist agent without executing unintended actions.

Existing approaches fall into two categories:

1. **Flat catalogs** - All skills accessible to all agents. No routing, no safety boundary. Users must manually select the right skill.
2. **Monolithic agents** - One large agent handles everything. Becomes unmanageable at scale, cannot enforce provider-scoped permissions.

Neither approach satisfies enterprise requirements:

- **Safety**: Incorrect routing can execute destructive operations on the wrong infrastructure
- **Auditability**: Security teams need to trace intent to execution through a defined path
- **Multi-cloud**: Organizations use 5-10+ cloud providers; skills must be scoped
- **Multi-harness**: Different AI coding tools have different interfaces; the architecture must abstract this

### Constraints

- The system is a content package (npm), not a runtime
- Skills are Markdown documents, not executable code
- The routing logic must be testable and deterministic
- Adding a new provider or harness should not require modifying core logic

## Decision

Adopt a three-layer architecture:

### Layer 1: Maestro Router

A routing layer that maps user intent to the correct specialist agent. The router:

- Accepts natural language intent
- Matches against known routing scenarios (357 as of v2.6.0)
- Routes to a provider-scoped specialist agent
- **Refuses by default**: unmatched intent is rejected, not guessed

### Layer 2: Specialist Agents

Provider-scoped agents (e.g., AWS IAM Auditor, Azure Entra ID Specialist) that:

- Have a defined set of skills they can reference
- Are assigned to install roles (e.g., cloud-security-engineer)
- Declare harness compatibility in metadata
- Are validated against `schemas/agent.frontmatter.schema.json`

### Layer 3: Cross-Functional Protocol

The execution layer consisting of:

- **Skills** - Structured Markdown with validated frontmatter
- **MCP References** - External tool integrations via Model Context Protocol
- **Rules** - Behavioral constraints on agents
- **Harness Adapters** - Per-tool manifest generators

### Directory structure reflecting the architecture

```
agents/<provider>/<agent-id>/metadata.json   (Layer 2)
skills/<provider>/<skill-id>.md              (Layer 3)
mcp/<reference-id>.json                      (Layer 3)
rules/<rule-id>.md                           (Layer 3)
tests/validate-maestro-routing.py            (Layer 1 validation)
```

## Consequences

### ✅ What becomes easier

- **Adding a provider**: Create `agents/<provider>/` and `skills/<provider>/`, add routing scenarios. No core changes.
- **Adding a harness**: Write a manifest generator in `scripts/`, add a validation gate. No skill changes.
- **Safety auditing**: Routing is deterministic and tested. Security teams can review the 357 scenarios.
- **Schema evolution**: Contracts are explicit (JSON Schema), so breaking changes are detectable.
- **Role-based access**: Install roles scope agent sets without runtime permission checks.

### ⚠️ What becomes harder

- **Simple one-off usage**: The three-layer structure is heavier than a flat file of prompts. Small deployments pay an overhead.
- **Dynamic routing**: The Maestro router uses fixture-based validation, not ML-based intent classification. New intents require new fixtures.
- **Cross-provider operations**: A task spanning AWS + Azure requires routing to multiple specialists. The architecture does not natively support multi-agent orchestration at the catalog level.
- **Catalog growth**: At the time of this decision (404 skills, 426 agents) the catalog was already large, and it has grown since. Validation time scales linearly.

### Risks

- **Single routing layer**: If Maestro routing has a bug, all requests to that provider are affected.
- **Fixture-based testing**: Routing quality depends on fixture coverage. Gaps in fixtures mean gaps in safety.
- **Refusal-by-default can over-refuse**: Legitimate requests that do not match known patterns are rejected.

### Mitigations

- 357 routing scenarios provide strong coverage
- Fuzz testing catches crashes on unexpected inputs
- The fixture set is regeneratable (`npm run maestro-routing:write`) and extensible
- Schema validation prevents structurally invalid content from entering the catalog
