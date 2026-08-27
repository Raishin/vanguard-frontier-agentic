# The Value Case Contract

The twelve-step contract every Databricks value case must satisfy, and the failure mode each step exists to prevent.

- Step one is the pain, stated as something a named team currently experiences. A pain written as a missing capability ("we do not have a lakehouse") cannot be measured before or after, which is precisely why it is the most common opening of a value case that never gets settled.
- Step two is the executive owner. The test is budget authority over the affected line, not enthusiasm for the project — an owner without budget authority cannot honour a kill condition, and an initiative whose kill condition cannot be honoured has no kill condition.
- Step three is the baseline, and it is the step that decides whether the remaining nine are worth doing. A baseline needs a source system, a measurement period long enough to show normal variation, and a granularity matching the KPI. Where variation is wide relative to the expected effect, say up front that the effect will not be distinguishable and adjust the metric or the window rather than proceeding.
- Step four names the intervention and the specific part of the pain it addresses. A platform migration that addresses four pains needs four value cases or one explicitly joint case, never one case that quietly borrows the strongest pain's numbers.
- Steps five and six split leading from lagging deliberately: the leading metric exists to give an early kill signal, and the lagging KPI exists to be the thing finance recognises. Collapsing them into one metric removes either the early signal or the business meaning.
- Step seven asks which data is required and which of it does not yet exist. Discovering at measurement time that the KPI was never instrumented is the single most common way a value case silently expires.
- Step eight is attribution, and the honest output is frequently that the effect is not separable. The usual confounds are concurrent releases, seasonality, pricing or packaging changes, headcount changes, marketing spend, and market movement.
- Step nine produces a range, never a point. The width of the range carries the decision information; a point estimate is a range whose assumptions were hidden rather than resolved.
- Step ten sets the measurement window from the KPI's reporting cadence plus its known lag. Declaring success inside the lag window is the most common route from a defensible case to an untrue one.
- Step eleven fixes the kill condition — a threshold and a date — before the intervention starts. Agreeing it afterwards inverts its purpose: it stops being a control and becomes a negotiation.
- Step twelve re-measures in production and compares realised movement to the predicted range. Reporting a miss as prominently as a hit is what makes the next forecast worth anything.
- A value case that fails any of steps two, three, or eleven is not a weaker case — it is not yet a case, and the correct deliverable is the missing step rather than a hedged number.

## Sources

- https://docs.databricks.com/aws/en/lakehouse-architecture/well-architected
- https://docs.databricks.com/aws/en/admin/usage/
