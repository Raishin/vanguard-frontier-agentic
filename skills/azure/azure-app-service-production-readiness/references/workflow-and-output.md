# Workflow and output contract for Azure App Service Production Readiness

## Minimal safe workflow

1. Classify workload: public web app, internal app, API, custom container, WebJob, or mixed.
2. Ground the review with Microsoft Learn through the user's configured documentation MCP.
3. Define current evidence: docs only, read-only current-state sample, IaC/config review, or user-supplied sanitized evidence.
4. Review reliability: instances, zones, health check, auto-heal, scaling, statelessness, dependencies, backup, and recovery.
5. Review security: managed identity, Key Vault references, HTTPS/TLS, public access, private endpoint, access restrictions, SCM, CORS, logs, and secret handling.
6. Review operations: slots, warm-up, swap, rollback, diagnostics, alerts, release flow, ownership, and drills.
7. Deliver verdict and blockers. Do not soften no-go findings.

## Output contract

```markdown
## Verdict
<go | conditional-go | no-go | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Runtime/config evidence: <sampled_read_only | config_review | not sampled>

## Findings
1. <finding> — Evidence: <docs_only|sampled_read_only|config_review|inference>

## Production blockers
- <blocker>

## Safe next actions
- <least-risk action>

## Open questions
- <question needed for readiness>
```

## Pushback triggers

Push back on direct-to-production deployments, no slot rollback, no health endpoint, public access with no threat model, secrets in app settings, no restore test, no alerts, or no named operational owner.
