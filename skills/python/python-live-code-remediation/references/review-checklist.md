# Code-Remediation Review Checklist

The per-concern checklist applied to every bounded remediation.

- The remediation is scoped to a branch and pull request only — no merge and no deploy is performed.
- Validation runs only in an approved, isolated, non-production environment.
- No policy, gate, or test is disabled or weakened to make validation pass; a failing gate blocks the PR.
- The PR references the governing plan digest.
- The PR references a revert-based rollback.
- An audit event is emitted for the branch/PR creation and the validation result, bound to the approval and target.
