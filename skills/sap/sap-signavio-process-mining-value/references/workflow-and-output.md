# Workflow and output contract — SAP Signavio Process Mining Value Review

Use this reference for all process mining finding classification, risk level assignment, recommendation selection, and output formatting.

## Process mining finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `event-log-coverage` | `end-to-end-process-not-extracted` | An in-scope end-to-end process (P2P, O2C, R2R, H2R, P2P, etc.) has no event log extracted — entire process is outside the mining scope |
| `event-log-coverage` | `handoff-event-missing` | Event log captures activities within one system but misses handoff events between systems (e.g., Ariba to S/4HANA handoff in P2P) — cross-system flow is invisible |
| `event-log-coverage` | `case-journey-truncated` | Event log case journeys are truncated at an arbitrary extraction window rather than the full process lifecycle — long-running cases appear shorter than they are |
| `event-log-coverage` | `attribute-completeness-gap` | Key case attributes (business unit, plant, material category, vendor category, employee group) are missing from the event log — drill-down and root cause analysis is limited |
| `process-discovery` | `variant-coverage-insufficient` | Reviewed process variants cover less than 80% of cases — the long tail of low-frequency variants contains unexamined behavior that may include exceptions, errors, or compliance risks |
| `process-discovery` | `outlier-variants-dismissed` | Low-frequency process variants are filtered out rather than investigated — potential error patterns, fraud vectors, or compliance exceptions are hidden |
| `process-discovery` | `process-map-too-aggregated` | Investigation process map abstraction level is so high that meaningful step-level analysis is not possible — findings are at too coarse a granularity to drive improvement |
| `conformance` | `no-target-model-defined` | Conformance checking is not configured because no reference process model (SAP Best Practice or customer-defined) has been established as the conformance target — deviation from intent cannot be measured |
| `conformance` | `deviation-severity-not-classified` | Conformance deviations are identified but not classified by severity (critical compliance deviation vs. acceptable variant) — all deviations appear equally important, obscuring the highest-risk ones |
| `conformance` | `root-cause-not-investigated` | High-frequency conformance deviations are identified but not traced to a root cause (missing system control, human behavior, missing automation) — findings are not actionable |
| `bottleneck-analysis` | `waiting-time-not-attributed` | Waiting time between process steps is measured but not attributed to a cause (system wait, resource wait, external dependency) — the bottleneck source is unknown |
| `bottleneck-analysis` | `bottleneck-not-linked-to-impact` | A bottleneck step is identified but its business impact (delay cost, downstream effect, SLA breach rate) is not quantified — prioritization is not evidence-based |
| `bottleneck-analysis` | `simulation-not-used` | Signavio simulation capability is not used to model the expected impact of bottleneck removal — improvement benefits are projected without analytical validation |
| `rework-analysis` | `rework-loop-not-defined` | Rework loops (repeated activities, reversal patterns, re-approval cycles) are not explicitly defined in the investigation — rework rate is unmeasured |
| `rework-analysis` | `rework-rate-not-trended` | Rework rate is measured at a point in time but not tracked over time — improvement initiatives targeting rework reduction cannot demonstrate progress |
| `rework-analysis` | `high-rework-process-not-prioritized` | Process with a high rework rate has not been prioritized for S/4HANA process improvement or automation — rework cost continues without a remediation plan |
| `value-realization` | `initiative-not-registered` | Process mining findings have generated recommendations but no improvement initiative has been formally registered with an owner, timeline, and target KPI — findings do not convert to action |
| `value-realization` | `post-implementation-not-measured` | An improvement initiative identified from mining findings has been implemented but no post-implementation KPI measurement has been performed — outcome is assumed, not confirmed |
| `value-realization` | `business-case-not-tracked` | The process mining program has an estimated ROI or business case but no mechanism to track actual realized value — investment justification cannot be confirmed |
| `s4hana-linkage` | `improvement-linkage-too-generic` | Process mining findings are described as informing the S/4HANA roadmap without identifying a specific SAP Best Practice variant, automation capability, or clean core decision they support |
| `s4hana-linkage` | `automation-candidate-not-identified` | Process mining has identified a high-frequency repetitive activity that is a strong automation candidate (SAP Build Process Automation, iRPA) but no automation initiative has been linked to the finding |
| `s4hana-linkage` | `clean-core-deviation-not-flagged` | Conformance deviations indicate process customizations that conflict with SAP clean core principles, but this has not been flagged to the S/4HANA governance team |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Event log coverage gap that invalidates all downstream findings for an in-scope process; an entire high-value process (P2P, O2C, R2R) entirely outside the mining scope with no compensating analytical coverage |
| `high` | Conformance checking without a target model; bottleneck attribution stops at symptom with no root cause; rework loops unmeasured in a high-cost process; improvement initiative not registered after mining engagement produces findings |
| `medium` | Value realization not tracked post-implementation; S/4HANA linkage too generic; simulation not used for bottleneck impact estimation; variant coverage below 80%; dashboard missing key drill-down capability |
| `low` | Best practice deviation in investigation design; cosmetic KPI or dashboard gap in a low-risk process; single attribute completeness gap that limits one specific drill-down dimension |

