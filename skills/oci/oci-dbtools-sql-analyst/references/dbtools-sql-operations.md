# OCI Database Tools SQL Analyst Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating SQL Worksheet as harmless because it is web-based.
- Running SELECT * on sensitive tables without minimization.
- Assuming a connection is read-only because the task is analytical.
- Downloading result sets without data classification and retention rules.
- Using live SQL when documentation or metadata is enough.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows Database Tools connection list operations expose compartment, lifecycle state, display name, type, runtime support, runtime identity, related resource, sorting, and pagination filters. Treat this as API shape evidence, not proof a connection is safe to query.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Default to metadata and narrowly scoped read-only queries.
- Confirm connection, database type, schema, data sensitivity, row limits, and business purpose before SQL execution.
- Reject destructive DDL/DML unless explicitly approved, backed up, and outside this read-only analysis default.
- Never expose credentials, connection strings, secrets, or sensitive result sets.
- Label live query output, metadata, documentation, and inference separately.

## Minimal safe implementation flow

- Confirm connection and analysis objective.
- Review connection metadata and runtime identity before querying.
- Prefer metadata/table-info and bounded projections over broad result sets.
- Apply row limits, predicates, masking, and aggregation for sensitive data.
- Return findings, query caveats, data-handling notes, and safer follow-up.

## High-risk assumptions to kill

- “A database connection implies permission to inspect all data.”
- “Read-only SQL cannot cause harm.”
- “Small result sets cannot contain sensitive data.”
- “A report definition is safe because it already exists.”
- “Metadata proves business meaning.”

Those are lazy assumptions.

## Safe command/code verification targets

- List Database Tools connections in confirmed scope and check lifecycle, type, runtime support, runtime identity, and related resource.
- Validate intended SQL is read-only, bounded, schema-qualified, and avoids sensitive columns unless justified.
- Use explain/metadata where available before running expensive queries.
- Review result handling, download format, retention, and redaction.
- Require approval for DDL, DML, package execution, grants, jobs, or long-running scripts.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
