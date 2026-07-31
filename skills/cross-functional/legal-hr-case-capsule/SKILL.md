---
name: legal-hr-case-capsule
description: Use this skill when a Legal or HR agent must hand a matter to another agent and the context, uncertainty, evidence quality, privilege posture, and privacy posture must survive the handoff. It defines the shared legal-hr-case-capsule — a controlled, auditable exchange record with redacted identifiers, risk labels, privilege and privacy labels, a decision-owner field, and an explicit do-not-do list. It does not give legal or HR advice and does not authorize any action.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-18"
  category: compliance
  lifecycle: experimental
---

# Legal-HR Case Capsule

## Purpose
This skill defines the **legal-hr-case-capsule** — the single structured record
that Legal and HR agents exchange when a matter crosses an agent boundary. The
capsule exists so that no agent works in a silo, no context is lost in a
handoff, and every cross-domain exchange is auditable. It is a data contract,
not a decision: a capsule never approves, denies, or directs an action. It
carries facts, uncertainty, risk posture, and an explicit decision owner.

## When to use
- A maestro agent routes a matter to one or more specialist agents.
- A specialist agent escalates a matter that has crossed into another domain.
- Any matter touches both Legal and HR risk and must be reviewed in parallel.
- A matter must be paused and escalated to a human owner.

## Core rules
- Every cross-agent handoff MUST be expressed as a capsule. No free-form
  agent-to-agent chatter.
- Every capsule MUST name exactly one `decision_owner` (an accountable human)
  and exactly one `primary_agent`.
- Every capsule MUST carry a `do_not_do_list`. An empty list is not acceptable;
  if nothing is prohibited, the capsule is not ready to send.
- Every capsule MUST label `privilege_sensitivity` and `privacy_sensitivity`.
- Identifiers MUST be redacted to the minimum necessary. Use role and
  business-unit references, not names, government IDs, or contact details.
- A capsule records uncertainty honestly: `assumptions`, `inferences`, and
  `missing_evidence` are mandatory fields, not optional.
- A capsule never states "this is legal", "this is compliant", or "this action
  is approved". `risk_rating` uses risk language only.
- High-risk cross-domain capsules MUST set `escalation_required: true` and
  `recommended_next_action` to a pause-and-escalate posture unless documented
  controls already exist.

## Capsule field set
The capsule has 30 required fields. See
[references/capsule-schema.md](references/capsule-schema.md) for the full field
definitions, allowed values, and redaction rules. Summary:

- Identity and routing: `case_id`, `source_agent`, `receiving_agent`,
  `primary_agent`, `secondary_agents`, `matter_type`,
  `employee_or_party_identifiers_redacted`, `jurisdiction_or_location_if_known`,
  `business_unit`, `timeline`.
- Evidence discipline: `facts`, `allegations`, `assumptions`, `inferences`,
  `missing_evidence`, `evidence_quality`.
- Risk posture: `risk_rating`, `privilege_sensitivity`, `privacy_sensitivity`,
  `retaliation_risk`, `discrimination_or_harassment_risk`, `regulatory_risk`,
  `litigation_hold_needed`, `data_minimization_notes`.
- Ownership and action: `decision_owner`, `human_approval_required`,
  `escalation_required`, `recommended_next_action`, `do_not_do_list`,
  `audit_log_summary`.

## References
Load only when needed:
- [Capsule schema and redaction rules](references/capsule-schema.md) — full
  field-by-field contract, allowed values, and worked example.

## Security notes
- The capsule is a minimum-necessary record. Never widen it to carry medical
  records, government IDs, credentials, privileged email text, or
  protected-class data beyond what the matter strictly requires.
- A capsule that would extend the circulation of privileged or investigation
  material must be narrowed before it is sent; flag privilege explicitly.
- The capsule never authorizes action. Any field that reads as a directive to
  terminate, discipline, settle, file, notify a regulator, or send an employee
  communication is a defect — rewrite it as a recommendation with a named human
  owner.
