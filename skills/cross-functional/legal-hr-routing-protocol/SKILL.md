---
name: legal-hr-routing-protocol
description: Use this skill when a Legal or HR matter must be classified and routed to the right specialist agent, when a matter crosses both domains and needs parallel review, or when Legal and HR agents disagree and the conflict must be resolved. It defines routing rules, the overlap handoff matrix, the controlled-handoff communication principles, and the conflict-resolution protocol. It does not give legal or HR advice and never makes a final routing decision binding on a human.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-18"
  category: compliance
  lifecycle: experimental
---

# Legal-HR Routing Protocol

## Purpose
This skill defines how a Legal or HR matter is classified, routed to the right
specialist agent, and coordinated when it crosses both domains. It exists so
agents never work in silos when risk crosses a domain boundary, and so every
handoff is controlled and auditable rather than free-form. It does not give
legal or HR advice; routing is a recommendation that a human owner confirms.

## When to use
- A maestro agent must classify an incoming matter and dispatch it.
- A specialist agent finds a matter has crossed into another domain.
- A matter triggers an escalation gate and must be paused.
- Legal and HR agents reach conflicting recommendations.

## Communication principles
- One accountable human owner per matter. One primary agent per matter.
- Parallel review only when the matter genuinely crosses domains.
- No agent makes a final legal, employment, disciplinary, regulatory,
  settlement, disclosure, or public-communication decision.
- All high-risk cross-domain matters produce a **pause and escalate**
  recommendation unless sufficient documented controls already exist.
- Every handoff uses a `legal-hr-case-capsule`. No free-form agent chatter.
- Every handoff preserves context, uncertainty, evidence quality, and open
  questions, and carries a `do_not_do_list`.
- Every handoff labels privilege sensitivity and privacy sensitivity.
- Every agent defaults to least-privilege access and minimum-necessary data.

## Routing rules
- Contract, clause, indemnity, limitation of liability, DPA, procurement,
  vendor, IP, confidentiality, termination rights → Legal contract / vendor /
  IP specialists.
- Privacy, data protection, cybersecurity-incident legal triage, employee data,
  regulatory mapping, cross-border transfer, data retention → Legal
  privacy / data-protection specialist.
- Litigation hold, discovery, subpoena, investigation preservation, document
  retention → Litigation / discovery specialist.
- Employee relations, discipline, misconduct, harassment, discrimination,
  retaliation, grievance → HR employee-relations or HR workplace-investigations
  specialist.
- Performance warnings, termination readiness, PIP, manager documentation → HR
  performance or HR termination-readiness specialist.
- Leave, accommodation, disability, medical information, return-to-work → HR
  leave / accommodation specialist AND Legal employment-law specialist.
- Recruiting, compensation, pay equity, promotion, selection criteria, adverse
  impact → HR recruiting / selection or HR compensation-equity specialist.
- Mixed Legal + HR matters route in parallel, with escalation to human
  Legal / HR owners explicitly marked.

## Overlap handoff matrix
Fifteen recurring cross-domain scenarios and their agent sets are defined in
[references/handoff-matrix.md](references/handoff-matrix.md). Each row names the
primary agent, the secondary agents, the escalation gate, and the mandatory
`do_not_do` items.

## Conflict-resolution protocol
When Legal and HR agents disagree, follow this order and stop at the first step
that is unmet:
1. Freeze any irreversible action.
2. Preserve evidence; recommend a litigation hold if discovery risk exists.
3. State the disagreement precisely — what each agent concluded and why.
4. Separate legal risk from HR operational risk; do not collapse them.
5. Identify the accountable human owner.
6. Escalate to qualified Legal / HR leadership.
7. Document unresolved assumptions and open questions.
8. Produce options, not a single conclusion.
9. Require human approval before any action.
10. Log the decision path in the audit log.

## References
- [Overlap handoff matrix](references/handoff-matrix.md) — the 15 scenarios,
  agent sets, escalation gates, and do-not-do items.

## Security notes
- Routing is a recommendation, never an authorization. The protocol never
  approves, denies, or directs an adverse action.
- A matter is routed on sanitized signals. Never request medical detail,
  government IDs, credentials, or protected-class data to classify a matter.
- When classification is ambiguous, route to a maestro agent and mark the
  matter `unclassified` rather than guessing a specialist.
