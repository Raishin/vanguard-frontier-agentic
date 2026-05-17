---
name: llm-ai-pipeline-test-review
description: Use this skill when reviewing how an LLM or AI pipeline is evaluated — metric selection, golden datasets, threshold governance, adversarial coverage, and regression gating — to determine whether low-quality or unsafe model outputs can ship undetected. Trigger when a user provides evaluation configuration files, DeepEval or RAGAS test scripts, eval CI steps, or asks whether their AI pipeline actually prevents a bad model from reaching production. This skill reviews evaluation setup statically; it does not call LLM APIs, run evaluations, or contact inference endpoints.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: ai
  lifecycle: experimental
---

# LLM AI Pipeline Test Review

## Purpose
This skill reviews how an LLM or AI pipeline is evaluated — not the model itself, but the evaluation setup that decides whether a model change is safe to ship. An evaluation suite only protects users if it measures the right things, gates on meaningful thresholds, covers adversarial inputs, and detects drift across model versions. The review catches missing hallucination and factuality metrics, absent answer-relevancy and faithfulness checks for RAG pipelines, unguarded bias and toxicity, no adversarial or red-team coverage, agent evals that ignore tool correctness and task completion, thresholds that are undefined or set to zero, single-shot evals on non-deterministic outputs, and no regression baseline to detect metric drift.

## Lean operating rules

- Treat a RAG or summarisation pipeline with no `HallucinationMetric` or no GEval with factuality criteria against source documents as HIGH — the pipeline can fabricate facts and ship them.
- Treat a pipeline with no golden dataset (fixed reference set for regression) as HIGH — metric drift across model versions is undetectable.
- Treat the absence of `AnswerRelevancyMetric` as MEDIUM — responses may be fluent but off-topic, and no eval catches it.
- Treat a RAG pipeline with no `FaithfulnessMetric` as HIGH — the model can ignore retrieved context and hallucinate; faithfulness is the primary RAG correctness signal.
- Treat missing `ContextualPrecisionMetric` or `ContextualRecallMetric` in a RAG pipeline as MEDIUM — retrieval quality is unmeasured; noisy or incomplete retrieval is invisible to the eval.
- Treat the absence of `BiasMetric` or `ToxicityMetric` as HIGH if the system is user-facing — unsafe outputs can reach users without detection; treat as CRITICAL if the audience is vulnerable (children, medical patients, crisis users).
- Treat no adversarial test cases and no red-team dataset as CRITICAL for agentic systems; HIGH for all other user-facing LLM products — prompt-injection and jailbreak paths are untested.
- Treat agent evals with no `ToolCorrectnessMetric` as HIGH — the agent can call wrong tools silently and the eval still passes.
- Treat multi-step agent evals with no `TaskCompletionMetric` as HIGH — end-to-end success is unmeasured even if individual steps look fine.
- Treat metric thresholds that are undefined, set to 0, or not reviewed by a domain expert as HIGH — a threshold of 0 means every output passes; an unreviewed threshold is a guess.
- Treat evals that run only once per input on non-deterministic outputs (no pass@k or mean-score aggregation across multiple runs) as MEDIUM — a single lucky sample masks systematic failure.
- Treat the absence of a golden dataset or scoring baseline that would detect metric regression across model versions as HIGH — a model update can silently degrade quality.
- Treat static golden datasets that have never been rotated or supplemented with synthetic adversarial data as MEDIUM — a suite that tests the same inputs repeatedly stops finding new defects (the pesticide paradox).
- Apply thresholds contextually: a faithfulness score of 0.7 may be acceptable for a joke generator and unacceptable for a medical chatbot — flag any threshold that appears copied from a tutorial without domain justification.
- Define eval metrics early in the model selection process, not after a model is chosen — catching defects before model selection is always cheaper than retrofitting evals.
- Label every finding with evidence basis: eval config provided, test script provided, documentation-based, or inference.
- Static review only — read eval configs and test source; never call LLM APIs, never run evaluations, never request model API keys or inference endpoints.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Hallucination and factual correctness findings
- Answer relevancy and faithfulness findings (especially for RAG pipelines)
- Safety metric findings (bias, toxicity)
- Adversarial and red-team coverage findings
- Agent-specific metric findings (tool correctness, task completion)
- Threshold governance and non-determinism findings
- Regression gating findings (golden dataset, baseline)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
