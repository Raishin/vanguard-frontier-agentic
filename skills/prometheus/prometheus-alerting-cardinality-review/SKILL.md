---
name: prometheus-alerting-cardinality-review
description: Use this skill when reviewing Prometheus or AlertManager configuration for cardinality, alerting correctness, scrape security, remote_write safety, or retention adequacy. Trigger when a user provides prometheus.yml, alertmanager.yml, recording rules YAML, alerting rules YAML, or asks whether their Prometheus setup is production-ready.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: observability
---

# Prometheus Alerting and Cardinality Review

## Purpose
This skill reviews Prometheus and AlertManager configuration for cardinality explosion risks, recording rule adequacy, alert expression correctness, routing tree safety, scrape configuration security, and retention posture. Cardinality explosion is the leading cause of Prometheus OOM crashes in production, and flapping alerts from missing `for:` durations erode on-call trust faster than any other alerting defect.

## Lean operating rules
- Flag any label dimension that is unbounded at the application level (e.g., `user_id`, `request_id`, `session_id`, `url_path`, `pod_hash`) — these cause cardinality explosion and must be moved off the label set or aggregated away.
- Treat `prometheus_tsdb_head_series` exceeding 5 million as a cardinality warning threshold; note it if the user reports series counts or if the config makes it likely.
- Treat any alert rule with `for: 0m`, `for: 0s`, or no `for:` field as HIGH — bare threshold alerts flap on every scrape jitter.
- Treat `honor_labels: true` on any scrape target that is not a trusted federation endpoint as HIGH — it allows the scraped workload to override `job` and `instance` labels.
- Treat any scrape config with a non-cluster HTTP scheme (`http://external-host`) as a potential SSRF candidate and flag it.
- Recording rules are required for any PromQL expression used in dashboards or SLO burn-rate calculations; flag their absence as MEDIUM.
- Multi-window multi-burn-rate (MWMB) alerting is the correct pattern for SLO breach detection; flag single-window SLO alerts as MEDIUM.
- Flag `remote_write` configs where `write_relabel_configs` drop non-`__` metric labels — data loss is silent.
- Flag retention under 30 days with no `remote_write` or Thanos/Cortex integration as MEDIUM compliance risk.
- Do not recommend disabling any existing alert or recording rule without stating the specific reason and risk trade-off.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Cardinality risk assessment (label audit findings)
- Alert expression correctness findings (for: duration, absent() misuse, MWMB posture)
- AlertManager routing and inhibition findings
- Scrape config security findings
- Retention and remote_write findings
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
