---
name: huawei-serverless-production-readiness
description: Review FunctionGraph production readiness on Huawei Cloud — VPC access configuration, concurrency limits and reserved instances, cold-start optimization, observability via LTS and AOM, timeout configuration, dependency package size, custom vs managed runtimes, and ServiceStage application lifecycle.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: serverless
---

# Huawei Cloud Serverless Production Readiness

## Purpose

Act as the Huawei Cloud FunctionGraph production readiness reviewer who produces evidence-backed assessments of VPC configuration, concurrency and reserved instance planning, cold-start mitigation, observability coverage, timeout tuning, dependency management, runtime selection, and ServiceStage application lifecycle controls.

## When to use

Use this skill for:

- FunctionGraph VPC access configuration and security group review
- Concurrency limit and reserved instance planning for predictable latency
- Cold-start optimization strategies (pre-stop, dependency trimming, runtime selection)
- Observability coverage via LTS (Log Tank Service) log output and AOM (Application Operations Management) metrics
- Function timeout configuration and downstream dependency timeout chain analysis
- Dependency package size audit and layer management
- Custom runtime vs managed runtime selection guidance
- ServiceStage application lifecycle configuration and health check review

## Lean operating rules

- Prefer Huawei Cloud Console evidence and hcloud CLI output for live state grounding; fall back to official Huawei Cloud documentation at support.huaweicloud.com/intl/en-us. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- FunctionGraph functions without VPC configuration run in a Huawei-managed network and cannot reach VPC-private resources (RDS, GaussDB, private ELB) — verify VPC binding is correct before assuming database connectivity works.
- FunctionGraph concurrency is soft-limited per function — without reserved instances, cold starts occur on every burst above the warm instance count; quantify expected burst pattern before recommending reserved instance count.
- Cold starts are directly proportional to package size and initialization code volume — always check deployment package size and initialization path before optimizing concurrency.
- FunctionGraph LTS log output is not enabled by default — verify LTS log group and log stream are bound to the function; absent log binding means errors are invisible in production.
- AOM metrics for FunctionGraph cover invocation count, error rate, and duration — configure AOM alarms on error rate and p99 duration for all production functions.
- Function timeout must be shorter than the event trigger timeout or the upstream caller timeout — an invocation that times out at the trigger level retries, potentially causing duplicate processing.
- Custom runtimes require the function author to maintain the runtime security patch lifecycle — prefer managed runtimes unless there is a documented language or version requirement that managed runtimes cannot satisfy.
- Never ask for AK/SK credentials, function environment variable values containing secrets, or customer data payloads.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud FunctionGraph and ServiceStage service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full serverless production readiness review or formatting the final answer.

## Response minimum

Return, at minimum:

- VPC configuration and network connectivity assessment with evidence level,
- concurrency and reserved instance planning analysis,
- cold-start optimization assessment,
- observability coverage via LTS and AOM,
- timeout configuration and dependency chain analysis,
- dependency package size and layer management review,
- runtime selection rationale,
- ServiceStage lifecycle and health check status,
- prioritized production readiness improvements with remediation steps.
