# LTS-to-LTS Migration and Language-Feature Map

> Anchored to JEP numbers, not vendor dates. Every fact below is a *release-anchored* fact — a JEP and the JDK feature-release that shipped it — which, unlike a support-end or license date, does not move once shipped. It is therefore safe to record here. It is **not** a substitute for verification: before you rely on any row, open its linked `openjdk.org/jeps/<n>` page (or the per-release delta) and confirm the JEP number, title, and target release. Anchor every blocker or feature you cite to its JEP number; if you cannot confirm a mapping, mark it `unknown (needs JEP page)` — never assert a JEP-to-version mapping from memory. **Do not put support-end or license dates here** — those are fail-closed and live only in `jdk-support-and-license-boundaries.md`.

## Why this reference exists

The lifecycle skill's Step 4 (discover upgrade blockers) and Step 5 (plan waves) need a concrete, per-corridor answer to two questions:

1. **What breaks** when moving to the target JDK — modules removed, internals strongly encapsulated, APIs deprecated-for-removal — so the review can name the specific removal and its replacement instead of hand-waving "some things may break."
2. **What the target lets you adopt** — the language and API features finalized in the corridor — so the upgrade is framed as capability gained, not just risk cleared, and so the review does not recommend adopting a feature the target does not actually ship as final.

This file maps both, per LTS-to-LTS corridor. It is release-anchored and vendor-neutral: which JDK *feature-release* introduced a change is the same across distributions (Temurin, Corretto, Zulu, Oracle JDK, Red Hat, Microsoft, GraalVM). **Which vendor still ships free security updates for that release, and until when, is a separate, fail-closed question** — always answer it from `jdk-support-and-license-boundaries.md`, never from this file.

## Primary sources (open these; do not trust memory)

| Source | Use |
|---|---|
| `https://openjdk.org/jeps/0` | The JEP index — the authoritative list of delivered JEPs, their numbers, titles, and target releases. |
| `https://openjdk.org/jeps/<n>` | The individual JEP — confirm a single row's number, title, `Release`, and status. |
| `https://openjdk.org/projects/jdk/17/jeps-since-jdk-11` | The authoritative 11→17 delta (every JEP integrated across JDK 12–17). |
| `https://openjdk.org/projects/jdk/21/jeps-since-jdk-17` | The authoritative 17→21 delta (JDK 18–21). |
| `https://openjdk.org/projects/jdk/25/jeps-since-jdk-21` | The authoritative 21→25 delta (JDK 22–25). |
| `https://docs.oracle.com/en/java/javase/<N>/migrate/` | Oracle's JDK Migration Guide for target release `<N>` — narrative of removed/changed behavior and the recommended `jdeps`/`jdeprscan` workflow. |
| `https://openjdk.org/projects/amber` | Project Amber status — the delivered vs. preview status of language features. |

> **Verification note.** The JDK 25 rows below were confirmed against `openjdk.org/projects/jdk/25` and its `jeps-since-jdk-21` delta during the last refresh of this file. The older corridors (8→11, 11→17, 17→21) list widely-referenced, permanent JEP mappings; still confirm any row you cite against its linked page. Record the read-on date in `metadata.json` (`last_verified`) when you refresh.

## The LTS spine

The Long-Term-Support feature-releases are **8, 11, 17, 21, 25**. Everything between them is an **interim** feature-release that, by construction, receives updates only until the next feature-release — running a fleet on an interim release long-term is a support gap. Migrate LTS→LTS in the **smallest supported step** that clears the risk; do not jump 8→25 in one move. Each corridor below is one such step.

`java --version` reports the running release; the build's target is `maven.compiler.release` / Gradle `toolchain.languageVersion`. The build target and the runtime image often differ — reconcile both against the same corridor.

JDK 25 reached General Availability on 2025-09-16 and is designated LTS by most vendors (verified). Confirm the *vendor's* support window separately in the boundaries reference.

---

## Corridor 8 → 11

The defining break of this corridor is the **module system** (JPMS, JDK 9) and the **removal of the Java EE / CORBA modules**. Most 8→11 failures are missing classes at compile or runtime, not language incompatibilities.

### Breaks (fail to compile or run)

- **Removed Java EE & CORBA modules** — `java.xml.bind` (JAXB), `java.xml.ws` (JAX-WS), `java.activation`, `java.xml.ws.annotation`, `java.corba`, `java.transaction`, and the `java.se.ee` aggregator were removed in JDK 11 (JEP 320; deprecated for removal in JDK 9). **Replacement:** add the standalone artifacts as normal dependencies — Jakarta XML Binding (JAXB), Jakarta XML Web Services (JAX-WS), Jakarta Activation, etc. This is the single most common 8→11 breaker.
- **Strong module boundaries (JPMS, JDK 9)** — split packages across the classpath, reflective access to `sun.*` / `jdk.internal.*`, and libraries that shade JDK internals now warn or fail. `--illegal-access` still defaults to *permit-with-warning* in this corridor (it is not yet denied — that comes in 11→17).
- **Removed tools & bundles** — `javah` removed (JEP 313, JDK 10); JavaFX unbundled from the JDK (JDK 11 — consume it as a separate module); Java Web Start and the browser applet plug-in removed; several deprecated GC flag combinations removed (JEP 214, JDK 9).
- **Version-string change** — the `java.version` scheme changed at JDK 9 (`1.8.0` → `9`, `11`); parsers that assume a leading `1.` break.

