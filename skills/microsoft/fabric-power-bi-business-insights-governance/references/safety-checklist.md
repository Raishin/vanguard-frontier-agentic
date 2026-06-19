# Safety checklist

Use this reference before any recommendation involving production workspace roles, RLS/OLS, sensitivity labels, DLP, or Fabric capacity.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, workspace URLs, or customer data into chat.
- Use admin portal exports, lineage view, or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent model inventories, endorsement status, RLS rules, or capacity metrics.
- Require explicit human approval before recommending any production workspace-role, RLS/OLS, sensitivity-label, DLP, or capacity change.
- Use current official Microsoft Learn documentation for Fabric/Power BI security and governance behavior.
- Keep recommendations least-privilege and reversible.

## Stress checks

- Which metrics come from duplicated or uncertified models (mistrust)?
- Which sensitive models lack RLS — or rely on RLS while exposing Admin/Member/Contributor roles (RLS only restricts Viewers)?
- Which reports are built on personal models rather than an endorsed shared model?
- Which workspace roles are broader than necessary?
- Which models lack sensitivity labels or DLP coverage?
- What rollback exists if an RLS or workspace-role change exposes or hides data incorrectly?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual model inventory, endorsement, RLS configuration, or workspace roles.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Changing production workspace role assignments (Admin/Member/Contributor/Viewer)
- Modifying production RLS/OLS roles or rules on shared semantic models
- Changing sensitivity labels or Purview DLP policies for Power BI/Fabric
- Endorsing (certifying) or un-endorsing a production semantic model
- Resizing, pausing, or reassigning Fabric capacity
