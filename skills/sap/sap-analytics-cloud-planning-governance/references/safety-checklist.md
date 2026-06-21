# Safety checklist — SAP Analytics Cloud Planning Governance Review

Use before making any remediation recommendation, especially for findings that affect planning version locking, data action execution, allocation design, or role-based access.

## Non-negotiables

- Do not access, connect to, or request access to any live SAC tenant, model, data action, or connected source system. This skill reviews artifacts only.
- Do not accept or request SAC tenant credentials, OAuth tokens, API keys, model data exports containing financial data, or dimension member lists that may contain personal or organizational data.
- Do not recommend unlocking a locked actuals version. Locked actuals versions represent closed-period data and must not be unlocked without a documented change control process involving data owners and finance governance.
- Do not recommend executing a data action or allocation on a production planning model based on review alone. All data action changes must be tested in a development or quality model before production deployment.
- Do not recommend changing the account dimension type or date dimension granularity of an existing planning model with active data. These are foundational model properties — changing them requires model rebuild and data migration, not in-place configuration change.
- Do not recommend granting BI Admin role as a workaround for access issues. BI Admin provides tenant-wide administration rights. The correct remedy for access gaps is a scoped custom role, not elevated built-in role assignment.
- Do not classify a finding as `critical` without being able to trace the specific planning integrity breach or unauthorized access path from user-provided artifacts or official documentation.

## What people get wrong

- **Treating data actions and multi-actions as equivalent**: Data actions contain steps that execute as a single unit within one model. Multi-actions orchestrate multiple data actions, potentially across models, with independent execution and separate failure points. Sequencing recommendations for data action steps do not apply at the multi-action orchestration level and vice versa.
- **Recommending live connection for planning models**: SAC planning models cannot be backed by a live connection — they require an import connection or are created natively in SAC. Recommending a live connection for a planning model is incorrect; live connections apply only to analytics-only models.
- **Assuming all dimension member restrictions apply automatically to all story users**: Dimension member-level data access restrictions in SAC apply to teams, not directly to individual users. A user must be a member of a correctly configured team to inherit dimension member restrictions. Recommending restrictions without confirming the team membership model is incomplete.
- **Conflating version locking with model locking**: Version locking prevents writes to a specific planning version (e.g., actuals). Model locking prevents any user from editing or running data actions on the entire model. They are different controls with different operational consequences.
- **Missing the impact of model structure changes on active stories**: Changes to the planning model (removing measures, renaming dimensions, changing account hierarchy) immediately break any SAC story that references the changed elements. There is no graceful degradation — stories display errors. Model changes must be coordinated with story owners before deployment.
- **Recommending full refresh for large import models where incremental load is available**: Full refresh replaces all model data on each run and is appropriate for small models. Large import models (millions of rows) should use incremental (delta) load where the source connection supports it. Recommending full refresh without confirming model size and refresh window creates data availability gaps.

## When to push back

- Push back when the user asks to confirm a data action correctness finding from a description alone without providing the data action script or step configuration.
- Push back when the user asks for specific allocation amounts or formula values — this skill reviews governance and design, not planning content.
- Push back when the user asks to recommend unlocking actuals or deploying data action changes to a production model without a development/test validation step.
- Push back when a request requires live SAC tenant access, model data inspection, data action execution, or allocation run — state clearly that live inspection is out of scope and ask the user to supply the relevant configuration exports or descriptions.

## Evidence labels

- `documentation-based` — grounded in SAP Analytics Cloud Help Portal documentation covering stories, planning models, data actions, allocations, connections, or access control
- `user-provided evidence` — story screenshots, model export files, data action scripts, planning model configuration summaries, connection descriptions, or architecture documents provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
