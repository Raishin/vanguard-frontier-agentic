# Workflow and output contract — SAP Supply Chain IBP Resilience Review

Use this reference for all IBP resilience finding classification, risk level assignment, remediation path selection, and output formatting.

## IBP resilience finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `demand-planning` | `forecast-bias-unaddressed` | Statistical demand forecast shows persistent positive or negative bias above a material threshold that is tracked but not being corrected through model parameter adjustment or override governance |
| `demand-planning` | `wrong-model-for-demand-pattern` | Forecasting algorithm is mismatched to the demand pattern — smooth demand model applied to intermittent or highly variable demand, or no lifecycle model for new product or end-of-life products |
| `demand-planning` | `consensus-process-governance-gap` | Consensus demand planning process has no structured override governance — demand planners or commercial teams can override statistical forecasts without documented justification, accountability, or accuracy tracking |
| `supply-planning` | `capacity-constraint-not-modeled` | A material production, transport, or storage capacity constraint exists in the physical supply chain but is not represented in the IBP supply planning model — supply plans can be infeasible against the unmodeled constraint |
| `supply-planning` | `single-source-not-in-plan` | A product or location relies on a single source of supply with no alternative source, and no supply planning scenario or contingency supply path is modeled in IBP |
| `supply-planning` | `supply-plan-exception-unresolved` | IBP supply plan exceptions (capacity violation, supply shortfall, lead time breach) are accumulating without a defined resolution workflow or ownership assignment |
| `sop-governance` | `competing-plan-versions` | S&OP process results in multiple unresolved plan versions across functions (finance, supply chain, commercial) without a governance-driven consensus resolution and single approved plan |
| `sop-governance` | `no-escalation-path` | S&OP cadence has no structured escalation path for unresolved volume or capacity gaps — disagreements between functions are not escalated to a decision authority within the S&OP cycle |
| `sop-governance` | `sop-cycle-not-structured` | S&OP meeting cadence exists but is not supported by a structured IBP plan version workflow — meetings discuss plans that are not version-controlled or traceable to the S&OP output |
| `inventory-optimization` | `safety-stock-excludes-lead-time-variability` | Safety stock calculation uses demand variability only and does not account for supplier lead time variability or replenishment cycle uncertainty — safety stock is structurally understated for disruption-exposed supply lanes |
| `inventory-optimization` | `service-level-target-not-stratified` | All products or locations are assigned the same service level target regardless of criticality or margin profile — high-value or critical products are not differentiated from low-priority SKUs in safety stock policy |
| `inventory-optimization` | `stratification-stale` | ABC/XYZ or equivalent inventory stratification has not been refreshed to reflect current demand patterns — products that have changed demand profile (seasonal, ramp-up, decline) retain stale inventory policy parameters |
| `forecast-accuracy` | `accuracy-metric-not-configured` | IBP forecast error metrics (MAPE, WMAPE, bias, tracking signal) are not configured or not reported — forecast quality is invisible to planning management |
| `forecast-accuracy` | `accuracy-not-driving-improvement` | Forecast accuracy metrics are reported but are not linked to a model review or parameter tuning process — persistent low accuracy in a product segment is not triggering a planning improvement response |
| `control-tower` | `alert-coverage-gap` | Material supply chain exception types (demand deviation, inventory below safety stock, supplier delivery failure, transportation delay) are not covered by control tower alert rules — exceptions occur without alert generation |
| `control-tower` | `alert-fatigue` | Alert volume is high relative to actionable exception rate — more than 30% of daily alerts are resolved without action, indicating threshold miscalibration or alert hierarchy design gap |
| `control-tower` | `alert-no-owner` | Control tower alerts are generated but not routed to a defined owner or team — alerts age without resolution and exception patterns are not escalated |
| `scenario-planning` | `disruption-scenario-missing` | High-probability supply disruption scenarios (single-source failure, major geographic event, demand shock) have not been modeled in IBP scenario planning — financial and service level impact is unquantified |
| `scenario-planning` | `scenario-not-in-sop` | Scenario planning outputs are not integrated into the S&OP review process — what-if analysis is conducted outside the governance cycle and does not inform the consensus plan |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Supply plan cannot respond to a high-probability disruption due to single-source dependency with no modeled contingency and no alternative supply path; S&OP producing competing unresolved plans with no governance escalation path and active supply constraint |
| `high` | Persistent forecast bias distorting supply plan at material scale; safety stock not accounting for lead time variability in a disruption-exposed supply lane; capacity constraint not modeled causing infeasible supply plans; critical control tower alert coverage gap masking material exceptions |
| `medium` | Alert fatigue with resolution rate indicating threshold miscalibration; inventory stratification stale; S&OP without structured escalation; scenario planning not integrated with S&OP; consensus demand planning without override governance |
| `low` | Best practice deviation in forecasting model selection or S&OP template design; minor alert rule refinement opportunity; cosmetic inventory stratification adjustment |

