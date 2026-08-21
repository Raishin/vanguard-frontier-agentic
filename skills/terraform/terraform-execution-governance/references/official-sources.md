# Official Sources

Primary sources for apply binding, plan artifact handling, dynamic credentials, and remote execution, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/cli/commands/apply> | HashiCorp | Applying a saved plan versus re-planning, and auto-approval | Whether the pipeline's apply step is bound to the reviewed plan or free to do something else | Terraform v1.15 | Vendor reference for the step where a pipeline actually changes infrastructure | The only source defining what apply does with and without a saved plan |
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | `-out` saved plan files and their cleartext sensitive contents | How a plan artifact must be handled as it moves between pipeline stages | Terraform v1.15 | Vendor statement that saved plans contain sensitive values in cleartext | Cited here for artifact handling, a different decision than the blast-radius board's plan semantics |
| <https://developer.hashicorp.com/terraform/cloud-docs/workspaces/dynamic-provider-credentials> | HashiCorp | Short-lived workload-identity credentials instead of stored static keys | Whether the runner's cloud credentials are long-lived secrets or short-lived and attributable | HCP Terraform / Terraform Enterprise, current | Vendor reference for the supported alternative to static credentials | The only source documenting the credential model this agent recommends |
| <https://developer.hashicorp.com/terraform/cloud-docs/run/remote-operations> | HashiCorp | Remote plan and apply execution, run modes, and where operations actually happen | Where the change executes and which environment's trust boundary applies | HCP Terraform / Terraform Enterprise, current | Vendor reference for remote execution semantics | Local and remote execution have different trust boundaries; no CLI page covers the remote case |
| <https://developer.hashicorp.com/terraform/cli/config/config-file> | HashiCorp | CLI configuration on the runner, including credential and installation blocks | Whether runner-side configuration silently changes what the pipeline executes | Terraform v1.15 | Vendor reference for configuration that lives outside the repository | Cited for runner environment integrity rather than for provider provenance |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
