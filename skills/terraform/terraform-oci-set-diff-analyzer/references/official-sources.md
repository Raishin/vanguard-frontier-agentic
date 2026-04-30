# Official sources

Primary official sources for this skill:

## Terraform core

- https://developer.hashicorp.com/terraform/cli/commands/plan
- https://developer.hashicorp.com/terraform/tutorials/cli/plan

Use these for:

- `terraform plan` versus `terraform apply`,
- saved plan files,
- why `terraform show -json <saved-plan>` is the preferred machine-readable evidence source.

## Terraform OCI provider

- https://registry.terraform.io/providers/oracle/oci/latest/docs
- https://registry.terraform.io/providers/oracle/oci/latest/docs/resources/core_route_table
- https://registry.terraform.io/providers/oracle/oci/latest/docs/resources/load_balancer_load_balancer
- https://registry.terraform.io/providers/oracle/oci/latest/docs/resources/load_balancer_load_balancer_routing_policy
- https://registry.terraform.io/providers/oracle/oci/latest/docs/resources/load_balancer_path_route_set
- https://registry.terraform.io/providers/oracle/oci/latest/docs/resources/load_balancer_rule_set

Use these for:

- Terraform schema and nested block names,
- route-table repeated blocks,
- load-balancer routing-policy, path-route-set, and rule-set nested collections.

## OCI operational API surface

Validated against OCI CLI help:

- `oci network route-table --help`
- `oci lb load-balancer --help`
- `oci lb routing-policy --help`
- `oci lb path-route-set --help`
- `oci lb rule-set --help`

Use these to confirm the real OCI control-plane shape:

- route tables are collections of `RouteRule` objects,
- routing policies are named ordered lists of routing rules,
- path route sets are named sets of path route rules,
- rule sets are named sets of listener-oriented rules.

## Source-grounding rule

Use Terraform core docs for plan and saved-plan behavior. Use OCI provider docs for Terraform schema. Use OCI API/CLI help to cross-check the actual OCI resource families when route-table or load-balancer collections are involved.

Treat the repeated-block-noise classification as a local heuristic unless the OCI provider docs explicitly document otherwise.
