# Business Idempotency And Reconciliation

Technical vs. business idempotency and separating process completion from business completion.

- At-least-once delivery in distributed job/task systems requires both technical idempotency (safe re-run) and business idempotency (no duplicate business effect) for any side-effecting job.
- Process completion (the job executed) is not the same as business completion (the intended business outcome occurred correctly); both must be confirmed.
- A blind mass-retry (retrying all failed jobs) causes duplicate business effects when jobs are not individually idempotency-guarded.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://docs.celeryq.dev/en/stable/userguide/tasks.html
- https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services
