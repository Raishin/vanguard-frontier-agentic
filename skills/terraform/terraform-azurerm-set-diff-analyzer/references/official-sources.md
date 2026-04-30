# Official sources

Primary official sources for this skill:

## Terraform core

- https://developer.hashicorp.com/terraform/cli/commands/plan
- https://developer.hashicorp.com/terraform/tutorials/cli/plan

Use these for:

- `terraform plan` versus `terraform apply`,
- saved plan files,
- why `terraform show -json <saved-plan>` is the preferred machine-readable evidence source.

## Terraform AzureRM provider

- https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs
- https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs/resources/application_gateway
- https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs/resources/network_security_group
- https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs/resources/route_table

Use these for:

- Terraform schema and nested block names,
- Application Gateway nested collections such as listeners, backend pools, request routing rules, rewrite rule sets, and URL path maps,
- NSG inline `security_rule` blocks and route-table `route` blocks.

## Azure service-domain docs

- https://learn.microsoft.com/azure/application-gateway/application-gateway-components
- https://learn.microsoft.com/azure/application-gateway/rewrite-http-headers-url
- https://learn.microsoft.com/azure/virtual-network/network-security-groups-overview

Use these for:

- Application Gateway listener, routing-rule, rewrite-set, and path-based-routing semantics,
- NSG rule priority, augmented rule behavior, and service-tag context.

## Source-grounding rule

Use Terraform core docs for plan and saved-plan behavior. Use AzureRM provider docs for schema and nested block structure. Use official Azure docs for the runtime meaning of Application Gateway routing graphs and NSG rule behavior.

Do not let vague memory override service-domain details such as:

- one listener binding to one request-routing rule,
- rewrite sets attaching through routing rules,
- NSG rules being evaluated by priority and remaining stateful for existing flows.
