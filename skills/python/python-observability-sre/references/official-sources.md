# Official Sources

Primary OpenTelemetry Python and stdlib logging documentation and Context7 provenance.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://opentelemetry-python.readthedocs.io/en/stable/
- https://opentelemetry.io/docs/concepts/context-propagation/
- https://opentelemetry.io/docs/specs/semconv/
- https://docs.python.org/3/library/logging.html

## Provenance notes

- opentelemetry.io (specification) and opentelemetry-python.readthedocs.io are the authoritative upstreams for the propagation and semantic-convention claims here; docs.python.org/3/library/logging is the stdlib logging reference.
- Context7 MCP provenance — library ID `/websites/opentelemetry-python_readthedocs_io_en_stable` (OpenTelemetry Python, source reputation High), retrieved 2026-07-26. Query: context propagation across threads and async, span-attribute/metric-label cardinality, avoiding sensitive data in attributes. Confirmed: a global textmap propagator extracts context from an inbound request and injects it into downstream calls; context is maintained across coroutines (async_context) and must be carried across executor boundaries; span attributes become metric labels whose cardinality drives backend cost. Limitation: OpenTelemetry Python APIs evolve across releases — the applicable SDK version must be confirmed from the user's dependencies.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
