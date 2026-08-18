# Official Sources

Primary sources for plan mechanics, lifecycle ordering, and plan-to-apply behaviour, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | `-target`, `-replace`, `-refresh-only`, and saved plan files | Whether a proposed plan invocation narrows scope safely, and whether the reviewed plan binds the apply | Terraform v1.15 | Vendor reference carrying the explicit warnings about targeting and saved-plan handling | The only source stating that `-target` is for exceptional circumstances and that saved plans hold sensitive values in cleartext |
| <https://developer.hashicorp.com/terraform/language/meta-arguments/lifecycle> | HashiCorp | `create_before_destroy` transitivity, `prevent_destroy` limits, `ignore_changes`, `replace_triggered_by` | Whether the replacement ordering in a plan is safe, and whether a destroy guard actually guards | Terraform v1.15 | Vendor reference for the meta-arguments that determine replacement ordering | Documents the transitivity rule and the `prevent_destroy` configuration-removal gap that no other page states |
| <https://developer.hashicorp.com/terraform/language/resources/syntax> | HashiCorp | Resource behaviour, `count`/`for_each` addressing, and destroy semantics | Whether an address change in the diff is a rename or a destroy-and-create | Terraform v1.15 | Vendor definition of how resource instances are addressed and destroyed | The lifecycle page covers ordering but not instance addressing, which is the usual cause of mass replacement |
| <https://developer.hashicorp.com/terraform/cli/commands/apply> | HashiCorp | Applying a saved plan versus re-planning at apply time | Whether the reviewed plan is binding or advisory for the apply that follows | Terraform v1.15 | Vendor reference for the plan-to-apply relationship | The plan page describes producing a plan; only this one defines what apply does with or without it |
| <https://opentofu.org/docs/cli/commands/> | OpenTofu (Linux Foundation) | OpenTofu's plan and apply command surface | Whether a plan-mechanics claim verified on Terraform holds for an OpenTofu run | OpenTofu 1.12 | The engine's own command reference is the only proof of its behaviour | HashiCorp documentation cannot establish OpenTofu's flag surface or defaults |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
