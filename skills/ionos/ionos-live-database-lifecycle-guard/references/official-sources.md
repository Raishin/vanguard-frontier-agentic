# Official sources

Use this reference only when grounding IONOS DBaaS service behavior, endpoint patterns, or IaC definitions.

## IONOS Cloud database documentation

Use these as starting points, not as proof of the user's live database state, backup existence, or cluster health:

- https://docs.ionos.com/cloud/databases — managed database concepts, supported engines (PostgreSQL, MariaDB, MongoDB), cluster lifecycle, and backup behavior
- https://docs.ionos.com/cloud/databases/postgresql — PostgreSQL-specific configuration, regional endpoint patterns (`https://postgresql.<region>.ionos.com`), version support, and PITR behavior
- https://registry.terraform.io/providers/ionos-cloud/ionoscloud/latest/docs/resources/pg_cluster — IaC PostgreSQL cluster resource definition, backup window arguments, and replica configuration
- https://api.ionos.com/docs/ — REST API reference for database lifecycle operations, cluster status queries, and backup management endpoints

## Grounding rule

Official IONOS documentation describes DBaaS service capabilities and expected behavior. It does not prove the user's current cluster state, backup existence, replication health, PITR availability, or regional endpoint assignment. Live API evidence or user-provided sanitized cluster state is mandatory for any backup verification or hard-stop assessment. When live tooling is unavailable, state this explicitly — documentation-based reasoning is insufficient to satisfy the hard-stop conditions.
