# Official Sources

Primary PyPA and pip documentation and Context7 provenance for the hash-checking claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://packaging.python.org/en/latest/specifications/pyproject-toml/
- https://pip.pypa.io/en/stable/topics/secure-installs/
- https://pip.pypa.io/en/stable/topics/repeatable-installs/
- https://packaging.python.org/en/latest/specifications/dependency-specifiers/

## Provenance notes

- packaging.python.org (PyPA specifications) and pip.pypa.io are the authoritative upstreams for metadata, locking, and hash-checking behaviour; a specific-version vulnerability claim additionally requires an advisory source (e.g. the PyPA advisory database), which is out of this board's scope to assert.
- Context7 MCP provenance — library ID `/websites/pip_pypa_io_en_stable` (pip, source reputation High), retrieved 2026-07-26. Query: hash-checking mode `--require-hashes` requiring all dependencies pinned with hashes. Confirmed: hash-checking is all-or-nothing across every requirement and transitive dependency, all pinned to exact versions, with `--require-hashes` forcing the mode. Limitation: pip CLI behaviour evolves across releases — the applicable pip version must be confirmed from the user's toolchain.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
