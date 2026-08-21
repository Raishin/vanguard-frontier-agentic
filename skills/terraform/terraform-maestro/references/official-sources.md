# Official Sources

Primary sources behind the routing inputs, each tied to the decision it supports.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | What a plan is and what it proves | Whether the user has supplied enough evidence to classify a change, or must be asked for a plan first | Terraform v1.15 | Vendor reference for the command every routing decision depends on | Only source defining plan semantics; no other row covers the routing input itself |
| <https://opentofu.org/docs/cli/commands/> | OpenTofu (Linux Foundation) | OpenTofu CLI surface | Whether a task phrased in `tofu` terms is engine-specific or shared, before routing | OpenTofu 1.12 | Vendor-neutral engine's own command reference | The Terraform CLI page does not enumerate OpenTofu's command set or its divergences |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
