# Workflow and output contract

Use this reference only when performing the full Power Automate automation risk review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Ownership and continuity: single-owner risk on business-critical flows, multiple owners, succession
- Sharing: run-only vs co-owner, flows shared outside the environment, external co-owners, run-only-user connection context
- Connector and DLP: business vs non-business classification, blocked combinations, HTTP/custom connector exposure
- Security segmentation: Environment Maker vs run-only users, environment security groups, least privilege
- Resilience: error handling (run-after, Terminate), retry policy with exponential backoff, idempotency
- Monitoring: failure notifications, Application Insights, CoE Starter Kit auditing and alerting
- Connection lifecycle: credential rotation, expired OAuth tokens (90+ day unused), service-account connections
- Criticality: which flows are business-critical and what fails if they stop

## Safe workflow

1. **Frame scope**
   - Flow(s) or environment in scope and business criticality:
   - Required outcome (continuity / sharing hardening / connector-DLP review / resilience / monitoring):
   - Available evidence (admin center export, CoE dashboards, flow run history):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer Power Platform admin center exports, CoE Starter Kit dashboards, and flow run histories.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Which business-critical flows have a single owner (bus-factor risk)?
   - Where is co-ownership broader than necessary, or shared outside the environment?
   - Which connectors are unscoped by DLP, or use HTTP/custom connectors that could exfiltrate data?
   - Which flows lack error handling, retry, or failure notifications?
   - Which connections risk expiry or use unrotated service-account credentials?

4. **Recommend the smallest safe action**
   - Prefer run-only sharing over co-ownership; add a second owner for continuity; scope connectors with DLP.
   - Production DLP, ownership, and connector changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# Power Automate Automation Risk Review: <scope>
## Executive verdict
- Status: LOW RISK / MODERATE RISK / HIGH RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
