# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized YAML snippets (no real endpoints, no auth tokens):
- `prometheus.yml` (global, scrape_configs, rule_files, remote_write, alerting)
- Alerting rules YAML (`groups[].rules[]` with `alert:`, `expr:`, `for:`, `labels:`, `annotations:`)
- Recording rules YAML (`groups[].rules[]` with `record:`, `expr:`)
- `alertmanager.yml` (route, inhibit_rules, receivers)
- Optional: current `prometheus_tsdb_head_series` metric value or approximate series count

If the user provides only a partial config, note which sections are absent and limit findings to the provided scope.

### Step 2 — Cardinality audit

Scan every `scrape_configs` job and every metric label dimension referenced in alerting and recording rules.

Check for:
- Labels sourced from high-cardinality application dimensions:
  - `user_id`, `request_id`, `session_id`, `transaction_id`, `trace_id`
  - `url_path`, `uri`, `endpoint` (unless aggressively normalized)
  - `pod` or `container` labels used as primary grouping in `sum by()` without aggregation
- Use of `__` internal labels in user-facing metric names

Example cardinality risk:
```yaml
# HIGH — request_id is unbounded; this creates one series per request
http_requests_total{method="GET", path="/api/v1/items", request_id="abc-123"} 1
```

Correct pattern:
```yaml
# CORRECT — drop high-cardinality label before exposition
http_requests_total{method="GET", path="/api/v1/items"} 1
```

Note the `prometheus_tsdb_head_series` threshold: above 5 million series, TSDB memory pressure becomes significant. Above 10 million, OOM risk is high without explicit memory tuning (`--storage.tsdb.max-block-duration`, chunk encoding).

### Step 3 — Recording rules audit

Check whether recording rules exist for:
- SLO error-rate expressions that appear in alerting rules
- High-cardinality aggregation queries used in Grafana dashboards
- Any `rate()` or `increase()` expression over a window longer than 5 minutes that is queried at sub-minute dashboard refresh

Flag absence of recording rules for any expression that appears more than once across rules files as MEDIUM.

Example correct recording rule:
```yaml
groups:
  - name: slo_recordings
    rules:
      - record: job:http_requests_total:rate5m
        expr: sum(rate(http_requests_total[5m])) by (job)
```

### Step 4 — Alert expression correctness audit

For every `alert:` rule, check:

**4a. `for:` duration**
- Missing `for:` or `for: 0m` → HIGH (bare threshold, flapping)
- `for:` less than two scrape intervals → flag as LOW (alert may still flap)
- Recommended minimum: `for: 5m` for infrastructure alerts, `for: 1m` for latency SLOs

```yaml
# HIGH — missing for:
- alert: HighErrorRate
  expr: rate(http_errors_total[5m]) > 0.05

# CORRECT
- alert: HighErrorRate
  expr: rate(http_errors_total[5m]) > 0.05
  for: 5m
```

**4b. `absent()` usage**
- `absent(some_metric)` fires if `some_metric` was never scraped — review whether the metric is always expected to exist
- If the metric only appears when the condition is active (e.g., an error counter), `absent()` fires in the absence of errors, which is a false positive

**4c. SLO alerting pattern**
- MWMB (multi-window multi-burn-rate) is the Google SRE-recommended SLO alerting pattern
- Single-window SLO alerts miss slow burns → MEDIUM finding

Example MWMB pattern:
```yaml
# MWMB — fast burn (1h + 5m windows) and slow burn (6h + 30m windows)
- alert: SLOFastBurn
  expr: >
    (
      job:slo_error_rate:rate1h > (14.4 * 0.001)
      and
      job:slo_error_rate:rate5m > (14.4 * 0.001)
    )
  for: 1m
  labels:
    severity: page
```

### Step 5 — AlertManager routing audit

Parse the `route:` tree and check:

**5a. Duplicate alert routing**
- Routes that lack `continue: false` on a catch-all receiver may send alerts to multiple receivers unexpectedly
- Verify whether `continue: true` on intermediate routes is intentional

**5b. Inhibition rules**
- `inhibit_rules[].source_matchers` and `target_matchers` must reference labels that actually appear on alerts
- Overly broad inhibition (e.g., `source_matchers: [severity="critical"]` without namespace scope) can suppress alerts across unrelated services

Example inhibition rule review:
```yaml
# RISKY — inhibits all warnings when any critical fires, across all namespaces
inhibit_rules:
  - source_matchers: [severity="critical"]
    target_matchers: [severity="warning"]
    equal: [alertname]
```

**5c. Receiver configuration**
- Slack/PagerDuty receivers must have `api_url` or `routing_key` from environment variables or Kubernetes secrets — never hardcoded in the YAML
- Check for hardcoded webhook URLs or tokens as a CRITICAL security finding

### Step 6 — Scrape config security audit

For every `scrape_configs` entry check:

**6a. `honor_labels`**
```yaml
# HIGH — untrusted workload can override job/instance labels
scrape_configs:
  - job_name: user-app
    honor_labels: true
```
Only `honor_labels: true` on trusted federation endpoints is acceptable.

**6b. External HTTP targets**
- Any target with a scheme pointing outside the cluster (e.g., `http://api.external.com`) is an SSRF candidate
- Flag all non-cluster targets for review

**6c. `job_name` uniqueness**
- Duplicate `job_name` values cause target label collisions — flag as HIGH

### Step 7 — remote_write and retention audit

**7a. remote_write queue memory**
```yaml
remote_write:
  - url: https://metrics.example.com/api/v1/write
    queue_config:
      capacity: 100000        # HIGH memory if series count is large
      max_samples_per_send: 10000
```
Flag `capacity` values above 10,000 combined with high series counts as a memory risk.

**7b. write_relabel_configs label drops**
```yaml
# MEDIUM — silently drops 'region' label before remote_write; data loss
write_relabel_configs:
  - source_labels: [region]
    action: labeldrop
```
Flag any `labeldrop` or `labelmap` action that targets non-`__` labels without explicit justification.

**7c. Retention**
- Default Prometheus retention is 15 days (`--storage.tsdb.retention.time=15d`)
- No remote_write + retention < 30d → MEDIUM (compliance gap for most regulated environments)
- Recommend Thanos, Cortex, or Grafana Mimir for long-term storage

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence summary: pass / needs work / critical issues found>

## Evidence level
<live evidence | user-provided sanitized config | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>
...

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never recommend setting `honor_labels: true` on any scrape target unless the user explicitly confirms it is a trusted Prometheus federation endpoint.
- Treat hardcoded webhook URLs, API keys, or tokens in `alertmanager.yml` receivers as CRITICAL — they must be moved to Kubernetes Secrets or environment variable references.
- Do not recommend disabling scrape TLS verification (`tls_config.insecure_skip_verify: true`) without flagging it as a security regression.
- Treat any recording rule or alert rule that references a metric with unbounded label cardinality as HIGH, even if the immediate symptom (OOM) has not yet occurred.
- Flag the absence of alerting on `prometheus_tsdb_head_series` itself — teams often have no alert for their own Prometheus health.
