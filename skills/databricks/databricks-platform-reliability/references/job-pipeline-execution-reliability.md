# Job And Pipeline Execution Reliability

Timeout semantics, retry behavior, dependencies, continuous-job constraints, and run-history retention for incident investigation.

- When both a timeout and retries are configured on a task, the timeout applies to EACH retry individually; a task with 3 retries and a 30-minute timeout can run up to 120 minutes total if each attempt times out. The timeout is not a global bound across all retries.
- Continuous jobs support only exponential backoff; they cannot use task dependencies, retry policies, or manual triggering. A continuous job configured with retries or dependencies is invalid and will error.
- A task exceeding its timeout is marked 'Timed Out' in the run history. The timeout behavior differs from a task that fails (marked 'FAILED') or is canceled (marked 'CANCELED').
- Run history for both jobs and pipelines is retained for 60 days. Runs older than 60 days are purged and cannot be recovered from the workspace. Incident investigation must happen within this window, or external log preservation is required.
- Task dependencies form a directed acyclic graph (DAG); a dependency failure stops downstream tasks. A cascade failure in a job with complex dependencies can trigger retries on all dependent tasks. Restart policy should be verified for recovery semantics.
- Parallel task execution is bounded by the workspace limit of 12,000 simultaneously running tasks. A job that creates many parallel tasks risks hitting this limit; the limit must be verified before expanding parallelism.
