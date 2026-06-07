# OCI Compute Instance Agent Operator Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating run-command as a harmless diagnostic channel.
- Executing across a fleet before confirming exact target, OS, owner, and maintenance window.
- Pasting scripts with secrets or customer data into command payloads.
- Assuming command success means application health.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows instance-agent command list operations expose compartment scope, pagination, sort fields, and command-reference linkage. Treat this as API shape evidence; command creation/execution is a mutation requiring approval.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Read command history before proposing new execution.
- Require explicit approval for any command creation or execution.
- Confirm target OS, shell, privileges, timeout, output retention, and rollback.
- Never include credentials, private keys, tokens, or raw customer payloads in commands.
- Prefer least-privilege read-only diagnostics before remediation.

## Minimal safe implementation flow

- Classify review, troubleshooting, or execution planning.
- Confirm compartment, instance/fleet, OS, owner, command intent, and blast radius.
- Use sampled read-only API evidence for command history and instance state where available.
- Risk-review payload, timeout, output handling, and rollback.
- Return safe runbook, approval requirements, validation, and cleanup.

## High-risk assumptions to kill

- “If the agent can run a command, it should.”
- “A Linux command is safe on every target.”
- “A read-only looking command cannot leak sensitive data.”
- “Fleet execution is only a scale multiplier, not a blast-radius multiplier.”
- “Command output can be pasted into chat without review.”

Those are lazy assumptions.

## Safe command/code verification targets

- List existing commands and executions in confirmed scope before new execution.
- Check target lifecycle state, platform/image assumptions, agent availability, and command plugin readiness.
- Review payload for mutation, secret exposure, network calls, package installs, and privilege escalation.
- Set timeout, output destination, and stop conditions intentionally.
- Validate post-command state with independent health checks, not just command exit status.

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
