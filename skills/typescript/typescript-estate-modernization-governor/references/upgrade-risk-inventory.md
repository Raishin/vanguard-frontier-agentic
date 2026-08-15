# Upgrade Risk Inventory

The breaking-change classes to enumerate per package and how to detect each.

- TypeScript 6.0 (2026-03-23) made `strict` default-true, changed the default `module` to `esnext`, and removed the `amd`, `umd`, and `system` module values along with `--outFile`, `--downlevelIteration`, and `target=es5`.
- TypeScript 7.0 (GA 2026-07-08) ships a native Go compiler and has no stable programmatic API until 7.1, so any tooling that consumes the compiler API (editor extensions, some framework integrations) cannot move to 7.0 until that API stabilizes.
- The removed `moduleResolution` values `classic` and `node10` are a distinct blocker class from the removed `module` values and must be inventoried separately per `tsconfig.json`.
- A package that still targets `es5` or relies on `--downlevelIteration` cannot upgrade past the release that removed those options without a prior, separate remediation step.
- The removed-value inventory is a per-`tsconfig.json` check, not a per-repository check — a monorepo can have some packages blocked and others clear.
