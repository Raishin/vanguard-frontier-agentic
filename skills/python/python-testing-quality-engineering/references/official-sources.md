# Official Sources

Primary pytest, unittest.mock, and hypothesis documentation.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.pytest.org/en/stable/how-to/fixtures.html
- https://docs.pytest.org/en/stable/how-to/monkeypatch.html
- https://docs.python.org/3/library/unittest.mock.html
- https://hypothesis.readthedocs.io/en/latest/

## Provenance notes

- docs.pytest.org, docs.python.org (unittest.mock), and hypothesis.readthedocs.io are the authoritative upstreams for the claims in this skill.
- Context7 MCP was not used as a separate source for this skill: the pytest fixture/monkeypatch, unittest.mock patch-target, and hypothesis semantics cited here are stable and are quoted from the official upstream documentation, which the repository treats as authoritative. The applicable pytest/plugin versions must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
