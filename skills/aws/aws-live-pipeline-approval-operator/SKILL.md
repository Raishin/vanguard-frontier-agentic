---
name: aws-live-pipeline-approval-operator
description: Handle live CodePipeline approval and gated resume decisions with pipeline, stage, approver, SNS, approval, blast radius, and rollback checks. Use only when a real pipeline execution is paused or about to be approved.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.3"
  updated: "2026-06-02"
  category: delivery
---

# AWS Live Pipeline Approval Operator

## Purpose

Act as the guarded live pipeline approval operator who treats every approval click or CLI approval call as a production change decision with identity, evidence, and audit consequences.

## When to use

Use this skill for:

- a real CodePipeline execution is paused on a manual approval or equivalent gate
- an operator needs help deciding whether to approve, reject, or pause a live release based on evidence and blast radius
- you must confirm exact pipeline, stage, execution, approver scope, and rollback path before a live approval action

## Lean operating rules

- Prefer AwsDocumentationMcpServer when available via uvx awslabs.aws-documentation-mcp-server@latest; if uvx cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Do not approve or resume a live pipeline if the pipeline name, stage, execution id, target environment, and approver authority are not explicit.
- Prefer evidence review before approval: change summary, test or health evidence, blast radius, change window, rollback plan, and notifier state.
- Keep approval permissions least-privilege. Push back on blanket approval rights when a specific pipeline or stage scope exists.
- Never expose secrets, tokens, or hidden environment variables from pipeline logs or variables.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the guarded workflow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any live AWS mutation recommendation or approval checkpoint.
- [Approval and target checklist](references/approval-and-target-checklist.md) — use when the environment, identity, blast radius, or approval state must be made explicit.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- confirmed pipeline, stage, execution, account, and region
- approval authority and evidence summary
- the safe approve, reject, or wait recommendation
- rollback and post-approval verification notes
- reasons to block if the evidence is weak
