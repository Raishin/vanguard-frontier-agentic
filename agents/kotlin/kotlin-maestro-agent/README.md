# Kotlin Maestro Agent

Entry point for the Kotlin board. Classifies a Kotlin/JVM/Android/KMP task and routes it to the narrowest static-review specialist (or a parallel team of up to four for genuinely multi-domain tasks). Classification and routing only — never reviews Kotlin work itself and never performs or recommends a live operation.

---

## How routing works

### Required skill

- `skills/kotlin/kotlin-maestro/SKILL.md`

### Routing modes

- `single` — one specialist owns the matter.
- `parallel (N)` — the task genuinely spans two to four domains; escalate conflicts.
- `unclassified` — insufficient signal; ask for the smallest sufficient artifact set.

### Out-of-board handoffs

- Generic JVM/GC, virtual threads, Spring Boot generics, JPA tuning, Kafka, generic deserialization → the Java board.
- Cluster/deploy/runtime → the kubernetes / cloud boards.
- Telemetry platform, SLOs, dashboards → the OpenTelemetry / Prometheus boards.
- Artifact signing, SLSA provenance attestation → the sigstore board.
- Web frontend → the frontend board; generic QA strategy → the qa board.

---

## The Kotlin domain taxonomy

| Domain | Primary agent | Typical signals |
|---|---|---|
| `estate-modernization-governor` | `kotlin-estate-modernization-governor-agent` | Java to Kotlin migration, J2K converter, strangler fig, interop boundary, platform types, mixed codebase |
| `language-api-correctness` | `kotlin-language-api-correctness-agent` | nullability, platform type, Java interop, inline function, reified, value class |
| `coroutines-flow-reliability` | `kotlin-coroutines-flow-reliability-agent` | coroutine, coroutines, Flow, suspend, dispatcher, StateFlow |
| `library-api-abi-governance` | `kotlin-library-api-abi-governance-agent` | binary compatibility, ABI, apiDump, apiCheck, explicit API mode, JvmOverloads |
| `backend-production-readiness` | `kotlin-backend-production-readiness-agent` | Ktor, graceful shutdown, StatusPages, server lifecycle, health check, Spring WebFlux |
| `serialization-wire-contract` | `kotlin-serialization-wire-contract-agent` | kotlinx.serialization, encodeDefaults, explicitNulls, ignoreUnknownKeys, sealed class, classDiscriminator |
| `android-architecture` | `kotlin-android-architecture-agent` | ViewModel, SavedStateHandle, unidirectional data flow, state hoisting, single source of truth, configuration change |
| `compose-ui-quality-accessibility` | `kotlin-compose-ui-quality-accessibility-agent` | Compose, recomposition, @Stable, @Immutable, LaunchedEffect, DisposableEffect |
| `android-security-privacy` | `kotlin-android-security-privacy-agent` | android security, exported, deep link, WebView, cleartext, allowBackup |
| `android-performance-reliability` | `kotlin-android-performance-reliability-agent` | startup, StartupTimingMetric, jank, FrameTimingMetric, Macrobenchmark, Baseline Profile |
| `kmp-portfolio-decision` | `kotlin-kmp-portfolio-decision-agent` | Kotlin Multiplatform, KMP adoption, code sharing, expect/actual, commonMain, platform differentiation |
| `kmp-boundary-interop` | `kotlin-kmp-boundary-interop-agent` | expect/actual, commonMain, source set, Kotlin/Native, Objective-C interop, memory manager |
| `gradle-build-engineering` | `kotlin-gradle-build-engineering-agent` | Gradle, configuration cache, build cache, incremental compilation, kapt, KSP |
| `supply-chain-release-integrity` | `kotlin-supply-chain-release-integrity-agent` | dependency verification, verification-metadata.xml, dependency locking, plugin trust, checksum verification, signature verification |
| `test-architecture` | `kotlin-test-architecture-agent` | runTest, TestDispatcher, StandardTestDispatcher, UnconfinedTestDispatcher, Turbine, Compose testing |

---

## What the maestro will refuse

- Requests for secrets, keystores, signing keys, or tokens.
- Direct execution of any build, deploy, publish, or live operation.
- Answering a Kotlin question directly instead of routing it.

---

## Eval coverage

Routing is covered by `tests/fixtures/kotlin-maestro-routing/`. Run `npm run validate:maestro-routing`.

---

Part of the Vanguard Frontier Agentic Kotlin board.
