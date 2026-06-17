# Workflow and output contract

Use this reference only when performing the full Copilot Studio agent governance and ALM health review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Environment strategy: dev/test/prod topology, sandbox vs. production environment types, Managed Environments licensing, security group assignment per environment, and environment routing configuration
- Solution-based ALM: agents created within solutions, preferred solution configured, managed solution exports for promotion, Power Platform Pipelines used for deployments, ALM golden rules followed (no customizations outside dev, environment variables for environment-specific settings)
- Authentication: agent authentication mode (none vs. Entra vs. manual OAuth), web channel security configuration, token-based access controls, and channel-specific authentication enforcement
- DLP policies: tenant-level and environment-level DLP configured, connector classification (Business vs. Non-Business vs. Blocked), unauthenticated usage blocked where required, channel restrictions applied, knowledge source controls active, and enforcement verified (effective since early 2025)
- Publishing and sharing governance: sharing rules defined, viewer/editor count limits set, organization-wide sharing restricted, app catalog publishing requires admin approval, broad-publishing guardrails confirmed
- Content moderation and safety: generative AI publishing controls configured, content filtering active, AI knowledge scope appropriate for audience
- Analytics and telemetry: Copilot Studio built-in analytics active, transcript review process defined, Azure Application Insights integration configured where needed, usage monitoring for policy alignment
- Human-handoff and approval boundaries: escalation topics defined, Power Automate approval flows for high-risk agent actions, human-in-the-loop patterns documented
- Compliance posture: Microsoft Purview sensitivity labels applied where applicable, audit logs enabled, data residency requirements met, GDPR compliance verified, Customer Lockbox considered for sensitive workloads

## Safe workflow

1. **Frame scope**
   - Environments in scope (dev/test/prod count and topology):
   - Agent count and approximate complexity (actions, connectors, knowledge sources):
   - ALM posture (solutions used, pipelines configured, Git integration):
   - Required outcome (governance posture / DLP review / ALM maturity / compliance assessment):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported artifacts: DLP policy reports, solution list with managed/unmanaged status, pipeline run history, analytics dashboard, sharing configuration.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What agents are deployed without authentication or with unauthenticated access enabled?
   - What connectors are classified as Non-Business or unclassified and accessible to agents?
   - What agents can be broadly shared or published without admin approval?
   - Are agents transported via solutions and pipelines, or via manual export/import?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer DLP and sharing configuration fixes over environment architecture changes.
   - Broad publishing changes, DLP policy modifications, and connector grant expansions require live-guard escalation with a documented rollback plan.

## Output contract

Return this structure:

```markdown
# Copilot Studio Agent Governance & ALM Review: <scope>
## Executive verdict
- Status: HEALTHY / HEALTHY WITH RISKS / AT RISK / NEEDS EVIDENCE
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
