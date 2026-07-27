# Review Workflow And Output Contract

The exception-governance recording/review workflow and the required output shape.

## Workflow

1. Confirm the exception has a named owner distinct from the requester before recording it.
2. Confirm an explicit scope, an expiration date, compensating controls, and a review date are all recorded.
3. Check for a self-approval or requester-benefit conflict and refuse to approve the exception itself.
4. Flag any existing exception that has expired or is missing a compensating control or expiry as a finding.
5. Route approval to a distinct authorized owner and route any expired-exception remediation to continuous control testing.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions that must be resolved before this action may proceed; empty if approved), the evidence level and quality dimensions, and the exception's particulars.
- Governance-fields, separation-of-duties, and expiry/completeness findings.
- Control results, the audit event emitted, and safe next actions/open questions including the distinct owner/approval the user must obtain.
