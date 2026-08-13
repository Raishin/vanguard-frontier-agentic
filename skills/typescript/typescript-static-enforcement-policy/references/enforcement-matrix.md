# Enforcement Matrix

What 'passes' means per package, flags and rules together, with the per-package divergence view.

- TypeScript 6.0 made `strict` default to `true`; the enforcement question for any package on TypeScript 6.0 or later is therefore whether an explicit opt-out exists somewhere in the effective configuration, not whether strict mode needs to be turned on.
- `module` also defaults to `esnext` since TypeScript 6.0 — a package pinning an older `module` value is itself a policy divergence worth naming even when strictness is otherwise uniform.
- Valid `moduleResolution` values are only `node16`, `nodenext`, and `bundler`; the official tsconfig prose page's value tables are confirmed stale on removed values, so the compiler binary — not the prose page — is authoritative for what a configuration actually accepts.
- Typed lint rules that require type information (`no-floating-promises`, `no-misused-promises`, `await-thenable`, `require-await`) are inert without it; `languageOptions.parserOptions.projectService: true` is the current documented enablement mechanism, and `allowDefaultProject` is a capped fallback that carries per-file overhead rather than a substitute for proper project membership.

## Sources

- https://www.typescriptlang.org/tsconfig
- https://devblogs.microsoft.com/typescript/announcing-typescript-6-0/
