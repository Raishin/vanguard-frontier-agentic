# Rollback

Rollback planning is mandatory for Azure Live ARM Deployment Stack Guard production-impacting actions.

- Identify the exact previous state, command or process to restore it, owner, and verification evidence before action.
- If rollback is impossible or materially limited, state that clearly before approval.
- Prefer reversible preview, reset, detach, disablement, or re-swap paths where the service supports them.
- Verify post-action state with read-only evidence and document open risks.
