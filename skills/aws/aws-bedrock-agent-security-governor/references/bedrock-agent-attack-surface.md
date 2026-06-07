# Bedrock Agent Attack Surface Guide

Use this reference for Amazon Bedrock agents, AgentCore, Guardrails, Knowledge Bases, action groups, memory, tool/MCP integrations, prompt-injection defenses, PII handling, encryption, logging, and least-privilege review.

## What people get wrong

The lazy story is:

> Guardrails plus IAM makes a Bedrock agent safe.

Wrong. Agent risk comes from composition: prompt instructions, retrieved data, memory, tool schemas, action-group permissions, logging, and downstream APIs interact in ways a single control cannot prove safe.

Common bad assumptions:

- Guardrails are an authorization boundary.
- Knowledge Base retrieval is safe if the S3/OpenSearch source is trusted.
- Action groups are safe because Lambda has IAM.
- Memory improves UX without creating poisoning or retention risk.
- Tool descriptions cannot leak sensitive implementation details.
- Model access availability proves agent production readiness.

## Bedrock-agent failure modes

- Prompt injection causes tool calls, data exfiltration, or policy bypass through retrieved context.
- Knowledge Base metadata filters do not enforce user-level authorization.
- Action group Lambda role can mutate or read resources beyond the agent use case.
- Guardrails block output but not downstream side effects already triggered by tools.
- Logs/traces capture prompts, PII, retrieved documents, credentials, or tool payloads.
- Memory or session state stores sensitive data without retention, deletion, or tenant boundary controls.

## Minimum safe workflow

1. Identify agent, model, Region, users, data classification, tools/action groups, knowledge sources, memory, and output channels.
2. Map trust boundaries: user prompt, system prompt, retrieved context, tool schema, Lambda/action group, downstream API, and logs.
3. Review Guardrails as defense-in-depth, not authorization; check denied topics, sensitive information filters, contextual grounding, and intervention telemetry.
4. Verify least privilege for action groups, service roles, knowledge base data sources, KMS keys, and logging destinations.
5. Define adversarial evals: prompt injection, data leakage, unauthorized retrieval, unsafe tool call, refusal, and jailbreak cases.
6. Require observability with redaction: guardrail interventions, tool calls, model errors, token/cost, latency, and audit events.
7. Do not approve production release without eval evidence, rollback/fallback, and owner signoff.

## Verification targets

- Bedrock agent configuration, model, aliases/versions, Guardrails, prompts, orchestration, and action group schemas
- Knowledge Base data source, sync status, metadata filters, vector store access, S3/KMS policies, and tenant/data boundaries
- action group Lambda/API permissions, IAM trust, resource policies, network access, and side-effect controls
- AgentCore runtime/gateway/memory/tool integration boundaries where applicable
- CloudWatch/CloudTrail logs, GenAI observability, prompt/tool redaction, retention, and alerting
- eval suite for injection, leakage, unsafe tool invocation, grounding, PII handling, and cost/latency thresholds

## When to push back

Push back if the user asks to:

- treat Guardrails as a complete security boundary
- allow agent tools to mutate production without approval gates
- use broad Bedrock, Lambda, S3, or KMS permissions
- skip adversarial prompt and retrieval evals
- log raw prompts, retrieved documents, or tool payloads broadly
- claim agent safety from documentation or service availability alone