## Remediation path decision tree

For each finding:

1. **Is there a single-source supply dependency with no modeled contingency and no alternative supply path for a critical product?**
   - Yes → `critical`. Escalate to supply chain leadership. Add the single-source disruption as an immediate what-if scenario in IBP. Identify and qualify alternative suppliers. Define a contingency supply path and minimum inventory buffer as an interim measure until alternative sourcing is secured.
   - No → continue.

2. **Is there persistent forecast bias above a material threshold that is not being corrected?**
   - Yes → `high`. Review and adjust the statistical forecasting model parameters for the biased product segments. Implement a bias correction adjustment in the consensus demand planning process. Track bias correction effectiveness monthly.
   - No → continue.

3. **Is safety stock excluding lead time variability for a disruption-exposed supply lane?**
   - Yes → `high`. Update the safety stock calculation method to incorporate supplier lead time standard deviation alongside demand variability. Recalculate safety stock targets for affected products and locations. Review service level targets for alignment with the updated calculation.
   - No → continue.

4. **Is a material capacity constraint missing from the supply planning model?**
   - Yes → `high`. Model the missing capacity constraint in IBP supply planning. Re-run supply plan for affected planning buckets and review plan feasibility. Add capacity violation alerts to the control tower for the newly modeled constraint.
   - No → continue.

5. **Is a critical control tower alert coverage gap masking material supply chain exceptions?**
   - Yes → `high`. Define and activate alert rules for the uncovered exception type. Assign an alert owner and resolution workflow for the new alert type. Monitor false positive rate for the first two weeks after activation and recalibrate thresholds if alert fatigue emerges.
   - No → continue.

6. **Is there an alert fatigue pattern (>30% of alerts resolved without action)?**
   - Yes → `medium`. Review alert thresholds for the highest-volume low-action alert types. Raise thresholds to filter noise. Implement an alert hierarchy that promotes only high-severity exceptions to active monitoring dashboards. Track actionable exception rate after recalibration.
   - No → classify as `medium` or `low` and provide guidance.

## Workflow

1. **Receive artifacts** — planning parameter exports, S&OP process documentation, forecast accuracy reports, inventory policy summaries, control tower alert rule descriptions, scenario planning notes, supply plan exception reports, or written descriptions of the IBP planning landscape.
2. **Classify each finding** by IBP planning domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical findings** immediately — single-source supply dependencies with no contingency and infeasible supply plans with no escalation path must be surfaced before other remediation.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical single-source and S&OP governance failures first; then high forecast bias, safety stock gaps, capacity modeling gaps, and control tower coverage gaps; then medium alert fatigue and stratification staleness; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. IBP planning domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected planning dimension (product, location, planning bucket), IBP module (demand planning, supply planning, S&OP, inventory optimization, control tower, scenario planning), and specific configuration or governance gap
5. Quantified impact estimate where data is available (forecast bias percentage, safety stock shortfall, alert noise rate, scenario financial impact gap)
6. Recommended remediation action (forecasting model parameter adjustment, safety stock method update, capacity constraint modeling, alert threshold recalibration, S&OP escalation path design, scenario addition, stratification refresh, etc.)
7. IBP resilience posture after remediation
8. Escalation notice for any critical single-source supply dependency with no contingency — explicit statement that this requires supply chain leadership review and contingency plan activation
9. Prioritized remediation sequence
10. Confirmation that no demand plans were modified, no supply plans were run, no inventory targets were changed, and no alert configurations were activated or modified in this review
