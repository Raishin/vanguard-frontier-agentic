# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Agent architecture: model, prompts, orchestration, action groups, Lambda/tools, knowledge bases, memory, guardrails, and data stores
- Threat model: prompt injection, prompt leakage, tool abuse, data exfiltration, memory poisoning, unsafe retrieval, and overbroad IAM
- Guardrail coverage: what input/output sections are evaluated, policy types, blocked messages, tests, and cost/latency impact
- Security evidence: IAM, KMS, logging, CloudTrail, application telemetry, PII handling, evals, and rollback/disable path

## Safe workflow

1. **Frame scope**
   - Workload/account/Region/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only AWS MCP or read-only AWS CLI evidence for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official AWS docs.
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
# AWS Bedrock Agent Security Governor: <scope>
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
