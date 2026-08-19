# Official Sources

Primary sources for import, moved blocks, refresh-only planning, and engine-specific limits, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/language/import> | HashiCorp | `import` blocks, `id` versus `identity`, and `-generate-config-out` | Whether a brownfield adoption can be expressed declaratively and previewed before it touches state | Terraform v1.15 | Vendor reference for the declarative import mechanism | The only source defining the `identity` argument and generated-configuration workflow |
| <https://developer.hashicorp.com/terraform/language/moved> | HashiCorp | `moved` blocks and address refactoring | Whether an address change can be carried in configuration rather than performed as state surgery | Terraform v1.15 | Vendor reference for the rename mechanism | The import page covers adoption, not renames; these are opposite operations on the same record |
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | `-refresh-only` planning mode | How to observe drift safely before deciding what to do about it | Terraform v1.15 | Vendor reference for the only mode that surfaces drift without proposing changes | Drift detection has no dedicated page; this flag is the documented mechanism |
| <https://developer.hashicorp.com/terraform/language/meta-arguments/lifecycle> | HashiCorp | `ignore_changes` as drift suppression | Whether an attribute is genuinely externally owned or merely being silenced | Terraform v1.15 | Vendor reference for the construct most often used to make drift disappear | Cited here for a different decision than on the blast-radius board: ownership, not ordering |
| <https://opentofu.org/docs/language/import/> | OpenTofu (Linux Foundation) | OpenTofu import blocks, `for_each` imports, and generated-configuration limits | Whether an import strategy verified on Terraform is available on an OpenTofu estate | OpenTofu 1.12 | The engine's own reference, which marks configuration generation experimental | Documents a `for_each`-plus-generation limitation that the HashiCorp page does not carry |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
