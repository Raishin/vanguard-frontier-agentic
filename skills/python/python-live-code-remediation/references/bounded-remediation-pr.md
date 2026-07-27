# Bounded Remediation Branch And PR

Why remediation is scoped to a branch/PR with isolated validation, never a merge or deploy.

- A code or dependency remediation is scoped to creating a branch and pull request with isolated validation — never a merge or a deploy.
- A failing gate, test, or policy check blocks the pull request; it is never disabled or weakened to force a pass.
- The pull request references the governing plan digest and a revert-based rollback, so the remediation can be undone cleanly.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://packaging.python.org/en/latest/
