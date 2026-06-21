# Safety checklist — SAP Datasphere Data Product Architecture Review

Use before making any remediation recommendation, especially for findings that affect data access controls, cross-space sharing, or SAC live connection model structure.

## Non-negotiables

- Do not access, connect to, or request access to any live Datasphere tenant, space, data flow execution engine, or connected source system. This skill reviews artifacts only.
- Do not accept or request Datasphere tenant credentials, space user credentials, OAuth tokens, connection credentials (HANA Cloud, SAP S/4HANA, third-party source), or data access control user assignment files that contain personal data.
- Do not recommend structural changes to a semantic model consumed via an active SAC live connection without first confirming which SAC stories and models depend on it. Structural changes (removing measures, renaming keys, changing semantic usage) break SAC stories immediately and silently.
- Do not recommend changing DAC combination methods (AND/OR) without first confirming the intended access rule logic. An incorrect combination method change can grant unintended access to regulated data or lock out legitimate users.
- Do not recommend reducing space storage quota without first confirming current space utilization. Quota reduction below actual usage terminates running data flows and blocks new data ingestion.
- Do not conflate data flows (transformation pipelines) with replication flows (bulk/delta replication). Error handling, scheduling, and target table configuration work differently for each. Recommendations designed for one type must not be applied to the other.
- Do not classify a finding as `critical` without being able to trace the specific data isolation breach or access control failure from user-provided artifacts or official documentation.

## What people get wrong

- **Assuming SAC can consume any Datasphere view via live connection**: SAC live connections to Datasphere require views with specific semantic usage types (analytic model, analytical dataset, or dimension). Graphical views without a recognized semantic usage type are not directly consumable by SAC live connection stories. Recommending a live connection to an unannoted relational view is incorrect.
- **Confusing table sharing with data product sharing**: Datasphere supports direct table sharing between spaces and data product-governed sharing. Direct table sharing bypasses data product governance and should not be recommended as a primary data distribution pattern — it is intended for ad hoc or internal space federation scenarios.
- **Treating replication flow delta load and data flow incremental append as equivalent**: Replication flow delta load uses source system change-data-capture (CDC) mechanisms. Data flow incremental append uses a filter on a timestamp or sequence column. They have different failure recovery behaviors and are not interchangeable.
- **Missing the DAC criteria entity dependency**: A data access control rule requires a correctly structured criteria entity (a view that maps user identifiers to data restriction values). Recommending DAC without confirming the criteria entity exists and returns the correct user identifier attribute leads to silent access grant (all data visible) or silent access denial (no data visible).
- **Recommending Open SQL schema output port without confirming HANA Cloud connection**: The Open SQL schema output port type requires an active Datasphere-to-HANA Cloud connection. Recommending this output port without confirming the HANA Cloud connection is configured and the schema is shared correctly leads to a non-functional data product.
- **Ignoring analytic model variable requirements for SAC**: Datasphere analytic models can have mandatory input variables. If an analytic model consumed by SAC has a mandatory variable with no default value, the SAC story will fail to load unless the variable is handled at the story level.

## When to push back

- Push back when the user asks to confirm a data access control finding from a description alone without providing the DAC rule definition, criteria entity structure, or view assignment.
- Push back when the user asks for specific storage quota numbers without providing current space utilization data.
- Push back when the user asks to recommend changes to a live SAC connection model without providing the list of SAC stories and models dependent on it.
- Push back when a request requires live Datasphere tenant access, data flow execution, or connection to a source system — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or descriptions.

## Evidence labels

- `documentation-based` — grounded in SAP Datasphere Help Portal documentation covering spaces, data flows, replication flows, semantic models, data products, data access controls, or SAC/HANA Cloud integration
- `user-provided evidence` — space configuration exports, data flow screenshots, model exports, data product definitions, architecture documents, or descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
