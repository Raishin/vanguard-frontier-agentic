# Safety checklist

Enforce these checks before executing or advising any IONOS DBaaS lifecycle mutation.

## Hard stops

Declare a hard stop and refuse to proceed if ANY of the following is missing, ambiguous, or unconfirmed:

1. **Target database cluster identifier** — the cluster name or UUID must be confirmed in writing. A description like "our production database" is not sufficient. State: "Hard stop: cluster identifier is not confirmed. Provide the cluster name or UUID."
2. **Named approving identity** — the full name or authenticated account identifier of the person authorizing this operation. A role name, team alias, or ticket number alone is not sufficient. State: "Hard stop: no named approving identity. Provide the full name or account ID of the authorized operator."
3. **Current backup verification** — a backup must exist with a verified timestamp confirming it falls within the declared RPO window. An assumption that backups exist is not sufficient. State: "Hard stop: backup existence not verified. Confirm backup timestamp and that it is within the RPO window."
4. **RPO and RTO targets documented** — the workload owner must have stated acceptable data loss and downtime bounds before any mutating operation. State: "Hard stop: RPO/RTO targets not documented. Cannot assess operation safety without them."
5. **Rollback or recovery plan** — a documented recovery path must exist if the operation produces unexpected results. State: "Hard stop: no rollback or recovery plan documented."

## Mandatory posture

- Do not treat a partial confirmation as sufficient — all five hard-stop conditions must be met.
- Do not execute destructive operations (drop cluster, restore-over, delete replica) without explicit written approval from a named authorized human operator.
- Validate IONOS DBaaS regional endpoint correctness before every operation: using the wrong region's endpoint may constitute a GDPR cross-border data transfer violation.
- After every mutation, verify cluster state and emit a recovery path — never leave a mutation without post-execution verification.
- Never expose database connection strings, passwords, API tokens, or customer account identifiers in responses.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- Is the cluster identifier confirmed and unambiguous?
- Is there a named human approver — not a role or alias?
- Is the most recent backup timestamp verified and within the RPO window?
- Are RPO and RTO targets documented and accepted by the workload owner?
- Is there a written recovery plan if the operation fails mid-execution?
- Does the DBaaS endpoint region match the declared GDPR processing location?
- Does the operation risk replication lag, split-brain, or data loss beyond the stated RPO?
- Is there any cluster deletion protection that must be disabled as part of this operation — and if so, has that been explicitly approved?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation describes expected DBaaS behavior — it does not prove backup existence, cluster health, replication state, or PITR availability. Live or user-provided evidence is mandatory for all hard-stop conditions.
