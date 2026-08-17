# Official Sources

Primary sources for policy enforcement, evaluation inputs, and in-language assertions, each tied to a decision.

Every row is a primary source verified 2026-08-17 by direct fetch. A URL earns a row only when it supports a decision this agent actually makes; a source that duplicates a claim another row already carries is removed rather than kept for completeness.

| Source | Publisher | Topic | Decision supported | Version | Why authoritative | Why not redundant |
|---|---|---|---|---|---|---|
| <https://developer.hashicorp.com/terraform/cloud-docs/policy-enforcement> | HashiCorp | Policy enforcement levels and where policy runs in the plan-apply cycle | Whether a policy actually blocks an apply or merely advises, and at which stage | HCP Terraform / Terraform Enterprise, current | Vendor reference for the enforcement semantics the whole verdict depends on | Enforcement levels exist nowhere else; policy language pages describe rules, not their power |
| <https://developer.hashicorp.com/terraform/cloud-docs/policy-enforcement/opa> | HashiCorp | OPA/Rego policy evaluation against plan data | Whether a control is expressible against the plan rather than against source text | HCP Terraform / Terraform Enterprise, current | Vendor reference for the open-source policy path most estates can adopt | Sentinel is proprietary and licence-gated; OPA is the portable option and behaves differently |
| <https://developer.hashicorp.com/terraform/cloud-docs/policy-enforcement/sentinel> | HashiCorp | Sentinel policy sets, imports, and enforcement | Whether an estate's existing Sentinel policy is portable, and what it costs if not | HCP Terraform / Terraform Enterprise, current | Vendor reference for the proprietary policy framework | Documents licence and platform coupling that the OPA path does not carry |
| <https://developer.hashicorp.com/terraform/language/checks> | HashiCorp | `check` blocks as continuous non-blocking assertions | Whether a control belongs in policy, in a blocking assertion, or in a continuous check | Terraform v1.15 | Vendor reference distinguishing blocking from advisory in-language assertions | In-language controls are the option most estates overlook when they reach for external policy |
| <https://developer.hashicorp.com/terraform/cli/commands/plan> | HashiCorp | Machine-readable plan output as the policy input | Whether the evidence artifact under review is the plan the policy actually evaluated | Terraform v1.15 | Vendor reference for the artifact every plan-stage policy consumes | Cited for evidence integrity, a different decision than the blast-radius board's use of the same page |

## Grounding rule

Documentation describes engine and provider behaviour in general. It does not prove the engine, engine version, provider versions, backend, or workspace the user actually runs. Treat any claim that depends on those as `assumption` until the supplied configuration, lock file, or plan confirms it — and name the engine (Terraform or OpenTofu) on every version-sensitive claim.
