---
name: dotnet-observability-otel-review
description: Use this skill when reviewing in-application OpenTelemetry wiring in an ASP.NET Core service — OpenTelemetry SDK registration, trace context propagation across service boundaries, structured logging, correlation and trace identifiers in logs, metrics instrumentation, trace sampling, the health-vs-readiness check distinction, and PII leakage into span attributes or log messages. Trigger when a user provides ASP.NET Core source (Program.cs, telemetry registration, logging configuration, instrumentation code) or sanitized appsettings, asks whether their telemetry is wired correctly, or wants to know why traces are missing or logs are uncorrelated. This skill reviews source and sanitized configuration statically; it never runs the app or contacts a telemetry backend.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-19"
  category: observability
  lifecycle: experimental
---

# .NET Observability & OpenTelemetry Review

## Purpose
This skill reviews how an ASP.NET Core service wires its own OpenTelemetry — the SDK registration, the instrumentation it enables, the logs it emits, the metrics it records, and the sampling it applies. Telemetry only helps an operator if traces propagate across service calls, logs carry a trace identifier, exceptions keep their structure, and the application does not write secrets or customer data into spans. The review catches PII in span attributes and log messages, missing trace context propagation on outbound calls, uncorrelated logs, exceptions logged as interpolated strings, missing request-rate/latency/error metrics, unbounded production sampling, and a health endpoint doing a readiness job. It is a static review of source and sanitized configuration; it never runs the app or contacts a telemetry backend.

EXPLICIT NON-GOAL: Collector topology, exporters and backends, and dashboard infrastructure are out of scope and belong to the `opentelemetry` provider board — route those there. This skill reviews only what the .NET application itself configures and emits.

## Trigger conditions
- A user provides ASP.NET Core source (`Program.cs`, OpenTelemetry registration, logging configuration, instrumentation code) or sanitized `appsettings`.
- A user asks whether their in-application OpenTelemetry wiring is correct.
- A user reports missing traces, uncorrelated logs, or unstructured exception logging.
- A user wants a pre-merge observability review of an ASP.NET Core service.

## Lean operating rules
- CRITICAL — Treat PII (email, access token, password, payment card number, full request body) written to span attributes or log messages as a telemetry data-leak defect.
- HIGH — Treat no trace context propagation across service boundaries (missing instrumentation on outbound `HttpClient` or messaging) as broken distributed tracing.
- HIGH — Treat the absence of a correlation or trace identifier in logs as an uncorrelatable logging surface.
- MEDIUM — Treat exceptions logged as interpolated strings, losing structure and stack, as a degraded error-observability defect.
- MEDIUM — Treat missing request-rate, latency, and error-rate metrics as an unmonitorable service surface.
- MEDIUM — Treat 100% trace sampling configured for production with no cost note as an unbounded telemetry-cost risk.
- MEDIUM — Treat health checks not distinguished from readiness checks as an orchestration defect.
- Never recommend "log everything"; never recommend 100% sampling in production without a cost caveat; never recommend disabling a failing gate as the fix.
- Static review only — never request secrets, connection strings, tokens, tenant identifiers, or customer data; never run builds, tests, or the application, or contact a telemetry backend or live system.
- Label every finding with an evidence-basis label: `confirmed (config provided)`, `inference (config partial)`, `assumption (config absent)`, or `unknown`.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- PII-in-telemetry findings (span attributes, log messages)
- Trace context propagation findings (outbound `HttpClient`, messaging instrumentation)
- Log-correlation findings (correlation or trace identifier in logs)
- Structured-logging findings (exceptions logged as interpolated strings)
- Metrics-instrumentation findings (request-rate, latency, error-rate)
- Sampling findings (production sampling rate and cost note)
- Health vs. readiness boundary findings
- Severity-labelled finding list (critical / high / medium / low), each with an evidence-basis label
- Safe next actions
