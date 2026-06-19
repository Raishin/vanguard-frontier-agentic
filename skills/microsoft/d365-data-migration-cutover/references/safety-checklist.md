# Safety checklist

Use this reference before any recommendation involving production data migration authorization, cutover go/no-go approval, rollback execution, or compliance-impacting data promotion decisions in Dynamics 365 implementations.

## Non-negotiables

- Never ask users to paste credentials, connection strings, environment URLs, tenant IDs, client secrets, certificates, or customer personally identifiable information into chat.
- Use documented migration artifacts or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent migration job results, staging error counts, reconciliation totals, entity coverage lists, or cutover window estimates.
- Require explicit human approval before recommending any production data migration, cutover execution, or rollback.
- Use current official Microsoft Learn documentation for Dynamics 365 Data Management Framework and cutover strategy behavior.
- Keep recommendations least-change, reversible, and scoped to the data domain in question.
- Production data migration and cutover are live-guard gated. Always escalate to the implementation lead and named business data owner before execution.

## Stress checks

- Has at least one mock migration been completed with documented results (staging errors, elapsed time, reconciliation totals)?
- Have all staging table errors from mock migrations been resolved and re-validated?
- Does reconciliation evidence include field-level sampling and financial balance checks, not just record counts?
- Is the cutover window duration realistic based on measured throughput plus a 20–30% buffer?
- Is there a tested rollback plan with named owner, rollback trigger criteria, rollback execution steps, and rollback validation?
- Has the business data owner signed off on migration results from the most recent mock migration?
- Is there a delta load strategy for data that changes between the final mock migration and production cutover?
- What post-migration validation steps are planned for day-one business operations?

## Evidence labels

Use `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual migration job results, staging validation status, reconciliation outcomes, or rollback readiness.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Authorizing or executing production data migration jobs
- Issuing a go/no-go decision for production cutover
- Executing rollback from production migration
- Promoting data from staging tables to production target tables in the live environment
- Freezing source legacy systems for final delta migration
- Signing off on reconciliation results on behalf of the business data owner
- Authorizing deletion or archival of legacy source data after migration validation
