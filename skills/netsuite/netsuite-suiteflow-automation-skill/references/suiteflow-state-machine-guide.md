# Suiteflow State Machine Guide

State machine correctness patterns for SuiteFlow — reachability, terminal states, and transition condition coverage

Scope: Validates SuiteFlow workflow design exports for state machine correctness, condition logic completeness, approval routing coverage, trigger configuration alignment, and security posture including least-privilege run-as settings. Ensures workflows cannot be inadvertently activated in production without human approval through netsuite-live-org-mutation-guard-agent.

- State machine design review — state reachability analysis, terminal state coverage, orphaned state detection, transition condition completeness
- Condition logic review — AND/OR tree correctness, field-type mismatch risks, null and empty value handling in workflow conditions
- Action configuration review — field update action correctness, email notification template assignments, SuiteScript action parameter mapping, subrecord creation risks
- Approval routing design — approver role assignments, delegate chain configuration, escalation timer coverage, rejection-path handling, approval bypass condition audit
- Trigger configuration review — record type alignment, trigger event (before-submit, after-submit, scheduled, button click) appropriateness, schedule parameter validation
- Run-as role least-privilege posture — workflow run-as role permission scope, 2FA designation requirements, prohibition on Administrator run-as
- SuiteScript action integration review — parameter passing from workflow context to script, script entry-point alignment with workflow trigger type
