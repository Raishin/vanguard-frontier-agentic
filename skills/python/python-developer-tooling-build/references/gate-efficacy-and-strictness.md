# Gate Efficacy And Type/Lint Strictness

What makes a quality gate actually catch defects, for mypy and ruff configuration.

- A non-blocking or repo-wide-silenced gate provides false assurance; effectiveness requires the gate fail the build on a real defect.
- A type-checker in a lax mode passes trivially — strictness must match risk and cover new code.
- A linter must enable correctness rules (undefined names, bare excepts, mutable defaults), not only formatting.

## Sources

- https://docs.astral.sh/ruff/
- https://mypy.readthedocs.io/en/stable/config_file.html