## Remediation path decision tree

For each finding:

1. **Is this an event log coverage gap that invalidates all downstream findings for the process?**
   - Yes → `critical`. Halt reliance on downstream findings for this process until the event log is extended. Engage the Signavio implementation team and the source system team to extend the extraction. Clearly communicate to stakeholders that current findings for this process are not reliable.
   - No → continue.

2. **Is conformance checking missing because no target model has been defined?**
   - Yes → `high`. Identify the appropriate reference model (SAP Best Practice process variant from SAP Signavio BPM Suite reference content or a customer-defined target process). Configure conformance checking in the investigation. Classify existing deviations using the newly established reference. This is a prerequisite for all conformance-based recommendations.
   - No → continue.

3. **Is a bottleneck identified but not attributed to a root cause?**
   - Yes → `high`. Drill down into the waiting time breakdown between the bottleneck steps. Attribute to system response time (transaction response, batch job schedule), resource constraint (team capacity, approval queue depth), or external dependency (vendor response, regulatory wait). Link the root cause to a specific remediation lever.
   - No → continue.

4. **Is rework unmeasured in a high-cost process?**
   - Yes → `high`. Define explicit rework loop patterns for the process (e.g., GR reversal in P2P, credit memo creation in O2C, journal entry reversal in R2R). Add rework rate KPIs to the investigation. Trend rework rate over time. Quantify the cost of rework per case to prioritize remediation.
   - No → continue.

5. **Is a process mining finding not linked to a registered improvement initiative?**
   - Yes → `high`. Register the finding as a formal improvement initiative with: responsible owner, target KPI (cycle time reduction %, rework rate %, cost saving estimate), timeline, and post-implementation measurement date. Link the initiative to the process mining investigation that surfaced it.
   - No → continue.

6. **Is the S/4HANA process improvement linkage too generic or an automation candidate not identified?**
   - Yes → `medium`. Map the specific mining finding (high-rework activity, conformance deviation type, bottleneck step) to a specific SAP Best Practice process variant, SAP Build Process Automation template, or clean core remediation action. Document the linkage explicitly so the S/4HANA governance team can act on it.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — event log configuration descriptions, investigation setup summaries, conformance check reports, bottleneck analysis outputs, rework analysis summaries, value tracking dashboard exports, improvement initiative registers, or written descriptions of the Signavio process mining setup.
2. **Classify each finding** by analytical domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag event log coverage gaps that invalidate downstream findings** immediately — halt reliance on downstream analysis for affected processes until coverage is confirmed.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — event log coverage invalidation first; then high conformance, bottleneck attribution, rework, and value realization findings; then medium initiative and linkage gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Analytical domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected process (P2P, O2C, R2R, etc.), investigation name (if provided), specific analytical gap or configuration deficit
5. Recommended remediation action (event log extension, target model definition, conformance severity classification, bottleneck root cause drill-down, rework loop definition, KPI addition, improvement initiative registration, specific S/4HANA process linkage, automation candidate identification, etc.)
6. Process intelligence posture after remediation
7. Escalation notice for any event log coverage gap that invalidates downstream findings — explicit statement that stakeholders should not act on those findings until coverage is confirmed
8. Prioritized remediation sequence
9. Confirmation that no live Signavio tenant was accessed and no process models, mining configurations, or live SAP systems were modified in this review
