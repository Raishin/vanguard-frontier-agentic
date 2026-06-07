# OCI MySQL HeatWave AI Specialist Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Running writes or data loads before proving scope and sensitivity.
- Treating AI/RAG output as correct without source freshness and evaluation evidence.
- Assuming every HeatWave feature is available on every version, shape, or platform.
- Loading object storage data without access, retention, lineage, and rollback review.

## Officially grounded service shape

- Official OCI documentation describes MySQL HeatWave as managed MySQL with transactions, analytics, machine learning, GenAI, and Lakehouse capabilities.
- Official OCI documentation describes HeatWave clusters as nodes that store data in memory and process queries while the DB system manages scheduling and result return.
- Official documentation for MySQL HeatWave GenAI describes in-database vector store and RAG routines, but deployment-specific support can vary by environment and version.
- OCI API evidence through the user’s configured read-only OCI MCP shows MySQL DB system listing is compartment-scoped and can filter by lifecycle state, configuration, Database Management setting, update state, HeatWave cluster attachment, sorting, and pagination.

Documentation evidence proves documented service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Microsoft Learn documentation through the user's configured documentation MCP can prove documented Azure behavior. None of these prove broad tenancy/subscription posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Default to read-only metadata, schema, and configuration evidence.
- Require explicit approval before SQL writes, loads, deletes, vector ingestion, model changes, or production config changes.
- Separate DB system state, HeatWave attachment, version, shape, Lakehouse/GenAI support, data sensitivity, and evaluation evidence.
- Do not expose connection strings, passwords, schema dumps with customer data, object names, payloads, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm DB system, workload, feature, data source, sensitivity, and requested decision using sanitized labels.
- Use official docs for HeatWave/Lakehouse/GenAI behavior and sampled read-only evidence for API shape/current DB state.
- Assess feature support, data access, SQL risk, object storage permissions, lineage, evals, and rollback.
- Return verdict, blockers, safe queries, approved next actions, and validation checks.

## High-risk assumptions to kill

- “HeatWave attached means GenAI/vector is ready.”
- “RAG answer quality is proven by one demo.”
- “Object storage load is read-only.”
- “Schema metadata has no sensitive information.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check DB lifecycle, HeatWave attachment, version, configuration, and feature support without exposing identifiers.
- Review schema/data sensitivity, source freshness, ingestion path, permissions, and rollback.
- Validate SQL as read-only unless explicit approval exists.
- Evaluate RAG/AI answers against source documents and test cases.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to run SQL writes or loads without approval.
- Feature support is inferred from marketing rather than current version/shape evidence.
- Evidence includes secrets, connection strings, data dumps, or customer payloads.
