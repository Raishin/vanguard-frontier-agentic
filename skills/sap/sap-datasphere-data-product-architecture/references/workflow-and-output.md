# Workflow and output contract — SAP Datasphere Data Product Architecture Review

Use this reference for all finding classification, risk assignment, remediation path selection, and output formatting.

## Design domain taxonomy

| Domain | Scope | Typical findings |
|--------|-------|-----------------|
| `space-design` | Space structure, storage quota, user and role assignment, connection management | Flat single-space design for multi-domain data, over-permissive space user assignment, missing space naming convention, absent quota limits |
| `data-flows` | Data flow design, transformation operators, scheduling, target table configuration | Missing run error notification, incorrect target table persistence type, missing delta-capable load strategy, scheduling gaps |
| `replication-flows` | Replication flow design, load type selection, delta replication configuration, replication monitor | Incorrect load type for use case (initial-only when delta is needed), missing replication monitor alerting, incompatible source object selection |
| `semantic-models` | Graphical views, SQL views, analytic models, semantic usage, star schema structure | Missing semantic usage annotation, incorrect fact-dimension join key, broken star schema, redundant intermediate views, missing analytic model measures |
| `data-products` | Data product definition, output port configuration, sharing scope | Missing output port definition, overly broad sharing scope, undocumented data product contract, cross-space sharing without data product governance |
| `data-access-controls` | DAC rule definition, criteria entity design, combination method, user assignment | Missing DAC on multi-tenant views, incorrect AND/OR combination method, orphaned DAC rules not assigned to any view, untested DAC value assignments |
| `sac-integration` | SAC live vs. import connections, model type compatibility, refresh scheduling | Live connection to a view without compatible semantic usage, import model refresh scheduling gap, broken SAC model after Datasphere view structural change |
| `hana-cloud-integration` | Remote table access, virtual tables, federation, HANA Cloud storage sharing | Remote table replication vs. federation choice mismatch, missing replication monitoring, virtual access to large tables without partition pruning |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Data isolation breach or access control failure: missing DAC on views exposing multi-tenant or regulated data, unauthorized cross-space access bypassing data product governance |
| `high` | Data architecture failure or pipeline reliability risk: replication flow with incorrect load type causing data loss or duplication, broken analytic model causing SAC story failure, missing output port blocking data product consumption |
| `medium` | Governance gap or modeling quality gap: missing semantic usage annotation, incorrect star schema structure, undocumented data product contract, SAC import model without refresh schedule |
| `low` | Best practice deviation: inconsistent space naming, redundant intermediate views, missing data flow run notification for non-critical pipelines |

## Remediation path decision tree

For each finding:

1. **Is this a missing or misconfigured data access control on a view containing multi-tenant or regulated data?**
   - Yes → `critical`. Add a DAC rule with a correctly defined criteria entity. Assign the DAC to all views and analytic models that expose the regulated data. Verify user DAC value assignments before publishing to SAC.
   - No → continue.

2. **Is this a cross-space data access that bypasses the data product model?**
   - Yes → `critical` or `high` depending on whether the access is unauthorized or simply ungoverned. Define a data product with an explicit output port in the source space. Remove any direct cross-space sharing that was not established via a governed data product.
   - No → continue.

3. **Is this a replication flow with an incorrect load type or a data flow with missing error handling?**
   - Yes → `high`. For replication flows, confirm the correct load type (initial and delta for ongoing sync; initial load only for one-time migration). For data flows, add run error notification and confirm the target table persistence type matches the pipeline intent (truncate and load vs. append).
   - No → continue.

4. **Is this a semantic modeling gap blocking SAC live connection compatibility?**
   - Yes → `high`. Set the correct semantic usage on the view or analytic model (dimension, fact, analytical dataset, or cube). Verify that all measures are correctly typed and that association join keys match dimension key columns.
   - No → continue.

5. **Is this a governance or observability gap?**
   - Yes → `medium`. Add missing data product output port definitions, correct star schema join structures, add SAC import model refresh schedules, or document data product contracts.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — space configuration exports, data flow screenshots, semantic model exports, data product definitions, architecture documents, or user descriptions.
2. **Classify each finding** by design domain and finding type.
3. **Assign risk level** (critical / high / medium / low).
4. **Apply remediation decision tree** per finding.
5. **Prioritize** — critical access control findings first; then high pipeline and modeling failures; then medium governance gaps; then low best-practice items.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Design domain and specific finding type
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended remediation action with specific implementation guidance
5. Data architecture posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live Datasphere tenant access is required to confirm or apply the finding
