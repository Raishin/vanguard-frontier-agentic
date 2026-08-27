# Expectations, Violation Modes, And Table Constraints

Decision guide for expectations violation modes (warn/drop/fail) and table constraint semantics.

- Expectations have three violation modes: warn (default; invalid records written with metrics emitted), drop via `expect_or_drop` (invalid records prevented before write), and fail via `expect_or_fail` (invalid records block the update; manual intervention required).
- In a triggered pipeline, a failed expectation fails and rolls back only that flow's update; other flows continue. In a continuous pipeline, a failed expectation stops the flow and all downstream flows.
- Warn and drop violations are logged as metrics; fail violations do not emit metrics because the update fails first.
- Python decorators `@dp.expect_or_drop(description, constraint)` and `@dp.expect_or_fail(description, constraint)` are applied after the table/materialized_view decorator; decorator order matters.
- Table constraints: NOT NULL and CHECK are enforced and prevent invalid writes. Primary key, foreign key, and unique constraints are informational only and do not prevent writes — they are optimization hints.
- A violation-mode choice must align to the risk: warn for metrics-only observations, drop for soft constraints, fail for hard invariants.

## Sources

- https://docs.databricks.com/aws/en/ldp/expectations
- https://docs.databricks.com/aws/en/ldp/expectation-patterns
- https://docs.databricks.com/aws/en/ldp/developer/ldp-python-ref-expectations
