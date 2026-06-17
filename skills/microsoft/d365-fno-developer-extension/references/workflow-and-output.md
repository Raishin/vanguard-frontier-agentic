# Workflow and output contract

Use this reference only when performing the full Finance & Operations extension or ALM review, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- X++ extension correctness: extension class pattern, CoC `next` call presence, `[ExtensionOf]` attribute correctness, wrappable method usage, no over-layering
- Table and form extensions: field additions, display methods, event handler wiring, no direct base object modification
- Extension model design: model references, package dependencies, no circular references, model version hygiene
- Deployable package hygiene: package contents, single merged package for multi-module deployments, no experimental code, no ISV module overlap issues
- Build pipeline: Azure DevOps pipeline tasks (NuGet install, model version update, build, package creation, artifact publish), pipeline triggers, agent type
- ALM process: branch strategy, code review gates, sandbox-first deployment policy, LCS asset library usage, release candidate marking
- Test automation: SysTest framework unit test coverage, RSAT business process test coverage, test pipeline integration
- Upgrade safety: deprecated API usage, over-layering indicators, hard-coded version assumptions, One Version compatibility posture
- Performance patterns: set-based operations vs. row-by-row, batch framework usage, query index alignment
- Production deployment readiness: sandbox validation evidence, test results, rollback plan, release manager sign-off

## Safe workflow

1. **Frame scope**
   - Finance & Operations workloads in scope (Finance, Supply Chain Management, Commerce, Human Resources):
   - Customization type (X++ extension, deployable package, ALM pipeline, or combination):
   - Target environments (sandbox, UAT, production):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer documented artifacts: code review records, build pipeline results, sandbox deployment logs, RSAT test execution reports, CoC correctness evidence, release manager sign-off.
   - Otherwise inspect sanitized user-provided code snippets or summaries, or official Finance & Operations documentation.
   - Label each finding as `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.
   - All X++ syntax review is advisory; flag for current-doc verification.

3. **Stress-test risk**
   - Does the extension use Chain of Command correctly with unconditional `next` calls (unless the method is `[Replaceable]`)?
   - Are there any over-layering violations modifying base application objects directly?
   - Does the extension model have clean dependencies with no circular references?
   - Has the deployable package been created by the Azure DevOps build pipeline, not from a developer machine?
   - Has the package been deployed to and validated in a sandbox environment before production?
   - Have automated tests (SysTest and/or RSAT) been executed and passed against the sandbox deployment?
   - Is there a rollback plan with a named owner and defined rollback trigger criteria?
   - Has the release manager signed off on the sandbox validation results?
   - What upgrade risks exist for the next One Version service update cycle?

4. **Recommend the smallest safe action**
   - Prefer extension pattern correction over workarounds, sandbox re-validation over proceeding to production, and additional RSAT coverage over partial test runs.
   - If the safest action is to stop and fix a CoC defect or over-layering violation before sandbox deployment, say that plainly.
   - Production package deployment requires live-guard escalation. Do not recommend production deployment without explicit human approval from the implementation lead and release manager.

## Output contract

Return this structure:

```markdown
# D365 Finance & Operations Extension Review: <scope>
## Executive verdict
- Status: READY / READY WITH CONDITIONS / NOT READY / NEEDS EVIDENCE
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
- Artifacts or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
