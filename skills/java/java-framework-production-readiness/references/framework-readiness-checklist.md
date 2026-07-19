> Scope: Spring Boot 3.x, Quarkus 3.x, and Micronaut 4.x, static review only. Covers seven readiness areas: externalized config, health/liveness/readiness wiring, graceful shutdown, jakarta namespace + JDK floor, build-time DI/AOT reflection safety, profile/config validation, and dependency BOM alignment. Does not cover authN/authZ/security-header posture (java-spring-security-agent), JDK lifecycle/EOL dates (java-jdk-lifecycle-and-upgrade-agent), or full native-image reachability-metadata enumeration (native-image reference/skill). Verify version-specific behavior against docs.spring.io/spring-boot/, quarkus.io/guides/, and docs.micronaut.io/latest/guide/ before finalizing a verdict — framework minor/patch releases move defaults.

# Framework Production Readiness Checklist

## Why a per-framework checklist, not a generic one

Spring Boot, Quarkus, and Micronaut solve the same production-readiness problems — health probing, graceful shutdown, safe reflection under ahead-of-time compilation — with different mechanisms and different failure modes when misconfigured. A reviewer who applies Spring Actuator intuition to a Quarkus service (or vice versa) will miss the actual gap. Identify the framework and its exact major version from the build file's BOM/parent coordinate before applying anything below.

## 1. Externalized configuration

No secret, credential, connection string, or API key belongs in source or in a checked-in `application.properties`/`application.yml` (including profile-specific files like `application-prod.yml`). The safe pattern is property-placeholder indirection to an environment variable or a secret manager:

```java
// Safe: indirected, no literal secret in the repo
@ConfigurationProperties(prefix = "payments")
public record PaymentsProperties(String apiKey) {}
```
```yaml
payments:
  api-key: ${PAYMENTS_API_KEY}
```

A literal value in a profile file — even one that looks like a placeholder (`changeme`, `xxx`) — is a CRITICAL finding unless proven to be dead/example config that is never loaded in a real profile. Do not ask the user for the live secret; ask for the file with the value redacted.

## 2. Health / readiness / liveness

All three frameworks distinguish (or should be made to distinguish) *liveness* (is the process alive, should it be restarted) from *readiness* (can it currently serve traffic, should it receive it). Collapsing both into one endpoint means a slow downstream dependency failing a readiness-style check also fails liveness and triggers an unnecessary restart — masking the real problem and adding churn.

- **Spring Boot** — Actuator health groups split the two: `management.endpoint.health.probes.enabled=true` plus `management.health.livenessstate.enabled=true` / `readinessstate.enabled=true` expose `/actuator/health/liveness` and `/actuator/health/readiness` distinctly. A bare `/actuator/health` wired to both K8s probes is a MEDIUM finding.
- **Quarkus** — SmallRye Health exposes `/q/health/live` and `/q/health/ready` out of the box once `quarkus-smallrye-health` is on the classpath; custom `@Liveness`/`@Readiness` `HealthCheck` beans should be scoped correctly (a slow external call belongs in a readiness check, not liveness).
- **Micronaut** — `/health` is exposed via `micronaut-management`; verify `HealthIndicator`s are scoped appropriately and that Kubernetes deployment manifests point liveness/readiness probes at distinct paths where the app distinguishes them (Micronaut's default health aggregation can also collapse the distinction if indicators are not split).

## 3. Graceful shutdown

Without it, a rolling deploy or scale-down can drop in-flight requests the instant SIGTERM arrives.

- **Spring Boot** — `server.shutdown=graceful` plus a bounded `spring.lifecycle.timeout-per-shutdown-phase` (e.g. `30s`). Unbounded or absent is HIGH for anything behind a load balancer or in Kubernetes.
- **Quarkus** — `quarkus.shutdown.timeout` (and `quarkus.shutdown.delay` if a pre-stop drain window is needed ahead of load-balancer deregistration).
- **Micronaut** — Netty server graceful-shutdown configuration; confirm it is not left at a zero/immediate default.

## 4. jakarta namespace correctness + JDK floor

Spring Boot 3.x, Quarkus 3.x, and Micronaut 4.x all moved to the `jakarta.*` namespace (from `javax.*`) and each documents a Java 17 minimum. Treat these as one combined check: a mixed-namespace import (`javax.persistence.Entity` alongside `jakarta.validation.constraints.NotNull`) is a build-breaking or classloading-breaking CRITICAL finding, and a compiler `release`/toolchain below the framework's documented floor is equally CRITICAL. This agent confirms the floor is met and stops there — it does not adjudicate whether that JDK line is itself still supported; that question routes to `java-jdk-lifecycle-and-upgrade-agent`.

## 5. Build-time DI / AOT reflection safety

Quarkus and Micronaut resolve dependency injection and much of their configuration at **build time**, not runtime — this is central to their fast-startup, low-memory, native-image-friendly design, but it means reflection must be explicitly registered or it silently breaks only under native compilation, not under the JVM build/test cycle.

```java
// Quarkus / Micronaut: explicit registration for a class only reached reflectively
@RegisterForReflection
public class WebhookPayload { /* fields via reflection-based (de)serialization */ }
```

Spring Boot's AOT processing (`spring-boot:process-aot`, used for GraalVM native-image builds) has the analogous requirement — `@Reflective` / `@RegisterReflectionForBinding` for types only reached reflectively. Any dynamic `Class.forName`, unregistered reflective field/method access, or runtime classpath scanning is a HIGH finding whenever the build declares native-image or AOT packaging (`quarkus.native.enabled=true`, a Micronaut native target, a GraalVM native Maven/Gradle profile). Enumerating the full `reflect-config.json`/metadata completeness is out of scope here — that belongs to the native-image reference/skill; this checklist only flags that the risk exists and is unaddressed.

## 6. Profile / config validation

A missing or malformed required property should fail fast at startup (or build, for Quarkus) — not silently produce a null/default that surfaces as a confusing runtime error later.

```java
@ConfigurationProperties(prefix = "payments")
@Validated
public record PaymentsProperties(
    @NotBlank String baseUrl,
    @Positive int timeoutMs) {}
```

A `@ConfigurationProperties`/`@ConfigMapping` class with no validation annotations and no documented default for a property the service cannot run without is MEDIUM.

## 7. Dependency BOM alignment

Each framework ships a Bill of Materials that pins a tested, mutually-compatible version set: `spring-boot-dependencies` (and `spring-cloud-dependencies` where Spring Cloud is used), `quarkus-bom`, and `micronaut-bom`/`micronaut-platform`. A dependency version hand-pinned outside the BOM can silently drift from that tested matrix. Flag any override: MEDIUM generally, HIGH when the overridden artifact is security- or data-access-adjacent (a JSON parser, a JWT library, a database driver) where a version skew has historically caused CVEs or data-corruption bugs.

## Known uncertainty

- Exact default values (e.g. whether health-group splitting is on by default) change across framework minor releases; confirm current defaults against the guides cited above rather than assuming this document's description is still the default in the version under review.
- Kubernetes probe wiring is only visible if the user supplies the deployment manifest; without it, health-endpoint findings are `inference (partial source)` at best.
