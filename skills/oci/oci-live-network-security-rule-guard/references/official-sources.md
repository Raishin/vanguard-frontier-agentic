# Official Sources

Load these only when needed:

- [Security Lists](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/securitylists.htm) — use for Security List model, ingress/egress rule structure, stateful vs stateless semantics, and maximum rule limits.
- [Network Security Groups](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/networksecuritygroups.htm) — use for NSG model, VNIC-level vs subnet-level application, and NSG vs Security List trade-offs.
- [Managing NSG Security Rules](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/manage-nsg-security-rules.htm) — use for `oci network nsg rules add`, `update`, `remove`, and `list` CLI syntax.
- [Updating a Security List](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/update-securitylist.htm) — use for `oci network security-list update` full-replace semantics and required parameters.
- [Network Path Analyzer](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/path_analyzer.htm) — use for simulating end-to-end network paths through Security Lists, NSGs, route tables, and gateways before approving a rule change.
- [VCN Flow Logs](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/vcn-flow-logs.htm) — use when enabling forensic coverage for a subnet before or after a security rule change.
- [OCI IAM Policy Reference — Network](https://docs.oracle.com/en-us/iaas/Content/Identity/Reference/networkpolicyreference.htm) — use for least-privilege IAM policy statements covering `security-lists`, `network-security-groups`, and `virtual-network-family`.

## Grounded insights worth carrying into the skill

- `oci network security-list update` performs a **full replace** of the entire ingress or egress rule set — partial updates are not possible. Always pass the complete desired rule list including rules you want to keep.
- OCI Security Lists are **stateful by default** (`stateless: false`). Return traffic is automatically allowed. Stateless rules require explicit return rules and are a common source of asymmetric traffic failures.
- NSG rule IDs are required for deletion (`oci network nsg rules remove`). Capture rule IDs from `oci network nsg rules list` before any mutation.
- A Security List is attached to a subnet, not a VNIC. One change affects every instance in that subnet simultaneously — blast radius scales with subnet size.
- NSGs are attached to individual VNICs, giving finer-grained control but requiring per-VNIC management. Prefer NSGs for production database servers over Security Lists for reduced blast radius.
- VCN Flow Logs must be explicitly enabled per subnet — they are not on by default. Without them, there is no record of traffic through an accidentally opened rule.
- The `0.0.0.0/0` ingress source in OCI context still includes traffic from peered VCNs, DRG-attached networks, and FastConnect circuits if routing allows — it is never safe to assume it means "internet only."
