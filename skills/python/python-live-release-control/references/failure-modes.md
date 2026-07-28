# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- Running a release now and writing the ticket later erases the approval trail an incident review depends on.
- Reusing a one-record approval across the remaining 99 records turns a bounded, reviewed change into an unbounded one.
- Executing with standing credentials instead of JIT access leaves a long-lived path an attacker can reuse.
- Self-attesting a release's success hides a failed canary from an independent reviewer.
- Changing the deploy target under an unchanged approval executes against a system nobody approved.
