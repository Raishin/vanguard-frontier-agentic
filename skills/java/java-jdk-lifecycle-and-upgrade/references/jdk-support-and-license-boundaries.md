# JDK Support and License Boundaries

> Fail-closed on dates. The specific support-end and license-boundary dates below are **not reproduced from memory**. This file records *which primary source* establishes each boundary and *how to read it*, plus a refresh protocol. Before you state any date to a user, open the cited vendor page and read the current value; if you cannot, mark the fact `unknown (needs vendor page)`. An out-of-date date here would produce confidently-wrong upgrade advice — the exact failure this agent exists to prevent.

## Why this decision matters

"Is this JDK supported?" has no single answer — it depends on the **vendor**, not just the version number. The same major version (say, JDK 17) can be:

- in free long-term security support from one OpenJDK distribution,
- past free updates and requiring a paid subscription from another vendor,
- and governed by an entirely separate commercial-license question for Oracle JDK builds.

Upgrade advice that ignores the vendor is cargo cult. The first job is always to identify the vendor, then map *that vendor's* boundary.

## Primary sources (open these; do not trust memory)

| Vendor / distribution | Authoritative page for support + license |
|---|---|
| Oracle JDK (Oracle Java SE) | `https://www.oracle.com/java/technologies/java-se-support-roadmap.html` (support roadmap) and the current Oracle Java SE licensing/FAQ pages for the license terms that apply to the specific build |
| OpenJDK project (release cadence, GA) | `https://openjdk.org/projects/jdk/` and the JEP process pages |
| Eclipse Temurin / Adoptium | `https://adoptium.net/support/` |
| Amazon Corretto | Corretto FAQ / end-of-support pages under `https://docs.aws.amazon.com/corretto/` |
| Azul Zulu | Azul's product-support / Zulu support-roadmap pages |
| Red Hat build of OpenJDK | Red Hat's OpenJDK life-cycle-and-support-policy page |
| Microsoft build of OpenJDK | Microsoft Learn "Microsoft build of OpenJDK" support pages |
| GraalVM | `https://www.graalvm.org/release-notes/` and Oracle GraalVM support docs |
| Cross-vendor overview (secondary, corroborating only) | `https://endoflife.date/java` — useful as a lead, **never** as the authority; confirm against the vendor page before stating a date |

## How to read a boundary

1. **License boundary** — for the identified vendor and build, is production use free, or does it require a paid subscription / commercial license? For Oracle JDK specifically, the applicable license changed across recent releases; the license question is separate from the security-update question. Do not conflate "OpenJDK is open source" with "this Oracle JDK build is free in production."
2. **Free-security-support boundary** — the date after which the vendor stops shipping free security updates for that line. Past it, the runtime accumulates unpatched CVEs.
3. **LTS vs interim** — interim releases (the non-LTS feature releases) receive updates only until the next release. Running a fleet on an interim release long-term is a support gap by construction.

## The boundary table (structure — fill from the cited page at review time)

Record findings for the estate under review in this shape, citing the page and the date you read it:

| Runtime | Vendor | Major version | LTS? | License in prod (free / paid) | Free security updates until | Source page | Read on |
|---|---|---|---|---|---|---|---|
| _(e.g. payments-api)_ | _(e.g. Oracle JDK)_ | _(e.g. 17)_ | _(yes/no)_ | _(verify)_ | _(verify — do not guess)_ | _(vendor URL)_ | _(YYYY-MM-DD)_ |

If a cell cannot be verified against the primary source during the review, write `unknown (needs vendor page)` — never a remembered value.

## Known uncertainty

- Vendor roadmaps are revised. Any date is a snapshot; treat this file's structure as durable and its dates as always-reverify.
- "LTS" is a vendor designation, not a JDK-project guarantee; different distributions choose different lines and windows.
- License terms for Oracle builds have changed more than once in recent years and can differ by build channel — read the license for the *specific* artifact, not a general recollection.

## Refresh protocol

- **Refresh owner:** the Java board maintainer (`github: VincentChuWaiChow`) or a named delegate.
- **Cadence:** re-verify every boundary in this file against its primary source at least quarterly, and immediately when a user reports a discrepancy or a new JDK LTS reaches GA.
- **On refresh:** update the `last_verified` in the skill's `metadata.json`, and record the read-on date in any table cell you touch.
- This engine never consults the wall clock — behavior changes only when this committed reference changes. Do not encode "as of today" logic; encode the source and the read-on date.

## Escalation conditions

- The estate runs an Oracle JDK build in production and the license/cost question is material → hand the commercial exposure to `java-application-server-exit-agent` (the license/portfolio owner); this agent stays on the technical support boundary.
- A required date cannot be verified against any primary source → return `unknown` and ask the user to supply the vendor page; do not proceed to a "you are unsupported / you are fine" conclusion on a guessed date.
