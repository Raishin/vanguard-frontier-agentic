# Review Workflow And Output Contract

The change-plan production workflow and the required output shape.

## Workflow

1. Identify the proposed change and the exact target fingerprint it applies to.
2. Produce an exact diff of the change and a stable action digest bound to the target fingerprint.
3. Define a pre-approved rollback procedure and machine-checkable verification criteria.
4. Define the before/after state digests the executor must capture.
5. Confirm the plan holds no production credentials and record it as evidence with its quality dimensions.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions preventing execution; empty if approved), and the evidence level and quality dimensions of the plan's inputs.
- Diff/plan-content, rollback/verification, and approval-binding findings.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
