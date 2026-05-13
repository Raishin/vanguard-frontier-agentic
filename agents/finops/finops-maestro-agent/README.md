# FinOps Maestro

Domain router for FinOps. Classifies the user's question and dispatches the narrowest specialist — or a parallel team of up to four — from the catalog.

---

## What it does

- Reads `skills/finops/finops-maestro/SKILL.md` to classify the incoming task.
- Routes to one or more FinOps specialists found in `catalog/agents.json`.
- Dispatches in parallel when two or more domains are involved (ceiling: 4 specialists).
- Synthesizes specialist outputs into a unified response.
- Produces a handoff packet for any mutating task and halts for human approval.

## What it does NOT do

- Answer FinOps questions directly.
- Call cloud APIs, pricing endpoints, or billing APIs.
- Accept, store, relay, or request cloud credentials or tenant data.
- Auto-dispatch any mutating or live-guard specialist.
- Use Bash, Edit, Write, or WebFetch.

---

## Bound skill

`skills/finops/finops-maestro/SKILL.md` (being built in parallel — reference only)

---

## Routing destinations (v1)

| Specialist | Domain |
|---|---|
| `finops-ai-economist-agent` | AI workload cost modeling, GPU/TPU economics |
| `finops-kubernetes-rightsizer-agent` | Kubernetes resource rightsizing |
| `finops-cloud-price-advisor-agent` | Multi-cloud public list price advisory |

---

## Trust posture

- Read-only. No credentials required or accepted.
- No mutation. No auto-dispatch of live-guard agents.
- All label claims as `live-evidence`, `documentation-based`, or `inference`.
- Handoff packet required before any mutating dispatch; human approval gate is non-negotiable.

---

## Full contract

See [AGENT.md](AGENT.md) for the complete canonical specification and [PERMISSIONS.md](PERMISSIONS.md) for the tool surface and credential refusal policy.
