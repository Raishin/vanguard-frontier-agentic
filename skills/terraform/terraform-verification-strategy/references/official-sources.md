# Official Sources

Primary sources for the test framework, plan semantics, and continuous assertions, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/language/tests> | HashiCorp | `.tftest.hcl` files, `run` blocks, `command = plan` versus `apply`, `assert` blocks, and mock providers | Which test mode proves the property in question, and whether it creates real infrastructure | Terraform v1.15 | Vendor reference for the built-in test framework | The only source defining test run modes and what each one actually executes |
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | What a plan evaluates and what it leaves unresolved | Whether a plan is sufficient verification for the property being checked | Terraform v1.15 | Vendor reference for the most common verification artifact | Cited here for verification adequacy rather than for blast-radius semantics |
| <https://developer.hashicorp.com/terraform/language/checks> | HashiCorp | `check` blocks as continuous post-apply assertions | Whether a property is better verified continuously than at change time | Terraform v1.15 | Vendor reference for the continuous-assertion construct | Covers the verification that happens after the change, which no test mode addresses |
| <https://opentofu.org/docs/cli/commands/> | OpenTofu (Linux Foundation) | OpenTofu's test and validate command surface | Whether a verification strategy written for Terraform runs on an OpenTofu estate | OpenTofu 1.12 | The engine's own command reference | Test framework surface must be confirmed per engine rather than assumed shared |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
