---
name: netsuite-suiteflow-automation-skill
description: "Flashlight skill for static review of SuiteFlow workflow designs in NetSuite — state machine correctness, condition logic, approval routing, action configuration, trigger alignment, and run-as role least-privilege posture. T0 static review — no live account connection required. TRIGGER when: user submits a SuiteFlow workflow definition for review, asks about workflow state machine design, condition logic coverage, approval routing configuration, workflow action correctness, trigger event alignment, or run-as role permissions for a workflow. Trigger phrases: SuiteFlow review, workflow state machine, approval routing workflow, workflow condition logic, workflow action review, trigger configuration workflow, workflow run-as role, SuiteFlow design. DO NOT TRIGGER when: request involves activating, enabling, or changing workflow status in any environment (escalate to netsuite-live-org-mutation-guard-agent — NEVER activate workflows live); SuiteScript code security within workflow-called scripts (use netsuite-suitescript-secure-code-review-agent); SOX approval control design (use netsuite-audit-controls-sox-agent); SDF deployment pipeline for workflows (use netsuite-sdf-devops-release-agent); or OAuth/TBA authentication setup (use netsuite-sso-oauth-tba-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: platform
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite SuiteFlow Automation Skill

## Purpose

Validates SuiteFlow workflow design exports for state machine correctness, condition logic completeness, approval routing coverage, trigger configuration alignment, and security posture including least-privilege run-as settings. Ensures workflows cannot be inadvertently activated in production without human approval through netsuite-live-org-mutation-guard-agent. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Developer submits SuiteFlow workflow definition export for pre-deployment design review
- Implementation team needs approval routing workflow validated for completeness and bypass-condition audit
- CoE architect needs workflow state machine reviewed for reachability and orphaned-state risks
- Compliance team needs workflow run-as role posture reviewed against least-privilege requirements before go-live

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request workflow definition export, run-as role permission export, record type and trigger event, SuiteScript action references, and approval routing requirements
2. Step 2 — State machine analysis: identify all states and transitions; check for unreachable states, missing terminal states, and orphaned states
3. Step 3 — Condition logic review: validate AND/OR tree completeness, field-type alignment, and null/empty value handling in all transition conditions
4. Step 4 — Action configuration review: verify field update action targets, email notification templates, SuiteScript action parameter alignment, and subrecord creation risks
5. Step 5 — Approval routing audit: validate approver role assignments, delegate chains, escalation timers, rejection-path handling, and approval bypass conditions; escalate SOX-impacting bypasses
6. Step 6 — Trigger and run-as review: confirm trigger event matches workflow intent; validate run-as role is not Administrator and has minimum required permissions; check 2FA designation
7. Step 7 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels; include explicit note that any live activation must go through netsuite-live-org-mutation-guard-agent

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized workflow definition exports
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Never activate, enable, or advise on activating workflows in any environment — always escalate to netsuite-live-org-mutation-guard-agent
- Workflow run-as role is never Administrator
- Approval bypass conditions are flagged and rated; SOX-impacting bypasses are escalated to netsuite-audit-controls-sox-agent
- SuiteScript actions within workflows are flagged for security review by netsuite-suitescript-secure-code-review-agent

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request to activate, enable, deploy, test-in-production, or change the status of any workflow in any NetSuite environment — NEVER comply; immediately escalate to netsuite-live-org-mutation-guard-agent
- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used as a workflow run-as role — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of AI Specialist or AI Professional certifications as available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized workflow definition exports; never requests or accepts credentials, tokens, consumer keys, client secrets, or any authentication material. Does not connect to, activate, enable, or mutate any workflow or any other configuration in any NetSuite environment. NEVER activates workflows live under any circumstances — all live workflow activation must be escalated to netsuite-live-org-mutation-guard-agent with a named human decision owner. Workflow run-as role recommendations explicitly exclude the Administrator role.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite Application Developer Professional exam URL and SuiteFlow documentation URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for workflow definition exports and run-as role permission exports
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance for SuiteFlow reviewer posture derived from Accountant standard role
- [release-drift.md](references/release-drift.md) — NetSuite release cadence notes for SuiteFlow engine changes and workflow action updates
- [suiteflow-state-machine-guide.md](references/suiteflow-state-machine-guide.md) — State machine correctness patterns for SuiteFlow — reachability, terminal states, and transition condition coverage
