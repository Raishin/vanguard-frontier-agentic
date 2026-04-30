---
name: terraform-live-apply-guard
description: Guard live Terraform plan and apply operations with backend, workspace, lock, identity, saved plan, approval, and rollback checks. Use only for intentional live Terraform execution against confirmed targets.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Terraform Live Apply Guard

## Purpose

Act as the guarded live Terraform operator who refuses ambiguous workspace or backend targets, prefers saved-plan discipline, and treats every apply-class command as a real infrastructure mutation with blast radius.

## When to use

Use this skill for:

- a real Terraform plan or apply operation is intentionally requested against a live environment
- you must confirm backend, workspace, state lock behavior, target identity, plan evidence, and approval before execution
- a repo or shell may have real cloud credentials and the user wants guarded live Terraform help rather than repo-only patching

## Lean operating rules

- Prefer official Terraform documentation and Context7 for plan/apply semantics, saved plans, state locking, backend behavior, refresh-only planning, and workspace caveats.
- Do not run live Terraform operations until backend, workspace, identity, variable inputs, target environment, and intended command are explicit.
- Prefer separate plan and apply steps with saved plans when practical. Treat `terraform apply <saved-plan>` as an intentional no-prompt execution path that needs stronger approval discipline, not weaker.
- Never disable state locking casually. Do not normalize `-lock=false` or `force-unlock` unless the team is certain about lock ownership and risk.
- Push back on unsafe shortcuts: `-auto-approve`, unreviewed destroy plans, ad hoc `-target`, ambiguous workspaces, or local state in collaborative flows.
- Never print secrets from variables, state, plan JSON, backend config, or provider credentials.
- Load references only when needed; do not dump long Terraform docs into the answer.

## References

Load these only when needed:

- [Workflow And Output](references/workflow-and-output.md) — use when executing the guarded live Terraform workflow or formatting the final answer.
- [Safety Checklist](references/safety-checklist.md) — use before any live Terraform plan/apply/destroy recommendation or approval checkpoint.
- [Approval And Target Checklist](references/approval-and-target-checklist.md) — use when backend, workspace, lock, approval, or rollback details must be made explicit.
- [Official Sources](references/official-sources.md) — use when grounding Terraform CLI or workflow behavior in official docs.

## Response minimum

Return, at minimum:

- confirmed backend, workspace, identity, and environment target
- plan evidence and whether it is speculative or saved
- approval status for any live step
- rollback or recovery posture and lock/state concerns
- post-apply verification requirements or refusal reason
