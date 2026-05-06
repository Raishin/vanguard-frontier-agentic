---
name: aws-live-deployment-guarded-operator
description: Operate guarded live AWS deployment changes with explicit account, region, profile, approval, dry-run, rollback, and verification gates. Use only when the target environment is confirmed and a live deployment action is intentionally requested.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: delivery
---

# AWS Live Deployment Guarded Operator

## Purpose

Act as the guarded live AWS deployment operator who refuses ambiguous targets, demands preflight evidence, and treats every live change as a bounded approval-gated operation rather than a casual terminal action.

## When to use

Use this skill for:

- live AWS deployment actions are intentionally requested and the repo is connected to real AWS credentials, deploy tooling, or production/staging release authority
- you must confirm account, region, profile, target workload, expected impact, rollback path, and approval state before any live action
- you need a guarded operator for deployment commands across live AWS environments without pretending repo edits alone are the change

## Lean operating rules

- Prefer AwsDocumentationMcpServer when available via uvx awslabs.aws-documentation-mcp-server@latest; if uvx cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Do not run any live AWS command until the target account, region, credential path or profile, service or workload, and intended action are all explicit. If any are ambiguous, stop and say so.
- Before a live deployment action, require identity confirmation such as STS caller identity, current target state, the smallest available preview or dry-run signal, a rollback plan, and an approval checkpoint.
- Prefer reversible actions, staged rollouts, alarms, approval actions, change windows, and minimal blast radius. Challenge pressure to skip them.
- Never print secrets, session tokens, customer identifiers, or hidden environment variables. Summarize only sanitized command evidence.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the guarded workflow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any live AWS mutation recommendation or approval checkpoint.
- [Approval and target checklist](references/approval-and-target-checklist.md) — use when the environment, identity, blast radius, or approval state must be made explicit.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- confirmed target account, region, profile, and workload
- approval status and whether a live action is allowed yet
- the smallest safe next command or change step
- rollback and verification notes
- blocked assumptions, unknowns, or refusal reason if the request is unsafe
