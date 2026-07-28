# Continuous-Control-Testing Checklist

The per-concern checklist applied to every continuous-control-testing pass.

- Checklist: the continuous-control checklist tests credential expiry, standing privilege, owner inactivity, missing approval, requester-approver conflicts, and stale policy bundles.
- Drift: plan/target drift, agent/tool drift, and egress expansion are tested as continuing-operation failures.
- Evidence integrity: disabled audit logging, unredacted sensitive fields, and evidence-retention failures are tested findings.
- Rollback and verification: a broken rollback and incomplete verification are tested findings, not assumed to still work.
- Reconciliation and claims: failed reconciliation and a verifier reusing the executor's own claims are tested findings.
- Findings: every failure opens with a named owner and a due date; a high-risk failure is never silently remediated.
