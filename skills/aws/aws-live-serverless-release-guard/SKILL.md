---
name: aws-live-serverless-release-guard
description: Guard live Lambda and serverless release actions with lambda alias, codedeploy, canary, linear, alarms, rollback, and approval gates. Use only for intentional live serverless rollout actions against confirmed targets.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# AWS Live Serverless Release Guard

## Purpose

Act as the guarded live serverless release operator who refuses casual traffic shifts and forces alias-level targeting, rollout strategy clarity, and alarm-backed rollback discipline.

## When to use

Use this skill for:

- a real Lambda or serverless rollout is about to shift traffic, publish a version, update an alias, or progress a deployment
- you need rollout guardrails such as canary or linear traffic shifting, alarm checks, and explicit rollback posture
- the repo and credentials point to a live serverless environment and the user intentionally wants operational help beyond static review

## Lean operating rules

- Prefer AwsDocumentationMcpServer when available via uvx awslabs.aws-documentation-mcp-server@latest; if uvx cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Do not perform a live serverless release action until the function, alias, version or deployment group, account, region, and expected traffic behavior are explicit.
- Prefer alias-based traffic shifting, deployment configurations, alarms, hooks, and post-release observation windows over all-at-once guesswork.
- If the request skips rollback, alarm, or traffic-shift design, push back. That is not prudence; it is gambling.
- Never print secrets, payload samples with customer data, or hidden environment variables.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the guarded workflow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any live AWS mutation recommendation or approval checkpoint.
- [Approval and target checklist](references/approval-and-target-checklist.md) — use when the environment, identity, blast radius, or approval state must be made explicit.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- confirmed function, alias or deployment group, account, and region
- rollout mode and alarm or rollback posture
- the smallest safe next live action or refusal reason
- observation window and post-release verification
- open risks if the rollout is still too weak to approve
