# Legal-HR Agent Communication Architecture

This document describes how Legal and HR agents communicate. The design goal is
controlled, auditable handoffs — never free-form agent-to-agent chatter — so
that context, uncertainty, evidence quality, privilege posture, and privacy
posture survive every boundary crossing.

## The case capsule is the only channel

Every cross-agent handoff is a `legal-hr-case-capsule` (defined by the skill of
the same name). Agents do not exchange unstructured messages. The capsule
carries 30 required fields across four groups: identity and routing, evidence
discipline, risk posture, and ownership and action.

Two rules make the capsule safe:

- It always names exactly one `primary_agent` and exactly one `decision_owner`
  (an accountable human).
- It always carries a non-empty `do_not_do_list`. If nothing is prohibited, the
  capsule is not ready to send.

## Controlled handoff principles

- Parallel review only when a matter crosses domains.
- Every handoff preserves uncertainty: `assumptions`, `inferences`, and
  `missing_evidence` are mandatory.
- Every handoff labels `privilege_sensitivity` and `privacy_sensitivity`.
- Every handoff is minimum-necessary: identifiers are redacted to role and
  business unit; sensitive content is summarized, never pasted.
- Every handoff emits one audit-log event.

## Audit trail

Each handoff and escalation produces one audit-log event using the schema in
the `legal-hr-risk-taxonomy` skill. The log carries labels and summaries only —
never raw medical, privileged, credential, or protected-class content. The log
is the evidence that the ecosystem behaved correctly: every recommendation
traces to facts, assumptions, evidence gaps, or stated uncertainty.

## Conflict resolution

When Legal and HR agents disagree, the `legal-hr-routing-protocol` conflict
protocol runs: freeze irreversible action, preserve evidence, state the
disagreement, separate legal risk from HR operational risk, identify the human
owner, escalate to leadership, document unresolved assumptions, produce options
rather than a conclusion, require human approval, and log the decision path.

## Worst-case modes this design defends against

- **Context loss** — mitigated by mandatory capsule fields for uncertainty.
- **Siloed agents** — mitigated by parallel routing for cross-domain matters.
- **Privilege leakage** — mitigated by privilege labels and summarize-not-paste.
- **Over-collection** — mitigated by redaction and minimum-necessary rules.
- **Human-accountability failure** — mitigated by one named `decision_owner`
  per matter and `human_approval_required` on adverse or irreversible actions.
- **Audit-trail insufficiency** — mitigated by one audit-log event per handoff.

## Non-goals

The communication layer never authorizes action. Any capsule field or log entry
that reads as a directive to terminate, discipline, settle, file, notify a
regulator, or send an employee communication is a defect and must be rewritten
as a recommendation with a named human owner.
