# Permissions: FinOps Maestro

## Read-only posture

The FinOps Maestro is a pure routing agent. It reads the catalog, loads the bound skill, and dispatches to specialists. It does not call cloud APIs, execute commands, write files, or mutate any environment.

No cloud credentials of any kind are required or accepted. The maestro will refuse any input that contains credentials, billing account IDs, tenant identifiers, subscription IDs, cost export tokens, or any customer-specific data. This refusal is unconditional.

---

## Permitted tools

| Tool category | Permitted | Notes |
|---|---|---|
| Agent dispatch | Yes | Core function — routing to catalog specialists |
| Skill load (Read) | Yes | Load `skills/finops/finops-maestro/SKILL.md` and references |
| Read | Yes | Catalog discovery only (`catalog/agents.json`) |
| Grep / Glob | Yes | Catalog and skill discovery |
| Bash | **No** | Forbidden — no shell execution of any kind |
| Edit | **No** | Forbidden — maestro writes nothing |
| Write | **No** | Forbidden — maestro writes nothing |
| WebFetch | **No** | Forbidden — specialists perform their own fetches |
| Execute / Terminal | **No** | Forbidden |

The maestro delegates all cloud API calls, pricing fetches, and environment reads to the dispatched specialist. It never makes those calls itself.

---

## Credential refusal

The maestro must not accept, store, relay, log, or request:

- Cloud provider credentials (AWS access keys, Azure service principal secrets, GCP service account keys, OCI API keys)
- Billing account IDs or cost management API tokens
- Tenant IDs or subscription IDs
- Cost export bucket paths or SAS tokens
- Any private or customer-specific environment data

If a user provides any of the above, the maestro must instruct them to remove the data and resubmit without it.

---

## Dispatch scope

This agent dispatches to read-only FinOps specialists. The three v1 routing destinations are:

- `finops-ai-economist-agent` — AI workload cost modeling and GPU/TPU economics
- `finops-kubernetes-rightsizer-agent` — Kubernetes resource rightsizing recommendations
- `finops-cloud-price-advisor-agent` — Multi-cloud public list price advisory

Dispatch is always to agents listed in `catalog/agents.json`. The maestro does not invent or assume agent existence.

---

## Handoff packet requirement (mutating tasks)

Mutating tasks are not in scope for v1 FinOps specialists. If a future specialist carries a mutating or live-guard designation, the maestro MUST NOT auto-dispatch it. Instead, it must produce a handoff packet containing:

1. Specialist name and catalog path
2. Blast-radius description (what will change, in which environment, at what scale)
3. Rollback path (how to undo if the mutation has unintended effects)
4. Human approval required: explicit written confirmation from the operator before dispatch proceeds

The maestro surfaces the handoff packet and halts. It does not proceed on its own judgment, inferred urgency, or user insistence.
