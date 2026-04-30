# Azure + OCI Live Guards eval

## EVAL DEFINITION: azure-oci-live-guards

### Capability evals

1. Six Azure live-guard agents exist under `agents/azure/`, each with a unique live-mutation domain (ARM stacks, PIM JIT, AKS rollout, App Service slot swap, Key Vault rotation, cost budget).
2. Six OCI live-guard agents exist under `agents/oci/`, each with a unique domain (Resource Manager stacks, IAM compartment, OKE rollout, Autonomous DB lifecycle, Vault key destruction, cost budget runaway).
3. Every live-guard agent contains the extended file set: `AGENT.md`, `PERMISSIONS.md`, `PREFLIGHT.md`, `ROLLBACK.md`, `metadata.json`, plus all 7 harness adapters.
4. `PERMISSIONS.md` for Azure agents uses Azure-native RBAC constructs (Microsoft.* actions, built-in role names, PIM concepts) — not AWS IAM syntax.
5. `PERMISSIONS.md` for OCI agents uses OCI IAM verb syntax (`Allow <subject> to <verb> <resource> in <location>`) with least-privilege verb selection (inspect/read/use/manage hierarchy).
6. `PREFLIGHT.md` for each agent contains actual provider CLI commands (`az`, `kubectl`, `oci`) — not generic prose.
7. `ROLLBACK.md` for each agent contains a concrete rollback playbook with actual commands — not generic advice.
8. Every live-guard agent has a paired skill under `skills/azure/` or `skills/oci/`.
9. All 12 agent entries and 12 skill entries appear in `catalog/agents.json` and `catalog/skills.json` respectively.
10. `metadata.json` for every asset passes schema validation (`schemas/agent.schema.json`, `schemas/skill.schema.json`).

### Anti-pattern evals (must all FAIL = zero violations)

A1. No PERMISSIONS.md grants bare `Owner` or `Contributor` at subscription scope without PIM/JIT scoping comment.
A2. No PERMISSIONS.md grants `AdministratorAccess` (AWS anti-pattern must not bleed into Azure/OCI files).
A3. No OCI PERMISSIONS.md contains `manage all-resources in tenancy` without explicit compartment scoping.
A4. No AGENT.md is ≥ 90% identical to another AGENT.md in the same provider directory (uniqueness).
A5. No metadata.json `last_verified` date is older than 2026-04-30.

### Regression evals

R1. Existing AWS live-guard agents (5) remain intact — file counts unchanged.
R2. Existing Azure non-live agents (24) remain intact.
R3. Existing OCI non-live agents (27) remain intact.
R4. `python3 tests/validate-catalog.py` — PASS.
R5. `npm run validate` — PASS.

### Success metrics

| Category | Target |
|---|---|
| Structural evals | pass^3 = 100% (deterministic, must never flake) |
| Anti-pattern evals | 0 violations |
| Regression evals | pass^3 = 100% |
| Quality evals (content) | pass@3 ≥ 90% |

---

## Grader

```bash
bash tests/validate-azure-oci-live-guards.sh
```

---

## EVAL REPORT (post-build)

Structural:
- azure-live-arm-deployment-stack-guard-agent:  PENDING
- azure-live-pim-jit-activation-guard-agent:    PENDING
- azure-live-aks-rollout-guard-agent:           PENDING
- azure-live-app-service-slot-swap-guard-agent: PENDING
- azure-live-keyvault-rotation-purge-guard-agent: PENDING
- azure-live-cost-budget-action-guard-agent:    PENDING
- oci-live-resource-manager-stack-guard-agent:  PENDING
- oci-live-iam-policy-compartment-guard-agent:  PENDING
- oci-live-oke-rollout-guard-agent:             PENDING
- oci-live-autonomous-db-lifecycle-guard-agent: PENDING
- oci-live-vault-key-destruction-guard-agent:   PENDING
- oci-live-cost-budget-runaway-guard-agent:     PENDING

Anti-patterns: PENDING
Regression:    PENDING
npm validate:  PENDING

Status: IN PROGRESS
