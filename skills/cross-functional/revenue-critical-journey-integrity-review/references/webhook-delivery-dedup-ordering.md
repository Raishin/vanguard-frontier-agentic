# Webhook delivery: dedup and ordering

## Why this matters

A webhook consumer that assumes a processor delivers each event exactly once,
in order, will eventually be wrong — in a way that moves money or fulfillment
state. The same event redelivered after a timeout, manual resend, or retry
can fulfill an order twice; two events for the same subscription arriving out
of order can push a state machine into a state it never designed for (e.g.
acting on `invoice.paid` before the consumer has recorded the
`customer.subscription.created` it depends on). This is a seam failure
between the processor and the consuming system, not a bug inside either one:
the processor behaves exactly as documented and the consumer's logic is
correct in isolation — the combination breaks.

## The failure

- **Duplicate fulfillment.** The same event ID is delivered more than once
  (automatic retry, a manual dashboard/CLI resend, or a genuine at-least-once
  redelivery), and a consumer with no dedupe re-runs the fulfillment side
  effect (ship the order, grant the entitlement, apply the coupon) a second
  time.
- **Stale-state action.** A later-arriving event is processed before an
  earlier one the consumer's state machine implicitly depends on, and the
  consumer errors, silently no-ops the wrong thing, or lands in an
  inconsistent state because it assumed in-order arrival.

## NORMATIVE: documented Stripe delivery behavior

Per Stripe's webhook best practices (`documentation-based`):

- **Retries are real and can repeat delivery of the same event.** In live
  mode, Stripe automatically retries a failing endpoint for up to **3 days**
  with exponential backoff. In sandbox/test mode, Stripe retries **3 times
  over a few hours**.
- **Manual resend is a separate, additive path.** A dashboard resend is
  available for up to **15 days** after event creation; a Stripe CLI resend
  (`stripe events resend <event_id> --webhook-endpoint=<endpoint_id>`) is
  available for up to **30 days**. Manually resending an event does **not**
  cancel Stripe's own automatic retry schedule, even if the manual resend
  gets a `2xx` response — so a single failing delivery can produce the
  automatic retries *and* an operator-triggered resend, multiplying the
  redelivery count a consumer must tolerate.
- **Endpoints are disabled on continuous failure.** A live-mode endpoint
  that fails continuously for 3 days is disabled by Stripe.
- **Ordering is explicitly not guaranteed.** Stripe states it does not
  guarantee delivery in the order events are generated — Stripe's own
  example is that a single subscription creation can generate
  `customer.subscription.created`, `invoice.created`, `invoice.paid`, and
  `charge.created` in any order.
- **The documented dedupe mechanism is event-ID logging.** Stripe's stated
  guidance is to guard against duplicate receipts by logging processed
  event IDs and not reprocessing an already-logged ID; when dedup needs to
  span distinct Event objects for the same underlying change, key on the
  `data.object` ID together with `event.type`.

These are Stripe-documented behaviors, not general webhook-delivery norms —
re-verify against the specific processor's own documentation before
generalizing to a non-Stripe processor.

## RECOMMENDATION: dedupe table and order-tolerant state machine

Dedupe table design:

- Maintain a persistent store of processed event IDs (Stripe's `evt_...` ID
  or the processor's equivalent) with a **unique constraint** on the ID
  column, not an in-memory set — the consumer must survive process restarts
  and concurrent delivery.
- Insert-then-act, not act-then-insert: attempt the insert first and treat a
  unique-constraint violation as "already processed," returning success
  without re-running the side effect; a check-then-insert races under
  concurrent redelivery.
- Where distinct Event objects can represent the same underlying business
  change (Stripe's documented case), dedupe on the business key
  (`data.object` ID + `event.type`), not solely on the wrapping event ID.
- Retain processed-event records at least as long as the processor's total
  possible redelivery window (automatic retries plus the longest manual
  resend window it documents) so a late resend still finds the record.

Order-tolerant state machine:

- No transition may assume a specific predecessor event has already been
  processed. Do not assume event B can never arrive before event A just
  because A "happens first" in the processor's normal flow.
- Where a later event logically depends on an earlier one (e.g. an invoice
  event depends on the subscription existing), make the handler tolerant of
  the missing precursor: fetch current object state from the processor's
  API rather than relying only on the event payload, or defer the dependent
  event until its precursor is observed — do not error or drop it.
- Treat each event as reporting the object's *current* state, not a delta.
  Reprocessing an idempotent "set to current state" handler is safe
  regardless of arrival order; reprocessing an "increment/decrement" handler
  is not.

## Authenticity: verify the signature, redact the secret

Confirm the consumer verifies the webhook signature on every inbound request,
using the processor's signing-secret-based verification, before acting on the
payload. Never request, log, echo, or reproduce the signing secret itself —
if a signing secret or other credential-shaped string appears in code, logs,
or configuration in scope, treat it as a redact-and-flag finding: describe its
presence and location, never print the value.

## Reviewer evidence criteria

For each webhook consumer that acts on a money-moving or fulfillment-relevant
event, check for:

- A persisted, uniquely-constrained store of processed event IDs (or
  business keys) that the handler checks before executing any side effect.
- Confirmation that a redelivered event (same ID, or same `data.object` +
  `event.type`) is a no-op on the side effect, not merely logged as a
  duplicate after the side effect already ran.
- No code path assuming one event type always arrives before another — look
  for handlers reading state the payload doesn't contain, which would only
  exist if a prior event had already been processed, with no fallback fetch
  or defer path.
- Signature verification on the inbound handler, using the processor's
  documented method, before any business logic runs.
- No signing secret, API key, or other credential committed, logged, or
  echoed anywhere in the consumer code or configuration.
- Retention of processed-event records for at least the processor's combined
  automatic-retry-plus-manual-resend window, so a legitimate late resend
  still hits the dedupe check.

Absence of event-ID (or business-key) dedupe, or a state machine that
silently assumes in-order arrival, on an event that moves money or fulfills
an order is a blocking finding per the skill's decision gates.

## Applicable versions

- The retry windows, resend windows, and disable-on-failure behavior above
  are Stripe's current documented webhook behavior as of this review;
  re-verify against Stripe's live page (or the equivalent page for a
  different processor) before citing an exact figure, since retry/resend
  windows are processor policy and can change.
- Whether a specific deployment's consumer actually implements event-ID
  dedupe and order-tolerant handling is an `inference` from code review —
  this documentation describes the processor's delivery contract, not any
  particular consumer's behavior.

## Sources

- [Stripe webhook best practices](https://docs.stripe.com/webhooks/best-practices) — supports the live-mode 3-day exponential-backoff automatic retry, the sandbox 3-retries-over-a-few-hours behavior, the 15-day dashboard / 30-day CLI manual resend windows, manual resend not cancelling automatic retries, the 3-day continuous-failure endpoint disable, out-of-order delivery, and the processed-event-ID/`data.object`+`event.type` dedupe recommendation.

Last verified: 2026-07-16.
