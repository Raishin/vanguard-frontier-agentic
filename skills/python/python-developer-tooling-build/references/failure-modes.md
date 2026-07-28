# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A mypy step configured in CI but marked non-blocking lets a type regression merge and ship a production `AttributeError`.
- A repo-wide `# type: ignore` on an entire module hides a real null-safety bug from every future change to that file.
- A linter enabled only for formatting (line length, quote style) misses a bare `except:` that swallows a database error in production.
- A tox matrix that never actually runs the oldest declared-supported Python version ships a syntax error only that version hits.
- A src-layout package with a misconfigured build backend publishes an empty wheel that passes CI because tests import from the working directory, not the built package.
