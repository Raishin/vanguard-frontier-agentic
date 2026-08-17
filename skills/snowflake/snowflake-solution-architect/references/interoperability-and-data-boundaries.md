# Interoperability and Data Boundaries

Where data leaves the Snowflake governance boundary, and what each exit costs in control. Load when a design involves Iceberg, external engines, sharing, or external access.

## Every exit is a governance decision

- Snowflake-managed tables keep the full governance surface — masking, row access, tags, lineage, and access history apply. Externally managed tables trade some of that reach for engine independence. Name which controls survive the boundary before choosing.
- Reading a table with an external engine means the engine's authorization decision matters as much as Snowflake's. A design that assumes Snowflake policy is the only enforcement point has an unmodelled path.
- Secure data sharing moves the access decision, not the data. Its governance question is who may see what, and that is `snowflake-governance-privacy-agent`'s to answer.
- External access integrations and external functions are outbound paths from inside the account. They belong in the architecture diagram because they are how data leaves it — route the design of the path itself to `snowflake-network-private-connectivity-agent`.
- A residency requirement is not satisfied by choosing a region. Replication targets, external stages, sharing consumers, marketplace listings, external access destinations, and AI service routing all need the same residency answer.

## Catalog choice

- Catalog choice is in the effectively-irreversible class once external consumers have bound to it. Treat it with the analysis budget that implies.
- Do not preserve a legacy interoperability architecture merely because it was once Snowflake's preferred model; re-verify the current recommendation before committing, because that recommendation has moved.
- State the operational surface each option adds: credential vending, catalog availability, schema evolution coordination, and who is paged when the external engine cannot read.
