---
name: gcp-networking-observability
description: "Investigate GCP network issues by analyzing VPC Flow Logs, firewall logs, Cloud NAT logs, threat logs, and networking metrics. Diagnose connectivity, packet loss, top talkers, and firewall block events using BigQuery-first methodology and Cloud Monitoring fallback. Use when investigating VPC traffic anomalies, firewall DENY events, NAT port exhaustion, latency spikes, or running Connectivity Tests for path diagnostics."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: observability
---

# GCP Networking Observability

## Core Directive: Results First

1. Identify the primary source (VPC Flow, firewall, NAT, threat, metrics)
2. Execute minimum required query; present direct answer
3. Once primary source answers the question — STOP and report

## Log & Telemetry Sources

- **Threat Logs**: Cloud Firewall Plus / Cloud IDS — malicious traffic, SQL injection, malware signatures
- **VPC Flow Logs**: sampled IP traffic to/from NICs — volume trends, top talkers
- **Firewall Logs**: connection attempts matched by rules — DENY/ALLOW verification
- **Cloud NAT Logs**: NAT translations — audit, port exhaustion diagnosis
- **Networking Metrics**: time-series throughput, RTT, packet loss via Cloud Monitoring
- **Connectivity Tests**: static path analysis — firewall and routing misconfiguration

## Procedures

1. **Log source preference**: ALWAYS check BigQuery linked datasets (`_AllLogs`) before Cloud Logging for high-volume/aggregation. For metadata NULL issue: if VM name returns nothing, retry with `src_ip` (EXCLUDE_ALL_METADATA scenario).
2. **Tool selection**: MCP servers first (Cloud Monitoring MCP, BigQuery MCP, Cloud Logging MCP), then gcloud/bq fallback.
3. **Schema verification**: if BigQuery fails with `Unrecognized name`, run `bq show --schema` to validate, do a `--dry_run` before executing.

## Analysis Reference Directory (load only when needed)

| Scenario | Reference |
|---|---|
| Threat log analysis | references/threat-analysis.md |
| VPC Flow analysis | references/vpc-flow-analysis.md |
| Cloud NAT analysis | references/cloud-nat-analysis.md |
| Firewall rule analysis | references/firewall-analysis.md |
| Networking metrics | references/metrics-analysis.md |
| Connectivity tests | references/connectivity-tests.md |

## Boundaries (CRITICAL — NEVER violate)

- ALWAYS present direct answer as soon as identified
- NEVER run more than 2 exploratory queries before showing results
- NEVER perform secondary verification without explicit user permission
- ALWAYS print SQL before execution for review
- NEVER query a second data source if the primary already answered
- NO DISCREPANCY LOOPS: if Tool A gives 80K and Tool B gives 1K, present Tool A's result and STOP
- Treat "0 results" / "No records found" as a definitive conclusive finding — report and terminate
- BigQuery aggregation on `_AllLogs` is PRIMARY for Top-N/volume analysis — never use Cloud Monitoring API for volume aggregation
- Do NOT write shell scripts (.sh) or python files for data retrieval — use tool calls directly
- ALWAYS include a link to the Flow Analyzer: https://console.cloud.google.com/net-intelligence/flow-analyzer

## Official Docs

- https://cloud.google.com/vpc/docs/flow-logs
- https://cloud.google.com/firewall/docs/firewall-rules-logging
- https://cloud.google.com/nat/docs/monitoring
- https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/overview

## Security Notes

Read-only forensic analysis. Never modify firewall rules, routes, or NAT configs. Never run queries that write data. Print SQL before executing for user review.
