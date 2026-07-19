# Idempotency and safe retries

## Why this matters

Every money-moving or account-creating request that can be retried — by a client
double-submit, a gateway timeout-then-retry, or a user hitting back/refresh — is a
duplicate-charge or duplicate-account risk unless the seam is explicitly made safe
to repeat. This is a seam failure, not a tier-internal bug: the client, server,
and processor can each look correct in isolation while the combination
double-charges a customer. Idempotency keys close the money-moving side;
bounded backoff-with-jitter retries close the availability side by preventing a
single downstream blip from becoming a self-inflicted retry storm.

## Where an idempotency mechanism is required

An idempotency mechanism is required at any request that both (a) moves money
or creates a billable resource (create charge, create payment intent, create
subscription, apply a coupon), and (b) is reachable by at least one
retry/replay path: client double-submit (double click, back-button resubmit),
gateway or SDK-level automatic retry on connection error/timeout, or a user
manually resubmitting after an ambiguous response.

A genuinely non-retryable internal call (no client retry, no gateway retry
configured, no user-facing resubmit path) is not a finding — confirm
reachability before flagging, per the skill's lean operating rules.

## NORMATIVE: what Stripe requires and provides

Per Stripe's API reference on idempotent requests (`documentation-based`;
Context7 was attempted first and returned a monthly-quota error, so this was
grounded via direct WebFetch of the official page):

- A client generates and attaches an idempotency key to a `POST` request (e.g.
  create charge, create customer). This lets the request be retried safely
  after a connection error without performing the operation twice.
- Stripe caches the first response for that key — including a 5xx error
  response — for **at least 24 hours**.
- A replay with the same key returns the cached first result rather than
  re-executing the operation.
- If a retry sends the same key with **different parameters** than the
  original request, Stripe returns an error rather than silently applying the
  new parameters. The key scopes to the exact original request, not just the
  endpoint.

These are processor-documented behaviors, not general recommendations — do not
generalize them to a processor without checking that processor's own docs.

## RECOMMENDATION: how to implement idempotency at your own seams

For a seam the merchant's own server owns (e.g. an internal "create order" call
in front of the processor call), the same pattern applies as a design
recommendation, not a processor mandate:

- **Client-generated key** — a UUID generated once per logical operation
  attempt, not per HTTP retry, sent in a header or body field.
- **Server-side unique constraint** — the server stores the key in a table
  with a unique constraint (e.g. a unique index on `idempotency_key`)
  alongside the operation's result. The insert-or-conflict on that constraint
  is the correctness mechanism, not an application-level `SELECT`-then-`INSERT`
  check, which races under concurrent retries.
- **Return the cached first result on replay** — on a unique-constraint
  conflict, look up the stored result for that key and return it unchanged,
  rather than re-running the operation.
- **Scope keys to exact request parameters**, mirroring Stripe's behavior:
  reject (or explicitly document accepting) a replay whose parameters differ
  from the original.

## Reviewer evidence criteria

For each money-moving or account-creating request in scope, check for:

- A client-generated idempotency key attached to the request (header or body
  field), generated once per user-intent attempt, not regenerated on every
  retry.
- A server-side store (table/cache) keyed on that value with a **unique
  constraint**, not merely an in-memory or best-effort check.
- Confirmation that a replay with the same key returns the original stored
  result rather than re-invoking the money-moving/account-creating operation.
- Confirmation that a replay with the same key but different parameters is
  rejected or explicitly handled, not silently accepted.
- For calls that pass through to Stripe (or another processor) directly,
  confirmation the processor's own idempotency-key parameter is used on the
  outbound call, in addition to (not instead of) any client-facing key.

Absence of all of the above at a seam where retry/replay is reachable is a
blocking finding per the skill's decision gates.

## Retry design: bounded backoff, jitter, and the retry-storm failure

Retry-storm mitigation is not a processor-documented requirement — no
NORMATIVE claim applies here. It is a reliability-engineering recommendation
grounded in AWS's Well-Architected Reliability Pillar guidance
(`documentation-based`).

## RECOMMENDATION: bounded, backoff-with-jitter retries

Per AWS's Well-Architected Reliability Pillar guidance on limiting retries, a
**retry storm** occurs when retries compound across multiple layers of a stack
under failure — each layer retrying independently — so the failing service
receives new requests plus every layer's retries simultaneously, saturating it
and reducing availability further rather than recovering it. The documented
mitigation is client-side exponential backoff, jitter, and a maximum retry cap.
A second AWS primary source, the Builders' Library article "Timeouts, retries,
and backoff with jitter," covers the same pattern in more implementation depth
and may be cited alongside it.

For every retry path at a revenue-critical seam (web client, mobile client, or
backend/queue consumer), check for:

- **Maximum attempt cap** — retries stop after a fixed, small number of
  attempts rather than continuing indefinitely.
- **Exponential backoff** — each successive retry waits longer than the last
  (e.g. doubling), rather than retrying at a fixed interval.
- **Jitter** — the wait interval is randomized within a range rather than
  deterministic, so many clients failing at once do not retry in lockstep and
  re-collide on the recovering service.
- **Request timeout** — each attempt has an explicit timeout rather than
  waiting indefinitely for a hung connection, so a stuck attempt does not
  block the retry budget.
- **Circuit breaker or dead-letter path for backend/queue consumers** — once
  failures exceed a threshold, the consumer stops retrying and either trips a
  circuit (short-circuits further calls for a cooldown period) or routes the
  message to a dead-letter queue, instead of retrying forever against a
  downstream that is already down.
- **No cross-tier amplification** — if a mobile client retries and the API
  gateway it calls also retries independently, confirm the combination is
  still bounded; check the effective worst-case request multiplier across all
  layers, not one layer's configuration in isolation.

Unbounded retries, retries with no backoff, or fixed-interval retries with no
jitter at a revenue-critical seam are retry-storm-risk findings, citable
against the AWS Well-Architected reference.

## Applicable versions

- Stripe idempotent-request behavior described here reflects the current
  Stripe API reference page as of this review; idempotency-key retention
  windows and behavior are processor-specific and must be re-verified against
  the exact processor/SDK version in scope, per the skill's Context7
  documentation protocol.
- The AWS Well-Architected Reliability Pillar guidance is framework-level
  (not tied to a specific AWS service version) and applies to any retry
  design, not only AWS-hosted systems.
- Documentation describes intended behavior only; whether a specific
  deployment actually implements a unique constraint, a bounded backoff
  policy, or a circuit breaker is an `inference` from code review, never
  proven by the existence of these docs.

## Sources

- [Stripe API reference — Idempotent requests](https://docs.stripe.com/api/idempotent_requests) — supports the client-generated-key mechanism, the 24-hour-minimum cache of the first response (including 5xx), and the error-on-differing-parameters behavior.
- [AWS Well-Architected Reliability Pillar — Limit retries](https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/rel_mitigate_interaction_failure_limit_retries.html) — supports the retry-storm definition and the exponential-backoff/jitter/max-retry-cap mitigation.
- [AWS Builders' Library — Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/) — supplementary primary source on backoff-with-jitter implementation.

Last verified: 2026-07-16.
