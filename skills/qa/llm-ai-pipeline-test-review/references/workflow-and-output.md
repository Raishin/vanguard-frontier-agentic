# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no API keys, no model weights, no real user PII — replace with placeholders):
- Evaluation configuration files (DeepEval `test_*.py`, RAGAS config, custom eval scripts)
- Golden dataset samples or references to a golden dataset (path, size, last-updated date)
- CI step that runs evals (workflow YAML, script, or description of the gate)
- The metric list and threshold values in use (even if embedded in test files)
- For RAG pipelines: retrieval configuration (vector store, top-k, similarity threshold)
- Optional: recent eval run report or score history showing metric trends

If CI gating configuration is not provided, regression-gate findings are stated as `inference` — say so and ask for it.
If threshold values are not provided, threshold-governance findings are stated as `inference`.

### Step 2 — Hallucination and factual correctness audit

Confirm the eval measures whether the model's claims are factually grounded.

```python
# HIGH — no hallucination check; fabrications pass the suite undetected
test_cases = [LLMTestCase(input=q, actual_output=answer)]
# no HallucinationMetric or GEval with factuality criteria

# Correct — hallucination measured against source
hallucination_metric = HallucinationMetric(threshold=0.2)
dataset = EvaluationDataset(test_cases=[
    LLMTestCase(input=q, actual_output=answer, context=[source_doc])
])
assert_test(dataset, [hallucination_metric])
```

Check for:
- Presence of `HallucinationMetric` or a GEval with `"factual consistency"` / `"faithfulness to source"` criteria
- Whether `context` (source documents) is provided to the metric — without it, the metric cannot detect contradiction
- Whether a golden dataset with expected answers exists for regression comparisons

### Step 3 — Answer relevancy and faithfulness audit (RAG focus)

For all pipelines, confirm responses address the input. For RAG pipelines, confirm outputs are grounded in retrieved context.

```python
# MEDIUM — relevancy not measured; off-topic responses pass
# missing AnswerRelevancyMetric

# HIGH — RAG pipeline without faithfulness check; model can ignore retrieved docs
# missing FaithfulnessMetric with retrieved_contexts

# Correct — both relevancy and faithfulness measured
relevancy = AnswerRelevancyMetric(threshold=0.7)
faithfulness = FaithfulnessMetric(threshold=0.7)
test_case = LLMTestCase(
    input=query,
    actual_output=answer,
    retrieval_context=retrieved_docs
)
```

Check for:
- `AnswerRelevancyMetric` present for any conversational or Q&A pipeline
- `FaithfulnessMetric` present for any RAG pipeline — this is the primary RAG correctness signal
- `ContextualPrecisionMetric` and `ContextualRecallMetric` for RAG pipelines measuring retrieval quality
- Whether `retrieval_context` is populated in test cases — an empty context silently disables the metric

### Step 4 — Safety metrics audit (bias, toxicity)

Confirm the eval catches unsafe outputs before they reach users.

```python
# HIGH (CRITICAL for vulnerable audiences) — no safety guardrails in eval
# missing BiasMetric and ToxicityMetric

# Correct — safety metrics applied
bias_metric = BiasMetric(threshold=0.5)
toxicity_metric = ToxicityMetric(threshold=0.5)
```

Check for:
- `BiasMetric` present for any user-facing system
- `ToxicityMetric` present for any user-facing system
- Threshold values reviewed for the deployment context — a threshold appropriate for an adult content filter may be too permissive for a children's education tool
- Whether bias and toxicity metrics are in the gating suite or are only advisory/non-blocking

### Step 5 — Adversarial and red-team coverage audit

Confirm the eval includes adversarial inputs, not only happy-path test cases.

```python
# CRITICAL for agentic / HIGH for others — no adversarial cases
test_cases = [LLMTestCase(input=normal_query, actual_output=answer)]
# only benign inputs; no prompt-injection attempts, no jailbreaks

# Correct — red-team dataset included
adversarial_cases = load_dataset("adversarial_prompts.json")
```

