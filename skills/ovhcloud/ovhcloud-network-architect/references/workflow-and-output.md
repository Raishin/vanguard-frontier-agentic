# Workflow and output contract

Use this reference only when performing a full vRack topology review, network isolation audit, private network design, or blast-radius assessment for OVHcloud networking.

## Review domains

Check these areas before giving a verdict:

- vRack member inventory: Public Cloud projects, dedicated servers, and other services attached to the vRack
- VLAN assignment and segmentation: VLAN IDs per tier, conflict risk with other members, and isolation between Public Cloud and dedicated infrastructure
- Private network attachment: `ovh_cloud_project_network_private` subnets, DHCP configuration, and routing between VLANs
- Security group rules: ingress and egress rules for Public Cloud instances, default-allow vs. least-exposure posture
- Load balancer placement and upstream routing path to backends
- DNS zone design: `ovh_domain_zone` record management, delegation, and split-horizon considerations
- Blast radius of proposed topology changes: what services lose connectivity if a vRack member or VLAN is removed
- Terraform IaC for `ovh_vrack`, `ovh_cloud_project_network_private`, and related resources

## Safe workflow

1. **Frame scope**
   - vRack ID, attached member list (if known), and environment:
   - Business criticality and traffic paths in scope:
   - Change type (design, review, topology modification, VLAN add/remove):
   - Required outcome:
   - Explicit non-goals (e.g., do not modify dedicated server connectivity):
2. **Collect evidence**
   - Prefer live OVHcloud API or Terraform state evidence if available.
   - Otherwise inspect repository `ovh_vrack` and `ovh_cloud_project_network_private` resources, sanitized user-provided topology diagrams, or official OVHcloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What topology change could isolate a production member or break a critical traffic path?
   - What VLAN conflict could cause cross-tenant or cross-tier leakage?
   - What security group gap leaves backend services reachable from the wrong tier or the public internet?
   - What DNS change could cause resolution failures or split-horizon inconsistencies?
   - What evidence is missing that prevents a confident blast-radius assessment?
4. **Recommend the smallest safe action**
   - Prefer additive changes (add VLAN, add private subnet) before removals.
   - Require a rollback plan before recommending any vRack member detachment or VLAN removal.
   - If the current vRack member inventory was not confirmed, stop and say so.

## Output contract

Return this structure:

```markdown
# OVHcloud Network Architecture Review: <vRack or scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Blast-radius assessment
- Members at risk: <list or none>
- VLAN conflict check: <confirmed / unconfirmed / not applicable>
## Residual risk
- <risk or explicit none>
```
