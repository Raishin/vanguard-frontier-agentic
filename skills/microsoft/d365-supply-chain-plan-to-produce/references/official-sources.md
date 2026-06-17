# Official sources

Use this reference only when you need source grounding for Dynamics 365 Supply Chain Management master planning, inventory management, procurement, warehouse management, or production control service behavior, or the detailed source list.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live environment state:

- https://learn.microsoft.com/dynamics365/supply-chain/master-planning/master-planning-home-page
- https://learn.microsoft.com/dynamics365/supply-chain/supply-chain-management-welcome
- https://learn.microsoft.com/training/modules/set-up-master-planning/
- https://learn.microsoft.com/training/modules/use-master-planning/
- https://learn.microsoft.com/dynamics365/supply-chain/production-control/production-process-overview
- https://learn.microsoft.com/dynamics365/supply-chain/inventory/inventory-home-page
- https://learn.microsoft.com/dynamics365/supply-chain/procurement/procurement-sourcing-overview
- https://learn.microsoft.com/dynamics365/supply-chain/warehousing/warehouse-configuration

## Grounding rule

Official documentation explains Dynamics 365 Supply Chain Management service behavior. It does not prove the user's current on-hand inventory positions, coverage settings, active master plans, planned order status, BOM versions, or route configurations. Prefer read-only evidence from the environment (e.g., planning log exports, on-hand inventory reports, coverage settings exports, production order status reports) over inference.

## Service facts (verified 2026-06-16)

Master planning model structure:
- **Planning Optimization** is the current master planning engine for Dynamics 365 Supply Chain Management. The deprecated built-in master planning engine has been retired for new implementations.
- The three main planning processes are: **Master planning** (net requirements, short-term), **Forecast planning** (gross requirements, long-term), and **Intercompany master planning** (net requirements across legal entities).
- Planning Optimization runs outside the SQL database, minimizing impact on live operations and enabling runs during business hours with near-real-time results.
- **Coverage groups** define replenishment settings per item: min/max, period, requirement, or safety stock coverage. Misconfigured coverage groups are the leading cause of excess inventory or stockout-driven production delays.

Inventory and coverage:
- **Safety stock** can be defined as a fixed quantity, a percentage of average demand, or a calculated value from demand forecasting. Undocumented safety stock levels create planning instability.
- **Item coverage** overrides at the item-warehouse level take precedence over coverage group defaults. Unreviewed item-level overrides are a common source of planning anomalies.
- **On-hand inventory** must be validated before trusting master planning output. Negative on-hand quantities, unposted inventory journals, and unclosed production orders all distort planning calculations.

Procurement and sourcing:
- **Purchase trade agreements** and **vendor default order settings** influence planned purchase order quantities, lead times, and vendor selection during planning runs. Stale trade agreement data produces unrealistic planned orders.
- The **Supply risk assessment** feature (Dynamics 365 Supply Chain Management 10.0.31+) surfaces procurement risks for review before supply disruptions materialize.

Production control:
- **Bills of materials (BOMs)** and **routes** drive production order material and capacity requirements. Inactive or incorrect BOM versions and route operations produce inaccurate planned production orders.
- **Operations scheduling** considers backlogged productions, material availability, and capacity availability. Finite capacity constraints are not considered in multi-threaded scheduling runs — document this limitation explicitly.
- **Tracked components** (10.0.40+) enable batch and serial number registration for finished products and their components from the production floor execution interface.

Review implications:
- Do not approve master plan output or planned order firming without current on-hand inventory validation, coverage settings review, and supply chain manager sign-off.
- Documentation cannot prove the user's actual inventory positions, active coverage settings, or production schedule state.
