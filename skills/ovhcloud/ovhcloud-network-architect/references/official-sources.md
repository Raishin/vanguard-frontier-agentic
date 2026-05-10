# Official sources

Use this reference only when you need source grounding for OVHcloud networking service behavior or the detailed source list.

## OVHcloud documentation

Use these as starting points, not as proof of the user's live vRack topology or network state:

- https://help.ovhcloud.com/csm/en-vrack?id=kb_article_view&sysparm_article=KB0044799
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/vrack
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/cloud_project_network_private

## Grounding rule

Official documentation explains OVHcloud vRack service semantics, VLAN configuration, private network attachment behavior, and Terraform resource capabilities. It does not prove the user's current vRack member inventory, VLAN assignments, security group rules, subnet configuration, or routing state. Prefer live OVHcloud API evidence or sanitized user-provided topology diagrams and Terraform state for current-state claims.
