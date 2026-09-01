# GRANT Privilege Model And Inheritance

How Unity Catalog privileges cascade, what they exclude, and how inheritance enables least-privilege design.

- Privileges cascade downward: a GRANT MANAGE on a catalog automatically grants MANAGE on all child schemas and tables without a separate GRANT statement. Inheritance applies to REVOKE identically.
- ALL PRIVILEGES does not grant every privilege; it explicitly excludes MANAGE, READ METADATA, EXTERNAL USE SCHEMA, EXTERNAL USE LOCATION, and other administrative capabilities. A grant of ALL PRIVILEGES confers no ownership, no delegation right, and no data-classification right.
- Ownership is a single principal per securable, never shared between two principals. If two users need admin rights on a table, GRANT them MANAGE privilege, do not create co-ownership; ownership and admin privileges are distinct concepts.
- Users get zero default access and must be explicitly granted. Workspace users auto-receive USE CATALOG on the workspace catalog plus CREATE on its default schema, but zero access to other catalogs.
- REVOKE succeeds even when the privilege was never granted; a successful REVOKE is not evidence that the grant existed — idempotent revoke patterns are valid and silent.
