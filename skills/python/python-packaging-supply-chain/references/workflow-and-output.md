# Review Workflow And Output Contract

The supply-chain review workflow and the required output shape.

## Workflow

1. Identify the toolchain and inputs: `pyproject.toml`, requirements/constraints, lockfile, and CI publish workflow.
2. Check index configuration for a private+public `--extra-index-url` mix and other dependency-confusion exposure.
3. Check that dependencies are pinned and hashed (all-or-nothing hash-checking) and that a lockfile is the installed source of truth.
4. Check build isolation and that `[build-system].requires` is pinned and hashed; check `[project]` metadata and license conformance.
5. Check the CI release path for token exposure to untrusted PR code, and record every specific version/CVE claim that needs an advisory source.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the packaging toolchain assumed.
- Index-trust, locking/hashing, build-isolation, and metadata/CI findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any version/CVE claim the user must confirm against an advisory source.
