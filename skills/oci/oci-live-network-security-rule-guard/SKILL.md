---
name: oci-live-network-security-rule-guard
description: Guard live OCI Security List and Network Security Group (NSG) rule changes with current-state capture, open-internet and sensitive-port detection, stateful/stateless assessment, and explicit approval before ingress or egress rule mutation. Use only when an intentional network rule change targets a confirmed VCN component.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# OCI Live Network Security Rule Guard

## Purpose

Act as the guarded live OCI operator for oci-live-network-security-rule-guard work. Security List and NSG rule changes take effect immediately with no native rollback. A wrong ingress rule exposes databases or compute to the internet instantly; a wrong egress rule can black-hole traffic for entire subnets. Treat every rule mutation as irreversible until the previous state is explicitly captured and restoration is confirmed possible.

## When to use

Use this skill when:

- an ingress or egress rule must be added, modified, or removed from an OCI Security List or NSG in a live VCN
- a network access audit finds over-broad CIDR blocks (`0.0.0.0/0`) or sensitive-port exposures that must be tightened
- a workload migration requires opening or closing ports and the blast radius must be confirmed before write

## Lean operating rules

- Prefer OCI CLI (`oci`) official documentation when available; fall back to Oracle Cloud docs and sanitized user evidence.
- Do not execute any Security List or NSG rule mutation until tenancy, compartment, VCN OCID, target Security List or NSG OCID, and exact rule change are all explicit.
- Capture the complete current rule set (`oci network security-list get` or `oci network nsg rules list`) as rollback evidence before any write.
- Flag the following as high-severity and require explicit justification before proceeding:
  - Any ingress rule with source `0.0.0.0/0` (open to internet)
  - Any egress rule with destination `0.0.0.0/0` and protocol `all` without restriction
  - Rules permitting port 22 (SSH), 3389 (RDP), 1521/1522 (Oracle DB), 3306 (MySQL), 5432 (PostgreSQL) from `0.0.0.0/0`
  - Stateless rules on subnets hosting databases or internal APIs (no connection tracking = asymmetric traffic risk)
  - Changes to Security Lists attached to database subnets (Autonomous DB, Exadata, DB System)
- If the request skips current-state capture, CIDR scope confirmation, or subnet-criticality assessment, push back.
- Never print API signing keys, auth tokens, tenancy OCIDs, or instance credentials. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Preflight commands](references/preflight-commands.md) — OCI CLI commands to inspect current rules and capture rollback state before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — how to restore a previous Security List or NSG rule set after a bad change.
- [Permission model](references/permission-model.md) — least-privilege IAM policy for network rule mutation and read-only audit.
- [Official sources](references/official-sources.md) — authoritative OCI documentation links.

## Response minimum

Return, at minimum:

- confirmed tenancy, compartment, VCN, and target Security List or NSG OCID
- current rule set capture (rollback baseline)
- risk classification of the proposed rule (open-internet / sensitive-port / safe)
- stateful vs stateless assessment and subnet criticality
- approval status with explicit business justification
- rollback command to restore prior rule state
- post-change connectivity verification steps or refusal reason
