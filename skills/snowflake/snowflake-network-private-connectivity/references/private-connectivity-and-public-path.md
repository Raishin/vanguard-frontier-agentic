# Private Connectivity and the Public Path

The two independent facts this domain constantly conflates, and how to evidence each. Load whenever private connectivity is claimed to provide isolation.

## Two facts, two proofs

- **Fact one:** a private endpoint exists and clients can reach the account through it. Evidence: the private connectivity configuration on the cloud side plus login history showing traffic arriving on that path.
- **Fact two:** the public path is closed. Evidence: the network policy or account configuration that closes it, read directly. The existence of fact one is not evidence of fact two.
- A design that assumes fact two while only having fact one has bought latency and an operational dependency, not isolation. Say so plainly; this is the single most common finding in the domain.
- Where private connectivity is in use, clients must resolve and connect to the private hostname. A client still using the public account URL is on the public path regardless of what the diagram shows.
- The cloud-side work — endpoint provisioning, DNS zones, route tables, firewall rules — is owned by the cloud board. State what it must provide and verify the Snowflake side against it; do not design it here.

## Egress is the other half

- External access integrations bind a set of network rules and, usually, secrets, and allow code running inside Snowflake to reach an external destination. Enumerate every one of them, its destinations, and who may create another.
- The security question for egress is not whether the destination is trusted today, but who can add a destination tomorrow and whether that addition is reviewed.
- An egress path is a data-exit path. Where a residency, sovereignty, or data-loss-prevention requirement exists, the egress inventory is part of proving it — not an operational detail.
- Storage integrations reach cloud storage for stages. Their scope should be the specific container or bucket path, not the account, and their credential should be a managed identity or role rather than a stored key wherever the cloud supports it.

## Evidence queries

Inventory the outbound surface and who can extend it.

```sql
SHOW INTEGRATIONS;
DESCRIBE INTEGRATION my_external_access_integration;
DESCRIBE INTEGRATION my_storage_integration;

-- Who can create another one — CREATE INTEGRATION is an account-level privilege,
-- so this list is the set of principals that can extend egress without review.
SELECT grantee_name AS role_name, privilege
  FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
 WHERE privilege ILIKE '%INTEGRATION%'
   AND deleted_on IS NULL
 ORDER BY role_name;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/developer-guide/external-network-access/external-network-access-overview — That outbound access from Snowflake is expressed through network rules bound into an external access integration, with secrets attached
- https://docs.snowflake.com/en/user-guide/network-rules — The ingress and egress modes of network rules and how they are referenced by policies and integrations
