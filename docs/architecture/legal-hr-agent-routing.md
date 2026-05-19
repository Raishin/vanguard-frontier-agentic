# Legal-HR Agent Routing Architecture

This document describes how Legal and HR matters are classified and routed
across the agent ecosystem. It is a design reference, not an authorization
surface — no routing decision in this ecosystem ever approves, denies, or
directs an adverse action.

## Three layers

1. **Maestro agents** — `legal-maestro-agent` and `hr-maestro-agent` classify
   an incoming matter, select the specialist(s), and coordinate synthesis.
2. **Specialist agents** — narrow review agents (contract, privacy, employee
   relations, termination readiness, and so on) that triage one domain.
3. **Cross-functional protocols** — the `legal-hr-case-capsule`,
   `legal-hr-routing-protocol`, and `legal-hr-risk-taxonomy` skills that govern
   how the layers exchange work.

## Routing flow

```
intake
  -> maestro classifies (legal-hr-routing-protocol routing rules)
     -> single domain  -> one primary specialist
     -> cross domain   -> one primary + parallel secondaries
     -> ambiguous      -> matter marked `unclassified`, held at maestro
  -> specialists review, each emitting a legal-hr-case-capsule
  -> maestro synthesizes capsules
  -> escalation gate check (legal-hr-risk-taxonomy)
     -> gate true  -> pause + escalate to named human owner
     -> gate false -> options + recommended next action to human owner
```

## Routing rules (summary)

The full routing rules live in the `legal-hr-routing-protocol` skill. Key
principles:

- One accountable human owner and one primary agent per matter.
- Parallel review only when a matter genuinely crosses domains.
- Mixed Legal + HR matters route in parallel with explicit human escalation.
- Ambiguous matters are never force-fit to a specialist; they stay
  `unclassified` at the maestro.

## Maestro routing contract

Each maestro is backed by a routing taxonomy fixture under
`tests/fixtures/<maestro>-maestro-routing/` containing `taxonomy.json`,
`inputs/`, and `expected/`. The taxonomy maps keyword-scored domains to
specialist agent ids. The `validate:maestro-routing` gate replays operator-style
tasks against the taxonomy and asserts the routed agent set and mode.

## Escalation gates

A matter is paused and escalated whenever an escalation gate in the
`legal-hr-risk-taxonomy` skill is true — escalation-grade matter type, a filed
or threatened claim, protected activity, possible privilege, material exposure,
a board/audit/regulatory trigger, an uncontrolled cross-domain matter, or a
Legal/HR agent disagreement.

## Non-goals

- No agent makes a final legal, employment, disciplinary, regulatory,
  settlement, disclosure, or public-communication decision.
- No agent mutates an HR or legal system of record.
- Routing never substitutes for human judgment; it produces a recommendation a
  named human owner confirms.
