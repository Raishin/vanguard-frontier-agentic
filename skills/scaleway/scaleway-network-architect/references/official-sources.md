# Official sources

Use this reference when grounding Scaleway VPC, networking, or Load Balancer service behavior.

## Scaleway networking documentation

Use these as starting points — not as proof of the user's live network state:

- https://www.scaleway.com/en/docs/network/vpc/ — VPC docs: regional VPC, Private Network creation, attachment, routing, and inter-project connectivity
- https://www.scaleway.com/en/docs/compute/instances/how-to/use-placement-groups/ — placement group docs: `max_availability` vs `enforced` policy behavior, zone scope, scheduling impact
- https://www.scaleway.com/en/docs/network/load-balancer/ — Load Balancer docs: front-end/backend configuration, health check setup, TLS termination, sticky sessions, zone coverage
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/vpc — Terraform `scaleway_vpc` resource: regional VPC creation and project scope
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/instance_placement_group — Terraform `scaleway_instance_placement_group` resource: policy types, zone binding, instance assignment

## Grounding rule

Official documentation describes Scaleway network service behavior and configuration semantics. It does not prove the user's current security group rules, active Private Network attachments, Load Balancer health state, or placement group assignments. Prefer repo evidence or sanitized user-provided evidence for current-state claims. Zone-boundary and security group findings must always be grounded in confirmed evidence, not assumed defaults.
