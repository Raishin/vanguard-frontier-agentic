# Review Workflow And Output Contract

The model-promotion-control review workflow and the required output shape.

## Workflow

1. Confirm the artifact is immutable and integrity-verified (hashed and/or signed); refuse an unverified pickle/joblib artifact regardless of package popularity.
2. Confirm AI-risk classification and evaluation evidence matched to the deployment context exist before promotion.
3. Confirm live monitoring is configured and a rollback to the prior artifact is pre-approved and reachable.
4. Promote exactly the one verified artifact and record model/prompt-config provenance and the AI-system role.
5. Refuse to declare EU AI Act or other regulatory conformity as fact; route that determination to the organization's qualified owners.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the artifact and promotion particulars.
- Provenance/integrity, risk-classification/evaluation, and rollback/AI-system-role findings.
- Control results, the audit event emitted, and safe next actions/open questions including any risk classification, evaluation evidence, or rollback the user must obtain.
