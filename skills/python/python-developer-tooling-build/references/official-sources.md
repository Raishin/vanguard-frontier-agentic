# Official Sources

Primary ruff, mypy, tox, and pre-commit documentation.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.astral.sh/ruff/
- https://mypy.readthedocs.io/en/stable/config_file.html
- https://tox.wiki/en/stable/
- https://pre-commit.com/

## Provenance notes

- docs.astral.sh/ruff, mypy.readthedocs.io, tox.wiki, and pre-commit.com are the authoritative upstreams for their respective tool's configuration semantics.
- Context7 NOT separately used — the ruff/mypy/tox/pre-commit configuration semantics are quoted from those tools' primary documentation; the applicable tool versions must be confirmed from the user's configuration.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
