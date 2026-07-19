# Workflow and Output Contract

> Static review only. Read build files, toolchain/CI config, Dockerfile base images, and source. Never run a build, invoke a JDK, or contact a live system. Never state a vendor lifecycle date from memory — see `jdk-support-and-license-boundaries.md`.

## Workflow

### Step 1 — Collect inputs

Ask the user for whichever of these apply, sanitized (no license keys, account identifiers, or contract/pricing data):
- `pom.xml` (`maven.compiler.release`/`source`/`target`, `maven-compiler-plugin`) or `build.gradle`/`settings.gradle` (`sourceCompatibility`/`targetCompatibility`, `java { toolchain { languageVersion } }`).
- Toolchain / CI pins: `.java-version`, `.sdkmanrc`, `.tool-versions`, CI workflow `setup-java` version + distribution.
- `Dockerfile` / base image tag (the *runtime* JDK, which often differs from the *build* JDK).
- Optional but valuable: user-supplied `jdeps`, `jdeprscan`, or build-failure output; the dependency list with versions.

### Step 2 — Identify vendor and version

Determine the JDK **vendor** and **exact version** for both build and runtime. The distribution vendor (Temurin, Corretto, Zulu, Oracle JDK, Red Hat, Microsoft, GraalVM, …) usually comes from the CI `distribution` field and the Docker base image, not the version number. Flag any disagreement between the build JDK, the CI toolchain, and the runtime image as a finding — they should agree or the discrepancy must be intentional and documented.

### Step 3 — Map lifecycle exposure

Using `jdk-support-and-license-boundaries.md`, map the identified vendor+version to its license and free-security-support boundary. Cite the primary-source page and the date you read it. If a value is not verifiable, mark it `unknown (needs vendor page)` and ask the user to supply the page. Do not conclude "unsupported" or "fine" on a remembered date.

### Step 4 — Discover upgrade blockers

For the intended (or recommended) target JDK, look for:
- **Removed/encapsulated internals:** `sun.misc.Unsafe`, runtime `--add-opens`/`--add-exports`, reflective access to JDK internals, modules removed after 8/11 (`java.xml.bind`/JAXB, `java.xml.ws`/JAX-WS, CORBA, `java.se.ee`), `Thread.stop`, finalization, deprecated GC/flags.
- **Deprecated-for-removal APIs** the target removes — request `jdeprscan` output rather than guessing.
- **Third-party floors:** any dependency whose minimum-supported JDK is below or above the target. The upgrade cannot land until every library floor is met — require the dependency versions.
- **Build-tooling floors:** Maven/Gradle and key plugin versions that must move for the target JDK.

Rate each blocker (see rubric) and label its evidence basis.

### Step 5 — Plan the upgrade in waves

Reject rewrite-by-default and big-bang jumps. Prefer the smallest supported step (LTS→LTS, or LTS→current where justified) that clears the risk, sequenced so each wave is independently testable and revertible. A wave typically is: raise the build toolchain → fix compile/removed-API breaks → raise the runtime image → run the test + canary → verify → proceed. Name the target and the ordering.

### Step 6 — Gate the recommendation

Do not declare an upgrade "safe" without: the compatibility evidence gathered, a test plan, a rollback path, and a measurable post-upgrade verification (e.g. error-rate and latency parity on a canary). Missing any of these downgrades the verdict to pass-with-conditions or block.

### Step 7 — Produce the output

Format using the Output contract below.

## Evidence checklist

- [ ] Build JDK version + vendor (from build files)
- [ ] Runtime JDK version + vendor (from Docker/base image)
- [ ] CI toolchain pin (version + distribution)
- [ ] Lifecycle boundary verified against the primary source (with read-on date) or marked `unknown`
- [ ] Removed-internals / deprecated-API evidence (`jdeps`/`jdeprscan`) if provided
- [ ] Dependency versions for floor analysis

Each unchecked item downgrades the related findings to `inference (partial source)` or `assumption (source absent)`.

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | Runtime past its free-security-support boundary (unpatched CVEs) or in an unlicensed-in-production state for the identified vendor. |
| high | Reliance on removed/encapsulated internals or a removed deprecated API on the target; a third-party or build-tool floor not met; build/runtime/CI JDK disagreement that changes the support answer. |
| medium | Running a non-LTS interim release long-term; missing compatibility evidence for a proposed jump; `--add-opens` band-aids without a migration plan. |
| low | Minor toolchain/plugin version lag with no functional blocker. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Scope
JDK (build): <vendor> <version>   JDK (runtime): <vendor> <version>   CI: <distribution> <version>
<note any disagreement>

## Lifecycle exposure
<boundary, cited from jdk-support-and-license-boundaries.md with read-on date, or "unknown (needs vendor page)">

## Upgrade blockers
### CRITICAL / HIGH / MEDIUM / LOW
- [id] <blocker> — <evidence basis> — <specific removal/floor> — <remediation>

## Prescribed upgrade path
- Target: <vendor> <version>
- Waves: <ordered, each independently testable + revertible>
- Per-wave gate: <test plan> / <rollback> / <post-upgrade verification>

## Safe next actions
1. <action>

## Open questions
- <any vendor date or dependency version the user must supply>
```

## Security notes

- Never request or accept license keys, Oracle/vendor account identifiers, support-contract details, or pricing data. The commercial/portfolio decision belongs to `java-application-server-exit-agent`.
- This is a static review: never run a build, invoke a JDK, or contact a live system.
- Never state a lifecycle date from memory; cite the primary source and the read-on date, or mark it `unknown`.
- Never recommend disabling a failing gate, or a permanent `--add-opens`/`--add-exports` as a fix without a migration plan behind it.
