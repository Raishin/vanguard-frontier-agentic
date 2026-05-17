---
name: rpa-workflow-resilience-review
description: Use this skill when reviewing exported RPA workflow definitions for resilience and security defects that cause unattended bots to fail silently in production. Trigger when a user provides UiPath XAML files, Automation Anywhere bot exports, Power Automate Desktop flow definitions, Blue Prism process XML, or project dependency manifests, or asks why an unattended bot crashes silently, double-processes transactions, or times out under load. This skill reviews workflow definitions statically; it never connects to a live orchestrator, never runs a bot, and never requests runner credentials or orchestrator URLs.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: resilience
  lifecycle: experimental
---

# RPA Workflow Resilience Review

## Purpose
This skill reviews exported RPA workflow definitions — UiPath XAML, Automation Anywhere task bots, Power Automate Desktop flows, and Blue Prism processes — for the resilience and security defects that destroy unattended production runs: hardcoded credentials, brittle UI selectors, missing exception handling, non-idempotent transaction logic, hard sleeps, and invisible failures. A bot that silently fails at 2 AM, double-posts a financial transaction, or leaks an RDP session is a production incident. This review catches those defects from the exported artifact before they reach the orchestrator scheduler.

## Lean operating rules
- Treat any hardcoded credential, API key, connection string, or token present in a workflow variable, argument, activity property, or annotation as CRITICAL — these must be stored in the orchestrator credential vault or a secure asset; no exceptions.
- Treat UI selectors built on volatile attributes — absolute screen coordinates, positional `idx` values, dynamic window titles, auto-generated IDs from frameworks like WinForms or SAP session GUIDs — as HIGH; they break on any application UI change or version bump; use stable anchors (semantic names, fixed automation IDs, accessible names).
- Treat any application or UI interaction boundary with no enclosing exception handler (Try/Catch or platform equivalent) as HIGH — an unattended run dies silently with no diagnostics and no item-level status update.
- Treat a workflow that is not idempotent — re-running it after a partial failure re-submits a form, re-sends an email, re-posts a transaction, or re-creates a record with no already-processed guard — as HIGH; every unattended workflow must be re-runnable without side-effect duplication.
- Treat any use of a fixed Delay activity to wait for an application or UI element instead of an element-exists, on-element-appear, or application-ready condition as HIGH — this is the RPA equivalent of a hard sleep; it either races the application or pads every run unnecessarily.
- Treat the absence of a retry or recovery scope around transient steps, OR the presence of an unbounded or infinite retry with no cap or circuit-breaker, as HIGH — transient failures (network blips, SAP logon timeouts) need bounded retry; infinite loops produce zombie runs.
- Treat attended-only constructs — message boxes, input dialogs, manual user prompts, pop-up confirmations — inside a workflow marked or scheduled for unattended execution as HIGH; they block indefinitely and consume a robot license slot.
- Treat any selector, screenshot artifact, annotation, test-data variable, or hardcoded string in the workflow that contains PII, real customer names, account numbers, or production data as HIGH — sanitize before sharing or storing in source control.
- Treat the complete absence of logging and per-transaction-item status updates as MEDIUM — unattended failures become invisible until a downstream system breaks or an SLA is missed.
- Treat mutation of a shared orchestrator queue item or shared asset without a lock or transaction boundary as MEDIUM — concurrent robot instances can corrupt processing state.
- Treat missing cleanup logic on failure paths — browser sessions, SAP connections, RDP sessions, or file handles left open after an exception — as MEDIUM; leaked sessions exhaust connection pools and degrade the orchestrator environment.
- Do not recommend disabling exception handling or removing logging to simplify a workflow — refuse and explain that both are load-bearing safety mechanisms for unattended operation.
- Label every finding with its evidence basis: exported workflow provided, documentation-based, or inference from absent artifact.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Credential and secrets findings (hardcoded values, unprotected variables)
- Selector resilience assessment (stable vs. volatile attribute strategy per workflow)
- Exception handling coverage (which interaction boundaries are unguarded)
- Idempotency and transaction-safety findings (re-run risk, queue locking)
- Wait strategy findings (fixed delays vs. element-ready conditions)
- Attended/unattended compatibility findings (blocking constructs in unattended flows)
- Observability findings (logging, item status, alerting)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
