# Workflow and output contract

Use this reference only when performing a full CodeArts pipeline review or CI/CD implementation guidance.

## CI/CD areas to check

- CodeHub: branch strategy, merge policies, webhook triggers
- Build: build scripts, environment images, artifact storage in SWR or OBS
- TestPlan: test case coverage, CI-linked test execution, quality gate thresholds
- Deploy: target type (CCE/ECS/FunctionGraph), health check configuration, rollback policy
- Pipeline: stage ordering, approval gates, quality gate placement, environment promotion rules
- SWR: image tag discipline, vulnerability scan status, production dependency audit

## Safe workflow

1. **Frame scope** — confirm target pipeline, deploy environment, and non-goals
2. **Collect evidence** — prefer live state; label all evidence types
3. **Stress-test** — blast radius, failure modes, missing evidence
4. **Recommend safest action** — narrow, staged, with rollback

## Output contract

Return this structure:

```markdown
# Huawei Cloud CodeArts DevOps: <scope>
## Scope and evidence level
## Pipeline stage inventory
## Quality gates and approval coverage
## SWR image posture
## Deploy target configuration
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