Check for:
- Presence of adversarial test cases or a red-team dataset (prompt-injection attempts, jailbreak patterns, boundary inputs)
- For agentic systems: test cases that verify the agent refuses or handles malicious tool-calling instructions
- Whether adversarial cases are rotated periodically — a static adversarial set becomes predictable (pesticide paradox)
- Whether adversarial inputs cluster around the topic or domain boundaries of the deployment (defect clustering)

### Step 6 — Agent-specific metrics audit (tool correctness, task completion)

For pipelines that include LLM agents, confirm the eval measures agent behavior, not only text quality.

```python
# HIGH — agent evals check only output text; wrong tool calls pass undetected
# missing ToolCorrectnessMetric

# HIGH — multi-step agent eval has no end-to-end success signal
# missing TaskCompletionMetric

# Correct — both agent metrics present
tool_correctness = ToolCorrectnessMetric
task_completion = TaskCompletionMetric(threshold=0.8)
agent_test_case = LLMTestCase(
    input=user_request,
    actual_output=final_answer,
    tools_called=agent_tool_log,
    expected_tools=["search", "summarize"]
)
```

Check for:
- `ToolCorrectnessMetric` present when an agent selects or calls tools
- `TaskCompletionMetric` present for multi-step agentic workflows
- Whether `tools_called` is logged and passed to tool metrics — without the log the metric cannot evaluate tool use
- Whether task completion is defined and measurable for the specific agent goal

### Step 7 — Threshold governance and non-determinism audit

Confirm thresholds are meaningful and results are statistically reliable.

```python
# HIGH — threshold of 0 means every output passes; the metric is decorative
HallucinationMetric(threshold=0)

# MEDIUM — single run on a non-deterministic model; one lucky sample masks failures
result = evaluate(dataset, metrics=[hallucination_metric])

# Correct — multiple runs aggregated; threshold domain-reviewed
scores = [evaluate(dataset, metrics=[hallucination_metric]).scores for _ in range(5)]
mean_score = sum(scores) / len(scores)
# threshold=0.2 reviewed by a domain expert for this medical-chatbot use case
```

Check for:
- Any threshold set to 0 or left at default without documented review — flag as HIGH
- Whether thresholds are documented with a rationale (use case, acceptable failure rate, domain expert sign-off)
- Whether multi-run aggregation (pass@k, mean score over N runs) is used for non-deterministic outputs
- Whether thresholds differ appropriately across deployment contexts (production vs. staging, medical vs. entertainment)

### Step 8 — Regression gate audit

Confirm the eval detects when a model update silently degrades quality.

```python
# HIGH — no baseline; a new model can score worse than the old one and ship
evaluate(dataset, metrics=[hallucination_metric])
# no comparison to previous run scores

# Correct — baseline scores recorded and compared
baseline = load_baseline("eval_baseline_v1.json")
current = evaluate(dataset, metrics=[hallucination_metric])
assert current.score >= baseline.score - ALLOWED_REGRESSION
```

Check for:
- A golden dataset that is versioned and stable enough to detect regression
- Baseline scores stored from prior runs and compared against current runs
- CI or eval step that fails when scores drop below the baseline by more than an allowed delta
- Whether the golden dataset is ever refreshed — a dataset that never changes stops finding new defect categories (pesticide paradox); rotate or supplement it with synthetic data periodically

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: eval suite gates unsafe outputs / eval runs but gates nothing / partial coverage with gaps>

## Evidence level
<eval config + test scripts provided | eval config only | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation>

### HIGH
- [H1] <finding>: <description> — <remediation>

### MEDIUM
- [M1] <finding>: <description> — <remediation>

### LOW
- [L1] <finding>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept model API keys, inference endpoint URLs, or model weights. Ask for sanitized eval configuration with placeholders.
- Never call LLM APIs, run evaluations, or contact inference endpoints — this is a static review only.
- Do not accept eval fixtures containing real user PII or private prompt chains; ask the user to anonymize them first.
- A metric with threshold=0 is functionally disabled — it is the eval equivalent of `continue-on-error: true` on a test step. Lead with it when present.
- Bias and toxicity without thresholds reviewed for the actual audience are a false signal of safety; flag the gap and ask what the audience is.
- Adversarial coverage is the most commonly absent category; absence is not evidence that the model is robust — it is evidence the question was never asked.
