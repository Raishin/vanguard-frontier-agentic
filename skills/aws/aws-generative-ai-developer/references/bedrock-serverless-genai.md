# Bedrock Serverless GenAI Guide

Use this reference for Amazon Bedrock serverless applications, RAG flows, prompt orchestration, Guardrails, Lambda/API Gateway/Step Functions integrations, and production-readiness gaps.

## What people get wrong

The lazy story is:

> Bedrock is managed, so the app is safe if IAM is scoped and the model works.

Wrong. Managed inference does not solve prompt injection, data boundary, retrieval leakage, token cost, evaluation, observability, or fallback design.

Common bad assumptions:

- Guardrails replace application authorization and data filtering.
- A model invocation test proves production behavior.
- RAG quality is a vector database problem only.
- Prompt/version changes do not need release control.
- Cross-Region inference profiles are just a performance feature, not a residency and cost decision.
- CloudWatch GenAI observability exists for the app unless explicitly enabled and scoped.

## GenAI-specific failure modes

- User prompt or retrieved context overrides system intent or tool boundaries.
- Knowledge base retrieves unauthorized, stale, or ungrounded content.
- Lambda/API Gateway timeout, payload, streaming, or concurrency limits are ignored.
- Token usage, retries, and long context windows create unbounded cost.
- Logs capture prompts, documents, PII, secrets, or customer data without retention controls.
- Model, Region, guardrail, prompt, or embedding changes are not versioned or evaluated.
- Fallback path silently downgrades quality, safety, or data residency.

## Minimum safe workflow

1. Identify the use case, data classification, user roles, model/Region, latency target, and cost boundary.
2. Choose the simplest serverless shape: API Gateway or AppSync, Lambda, Step Functions for orchestration, EventBridge/SQS for async, S3/DynamoDB for state, and Bedrock managed capabilities.
3. Define prompt, guardrail, retrieval, tool, and output contracts before implementation.
4. Add explicit evaluation criteria: golden prompts, refusal cases, retrieval-grounding checks, latency, token budget, and safety regressions.
5. Design observability with redaction: token counts, latency, model errors, guardrail interventions, retrieval IDs, and user-impact metrics.
6. Require IAM/KMS/network/data-retention boundaries for prompts, documents, embeddings, logs, and outputs.
7. Keep deploy, model access changes, guardrail publishing, and data-source ingestion approval-gated.

## Verification targets

- Bedrock model, Region, inference profile, guardrail, prompt/version, and invocation path
- Knowledge Base data source, chunking, metadata filters, embedding model, sync status, and authorization boundary
- Lambda/API Gateway/Step Functions timeout, payload, streaming, concurrency, retry, and error handling
- IAM permissions for `bedrock:*` actions narrowed by model/resource where practical
- KMS, S3, DynamoDB, CloudWatch Logs retention, VPC/private connectivity, and data residency settings
- eval set, expected outputs, refusal cases, retrieval citations, and regression threshold
- cost controls: token budget, max output tokens, retry limits, quotas, and alerting

## When to push back

Push back if the user asks to:

- ship a GenAI app without prompt/retrieval safety evals
- treat Guardrails as a complete security boundary
- log raw prompts or retrieved documents broadly
- use broad Bedrock/IAM permissions for convenience
- ignore token/cost limits during retries or long conversations
- claim model availability proves app readiness
