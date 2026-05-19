# 🔗 Legal-HR Routing Protocol

The **routing protocol** defines the rules, patterns, and conflict-resolution logic for when matters cross the legal-HR boundary. It answers: *When does a matter route from Legal to HR? When from HR to Legal? When do both own it? How do they coordinate?*

## What is the routing protocol?

A control-layer skill that specifies:
- **15 recurring cross-domain scenarios** (from the [`handoff-matrix.md`](references/handoff-matrix.md)) — e.g., wrongful-termination exposure → legal owns risk, HR owns execution; whistleblower report → legal owns privilege, HR owns investigation
- **Controlled-handoff communication principles** — only the case capsule flows between agents; context is redacted per the case capsule rules
- **Conflict resolution in 10 steps** — when a single matter triggers both legal and HR concern, this protocol defines the triage, the escalation path, and who decides

## The handoff matrix

The [`references/handoff-matrix.md`](references/handoff-matrix.md) is the truth table. It lists 15 scenarios in a rows-and-columns format:

| Scenario | Legal owns | HR owns | Decision path |
| -------- | ---------- | ------- | ------------- |
| Wrongful termination exposure | Risk analysis, legal holds, counsel escalation | Documentation, retaliation check, execution | Legal flags risk; HR confirms readiness |
| Whistleblower intake | Privilege, investigation design, board escalation | Interview sequencing, witness neutrality, closure | HR investigates; Legal advises on scope |
| ... and 13 more | ... | ... | ... |

See the full matrix in [`references/handoff-matrix.md`](references/handoff-matrix.md).

## Routing is a recommendation

**Important:** The routing protocol routes *requests* to the right specialist and *patterns* to the right owner. It is a recommendation that a human confirms. No agent executes a handoff unilaterally; every route is logged and traceable.

## For Legal agents

When your review flags an HR exposure (e.g., "this termination has retaliation risk"), you don't make the HR decision. Instead:
1. Compose a case capsule
2. Route to the HR maestro with a handoff reason
3. Log the escalation
4. Remain available if Legal is needed for litigation or board escalation

## For HR agents

When your review flags a legal exposure (e.g., "this investigation has whistleblower concerns"), you don't make the legal decision. Instead:
1. Compose a case capsule
2. Route to the Legal maestro with a handoff reason
3. Confirm Legal's guidance before proceeding
4. Remain available if HR evidence is needed for litigation support

## Conflict resolution

When a single matter triggers both legal and HR concern equally, neither agent decides in isolation. The protocol enforces a 10-step conflict-resolution flow:

1. Both agents signal the dual-ownership via case capsule
2. The legal-hr maestro router receives both signals and enters triage mode
3. Routing determines which agent's decision takes precedence (or if both approve)
4. The decision_owner is named explicitly
5. Both agents log the escalation and their agreement/dissent
6. If disagreement, counsel + senior HR + CFO are escalated
7. If time-critical, a temporary hold is issued and a meeting is scheduled
8. Decision is recorded with rationale
9. Both agents execute under the shared decision
10. Audit log records the entire conflict and resolution

## Cross-references

- [`SKILL.md`](SKILL.md) — the skill prompt and routing scaffold
- [`references/handoff-matrix.md`](references/handoff-matrix.md) — 15-scenario truth table with decision logic
- [`docs/architecture/legal-hr-agent-routing.md`](/docs/architecture/legal-hr-agent-routing.md) — three-layer architecture, maestro contract, routing flow
- [`skills/cross-functional/legal-hr-case-capsule`](/skills/cross-functional/legal-hr-case-capsule/) — the payload structure for handoffs

---

*The routing protocol is part of the vanguard frontier's cross-functional coordination layer. It enables agents to hand off, escalate, and coordinate across organizational boundaries without improvising.*
