# Workspace-Catalog Binding And Governed Tags

ISOLATED mode enforcement, tag account limits, inheritance rules, and character restrictions.

- ISOLATED workspace-catalog binding denies access from unbound workspaces EVEN IF the principal holds an explicit GRANT on that catalog. This is access-time enforcement; binding overrides any explicit privilege grant from an unbound workspace.
- All catalogs are accessible by default from any workspace on the same metastore; binding overrides this default and ISOLATED mode enforces the override.
- Governed tags are account-level scope (never workspace-local), managed by account admins, visible across all workspaces. Max 1,000 tags per account, max 500 values per tag, 256-character key limit.
- Prohibited characters in tag names and values include * . / < > % & ? \ = and all ASCII control characters 0-31. Flag any tag that does not pass this character set.
- Tag inheritance flows downward automatically (catalog → schemas → tables) EXCEPT columns, which require explicit tag application via ALTER TABLE ... ALTER COLUMN. A governance model assuming automatic column tagging does not exist.
