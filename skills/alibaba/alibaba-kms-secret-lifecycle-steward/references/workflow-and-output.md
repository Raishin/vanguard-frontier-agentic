# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- CMK purpose, data classification, owning service, key policy, RAM policy, key version history, rotation schedule, aliases, tags, deletion window, and break-glass path
- SSM secret type, rotation pattern (FC trigger), retry/client caching, resource policy, and monitoring
- Certificate Manager SSL/TLS certificates: expiry dates, auto-renewal status, domain coverage, and CloudMonitor alarm configuration
- HSM key custody: dedicated HSM instances, key wrapping, operator access, and FIPS compliance evidence
- Envelope encryption coverage: which services use CMK-wrapped data keys vs. plain storage
- Recovery risk: backups, cross-region restore, disabled/deleted CMKs, stale credentials, and operator ownership

## Safe workflow

1. **Frame scope**
   - Workload/account/Region/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data?
   - What can escalate privilege?
   - What can break production or block rollback?
   - What can create unbounded cost?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Alibaba Cloud KMS Secret Lifecycle Steward: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
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
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
