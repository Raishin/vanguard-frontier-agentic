# Java Agents

Role-based agent board for adversarial, evidence-first static review of **Java /
JVM** enterprise applications — JDK lifecycle, concurrency, JVM performance,
container sizing, framework readiness, security, data access, transactions,
migrations, messaging, resilience, testing, and the application-server
license-exit portfolio decision.

## Taxonomy note

`java` is a language/runtime, not a cloud provider, but it is a shipped topical
board and therefore has its own dedicated `provider` value: every asset uses
`provider: java` with a shared `java-` ID prefix. This mirrors the other
non-cloud topical boards (`dotnet`, `hr`, `legal`, `marketing`), each of which
carries its own `provider` value. See `docs/taxonomy.md` and
`docs/language-stack-boards.md`.

## Agent tiers

| Tier | Purpose | Default access | Live execution |
|---|---|---|---|
| Router | Classifies a Java/JVM task and dispatches the narrowest specialist set | read-only | not allowed |
| Review agents | Audit JDK, concurrency, performance, sizing, frameworks, security, data, messaging, resilience, and tests | read-only | not allowed |
| Portfolio | Translates specialist findings + supplied cost figures into an exit/stay ROI decision | read-only | not allowed |

Every agent in this board is **static-review** — it reads source and sanitized
configuration only. No agent runs a build, executes tests, opens a database or
broker connection, invokes a JDK, contacts a live system, or edits project files.

## Router

| Agent | Primary use |
|---|---|
| `java-maestro-agent` | Classify a Java/JVM task; dispatch one specialist (focused) or a parallel team of up to four (multi-domain). Routes only — never answers Java questions itself. |

## Review agents

| Agent | Primary use | Must refuse when |
|---|---|---|
| `java-jdk-lifecycle-and-upgrade-agent` | JDK vendor/version + support/license-boundary exposure and phased upgrade planning | asked to assert a vendor lifecycle date from memory, run a build, or invoke a JDK |
| `java-concurrency-and-virtual-thread-agent` | Virtual-thread adoption correctness (pooling/bounding anti-patterns, carrier pinning by JDK version) and classic concurrency hazards | asked to advise VT-at-scale without the JDK version or pinning (JFR) evidence |
| `java-jvm-performance-and-gc-agent` | GC selection/tuning, allocation/heap review, OOM/leak triage | asked to recommend a GC switch without supplied pause-time/allocation evidence |
| `java-container-and-kubernetes-readiness-agent` | JVM-in-container sizing (`MaxRAMPercentage` vs `-Xmx`, off-heap headroom, CPU/GC ergonomics, GC-pause vs liveness-probe) | asked to treat it as a generic pod-spec review |
| `java-framework-production-readiness-agent` | Spring Boot / Quarkus / Micronaut ship/don't-ship readiness (config, health probes, AOT/build-time DI, jakarta namespace) | asked to hardcode a framework EOL date or to own security/JDK-lifecycle |
| `java-spring-security-agent` | Spring Security 6 filter-chain authorization, method-security precedence, CSRF, Actuator exposure | asked to adjudicate deserialization/parser RCE (routes to the deserialization agent) |
| `java-deserialization-and-parser-security-agent` | Untrusted deserialization and parser RCE — `ObjectInputStream`, SnakeYAML, Jackson default typing, XML XXE | asked to deserialize or execute a sample payload |
| `java-jpa-hibernate-performance-agent` | JPA/Hibernate fetch strategy — N+1, `JOIN FETCH` vs `@EntityGraph` vs `@BatchSize` vs DTO, `open-in-view`, HikariCP sizing | asked for a connection string or to run a query/migration |
| `java-transaction-and-consistency-agent` | `@Transactional` boundary/propagation/isolation + dual-write (`save()`-then-`send()`) → outbox / saga | asked to own Kafka delivery-semantics wiring (routes to the Kafka agent) |
| `java-database-migration-safety-agent` | Flyway/Liquibase deploy safety — no in-place edits to applied migrations, expand-contract/phased drops | asked to run or apply a migration |
| `java-kafka-reliability-agent` | Kafka delivery semantics (idempotence vs exactly-once), consumer dedup, lag, ordering | asked to treat producer idempotence as exactly-once |
| `java-resilience-pattern-agent` | resilience4j composition — retry/circuit-breaker aspect order, retry-without-idempotency block, timeout/bulkhead | asked to add a retry to a non-idempotent write without a dedup key |
| `java-test-architecture-agent` | JVM test architecture — JUnit5 isolation/parallel gating, Testcontainers discipline, ArchUnit, test smells | asked to run the suite or a coverage tool |
| `java-application-server-exit-agent` | Portfolio/ROI: WebLogic/WebSphere/JBoss + Oracle-JDK license exit-vs-stay, consuming supplied cost figures | asked to produce a payback number without supplied cost inputs, or to hardcode vendor pricing |

## Operating notes

- A vendor lifecycle date stated from memory, an unfiltered `ObjectInputStream`
  on request data, a SnakeYAML bare `Constructor`, Jackson default typing without
  a `PolymorphicTypeValidator`, an unhardened XML parser, an N+1 on a request
  path, a `save()`-then-`send()` dual-write, an in-place edit to an applied
  migration, a `@Retry` on a non-idempotent write, and producer idempotence
  mistaken for exactly-once are among the highest-impact defects this board
  exists to catch.
- The board is **static-review only**. Production mutations (deploy, migrate,
  rollout, key/secret changes) and live telemetry (GC pauses, p99, consumer lag)
  are out of tier — the responsible agent refuses and hands off to the named
  human owner or the appropriate provider/observability board.
- `java-application-server-exit-agent` is the single **portfolio/ROI** agent: it
  consumes specialist findings plus **user-supplied** cost figures and refuses to
  fabricate a payback number. It never hardcodes vendor pricing or tenant data.
- Cross-domain concerns route out: cloud/Kubernetes platform operations to the
  provider/Kubernetes boards, in-cluster observability to the OpenTelemetry/
  Prometheus boards, generic CI-secret scanning to the CI supply-chain agent.
