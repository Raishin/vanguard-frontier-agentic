# Workflow and output contract — SAP Analytics Cloud Planning Governance Review

Use this reference for all finding classification, risk assignment, remediation path selection, and output formatting.

## Design domain taxonomy

| Domain | Scope | Typical findings |
|--------|-------|-----------------|
| `stories` | Story layout, widget binding, filter configuration, calculated measures | Incorrect model binding, filter scope too broad or too narrow, excessive page count, missing input control for planning tables |
| `planning-models` | Version category structure, dimension design, account dimension, date dimension, measure types | Missing actual version lock after period close, incorrect account dimension type, absence of rolling forecast version, incorrect date dimension granularity |
| `data-actions` | Step type selection, sequencing, Advanced Formula scope, trigger and schedule configuration | Incorrect step order causing data overwrite before read, unbounded MEMBERSET scope, missing trigger configuration, no multi-action failure handling |
| `allocations` | Allocation method selection, driver dimension, hierarchy scope, validation approach | Incorrect allocation method for use case (spread vs. distribution vs. breakback), unfiltered hierarchy traversal on large model, no result validation step |
| `value-driver-trees` | Node formula correctness, model binding, hierarchy connectivity | Broken node formula referencing deleted measure, disconnected VDT node, incorrect measure aggregation in VDT formula |
| `connections` | Live vs. import connection selection, import refresh schedule, incremental load | Live connection to incompatible source object, import model without refresh schedule, full refresh where incremental load is supported and preferred |
| `data-access-controls` | Role assignment scope, team data access, dimension member restrictions | BI Admin assigned as default role, planner access granted to view-only users, missing dimension member restriction on regulated data, ungoverned public story sharing |
| `performance` | Widget density, query count, model size, import model caching | Story with excessive widget count per page, live connection story with no default page filter, import model queried without caching strategy |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Planning integrity breach or unauthorized data access: missing version lock on actuals allowing post-close changes, BI Admin access granted to non-admin users, missing dimension-level data restriction on regulated financial data |
| `high` | Planning model failure or data action correctness error: data action step sequencing error producing incorrect plan data, live connection to incompatible source breaking story load, allocation with unfiltered hierarchy causing timeout |
| `medium` | Governance gap or performance risk: import model without refresh schedule, planning model without rolling forecast version, story with excessive widget density causing slow load |
| `low` | Best practice deviation: missing naming convention on versions, undocumented data action trigger schedule, VDT with no description on nodes |

## Remediation path decision tree

For each finding:

1. **Is this a missing version lock on an actuals version after the period has closed?**
   - Yes → `critical`. Lock the actuals version immediately. Audit all data action triggers that write to the actuals version and restrict them to authorized users only. Enable version change history to detect if unauthorized edits occurred during the unlocked window.
   - No → continue.

2. **Is this an unauthorized role assignment (BI Admin as default, Planner where Viewer is correct)?**
   - Yes → `critical` or `high` depending on the data sensitivity. Reduce the role to the minimum required (Viewer for read-only consumers, custom scoped Planner role for authorized contributors). Remove generic BI Admin assignments from users who do not require tenant administration.
   - No → continue.

3. **Is this a data action step sequencing error or unbounded Advanced Formula scope?**
   - Yes → `high`. For sequencing errors, reorder steps so read steps precede write steps for any shared data region. For unbounded scope, add explicit MEMBERSET restrictions to limit formula execution to the intended version, entity, and time dimensions.
   - No → continue.

4. **Is this a live connection to an incompatible source object type?**
   - Yes → `high`. Confirm the source object type against the SAC live connection compatibility matrix. Convert to a compatible source object (for Datasphere: analytic model or analytical dataset; for BW: BEx query or Composite Provider with defined characteristics and key figures) or switch to import connection if live is not required.
   - No → continue.

5. **Is this a governance or performance gap?**
   - Yes → `medium`. Define import model refresh schedules, add rolling forecast version to planning model, apply default page filters to live connection stories, or add dimension member restrictions to regulated data models.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — story screenshots, model configuration exports, data action scripts, planning model configuration summaries, connection descriptions, or user descriptions.
2. **Classify each finding** by design domain and finding type.
3. **Assign risk level** (critical / high / medium / low).
4. **Apply remediation decision tree** per finding.
5. **Prioritize** — critical planning integrity and access control findings first; then high model and data action failures; then medium governance and performance gaps; then low best-practice items.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Design domain and specific finding type
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended remediation action with specific implementation guidance
5. Planning governance posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live SAC tenant access, model refresh, or data action execution is required to confirm or apply the finding
