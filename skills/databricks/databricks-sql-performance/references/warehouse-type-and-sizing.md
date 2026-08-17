# Warehouse Type And Sizing

Warehouse-type capabilities, concurrency bounds, startup latency, queueing behaviour, and Intelligent Workload Management availability.

- Serverless warehouses include Photon and Intelligent Workload Management (IWM); they start in 2–6 seconds; pro and classic include Photon but do not include IWM and start in ~4 minutes.
- Serverless and pro support Predictive I/O (row filtering via learned model); classic does not. Predictive I/O requires Photon.
- Auto-stop defaults: serverless 10 minutes (minimum 5 via UI, 1 via API); pro and classic 45 minutes (minimum 10 via UI).
- Classic and pro warehouses scale at one cluster per ~10 concurrent queries; queue depth caps at 1000 queries; Intelligent Workload Management (IWM, serverless-only) manages queuing automatically.
- Serverless warehouses have a default 2.5-hour execution timeout for interactive notebooks (admin-configurable) as runaway-spend protection.

## Sources

- https://docs.databricks.com/aws/en/compute/sql-warehouse/warehouse-types
- https://docs.databricks.com/aws/en/compute/sql-warehouse/warehouse-behavior
- https://docs.databricks.com/aws/en/compute/photon
