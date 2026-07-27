# Bounded Release Execution And Approval Binding

One bounded action per approval, approval-to-plan-digest binding, and independent verification.

- A release-control action is exactly one bounded release, canary increment, rollback, or single-instance restart per approval — never a fleet-wide or unbounded change.
- An approval is bound to the plan digest and the exact target; per CM-3 configuration change control, a changed target or bound invalidates the existing approval and requires a new one.
- Just-in-time, target-scoped credentials are required instead of standing access for the executed action.
- Verification of the executed action is independent of the executor — the executor does not self-attest success.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services
