# Admin Roles And Separation Of Duties

Account, workspace, and metastore admin roles and their proper division of labour.

- Account admins have account-wide mutation authority (billing, workspace creation, identity federation, system configuration). Workspace admins have workspace-scoped authority (user and group management, cluster policy, workspace-level configurations).
- The account admin who creates a workspace automatically becomes its workspace admin; other account admins require explicit workspace-admin assignment. Workspace creation does not automatically include other account admins in workspace-admin roles.
- Metastore admin is an optional role (not all deployments have one) with metastore-scoped authority (storage credential, metastore admin privilege delegation). Metastore-admin role assignment propagates account-wide in up to 30 seconds.
- Databricks recommends a small number of account admins. An overly broad account-admin group concentrates mutation risk and defeats least-privilege design; account admin should be emergency escalation only, not routine operations.
- Service principals are API-only and do not support interactive login; assigning a service principal to a workspace as an admin or reader is a design error that grants it UI access it cannot use.
