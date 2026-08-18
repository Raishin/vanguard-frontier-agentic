# Official Sources

Primary sources for module structure, input validation, and assertion placement, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/language/modules/develop> | HashiCorp | Module structure, composition, and publishing conventions | Whether a module's structure and composition match the documented reusable-module contract | Terraform v1.15 | Vendor definition of what a module is expected to expose and how it is versioned | The only source defining module structure; the variables page covers inputs but not composition |
| <https://developer.hashicorp.com/terraform/language/values/variables> | HashiCorp | Variable types, defaults, `validation` blocks, `nullable`, and `sensitive` | Whether an input is constrained at the module boundary or merely documented in prose | Terraform v1.15 | Vendor reference for the only mechanism that enforces an input contract | Module-develop describes structure, not the validation semantics this agent tests inputs against |
| <https://developer.hashicorp.com/terraform/language/checks> | HashiCorp | `check` blocks and continuous validation as non-blocking assertions | Whether a module's invariant belongs in `validation`, `precondition`, `postcondition`, or a `check` block | Terraform v1.15 | Vendor reference distinguishing blocking from advisory assertions | Neither the variables nor module page explains the blocking-versus-advisory split that decides where an invariant goes |
| <https://opentofu.org/docs/language/functions/> | OpenTofu (Linux Foundation) | OpenTofu's function surface | Whether a module intended to run on both engines relies on a function available on only one | OpenTofu 1.12 | The engine's own function reference is the only proof of its actual surface | HashiCorp documentation cannot establish what OpenTofu supports; portability claims need both engines' own references |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
