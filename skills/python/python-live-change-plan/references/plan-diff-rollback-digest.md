# Change Plan, Diff, Rollback, And Digest

How a change plan binds a diff, rollback, and verification criteria to a target and a plan digest.

- A change plan binds its diff, rollback procedure, and verification criteria to a target fingerprint and a stable plan digest, per CM-3 configuration-change control.
- An approval is bound to the plan digest, so any change to the target invalidates the approval and requires a new plan and a new approval.
- Defining before/after state digests up front makes the change independently verifiable after execution, rather than relying on the executor's own claim.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
