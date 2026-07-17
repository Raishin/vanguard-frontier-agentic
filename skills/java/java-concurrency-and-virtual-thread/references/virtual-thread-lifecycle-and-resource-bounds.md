> Scope: Project Loom virtual threads as delivered by JEP 444 (openjdk.org/jeps/444 — Release 21, Status Closed/Delivered). The thread-per-task lifecycle model and its resource-bound implications described here have not changed in later releases; only carrier-pinning behavior has (see `carrier-pinning-and-jdk-version-gating.md`). Verify the JDK version in scope before combining this file's guidance with version-specific pinning detail. Cross-checked against Oracle's core Java SE concurrency documentation (docs.oracle.com/en/java/javase/).

## Why this decision matters

Virtual threads change one thing: the cost of creating and blocking a thread. They do not change the finite capacity of anything downstream — a JDBC connection pool still has a maximum size, a partner API still has a rate limit. A codebase can adopt virtual threads correctly (unbounded, one-per-task creation) and still become *less* safe than before, because the platform-thread pool it replaced was quietly doing double duty as a concurrency limiter for a scarce resource. Reviewing virtual-thread adoption means reviewing both the thread-creation pattern and every resource boundary a virtual-thread task reaches into.

## Anti-pattern 1: pooling virtual threads

```java
// WRONG: defeats the point of virtual threads
ExecutorService pool = Executors.newFixedThreadPool(200,
    Thread.ofVirtual().factory());
```

Using `Thread.ofVirtual().factory()` as the `ThreadFactory` for a *fixed-size* pool means at most 200 virtual threads exist at once, reused across tasks via the pool's internal queue. This throws away the entire benefit — virtual-thread creation is designed to be near-free and there is no reuse benefit — while reintroducing a queueing bottleneck identical to a platform-thread pool, plus virtual-thread bookkeeping overhead for none of the payoff.

```java
// CORRECT: one virtual thread per task, unbounded creation
ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor();
```

## Anti-pattern 2: capping virtual threads behind a small fixed pool

A variant of the same mistake: someone tries to "control concurrency" by capping the virtual-thread executor itself (a small explicit pool size, or a bounded queue placed in front of `newVirtualThreadPerTaskExecutor()`). The cap does limit concurrency, but at the wrong layer — it throttles unrelated work that has nothing to do with the actually-scarce resource, and it reintroduces exactly the queueing behavior virtual threads were adopted to avoid. If concurrency genuinely needs to be limited, limit access to the *resource*, not the *thread supply* (see the next section).

## The resource-bound-stripping bug — the dangerous migration

```java
// BEFORE: a platform-thread pool of 20 implicitly caps concurrent DB calls to 20
ExecutorService pool = Executors.newFixedThreadPool(20);
```

That `20` was very likely never chosen for CPU parallelism. It was, in practice, an implicit concurrency limiter — capping how many requests could simultaneously hold a connection from a similarly-sized HikariCP pool, or call a rate-limited downstream API. Migrating naively:

```java
// AFTER (migration): the implicit bound just vanished
ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor();
```

now allows unbounded fan-out. Every inbound request spawns a new virtual thread, and every one of those threads immediately contends for the same connection pool or the same rate-limited API. Once concurrent demand exceeds the resource's real capacity, the pool or API saturates, timeouts cascade, and the resource is effectively exhausted under load that the old code would have simply queued safely. This is a correctness regression, not a performance nuance — treat it as such.

Correct migration: keep the executor unbounded, and make the resource guard explicit:

```java
// AFTER (correct): unbounded executor, explicit resource guard
private final Semaphore dbPermits = new Semaphore(20); // sized to the pool's real maximumPoolSize

Result callDatabase() throws InterruptedException {
    dbPermits.acquire();
    try {
        try (Connection c = dataSource.getConnection()) {
            return doWork(c);
        }
    } finally {
        dbPermits.release();
    }
}
```

The guard's permit count must be sized to the resource's actual verified capacity (e.g. the connection pool's own `maximumPoolSize`, or the documented rate limit), never guessed, and its acquire/release must bracket the resource's real usage window — releasing a permit before the resource is actually free (e.g. releasing on task submission rather than on connection return) can silently reopen the bound it was meant to enforce.

A bare `Semaphore` around the call site is the simplest correct pattern; a resource-boundary rate limiter wrapping the client itself is equally acceptable. Either is fine as long as *some* explicit, correctly-sized bound exists after the migration — the finding is the absence of any bound, not the specific mechanism chosen.

## Escalation

If the constrained resource is a JPA/Hibernate-managed connection pool, the *sizing* question (what should `maximumPoolSize` actually be) belongs to `java-jpa-hibernate-performance-agent`. This agent's job stops at: a bound must be explicit and must be re-imposed after the migration; it does not prescribe the number.

## Known uncertainty

- Real capacity numbers (pool sizes, rate limits) are estate-specific; never assume a value — ask for the configuration.
- Not every platform-thread-pool size was an intentional resource limiter; some were simply arbitrary. Ask rather than assume intent, but treat the *absence* of any bound post-migration as the finding regardless of the original intent, since the risk (resource exhaustion under fan-out) is the same either way.
