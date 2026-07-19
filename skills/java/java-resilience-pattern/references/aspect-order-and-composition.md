# Resilience4j Aspect Order and Transaction Composition

> Static review only. Scope: resilience4j Spring annotations (`@Retry`, `@CircuitBreaker`, `@RateLimiter`, `@TimeLimiter`, `@Bulkhead`) and equivalent functional chaining (`Decorators.ofSupplier(...)` / `Decorators.ofCompletionStage(...)`) under `resilience4j-spring-boot2`/`resilience4j-spring-boot3`. Verified against the resilience4j Spring Boot integration guide and the CircuitBreaker/Retry module pages (see the agent's `officialDocs`) as of 2026-07-17. resilience4j is versioned independently of Spring Boot — confirm the project's actual `resilience4j-spring-boot*` artifact version before assuming the documented default hasn't changed; the vendor page is the source of truth, not this file's paraphrase of it.

## Why this decision matters

A retry and a circuit breaker are not interchangeable protections, and their **relative position in the call chain changes what each one actually measures**. There is no single "correct" order in the abstract — the correct order is the one the team can state and justify — but resilience4j ships a specific, non-obvious default, and a review that doesn't check for an explicit order is reviewing nothing.

## The documented default order

Per the resilience4j Spring Boot integration guide: when multiple resilience4j annotations decorate one method and no aspect-order properties are set, the composition is

```
Retry ( CircuitBreaker ( RateLimiter ( TimeLimiter ( Bulkhead ( Function ) ) ) ) )
```

`Retry` is outermost — the vendor docs describe it as "applied at the end (if needed)". Concretely: the Retry aspect calls the CircuitBreaker-decorated function, and if that call fails, calls it again — meaning **each retry attempt is a separate, independently-evaluated call into the CircuitBreaker.**

## The mechanical consequence (why this is a finding, not trivia)

With the default order, a single logical operation that fails and is retried three times contributes up to three recorded outcomes to the CircuitBreaker's sliding window, not one. Two practical effects:

- **Premature OPEN.** The CircuitBreaker's `failureRateThreshold` and `minimumNumberOfCalls` are usually calibrated in the team's head against *logical operations*, but the breaker is actually counting *attempts*. A dependency with a real 20% logical failure rate can look far worse to the breaker once each logical failure fans out into several recorded failures — tripping the breaker faster than the configured threshold was meant to allow.
- **Wasted retry budget after OPEN.** Once the breaker trips, every subsequent call — including the remaining attempts of an in-flight retry — throws `CallNotPermittedException` immediately. If that exception is not excluded from Retry's retryable-exception set (`ignoreExceptions`/`retryExceptions` predicate), Retry will burn its remaining attempts retrying a call the breaker is refusing to make at all, adding latency with zero chance of success.

## Setting an explicit order

resilience4j exposes per-module `*AspectOrder` properties (`resilience4j.retry.retryAspectOrder`, `resilience4j.circuitbreaker.circuitBreakerAspectOrder`, `resilience4j.ratelimiter.rateLimiterAspectOrder`, `resilience4j.timelimiter.timeLimiterAspectOrder`, `resilience4j.bulkhead.bulkheadAspectOrder`). The vendor docs state plainly: **higher value = higher priority = more outer.** Their own example:

```yaml
resilience4j:
  circuitbreaker:
    circuitBreakerAspectOrder: 1
  retry:
    retryAspectOrder: 2
```

keeps Retry outermost (the default) — explicitly, rather than by omission. To make CircuitBreaker outermost instead (so it evaluates one aggregate outcome per logical operation, not per attempt), `circuitBreakerAspectOrder` must be set **higher** than `retryAspectOrder` — the inverse of the vendor's own example.

**Critical review point: the order these annotations are physically stacked on the method in Java source is not the execution order.** Unlike a linear list of independent, `@Order`-annotated Spring advisors, resilience4j composes its own aspect internally according to the `*AspectOrder` properties (or the order of chained calls in a functional-style `Decorators.ofSupplier(...)` builder). Do not accept "the source lists `@Retry` above `@CircuitBreaker`" as evidence of anything. The only valid evidence is (a) explicit `*AspectOrder` properties in configuration, or (b) explicit functional-chaining code where the nesting is literal Java. Absent either, the order **is** the vendor default above — treat that as a finding requiring the team to confirm it was a deliberate choice, not a silent one.

## What "usually intended" looks like

Most services reviewing dependency health want the CircuitBreaker to answer "is this logical operation, after whatever retries we allow, failing at an unacceptable rate?" — not "is any individual attempt failing?". That reading points at CircuitBreaker-outermost (`circuitBreakerAspectOrder` > `retryAspectOrder`), the inverse of the shipped default. Treat this as the usual intent only as a prior, not a rule: some teams deliberately want the vendor default (fast-tripping breaker, cheap fail-fast retries after OPEN) to shed load quickly during a real outage. Either choice is defensible — an *unexamined* default is not. State which one the evidence shows and, if the config is silent, say so as `assumption (source absent)`.

## Retry composed with `@Transactional`

`@Transactional` is implemented the same way resilience4j's Spring integration is — as an AOP proxy advice around the method. Two independent, well-documented Spring AOP consequences apply directly here:

1. **Retry must wrap the transaction, not sit inside it.** If `@Retry` is the *inner* aspect relative to `@Transactional` (the transaction opens, then the retry loop runs inside that one transaction/connection), a failure on attempt one — e.g. a deadlock or lock-wait timeout — happens **inside an already-open transaction**. Depending on the exception, Spring may mark that transaction rollback-only (surfacing as `UnexpectedRollbackException` at the eventual commit), or the retry may reuse a connection whose transaction state is now inconsistent. The safe composition is Retry **outside** the transactional boundary — each attempt opens (and, on failure, rolls back and closes) its own transaction. In practice this means the `@Retry`-annotated method must be a *different, outer* method than the `@Transactional` one, calling into it.
2. **Self-invocation silently drops one of the two aspects.** Both `@Retry` and `@Transactional` only take effect on calls that arrive through the Spring proxy — an *external* call to the bean. If a `@Retry` method calls `this.otherMethod()` on the same bean where `otherMethod()` is `@Transactional`, that call bypasses the proxy entirely and the transactional advice never runs (or vice versa, if `@Transactional` is on the outer method and it internally calls a `@Retry` method via `this.`). Flag any same-class internal call between a `@Retry` method and a `@Transactional` method as a finding regardless of which one is "outer" on paper — the annotation on the callee is dead code in that call path.

The exact default precedence between resilience4j's combined Spring aspect and Spring's own `TransactionInterceptor` (i.e., which one Spring's auto-proxy creator places outermost when *neither* has an explicit `@Order`) is not established by the pages verified for this skill — treat any claim about that specific default as `inference (partial source)` unless the codebase shows explicit `@Order`/`@EnableTransactionManagement(order=...)` configuration or the two annotations are demonstrably on different beans/methods with a clear external call between them. The structural rule (retry must wrap the transaction; self-invocation drops the inner aspect) holds regardless of that unresolved precedence question.

## Escalation conditions

- The finding is about transaction propagation/isolation *semantics* themselves (not the retry/transaction ordering) → hand to the Java transaction and consistency agent.
- The finding is about JPA/Hibernate fetch strategy or connection-pool sizing on the same code path → hand to `java-jpa-hibernate-performance-agent`.
- The user asks to actually execute the composed call chain to observe order at runtime → out of scope for static review; describe what to instrument and who runs it.
