# Observability Rubric

Metric thresholds, interpretation guidance, and escalation matrix for
`salesforce-agentforce-stdm-observer-skill`.

**Note:** Salesforce does not publish official fixed thresholds
for all STDM metrics. Where official thresholds exist, they are cited. Where
they do not, operationally reasonable defaults are provided with a
`[PLACEHOLDER]` annotation. Validate against your
org's baseline before treating any threshold as authoritative.

---

## Faithfulness

Faithfulness measures whether the agent's response is grounded in retrieved
content rather than hallucinated. A faithfulness score of 1.0 means the
response was fully supported by retrieved knowledge; 0.0 means none of it was.

| Score range | Interpretation | Action |
|---|---|---|
| 0.90 – 1.00 | Excellent — responses strongly grounded | No action required |
| 0.80 – 0.89 | Acceptable — minor grounding gaps | Monitor trend; investigate individual low-score moments |
| 0.70 – 0.79 | Warning — meaningful hallucination risk | Hand off to `salesforce-agentforce-risk-review-skill` |
| < 0.70 | Critical — high hallucination rate | Immediate hand off; flag for human review of agent knowledge base |

**Official reference:** Salesforce STDM `avg_faithfulness` is derived from
the `AiRetrieverQualityMetric` DMO. The 0.8 threshold for the `Hallucination`
observability query type is set in `AgentforceOptimizeService`. A threshold
of 0.80 for the `Hallucination` query type is the value used by the
forcedotcom/sf-skills `observing-agentforce` reference implementation. Treat
it as a reasonable default; calibrate to your org's baseline.

**Escalation trigger:** `avg_faithfulness` < 0.80 → hand off to
`salesforce-agentforce-risk-review-skill` with `anomalies_detected` payload.

---

## Answer Relevance

Answer relevance measures whether the agent's response actually addresses the
user's question. A score of 1.0 means fully relevant; 0.0 means unrelated.

| Score range | Interpretation | Action |
|---|---|---|
| 0.85 – 1.00 | Excellent — responses on-topic | No action required |
| 0.70 – 0.84 | Acceptable — occasional off-topic responses | Monitor; review top intents vs. agent topics |
| 0.60 – 0.69 | Warning — frequent off-topic responses | Hand off to `salesforce-agentforce-risk-review-skill` |
| < 0.60 | Critical — agent consistently off-topic | Immediate hand off; likely topic routing configuration issue |

**Official reference:** The `AnswerRelevancy` query type in
`AgentforceOptimizeService` flags subagents with `avg_answer_relevance < 0.7`.
[PLACEHOLDER: Salesforce has not published a canonical
"good" answer relevance target in its public documentation as of 2026-05-21.
The 0.70 threshold in the sf-skills observability query is used here as the
lower bound; 0.85 as the "acceptable" floor reflects operationally reasonable
calibration.]

**Escalation trigger:** `avg_answer_relevance` < 0.70 → hand off to
`salesforce-agentforce-risk-review-skill`.

---

## Context Precision

Context precision measures the quality of RAG retrieval — whether the
retrieved knowledge chunks were relevant to the query.

| Score range | Interpretation | Action |
|---|---|---|
| 0.85 – 1.00 | Excellent — retrieval highly targeted | No action required |
| 0.70 – 0.84 | Acceptable — some irrelevant chunks retrieved | Monitor; may indicate knowledge base over-breadth |
| 0.60 – 0.69 | Warning — retrieval quality degraded | Hand off; review knowledge base indexing and retrieval config |
| < 0.60 | Critical — retrieval largely irrelevant | Immediate hand off; agent responses likely grounded on wrong content |

[PLACEHOLDER: Salesforce STDM documentation does not
specify canonical context precision thresholds. These values are adapted from
RAG evaluation literature and the sf-skills `KnowledgeGap` query type which
reports the lowest-scoring retrievers first.]

**Escalation trigger:** `avg_context_precision` < 0.70 → include in
`anomalies_detected`; hand off to `salesforce-agentforce-risk-review-skill`
with `KnowledgeGap` observability query results.

---

## Quality Score (1–5 Scale)

The STDM quality score is a 1–5 integer assigned per moment by an LLM
evaluator. It maps to UI labels in Agentforce: 5=High, 3-4=Medium, 2=Low,
1=Very Low.

| avg_quality_score | Interpretation | Action |
|---|---|---|
| 4.5 – 5.0 | Excellent | No action required |
| 4.0 – 4.4 | Good | Monitor |
| 3.0 – 3.9 | Moderate — investigate low-scoring moments | Review quality_distribution; identify dominant failure mode |
| 2.0 – 2.9 | Poor | Hand off to `salesforce-agentforce-risk-review-skill` |
| < 2.0 | Critical | Immediate escalation; agent may be actively harming user experience |

**Quality distribution signal:** A bimodal distribution (many 5s and many 1s)
suggests inconsistent behavior that is harder to diagnose than a uniformly
low score. Both patterns warrant escalation.

**Escalation trigger:** `avg_quality_score` < 4.0 → include in
`anomalies_detected`; investigate quality_distribution for dominant failure
category.

---

