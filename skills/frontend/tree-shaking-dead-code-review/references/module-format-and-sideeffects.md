# Module Format and the sideEffects Field

Use this reference when determining whether a dependency is structurally tree-shakeable, and when reading or writing the `sideEffects` field for webpack-family bundlers.

## What people get wrong

The naive story is:

> The bundler is smart enough to figure out what's unused. If it doesn't, the bundler is broken.

Wrong. Tree-shaking is a conditional optimization built on two independent prerequisites, and if either is missing the bundler is not broken — it is behaving exactly as documented:

1. The module must be statically analyzable ES modules (`import`/`export`), not `require`/`module.exports`.
2. The bundler must be told, explicitly or by default assumption, which modules have no side effects, so it is safe to drop unreferenced exports.

Neither prerequisite is automatic. Both must be verified, not assumed.

## Module format: the first gate

Static analysis of `import`/`export` bindings is what lets a bundler know, at build time, exactly which exports are used and which are not. `require()` calls are dynamic and CommonJS's `module.exports` object can be mutated at runtime in ways a static analyzer cannot fully prove safe — so a CJS module is generally not tree-shakeable, regardless of which bundler processes it.

Check module format from the dependency's own `package.json`, not from assumption:

- An `"exports"` field with an `"import"` condition, or a top-level `"module"` field pointing at an ESM build, signals the package ships (or can ship) ES modules.
- A `"main"` field with no ESM entry point, or a package that only exposes `require()`-compatible output, is CJS-only. Static tree-shaking of that package's internals is not achievable through configuration; the fix is choosing a narrower import path the package exposes, or an ESM-native alternative.

Do not assume format from the package's popularity, age, or name. Long-lived, widely used packages are exactly the ones most likely to still ship a CJS-only default entry for backward compatibility while also shipping an ESM build the app is failing to resolve.

## The sideEffects field: the second gate

Per webpack's own guidance, tree-shaking of unused *exports* within an otherwise-imported ESM module is a separate mechanism (`usedExports`) from tree-shaking of unused *whole modules* (`sideEffects`). The `sideEffects` field governs the latter: it tells webpack whether a module can be dropped entirely if none of its exports are referenced.

- `"sideEffects": false` at the package level tells the bundler: assume nothing in this package has an import-time side effect; safe to drop any file whose exports are unused.
- Omitting the field, or `"sideEffects": true`, tells the bundler to assume every file might have a side effect and to keep every imported file regardless of whether its exports are used.
- A module that genuinely does have import-time side effects — CSS imports, global polyfills, prototype patching, analytics auto-registration — must be listed explicitly if the rest of the package is marked side-effect-free, e.g.:

```json
{
  "name": "awesome-ui",
  "sideEffects": ["**/*.css"]
}
```

Marking the whole package `sideEffects: false` while a CSS import exists uninstrumented inside it will silently drop the CSS from the production bundle. This does not produce a build error. It produces broken styling in production that only shows up after deploy.

## The production-mode requirement

Per webpack's documented guidance, tree-shaking's user-visible effect (actual removal of dead code from output) requires `mode: 'production'`, or `optimization.usedExports` explicitly enabled in a non-production config. In development mode, webpack intentionally may keep `usedExports` analysis observable in the module graph without physically removing the code from the bundle, specifically so developers can inspect what *would* be removed. A development-mode bundle showing a module present is not evidence tree-shaking is broken — it may be evidence tree-shaking was never active for that build.

`"sideEffects": false` additionally depends on `optimization.providedExports` to identify which exports are actually used before webpack can drop unused ones; this is enabled by default under `mode: 'production'`, but if a project runs a custom production-like config outside the `mode` shorthand, confirm the option is not disabled.

## What to check, in order

1. Confirm the analyzer output being reviewed came from a production-mode build. If not, stop and get one — nothing below is meaningful otherwise.
2. Read the suspect dependency's `package.json` `exports`/`module`/`main` fields to determine module format.
3. Read `sideEffects` on both the app's own `package.json` and the dependency's `package.json`.
4. If proposing `sideEffects: false` (or adding a package to an existing `sideEffects` array as an exception), read the module's source for top-level side-effecting statements before proposing it — do not propose it from the field being merely absent.

## Verification target

A `package.json` diff and a bundle-analyzer module-list diff, both taken from a production build, one before and one after the change. "The build still succeeds" is not verification of either format or side-effect claims.
