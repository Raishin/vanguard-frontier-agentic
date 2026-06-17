# Workflow and output contract

Use this reference only when performing the full Power Platform ALM and Pipelines health review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Solution posture: managed vs. unmanaged solutions in each environment, publisher and prefix discipline, default-solution avoidance, and solution layering conflicts
- Environment strategy: dev/test/prod topology, Managed Environments licensing for target and host environments, environment types (sandbox vs. production), and security group assignment
- Pipeline configuration: pipeline host type (platform host vs. custom host), stage ordering, pre-flight validation, automatic solution backup, and deployment approval gates
- Connection references and environment variables: environment-specific configuration injected at pipeline time rather than hardcoded in components; connection reference mapping per environment
- Source control: Git integration enabled, branch strategy defined, solution export/import lifecycle tracked, no manual customizations.xml edits
- Quality gates: Solution Checker run before promotion, critical and high-severity findings resolved, no bypassed stages
- CI/CD integration: Azure DevOps Build Tools or GitHub Actions configuration, pac CLI authentication, service principal usage
- Rollback readiness: automatic pipeline solution backups accessible, rollback procedure documented and tested, version history tracked

## Safe workflow

1. **Frame scope**
   - Environments in scope (dev/test/prod count and topology):
   - Pipeline type (platform host vs. custom host):
   - Solution count and approximate complexity:
   - Required outcome (solution posture / pipeline review / CI/CD integration / rollback readiness):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported artifacts: solution list with managed/unmanaged status per environment, pipeline run history, Solution Checker report, Git commit log.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What solutions exist in an unmanaged state in target (test/prod) environments?
   - What pipeline stages are missing or can be bypassed?
   - What connection references or environment variables are hardcoded rather than injected?
   - Is Solution Checker passing before each promotion?
   - What rollback exists if a promoted solution breaks production?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer solution hygiene and configuration fixes over pipeline architecture changes.
   - Production pipeline configuration, Managed Environment policy changes, and deployment stage removals require live-guard escalation with a documented rollback plan.

## Output contract

Return this structure:

```markdown
# Power Platform ALM & Pipelines Health Review: <scope>
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
