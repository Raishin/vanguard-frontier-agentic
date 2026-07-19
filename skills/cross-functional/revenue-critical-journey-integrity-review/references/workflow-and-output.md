# Workflow and output contract

## Why this matters

A revenue-critical journey can pass review tier by tier — the client validates correctly,
the server handles the happy path, the webhook consumer parses events fine — and still
break at the seams between those tiers. This reference is the fixed procedure for finding
those seam failures and the fixed shape for reporting them, so two reviews of the same
journey converge on the same findings instead of drifting with reviewer style.

## Workflow

Work the seams in scope, not the tiers. For the journeys named in the skill's scope
(checkout, payment, subscription, coupon, signup, login), follow these steps in order:

1. **Map the seams in scope.** For each journey, enumerate every point where a request
   crosses a trust or process boundary: client-to-server, system-to-processor, and
   webhook-back-into-system. A journey with no payment step still has a client-to-server
   seam (signup, login) even if it has no processor or webhook seam.
2. **Check idempotency at money-moving/account-creating requests.** For every request that
   creates a charge, customer, subscription, order, or coupon application, determine whether
   a client retry, gateway retry, or user-driven replay can actually reach it, then check for
   an idempotency mechanism. See
   [idempotency and safe retries](idempotency-and-safe-retries.md).
3. **Check server-side re-validation of client-enforced rules.** For every rule the client
   enforces (price, discount, quantity, eligibility, step-completion), confirm the server
   independently re-checks it rather than trusting the client's decision. See
   [server-side re-validation and the client trust boundary](server-side-revalidation-trust-boundary.md).
4. **Check webhook consumers for dedup and order-tolerance.** For every webhook consumer
   acting on a money-moving or fulfillment event, verify it dedupes by event id or a business
   key and does not assume in-order delivery. See
   [webhook delivery: dedup and ordering](webhook-delivery-dedup-ordering.md).
5. **Check retry safety.** For every retry path at a revenue seam (client, mobile, backend
   queue consumer), verify a bounded attempt count, backoff with jitter, a timeout, and — for
   backend/queue consumers — a circuit breaker or dead-letter path. Flag unbounded or
   synchronized retries as retry-storm risk.
6. **Form the advisory SAQ-scope opinion, if requested.** Match the integration model
   actually present in the code (redirect, iframe/hosted fields, direct post/custom form) to
   a candidate SAQ and label the opinion advisory. See
   [PCI DSS SAQ scope boundaries](pci-saq-scope-boundaries.md).
7. **Emit findings** using the schema below.

## Finding schema

Each finding carries:

- **seam** — the specific cross-tier boundary (e.g. "mobile client -> payment intent create
  endpoint", "Stripe webhook -> order fulfillment service").
- **failure class** — one of `idempotency`, `client-trust`, `webhook-dedup-ordering`,
  `retry-storm`, `saq-scope`.
- **evidence tier** — one of `repo evidence`, `context7-grounded`, `documentation-based`,
  `inference`, per the skill's evidence classification.
- **cross-tier failure narrative** — the concrete path from a retry, replay, or bypass to a
  wrong outcome (double charge, double fulfillment, skipped step, wrong SAQ).
- **remediation** — the concrete mechanism (idempotency-key column plus unique constraint,
  event-id dedupe table, server-side price re-check, bounded backoff with jitter).
- **verification step** — an exact, reproducible check that would confirm the fix.
- **owning-tier handoff** — the specialist who owns any tier-internal portion of the finding,
  or "none" if the finding is fully seam-scoped.

## Decision gates

- **Block only on a demonstrated reachable retry/replay/bypass path.** A seam without a
  demonstrated reachability path (e.g. a genuinely non-retryable internal call) is not a
  blocking finding.
- **Processor claims are grounded, not memory.** Every processor-specific idempotency,
  webhook, or signature-verification claim is `context7-grounded` or `documentation-based`,
  per [official sources](official-sources.md); never asserted from memory.
- **SAQ opinions are advisory.** Every SAQ-scope statement is labeled advisory and tied to
  the integration model actually in the code — never presented as a compliance
  determination.
- **Tier-internal findings are handed off, not adjudicated.** A finding that lives entirely
  inside one tier (a DOM sink, an authorization-model design choice, a mobile-platform
  detail) is routed to the owning agent, not resolved here.

## Response minimum

Every review returns, at minimum:

- the seam(s) in scope and, per finding, the failure class and evidence tier;
- the cross-tier failure narrative for each finding;
- concrete remediation and an exact verification step per finding;
- the advisory SAQ-scope opinion, labeled advisory, when requested;
- tier-internal handoffs and any incident-response escalation (evidence of a live failure —
  duplicate charges in logs, replayed webhooks, retry amplification — escalates immediately
  rather than being filed as a normal review comment).

## Sources

This is a process reference internal to this skill; it draws on the skill's own workflow
definition and the sibling references rather than external primary sources:

- [Idempotency and safe retries](idempotency-and-safe-retries.md)
- [Server-side re-validation and the client trust boundary](server-side-revalidation-trust-boundary.md)
- [Webhook delivery: dedup and ordering](webhook-delivery-dedup-ordering.md)
- [PCI DSS SAQ scope boundaries](pci-saq-scope-boundaries.md)
- [Official sources](official-sources.md) — the primary-source ledger for every
  processor- and standard-specific claim these references make.

Last verified: 2026-07-16.
