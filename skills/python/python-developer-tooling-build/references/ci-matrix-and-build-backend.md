# CI Matrix Isolation And Build Backend Correctness

tox/nox environment isolation, build-backend/package-discovery correctness, and pre-commit parity with CI.

- tox/nox must test the declared supported Python versions in clean pinned environments, not ambient state.
- The pyproject build backend and package discovery must include the intended packages (src-layout/editable-install pitfalls).
- pre-commit should mirror the fast CI gates for local feedback.

## Sources

- https://tox.wiki/en/stable/
- https://pre-commit.com/
