---
name: revenue-critical-journey-integrity-review
description: Use this skill to review the cross-tier seams of revenue-critical journeys — checkout, payment submission, account creation, and login — for idempotency of money-moving and account-creating requests, server-side re-validation of client-enforced rules, webhook duplicate/out-of-order handling, retry-storm safeguards, and PCI DSS SAQ-scope judgment. Use when a request crosses client-to-server, system-to-processor, or webhook-back-into-system and a failure at that seam would double-charge, double-fulfill, bypass a required step, drop revenue, or misjudge PCI scope. Static review only; it does not execute payment flows and its PCI SAQ output is an advisory scoping opinion, never a compliance attestation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-16"
  category: resilience
  lifecycle: experimental
---

# Revenue-Critical Journey Integrity Review

## Purpose

Review the seams of revenue-critical journeys — the points where a request crosses from client to server, from your system to a payment processor, or from a webhook back into your system — so a journey that looks correct in any one tier does not break where the tiers meet. The dominant seam failures are non-idempotent money-moving requests, client-enforced rules the server never re-validates, webhook consumers that assume exactly-once/in-order delivery, unbounded retries that become retry storms, and PCI DSS SAQ-scope misjudgment.

## When to use

Use this skill when the user asks to:

- review whether a checkout, payment, subscription, coupon, or account-creation request is safe to retry (idempotency at money-moving seams),
- confirm the server re-validates rules the client enforces (price, discount, quantity, eligibility, step-completion),
- review a webhook consumer for duplicate-delivery and out-of-order handling,
- review retry/backoff/circuit-breaker safety at a revenue-critical seam across web, mobile, or backend consumers,
- get an advisory PCI DSS SAQ-scope opinion for the payment integration model actually in the code.

## When not to use

Do not use this skill for:

- tier-internal review that an owning specialist owns — DOM XSS/CSP and client injection (use the frontend security review), backend authorization-model design, mobile-platform specifics, or infrastructure hardening. This skill reviews the seam, not the interior; hand tier-internal findings to the owning agent.
- issuing a PCI compliance attestation, signing an SAQ, or acting as an assessment of record. SAQ-scope output here is advisory only.
- any live exercise of a payment system — executing flows, replaying webhooks, or sending requests to live/sandbox/staging processors. This skill is static review only.

## Preconditions

- The money-moving and account-creating request paths in scope, across whichever tiers exist.
- The webhook consumer code and the event types it acts on.
- The retry configuration (max attempts, backoff, jitter, timeout, circuit breaker) for the seams in scope.
- The payment integration model (redirect, iframe/hosted fields, direct post/custom form) if a SAQ-scope opinion is requested.
- The processor/SDK and version in scope, so idempotency and webhook guidance matches the real API surface.

## Lean operating rules

- Confirm retry/replay reachability before flagging an idempotency gap; a genuinely non-retryable internal call is not a finding.
- Treat the server as the only enforcement boundary; a client-only check is UX, not enforcement.
- Require webhook consumers to be both idempotent (dedupe by event id or business key) and order-tolerant.
- Require bounded, backoff-with-jitter retries with a timeout at every revenue seam; add a circuit breaker or dead-letter path for backend/queue consumers.
- Give a PCI SAQ-scope opinion only against the integration model present in the code, name the candidate SAQ, and label it advisory.
- Never request, echo, store, or reproduce cardholder data, API keys, session tokens, or webhook signing secrets; redact-and-flag any that appear.
- Label every claim `repo evidence`, `context7-grounded`, `documentation-based`, or `inference`.

## Context7 documentation protocol

Processor idempotency semantics, webhook retry windows, event ordering, and signature verification are version-sensitive. Before asserting any processor-specific behavior, load the Context7 tools (`resolve-library-id` then `query-docs`) and cite the current documented behavior, labeled `context7-grounded`; if Context7 lacks coverage, use the processor's official documentation and label it `documentation-based`. Never rely on memorized API details. See [official sources](references/official-sources.md).

## Workflow

Follow the step-by-step review and output contract in [workflow and output](references/workflow-and-output.md). At a high level: (1) map the seams in scope; (2) for each money-moving/account-creating request, check idempotency against reachable retry/replay; (3) check server re-validation of every client-enforced rule; (4) check webhook consumers for idempotency + order-tolerance; (5) check retry safety; (6) if requested, form the advisory SAQ-scope opinion; (7) emit findings with evidence tiers and tier-internal handoffs.

## Decision gates

- Block only on a seam failure with a demonstrated reachable retry/replay/bypass path.
- Every processor-specific claim is Context7-grounded or documentation-based, never memory.
- Every SAQ-scope statement is advisory and tied to the integration model in the code.
- Every tier-internal finding is handed off, not adjudicated.

## Evidence classification

Label each finding `repo evidence` (seen in the code), `context7-grounded` (current provider docs via Context7), `documentation-based` (official docs), or `inference`. Documentation never proves a specific deployment's live behavior — say so.

## Security and privacy constraints

Static review only. Never transmit, request, store, or reproduce cardholder data (PAN/CVV), API keys, session tokens, or webhook signing secrets; treat any such string as a redact-and-flag finding. Never execute payment flows or contact any live/sandbox/staging payment system. PCI SAQ-scope output is an advisory opinion, never an attestation.

## Escalation conditions

Escalate to incident response on any evidence of a live failure (duplicate charges in logs, replayed webhooks, retry amplification). Escalate SAQ-scope opinions to the merchant's compliance owner or a QSA as advisory input.

## References

Load these only when needed:

- [Workflow and output](references/workflow-and-output.md) — the end-to-end review steps and the finding/output contract.
- [Idempotency and safe retries](references/idempotency-and-safe-retries.md) — idempotency keys at money-moving seams and bounded backoff-with-jitter retry design.
- [Webhook delivery: dedup and ordering](references/webhook-delivery-dedup-ordering.md) — duplicate-delivery and out-of-order handling for webhook consumers.
- [Server-side re-validation and the client trust boundary](references/server-side-revalidation-trust-boundary.md) — why the server is the only enforcement boundary and what to re-check.
- [PCI DSS SAQ scope boundaries](references/pci-saq-scope-boundaries.md) — SAQ A vs A-EP vs D by integration model and the 6.4.3/11.6.1 payment-page expectations.
- [Official sources](references/official-sources.md) — the primary-source ledger for every claim in this skill.

## Response minimum

Return, at minimum:

- the seam(s) in scope and, per finding, the failure class and evidence tier;
- the cross-tier failure narrative (how a retry, replay, or bypass reaches a wrong outcome);
- concrete remediation and an exact verification step;
- the advisory SAQ-scope opinion when requested, labeled advisory;
- tier-internal handoffs and any incident-response escalation.

## Anti-goals

- Do not expand into tier-internal review; own the seam, hand off the interior.
- Do not present a SAQ-scope opinion as a compliance determination.
- Do not exercise any live payment system or reproduce any secret or PAN.
- Do not assert processor behavior from memory.
