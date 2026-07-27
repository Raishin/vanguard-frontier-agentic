# Review Workflow And Output Contract

The policy-applicability evaluation workflow and the required output shape.

## Workflow

1. Identify the action's recorded applicability inputs and risk tier, and the versioned policy bundle in force.
2. Determine control applicability strictly from the recorded inputs.
3. Evaluate each applicable control_id and produce a candidate pass/fail/not-applicable result.
4. Record the policy_bundle_version evaluated.
5. Present the result as an owner-confirmable candidate and record it as evidence with its quality dimensions.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions preventing execution; empty if approved), and the evidence level and quality dimensions of the evaluation.
- Policy-bundle/applicability and candidate control-result findings.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
