# Testcontainers Discipline and ArchUnit Adoption

> Static review only. Scope: Testcontainers container-lifecycle/sharing strategy and readiness signaling, and ArchUnit layering/cycle rule adoption (including brownfield freezing). Sources: the Testcontainers documentation (JUnit 5 integration, container reuse, and Wait Strategies pages) and the ArchUnit User Guide (rules and `FreezingArchRule` sections) — see the skill's official docs. Testcontainers' reuse feature has at various points been labeled experimental/opt-in and its exact enablement mechanism (`~/.testcontainers.properties`, `TESTCONTAINERS_REUSE_ENABLE`) should be confirmed against the version in use rather than assumed; treat an unconfirmed reuse-mechanism claim as `inference (partial source)`.

## Container-sharing strategy: singleton-with-reuse vs per-test @Container

Two legitimate patterns exist, and the review's job is to confirm the chosen one is followed consistently, not to prefer one universally:

**Per-test `@Container`** — a `@Container`-annotated field, managed by the `Testcontainers` JUnit 5 extension, started and stopped around each test class (or, with an instance field under `PER_METHOD` lifecycle semantics, effectively per test method if not hoisted to a base class). Correct default for lightweight containers or when tests must not share any state. Wrong when the container is heavy (a real database engine, Kafka) and many test classes each pay full startup cost.

**Singleton-container pattern with Ryuk reuse** — a `static` container field on a shared base test class, started once (often via a static initializer or `start()` called without a corresponding `stop()`), left running for the JVM's lifetime, and cleaned up by Testcontainers' Ryuk resource-reaper container rather than by test code. Reuse across separate test-suite *runs* (not just within one JVM) additionally requires `.withReuse(true)` on the container plus `testcontainers.reuse.enable=true` enabled locally/in CI:

```java
abstract class PostgresIntegrationTest {
    static final PostgreSQLContainer<?> POSTGRES =
        new PostgreSQLContainer<>("postgres:16")
            .withReuse(true);

    static {
        POSTGRES.start(); // no stop() — Ryuk reaps it when the JVM exits
    }
}
```

This pattern amortizes startup cost across every test class that extends the base class. It is only safe if every test resets the container's mutable state (truncate tables, delete topics, drop and recreate schema) in `@BeforeEach`/`@AfterEach` — the review must confirm this reset exists whenever a singleton container is found; its absence is a HIGH cross-test-pollution finding even though the sharing pattern itself is correct.

## Wait strategies vs Thread.sleep

A container reporting "started" does not mean the service inside it is ready to accept connections — Postgres, Kafka, and most application containers have a gap between process start and readiness. `Thread.sleep(n)` as a bridge is wrong in both directions: too short under CI load (flaky), too long by default (slow every run, every time). Testcontainers' built-in `Wait` strategies observe the actual readiness signal:

```java
new GenericContainer<>("myapp:latest")
    .waitingFor(Wait.forHttp("/health").forStatusCode(200))
    .withStartupTimeout(Duration.ofSeconds(60));
```

Prefer, in order of specificity: a health-check strategy the image already defines (`Wait.forHealthcheck()`), an HTTP/TCP readiness probe (`Wait.forHttp`, `Wait.forListeningPort`), or a log-message pattern known to indicate readiness (`Wait.forLogMessage(...)`) as a last resort when no probe exists. Flag any `Thread.sleep` adjacent to container startup or to any other async operation (message consumption, eventual-consistency polling) the same way — as a timing defect, not a style preference.

## ArchUnit layering and cycle rules

ArchUnit rules encode architectural intent as executable tests:

```java
@ArchTest
static final ArchRule layerDependenciesAreRespected =
    layeredArchitecture()
        .consideringAllDependencies()
        .layer("Controller").definedBy("..controller..")
        .layer("Service").definedBy("..service..")
        .layer("Repository").definedBy("..repository..")
        .whereLayer("Controller").mayNotBeAccessedByAnyLayer()
        .whereLayer("Service").mayOnlyBeAccessedByLayers("Controller")
        .whereLayer("Repository").mayOnlyBeAccessedByLayers("Service");

@ArchTest
static final ArchRule noCycles =
    slices().matching("..(*)..").should().beFreeOfCycles();
```

## FreezingArchRule for brownfield adoption

Introducing a layering or cycle rule on an existing codebase almost always surfaces pre-existing violations; failing the build on all of them at once blocks unrelated work and invites the rule being disabled outright — the outcome this skill must prevent. `FreezingArchRule.freeze(rule)` is the correct adoption path: it persists the current violation set to a store (by default a text file under `archunit_store/`), fails the build only on *new* violations not already in the store, and provides a path to shrink the store over time as violations are fixed. Requirements for a sound adoption:

- The freeze store must be committed to version control (it is the enforcement baseline, not a local cache).
- There must be a visible, owned plan or tracked issue to shrink the store — a freeze with no shrink plan is a permanent exemption wearing an enforcement costume.
- Never recommend deleting the freeze store, regenerating it to absorb new violations, or replacing `FreezingArchRule` with an always-pass rule to make CI green — any of these defeats the rule's purpose exactly like disabling a failing gate would.

## Escalation conditions

- The container being reviewed is used for load/performance testing rather than functional test isolation → describe the isolation concern found, but defer load-testing methodology.
- The user asks this agent to actually start a container or run an ArchUnit check against compiled classes → out of scope for static review; describe what to run and who runs it.
