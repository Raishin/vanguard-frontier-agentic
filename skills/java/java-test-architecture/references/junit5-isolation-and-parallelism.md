# JUnit 5 Lifecycle, Isolation, and Parallelism

> Static review only. Scope: JUnit 5 (Jupiter) test lifecycle, instance management, and parallel-execution safety on the JVM. Sources: the JUnit 5 User Guide's "Test Classes and Methods," "Test Instance Lifecycle," and "Parallel Execution" chapters (see the skill's official docs). Parallel-execution defaults, the exact `@Isolated` and `@ResourceLock` semantics, and which `junit-platform.properties` keys exist have moved across JUnit 5 minor releases — when the reviewed project's `junit-jupiter` version is not stated, treat any claim about a specific annotation's or config key's availability as `inference (partial source)` and ask for the version before asserting it as `confirmed`.

## Why isolation is the primary flakiness lever

A JVM test class is, by default, instantiated fresh per test method (`TestInstance.Lifecycle.PER_METHOD`), which is JUnit 5's baseline isolation guarantee. Every pathology in this reference is a way that guarantee gets defeated: through static state that survives instantiation, through an opt-in `PER_CLASS` lifecycle that is then mismanaged, or through parallel execution that turns a latent bug into a nondeterministic one.

## Shared mutable static state

Static fields, singleton registries, un-cleared `ThreadLocal` values, static caches, and `System.setProperty` calls all persist across test instances because they live on the class, not the instance. Flag:

```java
class OrderServiceTest {
    static List<Order> seenOrders = new ArrayList<>(); // survives every test instance

    @Test
    void firstTest() {
        seenOrders.add(new Order("A"));
        assertThat(seenOrders).hasSize(1); // passes alone, fails after other tests run first
    }
}
```

The fix is either to stop using static state (move it to an instance field, which is safe under `PER_METHOD`) or, if `@TestInstance(Lifecycle.PER_CLASS)` is intentionally chosen (e.g. for an expensive shared fixture), to reset the state explicitly in `@BeforeEach`/`@AfterEach`. `PER_CLASS` alone does not grant isolation — it removes the default isolation and shifts the isolation obligation onto the test author.

## Order dependence

JUnit 5 does not guarantee declaration order or alphabetical order by default; the actual order is intentionally deterministic-but-unspecified unless a `MethodOrderer` is configured (`@TestMethodOrder(MethodOrderer.OrderAnnotation.class)`, `MethodName`, `Random`, etc.). A suite that passes only under one ordering — because an earlier test populated state a later test reads — is order-dependent even if it currently passes reliably in CI; a build-tool upgrade, a JUnit engine change, or `Random` ordering can break it without any test code changing. Treat any test that implicitly depends on another test having run first as a defect, and treat `@TestMethodOrder` used to paper over an order dependency (rather than to express an intentionally sequential fixture, which is rare and should be justified) as the wrong fix.

## Time, locale, and timezone dependence

`System.currentTimeMillis()`, `Instant.now()`, `LocalDate.now()`, and the JVM's default `Locale`/`TimeZone` are all ambient, mutable, environment-dependent state:

- A test asserting on "today" or "this month" without freezing time fails at midnight, at a month/year boundary, or under DST transitions.
- A test formatting numbers, currency, or dates using the default `Locale` fails when CI or a contributor's machine runs under a non-`en-US` locale.
- A test comparing timestamps without accounting for the runner's default `TimeZone` fails when CI runs in UTC and a developer's machine does not, or vice versa.

The correct remedy is dependency-injecting a `java.time.Clock` (`Clock.fixed(...)` in tests) rather than calling `Instant.now()` directly in code under test, and explicitly setting `Locale`/`TimeZone` for the test (JUnit 5 has no built-in Locale/TimeZone extension in core; a project-level test rule/extension or explicit `try/finally` restore around `Locale.setDefault`/`TimeZone.setDefault` is required — and doing this under parallel execution requires an `@ResourceLock` because `Locale`/`TimeZone` defaults are JVM-global, not per-thread).

## Unguarded parallel execution

Parallel execution is opt-in via `junit.jupiter.execution.parallel.enabled=true` (typically in `junit-platform.properties`) plus a `mode` (`same_thread` by default even when enabled, or `concurrent` via `junit.jupiter.execution.parallel.mode.default=concurrent` or the `@Execution(CONCURRENT)` annotation). Enabling this without auditing every shared resource is the single highest-leverage defect this reference covers, because it converts every latent static-state or ambient-locale bug above from "reliable" to "randomly fails under load":

```java
@ResourceLock("system-properties")
@Test
void mutatesGlobalConfig() {
    System.setProperty("feature.flag", "on");
    ...
}
```

Require, before signing off on parallel execution: (1) every test that reads or writes JVM-global state (`System` properties, default `Locale`/`TimeZone`, a shared file/port/temp directory, a shared Testcontainers singleton) is annotated `@ResourceLock` with a resource key shared by every other test touching that same resource, or the test class is annotated `@Isolated` to force it onto its own execution lane; (2) resource-lock keys are consistent (`ResourceAccessMode.READ_WRITE` vs `READ` matters — two tests both taking a `READ` lock on the same resource can still run concurrently, which is correct only if neither mutates it).

## Escalation conditions

- The flake reproduces only under a specific CI runner's retry/parallelism configuration rather than in the test code itself → the CI-mechanics half of the diagnosis belongs to `ci-test-pipeline-review-agent`; this skill still owns the JVM-side root cause.
- The user asks to actually run the suite with `-Djunit.jupiter.execution.parallel.enabled=true` to observe failures → out of scope for static review; describe what to run and who runs it.
