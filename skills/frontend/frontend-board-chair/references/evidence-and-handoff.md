# Evidence and Handoff Contract

## What people get wrong

The naive story is:

> "The specialists approved it, so I'll write 'Approved' and move on."

Wrong. A verdict without a structured evidence table and a named receiving owner is not a governance decision — it is an unaccountable rubber stamp, and it is exactly the failure mode this board exists to remove (see the Board Chair agent's Business Pain Removed statement: eliminating single-reviewer blind spots and inconsistent, person-dependent sign-off).

## Evidence table (mandatory for every verdict)

Every claim that feeds the verdict must appear as a row:

| Claim | Evidence label | Source |
|---|---|---|
| e.g. "No confirmed XSS path in the new component" | `repo evidence` | `frontend-security-agent` report, file/line cited |
| e.g. "WCAG 2.2 AA color-contrast passes" | `live evidence` | axe-core run output pasted by user |
| e.g. "Hydration mismatch is a React error, not a warning, in React 18+" | `documentation-based` (Context7: `/reactjs/react.dev`) | React 18 upgrade guide |
| e.g. "Field CWV data not yet available" | n/a | flagged as an open gap, not filled with inference |

Evidence labels, in the same five-tier vocabulary used for conflict resolution:

- `live evidence` — observed directly against a running system/build/test output.
- `repo evidence` — observed directly in the actual codebase.
- `user-provided sanitized evidence` — pasted output the user attests is real and current.
- `documentation-based` — grounded in official docs / Context7, not verified against this specific codebase.
- `inference` — reasoning without a cited source.

Any HARD-gate claim (security, accessibility) resting on `documentation-based` or `inference` alone is not sufficient for a full approve — escalate to a request for live/repo evidence or route to conditional-approve with a named owner responsible for producing that evidence before ship.

## Verdict definitions

- **Approve** — every required specialist reported, no HARD-gate reject is outstanding, and performance claims (where applicable) have both lab and field evidence at an acceptable tier.
- **Conditional-approve** — approvable once a specific, named condition is met by a specific, named owner (e.g. "confirm CWV field data within 5 business days — owner: web-platform team lead," or "security team lead records written risk acceptance for the residual finding — owner: named individual"). A conditional-approve without both the condition and the owner named is invalid — it is an approve with extra words.
- **Reject** — a HARD-gate finding is confirmed and unresolved, or a required specialist never reported and the gap cannot be closed with available evidence. State the specific blocking finding, not a vague "needs more work."

## Handoff record (mandatory for every verdict, including reject)

Every response must name a **receiving human or team owner** — never an anonymous "the team should..." Handoff content depends on verdict:

- **Approve** → name who receives the sign-off record (e.g. the requesting engineer/team lead) for their records; no further action required of them.
- **Conditional-approve** → name the owner responsible for satisfying the condition, the condition itself, and a target timeframe if one was given or can reasonably be inferred from the workflow type (e.g. CWV field-confirmation windows are typically measured in days, not months).
- **Reject** → hand back to the originating specialist/team with the specific blocking evidence (claim + evidence label + source), not "try again" — the recipient must be able to act on the finding without re-deriving it.

## Rollback and escalation notes

For any workflow touching a live/production system (production incident, CWV field failure, and any workflow where a specialist's report references a deployed change):

- Require an explicit rollback path or blast-radius statement as part of the required inputs before adjudicating.
- If none was provided, that absence is itself a blocker — do not infer a rollback path on the specialists' behalf.
- Escalation triggers (from the Board Chair agent's own escalation-triggers list) that must be surfaced explicitly in the response when present: any HARD-gate reject, any live/production-mutation request without a disclosed rollback path, any unresolved specialist disagreement, any production-incident/CWV workflow with unclear root cause, and any framework-migration workflow proposing a rewrite without a narrower-path justification.

## When to push back

Push back if asked to:

- issue "Approved" or "Rejected" as a bare word with no evidence table,
- name "the team" or "engineering" as a handoff owner instead of a specific person or role,
- skip the rollback/blast-radius requirement because the change is "small" or "low-risk" — that determination belongs to the specialists' evidence, not to the request's framing,
- treat a conditional-approve as equivalent to an approve once the record is written, without actually tracking whether the named owner met the condition.

A decision record that cannot be acted on by its recipient has failed at its one job.
