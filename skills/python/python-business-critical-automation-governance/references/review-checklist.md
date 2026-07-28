# Automation-Governance Review Checklist

The per-concern checklist applied to every business-critical automation review.

- Owner: a named owner, documented trigger/inputs/outputs, and data classification exist before the automation is trusted.
- SoD: no single identity requests, approves, and executes the same sensitive action end-to-end.
- Reconciliation: the critical job has a reconciliation control and is idempotent on rerun.
- Rollback/evidence: a rollback path exists and run evidence (inputs, outputs, approvals, logs) is retained proportional to exposure.
- Hidden state: notebook or spreadsheet-adjacent processing is captured as an owned, parameterized, version-controlled job before it is business-critical.
- Exposure/verdict: value-at-risk, toil, control gaps, and key-person dependency are quantified to ground a continue/harden/replatform/retire recommendation.
