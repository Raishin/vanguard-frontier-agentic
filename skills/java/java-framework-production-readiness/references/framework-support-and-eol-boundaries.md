> Fail-closed on dates. This file does not reproduce specific Spring Boot / Quarkus / Micronaut end-of-life, support-window, or LTS-designation dates from memory. It records which primary source establishes each boundary, how to read it, and a refresh protocol. Before stating any date to a user, open the cited page and read the current value; if you cannot, mark the fact `unknown (needs vendor page)`. A stale date here produces a confidently-wrong ship verdict — the exact failure this reference exists to prevent.

# Framework Support and EOL Boundaries

## Why this decision matters

"Is this framework version still supported" is a different question from "does the code compile and pass tests." A service can build cleanly on a framework line that has stopped receiving security patches. Unlike the JDK (where vendor is the primary axis of variation), each of these three frameworks publishes its own release/support cadence directly — but the cadences differ in shape (Spring Boot's OSS-then-commercial support model, Quarkus's platform-release train, Micronaut's rolling major-version support), so the right primary source is framework-specific.

## Primary sources (open these; do not trust memory)

| Framework | Authoritative page for release cadence / support status |
|---|---|
| Spring Boot | The Spring Boot project page's support section (under `https://spring.io/projects/spring-boot`), which documents OSS support duration and commercial/extended support via Spring's support offerings; cross-check version-specific behavior against `https://docs.spring.io/spring-boot/` |
| Quarkus | Quarkus release notes and the platform release documentation under `https://quarkus.io/guides/` (search the guides index for "releases"/"platform"); Quarkus also publishes designated LTS streams — confirm current LTS status on the release notes, not from memory |
| Micronaut | The Micronaut releases/versions information reachable from `https://docs.micronaut.io/latest/guide/` and the framework's GitHub releases page for the specific major version in scope |
| Jakarta EE (namespace/spec baseline underneath all three) | `https://jakarta.ee/specifications/` — confirms which Jakarta EE platform version a given framework major version implements |

## How to read a boundary

1. **Framework major-version support status** — is the major version in scope (e.g. Spring Boot 3.x, Quarkus 3.x, Micronaut 4.x) currently receiving patches, in a maintenance-only window, or past end-of-support? Read this from the framework's own release notes/support page for that specific major version, not a general "latest is fine" assumption.
2. **LTS vs. rolling** — Spring Boot and Quarkus both designate certain lines for longer support; Micronaut's support model is closer to "support the current and immediately prior major." Confirm which model applies to the version in scope before promising a support window.
3. **Jakarta EE baseline** — each framework major version targets a specific Jakarta EE platform release; a framework upgrade can force a Jakarta EE spec-version bump with its own compatibility implications, separate from the framework's own API changes.

## The boundary table (structure — fill from the cited page at review time)

| Service | Framework | Major version | Support status | Patches until | Source page | Read on |
|---|---|---|---|---|---|---|
| _(e.g. checkout-api)_ | _(e.g. Spring Boot)_ | _(e.g. 3.2)_ | _(verify)_ | _(verify — do not guess)_ | _(framework URL)_ | _(YYYY-MM-DD)_ |

If a cell cannot be verified against the primary source during the review, write `unknown (needs vendor page)` — never a remembered value.

## Known uncertainty

- Release cadences and support-window policies are themselves subject to change by the maintaining organization; treat this file's *structure* as durable and any date you read as a snapshot requiring re-verification on the next review.
- Quarkus and Micronaut are community-driven projects without a single commercial-vendor support page in the way Spring Boot has one via Spring's support offerings — confirm whether the version in scope has any vendor-backed extended support (e.g. via a Red Hat or other commercial build) as a separate question from the upstream community support window.
- "LTS" as used by these frameworks is a project designation, not a guarantee that matches JDK LTS semantics — do not assume alignment between a framework's LTS line and a JDK LTS line without checking both independently.

## Refresh protocol

- **Refresh owner:** the Java board maintainer (`github: Raishin`) or a named delegate.
- **Cadence:** re-verify every boundary in this file against its primary source at least quarterly, and immediately when a user reports a discrepancy or a framework announces an EOL/support-window change.
- **On refresh:** update the companion skill's `metadata.json` `last_verified` field, and record the read-on date in any table cell touched.
- This reference never consults the wall clock — behavior changes only when the committed content changes. Encode the source and the read-on date, never "as of today" logic.

## Escalation conditions

- If the support-window question becomes a licensing/commercial-contract decision (e.g. evaluating a paid support subscription), that is outside this agent's static-review scope — surface it as an open question for the user's own commercial/vendor-management process rather than adjudicating it here.
