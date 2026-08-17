# Architecture Decision Framework

How a Snowflake structural decision is made, recorded, and reopened. Load when writing or reviewing an architecture decision record.

## Reversibility classes

- **Cheap and reversible** — warehouse size, auto-suspend, clustering keys, materialized views, search optimization. Decide fast, measure, change. Over-deliberating these is its own waste.
- **Expensive but reversible** — database and schema layout, role model, tagging taxonomy, warehouse-to-workload mapping. Changing them costs coordination, not migration.
- **Effectively irreversible without a migration** — account topology, cloud, region, edition downgrades, data residency commitments, table format and catalog choice, and any boundary that external consumers have already bound to. These deserve the analysis budget.
- The most common architecture failure is the inverse allocation: weeks spent on warehouse sizing and an afternoon on the account topology.

## What a boundary must buy

- A **separate account** can buy blast-radius isolation, a residency or sovereignty boundary, an edition difference, a billing and contractual boundary, or a hard administrative ownership split. If it buys none of these it adds replication, identity federation, sharing, and monitoring cost for nothing.
- A **separate database** buys an ownership and grant boundary and a replication unit. It does not buy compute isolation.
- A **separate warehouse** buys compute isolation, independent sizing, independent suspension economics, and attributable cost. It does not buy access control.
- A **separate role** buys an access boundary only if the hierarchy above it does not silently re-merge what was separated — a boundary erased by inheritance is a boundary that exists on the diagram only.
- A boundary enforced by convention rather than by RBAC is not a boundary. State which enforcement mechanism holds each one.

## The business-case gate

- Every material proposal answers all of: current-state cost and risk; target-state cost and risk; implementation cost; expected business benefit; time to value; operational burden; reversibility; lock-in implications; security delta; resilience delta; confidence; decision owner.
- A proposal with no decision owner is not a proposal — it is a suggestion that will be re-litigated.
- No technology wins because it is Snowflake-native. Native is a cost and operability argument, not an outcome argument, and it must be stated as one.