## Action Invocation Error Rate

The action error rate is computed as:
`error_rate = action_error_count / action_invocation_count`

| Error rate | Interpretation | Severity | Action |
|---|---|---|---|
| 0% – 2% | Normal — transient errors within tolerance | Low | No action required |
| 2% – 5% | Elevated — investigate action reliability | Medium | Monitor trend; review action error messages |
| 5% – 10% | High — action reliability issue | High | Hand off to `salesforce-agentforce-risk-review-skill` |
| > 10% | Critical — systemic action failure | Critical | Immediate hand off; agent may be failing to complete tasks |

**Escalation trigger:** `error_rate` > 0.05 → hand off to
`salesforce-agentforce-risk-review-skill` with error breakdown payload.

---

## Session Abandonment Rate

Abandonment rate = sessions that ended without the agent completing the user's
intent (proxied by `UNKNOWN` end_type or sessions shorter than a minimum
meaningful duration).

| Abandonment rate | Interpretation | Action |
|---|---|---|
| 0% – 15% | Normal | No action required |
| 15% – 30% | Elevated | Investigate top intents for dead-ends; check action availability |
| 30% – 50% | High | Hand off; agent likely has missing actions or broken routing |
| > 50% | Critical | Immediate escalation; agent not usable in production |

[PLACEHOLDER: Salesforce does not publish canonical
abandonment thresholds. These ranges are operationally calibrated and should
be adjusted to org-specific baseline. An org with a narrow, self-service use
case may have a naturally lower abandonment rate than a broad-intent agent.]

**Escalation trigger:** `abandonment_rate` > 0.30 → include in
`anomalies_detected` with severity `high`.

---

## Faithfulness Sudden Drop (Regression Detection)

A sudden drop in `avg_faithfulness` between two consecutive observation
windows (even if the absolute value remains above threshold) is an anomaly
signal:

| Drop magnitude (vs. prior window) | Severity | Action |
|---|---|---|
| < 5% decrease | Normal variation | No action |
| 5% – 15% decrease | Warning | Note in anomalies; observe next window |
| > 15% decrease | High | Hand off; potential knowledge base change or agent version drift |
| Faithfulness drops from > 0.85 to < 0.75 in one window | Critical | Immediate hand off regardless of absolute value |

**Hallucination signature patterns:**
- `avg_faithfulness` drops sharply while `avg_answer_relevance` remains
  stable → agent is generating confident but ungrounded responses; likely
  a retrieval failure or knowledge base gap.
- Both `avg_faithfulness` and `avg_answer_relevance` drop → agent may have
  been updated or its topic routing changed; compare against prior config.
- Quality score distribution shifts toward 1-2 with no change in error rate
  → likely LLM generation quality degraded (possible model change).

---

## Escalation Matrix

| Condition | Severity | Downstream skill | Payload |
|---|---|---|---|
| avg_faithfulness < 0.80 | High | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics, Hallucination observability results |
| avg_answer_relevance < 0.70 | High | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics, AnswerRelevancy results |
| avg_context_precision < 0.70 | Medium | `salesforce-agentforce-risk-review-skill` | audit_envelope, KnowledgeGap results |
| action error_rate > 0.05 | High | `salesforce-agentforce-risk-review-skill` | audit_envelope, error breakdown |
| action error_rate > 0.10 | Critical | `salesforce-agentforce-risk-review-skill` + notify Live Guard | audit_envelope, error breakdown, change_proposal flag |
| abandonment_rate > 0.30 | High | `salesforce-agentforce-risk-review-skill` | audit_envelope, aggregate_metrics |
| avg_quality_score < 4.0 | Medium | `salesforce-agentforce-risk-review-skill` | audit_envelope, quality_distribution |
| Faithfulness regression > 15% in one window | High | `salesforce-agentforce-risk-review-skill` | audit_envelope, regression signal |
| A configuration change is proposed | Any | `salesforce-live-guard-agent` | audit_envelope, change_proposal |
| Regulated-vertical org + any anomaly | Any | `salesforce-compliance-privacy-agent` | audit_envelope, anomalies_detected, vertical_flag |

---

## Composite Health Score

When reporting overall agent health, compute a composite score from
the component metrics. This is advisory only; the threshold is not authoritative.

```
composite_health = (
    (avg_faithfulness     * 0.35) +
    (avg_answer_relevance * 0.30) +
    (avg_context_precision * 0.15) +
    ((avg_quality_score / 5.0) * 0.10) +
    ((1.0 - error_rate) * 0.05) +
    ((1.0 - abandonment_rate) * 0.05)
)
```

| Composite health | Interpretation |
|---|---|
| 0.90 – 1.00 | Excellent |
| 0.80 – 0.89 | Good |
| 0.70 – 0.79 | Acceptable — monitoring recommended |
| 0.60 – 0.69 | Concerning — hand off to risk-review |
| < 0.60 | Poor — immediate escalation |

[PLACEHOLDER: The composite weighting above is an
operationally calibrated heuristic. Adjust weights to reflect your org's
business priorities — e.g., a regulated financial services agent may weight
faithfulness more heavily than context precision.]
