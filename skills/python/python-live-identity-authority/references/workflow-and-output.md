# Review Workflow And Output Contract

The identity-and-authority verification workflow and the required output shape.

## Workflow

1. Identify the acting principal and confirm it is an identified individual, not a shared or anonymous account.
2. Confirm the credential is current and the grant is target-scoped and time-bound (JIT), not standing administrative access.
3. Verify the approver is a distinct principal from the requester and holds authority for the target scope.
4. Confirm the identity's granted scope matches the exact target of the action; reject scope mismatch or cross-target reuse.
5. Record the verification as evidence with its quality dimensions and flag any blocking condition found.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the evidence level and quality dimensions of the identity/authority check.
- Identity/credential, approval-authority/separation-of-duties, and scope-match findings.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
