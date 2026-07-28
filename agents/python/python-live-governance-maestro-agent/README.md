# Python Live Governance Maestro

Entry point for the Python **live control plane** — the read-only-runtime and mutating-runtime agents that interact with live systems under controlled execution with provable accountability. Classifies the runtime, business process, data class, environment, and control profile, then routes to the narrowest live specialist. Routes only: it never mutates, never approves, and never declares compliance.

---

## Operating model

Every live action follows: Inventory -> Classify -> Observe -> Plan -> Evaluate controls -> Obtain authority -> Execute -> Verify -> Reconcile -> Seal evidence -> Monitor -> Reassess. The maestro owns Classify and routing only.

## How routing works

### Required skill

- `skills/python/python-live-governance-maestro/SKILL.md`

### Routing modes

- `single` / `parallel (N)` (max 4) — read-only-runtime specialists.
- `runtime-evidence-gate` — read-only-runtime actions requiring captured evidence.
- `live-guard-gate` — mutating-runtime operators; NEVER auto-dispatched. Surfaced only with an external signed approval bound to the target, JIT credentials, and a pre-approved rollback. The maestro gates these to a named human owner.
- `unclassified` — insufficient signal or missing applicability inputs; ask for the smallest sufficient set.

### Out-of-board handoffs

- Cloud/Kubernetes/Terraform infrastructure mutation, OpenTelemetry collector topology, Prometheus infra, sigstore signing operations, NVIDIA GPU infra, and data-warehouse administration route to their respective boards. Accounting/finance policy, legal/regulatory interpretation, and HR matters route to those boards.

---

## The Python live-plane taxonomy

| Domain | Primary agent | Tier | Typical signals |
|---|---|---|---|
| `system-inventory` | `python-live-system-inventory-agent` | read-only-runtime | inventory, discover, runtime, service, job, notebook |
| `identity-authority` | `python-live-identity-authority-agent` | read-only-runtime | identity, authority, credential age, JIT, just-in-time, approval |
| `runtime-control` | `python-live-runtime-control-agent` | read-only-runtime | interpreter, process, worker, task, thread, memory |
| `change-plan` | `python-live-change-plan-agent` | read-only-runtime | plan, diff, rollback procedure, verification criteria, action digest, change plan |
| `policy-gate` | `python-live-policy-gate-agent` | read-only-runtime | policy, control applicability, policy bundle, gate, machine-readable policy, control profile |
| `code-remediation` (live-guard — gated) | `python-live-code-remediation-agent` | mutating-runtime | branch, pull request, remediation, isolated validation, fix PR, dependency fix |
| `release-control` (live-guard — gated) | `python-live-release-control-agent` | mutating-runtime | release, canary, deploy, rollback, restart, one instance |
| `data-change-control` (live-guard — gated) | `python-live-data-change-control-agent` | mutating-runtime | migration, backfill, reprocessing, data correction, bounded update, ownership |
| `job-control` (live-guard — gated) | `python-live-job-control-agent` | mutating-runtime | job, distributed job, business automation, retry, idempotency, business completion |
| `model-promotion-control` (live-guard — gated) | `python-live-model-promotion-control-agent` | mutating-runtime | model promotion, immutable artifact, model risk, evaluation evidence, monitoring, rollback |
| `control-evidence` | `python-live-control-evidence-agent` | read-only-runtime | evidence, hash, seal, evidence store, control mapping, audit artifact |
| `continuous-control-testing` | `python-live-continuous-control-testing-agent` | read-only-runtime | continuous control testing, control operating, finding, drift, expired, monitoring |
| `rollback-recovery` (live-guard — gated) | `python-live-rollback-and-recovery-agent` | mutating-runtime | rollback, recovery, restore, revert, pre-approved rollback, affected target |
| `exception-governance` | `python-live-exception-governance-agent` | read-only-runtime | exception, policy exception, compensating control, expiration, review date, risk acceptance |

---

## What the maestro will refuse

- Any mutation, approval, or compliance declaration — it routes only.
- Auto-dispatching a mutating (live-guard) operator without external approval, JIT credentials, target binding, and a pre-approved rollback.
- Acting on a verbal/self-claimed approval, a requester-as-approver, shared or unidentified identities, or standing administrative credentials.
- Proceeding on an R3+ action when audit logging is unavailable (fail closed).

---

## Eval coverage

Routing and adversarial-authority tests are in `tests/fixtures/python-live-maestro-routing/`. Run `npm run validate:maestro-routing`.

---

Part of the Vanguard Frontier Agentic Python board (live control plane).
