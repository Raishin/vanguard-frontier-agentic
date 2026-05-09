# Workflow and output contract

Use this reference only when performing a full CI/CD pipeline review, DevOps maturity assessment, or implementation guidance.

## CI/CD areas to check

- RDC pipeline configuration: code, build, test, deploy stage coverage; approval gates; notification configuration
- Cloud Build: trigger rules, build environment spec, artifact output path, build log retention
- Flow pipeline: stage dependencies, gate conditions (automated test pass, manual approval), rollback step configuration
- ACR image lifecycle: tag immutability, vulnerability scanning enablement, lifecycle policy (auto-delete rules), geo-replication
- Environment promotion: dev/staging/production gate conditions, deployment approval workflow, environment parity
- ECS Deployment Sets: spread strategy, fault domain coverage, HA validation

## Safe workflow

1. **Frame scope** — confirm target pipeline, environment, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what fails without rollback? what images are mutable in production?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud CI/CD: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
