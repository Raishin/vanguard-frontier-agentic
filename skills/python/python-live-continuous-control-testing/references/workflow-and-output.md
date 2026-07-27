# Review Workflow And Output Contract

The continuous-control-testing workflow and the required output shape.

## Workflow

1. Identify the population of controls and the period to be tested.
2. Run the continuous-control checklist against that population (credentials, privilege, ownership, approval, drift, audit logging, rollback, verification, redaction, retention, provenance, reconciliation, and related failure classes).
3. For each failure, open a finding with a named owner and a due date; never silently remediate a high-risk failure in production.
4. Distinguish a single passing observation from continuing operating effectiveness and report the population/period tested.
5. Route any needed remediation to the owning live-guard operator and any known, accepted gap to exception governance.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions that must be resolved before this action may proceed; empty if approved), the evidence level and quality dimensions, and the tested population and period.
- Checklist, ownership, and operating-effectiveness findings.
- Control results, the audit event emitted, and safe next actions/open questions including any remediation or exception the user must obtain.
