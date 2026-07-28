# Official Sources

Primary Python typing, typing-spec, and mypy documentation.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/library/typing.html
- https://typing.readthedocs.io/en/latest/spec/
- https://mypy.readthedocs.io/en/stable/
- https://peps.python.org/pep-0484/

## Provenance notes

- docs.python.org (typing), the Python typing specification (typing.readthedocs.io/spec), the PEPs, and the mypy documentation are the authoritative upstreams; a Pyright-specific behaviour must be confirmed against Pyright's own documentation.
- Context7 MCP was not used as a separate source for this skill: the gradual-typing and variance semantics cited here are defined in the Python typing specification and PEP 484 and are quoted from those primary upstreams, which the repository treats as authoritative. The applicable type-checker and its strictness must be confirmed from the user's configuration.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
