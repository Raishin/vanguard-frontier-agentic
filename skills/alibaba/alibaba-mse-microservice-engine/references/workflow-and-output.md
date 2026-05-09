# Workflow and output contract

Use this reference only when performing a full MSE review, incident triage, or microservice platform implementation guidance.

## MSE areas to check

- Nacos: namespace isolation (prod/staging/dev separation), config group design, config encryption for secrets, health check configuration, listener push latency
- Sentinel: flow control rules (QPS thresholds, warm-up, queue), degrade rules (error ratio, slow call ratio), hotspot parameter rules, system adaptive rules
- Seata: transaction mode selection (AT/TCC/XA), global transaction coordinator sizing, `undo_log` table presence (AT mode), timeout configuration, transaction monitoring
- ARMS APM: agent instrumentation coverage, distributed trace sampling rate, service topology completeness, SLO definition and alert rules
- MSE cluster sizing: instance spec vs. expected service count and config push volume

## Safe workflow

1. **Frame scope** — confirm MSE instance, component (Nacos/Sentinel/Seata/ARMS), environment, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius of a Sentinel rule change? what Nacos namespaces are in scope? what Seata transactions are in-flight?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud MSE Microservice Engine: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