### Deprecated-for-removal (warns now, removed downstream)

- **Nashorn JavaScript engine** deprecated (JEP 335, JDK 11) — removed later in the 11→17 corridor. Plan a replacement now.

### Adoptable (what 11 gives you)

- `var` local-variable type inference (JDK 10; JEP 286).
- Standardized `java.net.http.HttpClient` (JEP 321, JDK 11) — retire third-party HTTP clients where practical.
- Single-file source-code launch: `java Foo.java` (JEP 330, JDK 11).
- **G1 is the default garbage collector** (since JDK 9). Also: ZGC and Epsilon land as *experimental* (JDK 11); Flight Recorder and Mission Control open-sourced (JEP 328, JDK 11). GC tuning belongs to `java-jvm-performance-and-gc-agent`, not this agent.

---

## Corridor 11 → 17

The defining break of this corridor is **strong encapsulation of JDK internals becoming the default**. Code that reached into `sun.*` / `jdk.internal.*` — directly or through an old library — that merely *warned* on 11 now *fails* on 17 unless the access is made explicit with `--add-opens` / `--add-exports`.

### Breaks (fail to compile or run)

- **Strong encapsulation by default** — JEP 396 (JDK 16) flips the default from permit-with-warning to *deny* illegal reflective access; JEP 403 (JDK 17) removes the `--illegal-access` escape hatch entirely, so it can no longer be re-enabled wholesale. **Replacement:** enumerate the exact `--add-opens` / `--add-exports` a component needs (as a documented, time-boxed bridge) and drive the dependency toward a version that no longer reaches into internals. A blanket `--add-opens` is a band-aid, not a fix.
- **Nashorn removed** (JEP 372, JDK 15) — the JS engine deprecated in the previous corridor is gone. Migrate to GraalVM's JavaScript or another engine.
- **RMI Activation removed** (JEP 407, JDK 17).
- **Experimental AOT and JIT (`jaotc` / Graal-as-JIT) removed** (JEP 410, JDK 17).
- **CMS garbage collector removed** (JEP 363, JDK 14) — pipelines pinned to `-XX:+UseConcMarkSweepGC` fail to start; move to G1 (default) or ZGC/Shenandoah. Route the tuning decision to `java-jvm-performance-and-gc-agent`.
- **Pack200 tools and API removed** (JEP 367, JDK 14); **Solaris and SPARC ports removed** (JEP 381, JDK 15); **biased locking disabled and deprecated** (JEP 374, JDK 15).

### Deprecated-for-removal (warns now, removed downstream)

- **Security Manager** deprecated for removal (JEP 411, JDK 17) — if the estate relies on `SecurityManager` / policy files, this is a HIGH structural blocker for the *next* corridor; start designing the replacement (e.g. OS/container isolation) now.
- **Applet API** deprecated for removal (JEP 398, JDK 17).

### Adoptable (what 17 gives you)

- **Switch Expressions** (JEP 361, JDK 14) — finalized.
- **Text Blocks** (JEP 378, JDK 15) — finalized.
- **Records** (JEP 395, JDK 16) — finalized; concise immutable data carriers.
- **Pattern Matching for `instanceof`** (JEP 394, JDK 16) — finalized.
- **Sealed Classes** (JEP 409, JDK 17) — finalized; constrained type hierarchies.
- **Helpful NullPointerExceptions** (JEP 358, on by default from JDK 15).
- **ZGC** and **Shenandoah** reach *production* (JEP 377 and JEP 379, JDK 15); **macOS/AArch64 port** (JEP 391, JDK 17).

---

## Corridor 17 → 21

This is the highest-value adoption corridor: **virtual threads** and the pattern-matching family land as final. The breaks are comparatively small; the opportunity is large.

### Breaks (fail to compile or run)

- **Legacy thread-suspension primitives** — `Thread.stop`, `Thread.suspend`, `Thread.resume` are removed or degraded to throw across this range; code still calling them breaks. Redesign around interruption/cancellation.
- **Security Manager effectively unusable** — building on its 11→17 deprecation, use of the Security Manager now warns loudly and is on a removal path; do not treat it as a supported isolation mechanism.

### Deprecated-for-removal (warns now, removed downstream)

- **Finalization** deprecated for removal (JEP 421, JDK 18) — `Object.finalize()` and finalizer-based cleanup are on the way out. **Replacement:** `java.lang.ref.Cleaner` and try-with-resources / `AutoCloseable`.
- **Windows 32-bit x86 port** deprecated for removal (JDK 21) — plan off 32-bit Windows runtimes.

