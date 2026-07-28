# Developer-Tooling Review Checklist

The per-concern checklist applied to every developer-tooling review.

- Enforcement: every gate (lint/type/test) actually fails the build on a real defect; no blanket ignore or exclude of whole trees.
- Type-checker: strictness proportional to risk; new code is checked, not grandfathered into a lax mode.
- Linter: correctness rules are enabled (undefined names, bare excepts, mutable defaults), not only formatting.
- CI coverage: lint, type, test, and security-scan gates all run, on every relevant branch and fork.
- tox/nox: tests run against every declared supported Python version in a clean, pinned environment.
- Build: the pyproject build backend and package discovery actually ship the intended packages (src-layout/editable-install correctness).
