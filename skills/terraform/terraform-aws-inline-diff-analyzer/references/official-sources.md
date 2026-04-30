# Official sources

Primary official sources for this skill:

## Terraform core

- https://developer.hashicorp.com/terraform/cli/commands/plan
- https://developer.hashicorp.com/terraform/tutorials/cli/plan

Use these for:

- `terraform plan` versus `terraform apply`,
- saved plan files,
- why `terraform show -json <saved-plan>` is the preferred machine-readable evidence source.

## Terraform AWS provider

- https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/lb_listener_rule
- https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/route_table
- https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group
- https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/wafv2_web_acl

Use these for:

- Terraform schema and nested block names,
- provider-specific caveats about inline versus standalone rule resources,
- WAFv2 inline rule limitations documented by the provider,
- listener-rule condition and action block shape.

## AWS service-domain docs

- https://docs.aws.amazon.com/elasticloadbalancing/latest/application/listener-rules.html
- https://docs.aws.amazon.com/elasticloadbalancing/latest/application/rule-condition-types.html
- https://docs.aws.amazon.com/vpc/latest/userguide/RouteTables.html
- https://docs.aws.amazon.com/vpc/latest/userguide/create-vpc-route-table.html
- https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/changing-security-group.html
- https://docs.aws.amazon.com/waf/latest/developerguide/web-acl-editing.html
- https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups.html

Use these for:

- ALB listener-rule priority and condition semantics,
- VPC route-table domain behavior,
- security-group rule semantics,
- WAF web ACL editing and managed-rule context.

## Source-grounding rule

Use Terraform core docs for plan and saved-plan behavior. Use Terraform AWS provider docs for schema, nested blocks, and provider caveats. Use official AWS service docs for the runtime meaning of listener rules, route tables, security groups, and WAF rules.

Do not let vague memory override provider caveats such as:

- inline security-group rules conflicting with standalone rule resources,
- inline route-table routes conflicting with standalone route resources,
- WAFv2 inline rules having ordering and rewrite limitations.
