# Workflow And Output

The twelve-step sequence and the output contract for a Databricks value case.

## Workflow

1. Identify the pain in the operating business, stated as something a named team currently experiences — not as a missing capability.
2. Identify the executive owner who will be accountable for reporting the outcome and for honouring the kill condition; stop if no named person accepts it.
3. Capture the baseline: the lagging KPI's value before any change, its source, its period, its granularity, and its normal variation. Stop here and instruct baseline capture if it does not exist.
4. Identify the specific Databricks-enabled intervention and state precisely which part of the pain it addresses.
5. Define the leading metric that moves in weeks, and write the one-sentence causal chain linking it to the lagging KPI.
6. Define the lagging business KPI in the organisation's own reporting terms, so that finance recognises it without translation.
7. Identify the data required to compute both metrics, naming the systems and tables, and flag every element that does not yet exist.
8. Enumerate the attribution limits: every concurrent change and external factor that could produce the same movement.
9. Estimate an economic range, tracing each bound to a stated assumption; if no defensible range exists, say so explicitly rather than narrowing to a point.
10. Define the measurement window from the KPI's reporting cadence and known lag, not from the project schedule.
11. Define the kill condition — threshold and date — and record that it was agreed before the intervention began.
12. Re-measure after production, compare realised movement against the predicted range, and report a miss at least as prominently as a hit.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (measurable / measurable-with-conditions / not measurable as stated) with the evidence level behind it.
- The full value case where one is possible: pain, executive owner, baseline with source and period, intervention, leading metric, lagging KPI, required data.
- Attribution limits with every confound named, and an explicit statement of the share of movement the intervention can and cannot claim.
- An economic range with each bound traced to its assumption, or an explicit refusal to produce one and the reason.
- Measurement window, pre-agreed kill condition, re-measurement plan, and the open questions the named owner must resolve.
