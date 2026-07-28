# pyproject Metadata And Build Isolation

Standardized project/build metadata and why build isolation matters.

- The PyPA `pyproject.toml` specification defines the `[build-system]` table (build backend and its `requires`) and the `[project]` table (name, version, `requires-python`, dependencies, optional-dependencies, license); a conformant file is what makes resolution and build reproducible across tools.
- Dependency specifiers follow the PyPA dependency-specifier specification (PEP 508 grammar): version constraints, environment markers, and extras have defined semantics that both a loose and an overly tight specifier can violate.
- Build isolation runs the build in an environment containing only the declared `[build-system].requires`; disabling isolation or leaving those requirements unpinned lets build-time code run with the builder's privileges against an unpinned dependency set.

## Sources

- https://packaging.python.org/en/latest/specifications/pyproject-toml/
- https://packaging.python.org/en/latest/specifications/dependency-specifiers/
