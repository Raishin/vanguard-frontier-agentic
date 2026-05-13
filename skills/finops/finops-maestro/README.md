# FinOps Maestro

A routing skill that classifies FinOps tasks and dispatches them to the narrowest available specialist. Maestro never answers questions directly; it classifies domains, selects agents, and synthesizes outputs.

## Allowed tools

`Agent` `Skill` `Read` `Grep` `Glob`

## Usage

**Single domain:** Provide a task with a clear FinOps signal (e.g., "What is the on-demand price for Claude Sonnet on Bedrock?"). Maestro routes to `finops-cloud-price-advisor-agent`.

**Multi-domain:** Provide a task spanning two or more domains (e.g., "Analyze my AI model costs and rightsize my GPU pods"). Maestro routes to `finops-ai-economist-agent` and `finops-kubernetes-rightsizer-agent` in parallel.

## Specialists (v1)

| Agent ID | Domain |
|---|---|
| `finops-ai-economist-agent` | AI/ML model cost economics |
| `finops-kubernetes-rightsizer-agent` | Kubernetes pod rightsizing and allocation |
| `finops-cloud-price-advisor-agent` | Cloud resource on-demand pricing |

## Trust posture

Read-only. No live-guard agents exist in v1. Mutation requests are refused and escalated to a human operator. No credentials, billing account IDs, or tenant data accepted at any point in the routing chain.

See [SKILL.md](SKILL.md) for the full routing protocol and response shape.
