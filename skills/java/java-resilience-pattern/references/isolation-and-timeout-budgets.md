# Isolation, Timeout Budgets, and Fallback Correctness

> Static review only. Scope: `@Bulkhead`/`ThreadPoolBulkhead`, `@TimeLimiter`, `@RateLimiter`, and fallback methods (`@Recover`, a recovery method, or `Decorators` `.withFallback(...)`) in resilience4j-decorated Java/Spring code. Verified against the resilience4j Bulkhead, TimeLimiter, and RateLimiter module pages (see the agent's `officialDocs`) as of 2026-07-17. Numeric defaults (queue sizes, timeouts) quoted below illustrate the *shape* of the decision, not values to assert without seeing the actual config — a specific default value should be re-checked against the module page for the resilience4j version in use before being stated as fact.

## Bulkhead: semaphore vs thread pool is an isolation decision, not a rate decision

resilience4j ships two bulkhead implementations, and the choice changes *what* is protected, not just *how many* calls are allowed concurrently:

| Type | Mechanism | What it isolates | What it does not do |
|---|---|---|---|
| `SemaphoreBulkhead` (`@Bulkhead(type = SEMAPHORE)`, the annotation default) | A semaphore limiting concurrent permits | Concurrent call count on whatever thread called it | The call still runs **on the caller's own thread**; a slow call still occupies that thread for its full duration |
| `ThreadPoolBulkhead` (`@Bulkhead(type = THREADPOOL)`) | A bounded queue plus a dedicated, separately-sized thread pool | The caller's thread pool from a slow/blocked dependency — the call runs on the bulkhead's own threads | Requires the decorated method to return `CompletionStage`/`Future`; changes the calling convention |

The vendor docs are explicit that with `SemaphoreBulkhead`, "it is up to the client to ensure correct thread pool sizing that will be consistent with bulkhead configuration" — resilience4j does not manage the caller's threads for you. If the review goal stated (explicitly, or implied by the surrounding code — e.g. protecting a web-tier request-handling pool from a slow downstream) is *thread isolation*, `SemaphoreBulkhead` does not deliver it: a hung call under `SEMAPHORE` still pins whatever thread invoked it. Flag `SEMAPHORE` used where the comment, ticket, or surrounding retry/timeout configuration implies the intent was to protect the caller's own executor.

## TimeLimiter: it only bounds a call it can cancel

`TimeLimiter` decorates a `Supplier<CompletionStage<T>>` or a `Future<T>` supplier (`executeCompletionStage` / `executeFutureSupplier`, per the vendor TimeLimiter page) — it is fundamentally a bound on an **asynchronous** operation. Two review-relevant consequences:

- **No-op on a bare synchronous call.** A `@TimeLimiter` annotation on a method that isn't itself returning a `CompletionStage`/`Future` (and isn't composed with a `ThreadPoolBulkhead` or an explicit executor that produces one) has nothing to time out — the underlying blocking call runs to completion on whatever thread invoked it, regardless of the configured `timeoutDuration`. Flag `@TimeLimiter` stacked on a synchronous-only call path as ineffective unless the composition (e.g. `ThreadPoolBulkhead`, which naturally returns a future) actually makes the call asynchronous.
- **Timeout is not cancellation of the underlying work.** Even when TimeLimiter is correctly bounding a `Future`, timing out the wrapper does not necessarily interrupt the underlying I/O (a blocked JDBC call, an HTTP client without its own read timeout, a thread stuck in native code) unless that layer honors interruption or has its own timeout. TimeLimiter caps how long the *caller* waits; it does not by itself guarantee the *callee's* thread is freed. Flag a `TimeLimiter` used as the only timeout in the path with no underlying client/socket timeout configured beneath it — the caller stops waiting, but the resource (thread, connection) can still be held by the abandoned call.

## Timeout budget coherence

A composed path typically has several independent timing knobs; the review's job is to check they compose into a coherent total, not that any one of them looks reasonable in isolation:

- `TimeLimiter.timeoutDuration` — bound on one attempt.
- `Retry` `maxAttempts` × (`waitDuration` or the backoff function's output) — total retry wall-clock budget, layered on top of the per-attempt timeout when Retry is outside TimeLimiter.
- `CircuitBreaker.slowCallDurationThreshold` — a call slower than this counts as a *slow* call toward the failure rate even if it eventually succeeds; if this is set looser than `TimeLimiter.timeoutDuration`, the breaker will never see a "slow call," only "timed out" failures, silently disabling the slow-call-detection feature.
- The caller's own deadline (an upstream HTTP client timeout, a Kafka consumer poll interval, a user-facing SLA) — if `attempts × per-attempt-timeout` can exceed it, the caller times out and disconnects *before* the retry loop gives up, wasting every attempt after the caller has already moved on (and, on a write path, is exactly the kind of duplicate-effect risk the idempotency rule covers).

Flag any composed path where these numbers are visible in configuration but were not evidently checked against each other, and flag a path where the caller's own deadline is unstated as `assumption (source absent)` for the budget-coherence finding specifically (the other numbers can still be confirmed independently).

## RateLimiter

Per the vendor RateLimiter page, `limitForPeriod` and `limitRefreshPeriod` set the sustained rate, and `timeoutDuration` is how long a caller **blocks** waiting for a permit before receiving `RequestNotPermitted`. Review points:

- A `timeoutDuration` long enough to matter (seconds, not tens of milliseconds) turns a rate limiter into a latency and thread-occupancy hazard on the calling thread — the same "still ties up the caller's thread" concern as `SemaphoreBulkhead`. Flag a long or default-unexamined `timeoutDuration` on a request-path rate limiter.
- `RequestNotPermitted` must be handled deliberately (reject with a clear signal, shed load, or route to a fallback) — not silently retried by an outer `Retry` without backoff (that just re-creates the load the limiter exists to shed), and not swallowed into a generic catch-and-continue.

## Fallback correctness — must not swallow

A fallback (`@Recover`, a recovery method, or `Decorators...withFallback(...)`) is a deliberate degraded-mode response, not a way to make an error disappear. Flag a fallback that:

- Returns a default/empty/zero-value result **presented as if it were a normal, fresh result** (no flag, header, or log line marking it degraded) — callers and dashboards can't distinguish "the real answer was empty" from "the dependency was down."
- Catches the triggering exception type too broadly (e.g. a bare `Exception`/`Throwable` fallback signature) such that it also absorbs failures unrelated to the resilience event it's meant to handle (a `NullPointerException` from a bug looks identical to a `CallNotPermittedException` from an open circuit).
- Has no accompanying metric/log for the degraded path — a fallback with zero observability means every open-circuit or exhausted-retry event is invisible until a downstream symptom is noticed.

## Bounded queues / backpressure

`ThreadPoolBulkhead` (and any executor the reviewed code wires up manually alongside it) has a `queueCapacity`. An unbounded or very large queue does not prevent overload — it just moves the failure from "fast, explicit rejection" (`BulkheadFullException`) to "slow, silent memory growth," and the eventual failure mode is an OOM or a cascading GC pause instead of a clean, fast rejection the caller can react to. Flag unbounded/very large `queueCapacity`, and flag a bulkhead-full/queue-full condition that isn't itself handled by a fallback or explicit shed-load response.

## Escalation conditions

- The finding is about JVM thread-pool sizing/tuning unrelated to a resilience4j `ThreadPoolBulkhead` → hand to the Java concurrency and thread-pool agent.
- The finding is about the correctness of the business logic *inside* the fallback (not whether it swallows failure) → out of this agent's decision; note it and move on.
- The user asks to load-test or benchmark the composed path live → out of scope for static review.
