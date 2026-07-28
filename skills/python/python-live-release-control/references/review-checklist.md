# Release-Control Review Checklist

The per-concern checklist applied to every bounded release/canary/rollback/restart request.

- Bound: exactly one release, canary increment, rollback, or single-instance restart executes per approval; no fleet-wide or unbounded change.
- Approval: an independent approval is bound to the exact plan digest and target before execution.
- Credentials: only target-scoped, just-in-time credentials are used, never standing access.
- Before-state: a before-state is captured prior to execution.
- Verification: the after-state is verified by an independent check, never self-attested by the executor.
- Reuse: an approval is never reused when the target or bound changes.
