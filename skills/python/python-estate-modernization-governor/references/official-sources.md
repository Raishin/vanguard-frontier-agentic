# Official Sources

Primary CPython version/EOL and release-note documentation for estate-modernization review.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://devguide.python.org/versions/
- https://peps.python.org/pep-0602/
- https://docs.python.org/3/whatsnew/index.html
- https://packaging.python.org/en/latest/

## Provenance notes

- devguide.python.org (CPython versions/EOL schedule) and peps.python.org (PEP 602 release cadence) are the authoritative upstreams for interpreter support status; docs.python.org/3/whatsnew and packaging.python.org are authoritative for release-note and packaging/compatibility guidance.
- Context7 NOT separately used — CPython release-cadence/EOL semantics are defined in PEP 602 and the devguide, quoted from those primary upstreams; the applicable EOL dates must be confirmed against the official schedule for the user's versions.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
