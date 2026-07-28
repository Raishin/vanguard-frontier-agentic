# Review Workflow And Output Contract

The bounded-remediation branch/PR workflow and the required output shape.

## Workflow

1. Confirm the governing approval, plan digest, target binding, and JIT credentials are in place before creating anything.
2. Create a branch and a pull request referencing the plan digest and a revert-based rollback.
3. Run only approved, isolated non-production validation against the branch.
4. Confirm no policy, gate, or test was disabled or weakened to force a pass; treat a failing gate as blocking.
5. Emit an audit event for the branch/PR creation and validation result, bound to the approval and target.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions preventing execution; empty if approved), and the evidence level and quality dimensions of the remediation.
- Branch/PR-creation and isolated-validation findings.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
