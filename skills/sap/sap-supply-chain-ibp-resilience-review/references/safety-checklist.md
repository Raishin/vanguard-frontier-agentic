# Safety checklist — SAP Supply Chain IBP Resilience Review

Use before making any IBP resilience remediation recommendation, especially for findings involving single-source supply dependencies, forecast bias in critical product segments, infeasible supply plans, or missing control tower coverage for material exceptions.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP IBP tenant, SAP S/4HANA system, IBP Excel add-in session, IBP Fiori launchpad, or production supply chain planning database. This skill reviews artifacts only.
- Do not accept or request SAP IBP login credentials, API tokens, IBP Excel add-in connection details, or direct planning database access.
- Do not create, modify, approve, publish, or activate demand plans, supply plans, inventory targets, alert rules, scenario configurations, or S&OP plan versions. There is no planning data mutation in this skill's execution path. Recommendations always describe configuration and process design changes, not direct plan modifications.
- Do not recommend implementing IBP configuration changes (alert threshold edits, safety stock parameter updates, forecasting model changes) directly in a production IBP tenant. All recommendations must first be tested in a non-production IBP environment before promoting to the live planning system.
- Do not use memory alone to assert what forecasting algorithms are active in the user's IBP tenant, what safety stock parameters are configured, or what control tower alert thresholds are set. All findings must be grounded in user-provided artifacts or official SAP IBP documentation.
- Do not conflate forecast accuracy improvement (a model and process improvement action) with forecast override (a one-time correction to a specific period forecast). Overrides address symptoms; model improvement addresses root cause. Distinguish between these when recommending remediation.
- Do not treat alert threshold recalibration as a substitute for alert coverage gap resolution. Raising a threshold to reduce noise is different from adding a missing alert rule for an uncovered exception type. Both may be needed but for different reasons.

## What people get wrong

- **Treating safety stock as a demand-only buffer**: Safety stock that only buffers demand variability (demand standard deviation × service level factor × replenishment lead time) but ignores supplier lead time variability systematically understates the required buffer for supply-side disruption scenarios. The formula must incorporate both demand and supply uncertainty to be structurally complete.
- **Conflating a completed S&OP meeting with a consensus plan**: An S&OP meeting that reviews plans but does not resolve disagreements, close capacity gaps, or produce a single version-controlled approved plan is not a functioning S&OP process regardless of meeting frequency. S&OP governance requires a decision output, not just a discussion forum.
- **Overlooking lifecycle planning gaps for new product introduction**: Demand planning reviews often focus on established product lines with historical data. New product introduction (NPI) and end-of-life (EOL) transitions require lifecycle planning configuration in IBP because statistical algorithms cannot produce useful forecasts without historical data or lifecycle profile guidance. Missing lifecycle planning for NPI/EOL is a common and high-impact demand planning gap.
- **Accepting control tower alert counts as a proxy for control tower effectiveness**: High daily alert counts may indicate good coverage or may indicate threshold miscalibration producing noise. Low alert counts may indicate good calibration or may indicate coverage gaps masking real exceptions. Alert count alone does not assess control tower quality — the actionable exception rate and coverage mapping against known exception types are the meaningful measures.
- **Missing the single-source supply dependency check**: Supply planning reviews often assess whether the plan is feasible against modeled constraints but do not systematically identify single-source dependencies that exist outside the model. A feasible supply plan built on an unmodeled single-source dependency can fail catastrophically in a real disruption even when the plan shows no exceptions.
- **Treating IBP scenario planning as a crisis response tool**: Scenario planning that is only activated during or after a disruption event (reactive) provides limited resilience value. Proactive scenario planning — modeling high-probability disruption scenarios before they occur and integrating the outputs into S&OP — is the governance model that delivers resilience. A reactive-only scenario planning posture is a medium finding regardless of scenario quality.
- **Ignoring forecast bias direction**: Positive systematic bias (forecasting too high) creates excess inventory and working capital consumption. Negative systematic bias (forecasting too low) creates stockouts and service level failures. Both are material, but they create different downstream impacts. Bias direction must be noted when classifying forecast accuracy findings.

## When to push back

- Push back (and escalate to supply chain leadership) when a critical single-source supply dependency with no contingency plan is identified for a product critical to customer service or revenue — do not defer this for a routine planning cycle.
- Push back when the user asks to confirm IBP planning quality from memory alone without providing planning parameter exports, forecast accuracy reports, alert rule summaries, or written descriptions of the IBP configuration.
- Push back when the request requires live IBP tenant access, IBP API calls, or real-time supply plan inspection — state clearly that live inspection is out of scope and ask the user to supply the relevant exports, reports, or summaries.
- Push back when asked to create demand plans, modify supply plans, change inventory targets, activate alert rules, or publish S&OP plan versions — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to confirm that the supply chain is resilient without evidence that single-source dependencies have been modeled in scenario planning and that safety stock incorporates lead time variability.
- Push back when the user presents a single planning horizon or single product segment view as a complete IBP resilience assessment — demand planning, supply planning, inventory optimization, control tower, and scenario planning must all be assessed for a meaningful resilience posture evaluation.

## Evidence labels

- `documentation-based` — grounded in SAP Integrated Business Planning Help Portal documentation, SAP IBP configuration guides, SAP supply chain best practice documentation, or SAP S&OP governance guidance
- `user-provided evidence` — planning parameter exports, S&OP process documentation, forecast accuracy metric reports, inventory policy summaries, control tower alert rule descriptions, scenario planning configuration notes, supply plan exception reports, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