### Adoptable (what 21 gives you)

- **Virtual Threads** (JEP 444, JDK 21) — finalized; cheap threads for high-throughput, thread-per-request server code. This is the headline of the corridor. Concurrency-model review (pinning, structured cancellation, thread-locals) belongs to `java-concurrency-and-virtual-thread-agent`.
- **Pattern Matching for `switch`** (JEP 441, JDK 21) — finalized.
- **Record Patterns** (JEP 440, JDK 21) — finalized; destructure records in patterns.
- **Sequenced Collections** (JEP 431, JDK 21) — finalized; first/last access and reversed views.
- **Generational ZGC** (JEP 439, JDK 21) — GC choice/tuning is owned by `java-jvm-performance-and-gc-agent`.
- **UTF-8 by default** (JEP 400, JDK 18) — a *behavior* change to watch: code that relied on the platform default charset (file/stream reads without an explicit charset) can change behavior. Audit for implicit-charset usage as part of the upgrade, not after.
- Simple Web Server (JEP 408, JDK 18) and Code Snippets in Javadoc (JEP 413, JDK 18).
- **Preview in this corridor — do not rely on as stable:** Foreign Function & Memory API, Structured Concurrency, and Scoped Values were still *preview* through JDK 21. Treat preview APIs as non-production: they can change or require `--enable-preview`, which pins the exact JDK. Do not recommend adopting them as a load-bearing part of an upgrade.

---

## Corridor 21 → 25

JDK 25 (GA 2025-09-16, LTS — verified this cycle) is the current LTS target. The language additions here streamline application entry points and constructors; several concurrency features that were preview in the previous corridor **remained preview** and must not be presented as finished.

### Adoptable (finalized in this corridor — verified against the JDK 25 delta)

- **Module Import Declarations** — import all packages exported by a module with a single declaration.
- **Flexible Constructor Bodies** — statements are allowed before an explicit `super(...)` / `this(...)` call.
- **Compact Source Files and Instance Main Methods** — simpler program entry points (the modern `void main()` form), useful for small tools and onboarding.
- **Unnamed Variables & Patterns** — finalized earlier (JDK 22) and present here: `_` for unused bindings.
- Tooling: launch multi-file source-code programs, link run-time images without JMODs, and Markdown documentation comments in Javadoc.

Look up each JEP number for these on `openjdk.org/projects/jdk/25/jeps-since-jdk-21` at review time rather than quoting a number from memory.

### Still preview or incubating in JDK 25 — NOT final (verified)

Do **not** recommend these as stable, and flag any codebase that depends on them as carrying preview risk (they require `--enable-preview` and can change between releases):

- **Structured Concurrency** — still preview (fifth preview) in JDK 25.
- **Stable Values** — preview in JDK 25.
- **Primitive Types in Patterns, `instanceof`, and `switch`** — preview (third preview) in JDK 25.
- **PEM Encodings of Cryptographic Objects** — preview in JDK 25.
- **Vector API** — still incubating (tenth incubator) in JDK 25.

### Breaks & deprecations across JDK 22–25

Removals/deprecations continue in this range (the Security Manager wind-down, 32-bit x86 retirement, and the tail of finalization removal among them). This file does **not** enumerate the 22–25 removals as a verified list — confirm them against each release's `jeps-since` delta and the Oracle Migration Guide for the target release before relying on them, and mark anything unconfirmed `unknown (needs JEP page)`.

---

## How to use this in a review

- **Blocker discovery (Step 4):** for the target JDK, walk its corridor's *Breaks* and *Deprecated-for-removal* lists against the estate's `jdeps` / `jdeprscan` evidence. Name the specific JEP and the replacement. A blocker with a JEP citation is `confirmed`/`inference`; a suspected one with no evidence is `assumption` — label it so.
- **Multi-corridor jumps:** an 8→17 or 11→21 move crosses *every* intermediate corridor's breaks — accumulate them. This is why the skill prefers the smallest supported LTS-to-LTS step: each corridor is independently testable.
- **Feature adoption:** only recommend adopting a feature the target ships as **final**. Preview/incubating features (see JDK 25 list above) pin the JDK via `--enable-preview` and are not a safe upgrade payload.
- **Stay in lane — route the rest:**
  - GC selection / ZGC / generational ZGC tuning → `java-jvm-performance-and-gc-agent`.
  - Virtual-thread adoption, pinning, and structured-concurrency design → `java-concurrency-and-virtual-thread-agent`.
  - Framework version floors (Spring Boot, Jakarta EE) that gate the JDK move → `java-framework-production-readiness-agent`.
  - Oracle JDK commercial-license / portfolio exposure → `java-application-server-exit-agent`.
  - Support-end / license *dates* for the identified vendor → `jdk-support-and-license-boundaries.md` (fail-closed; never quoted from this file).
