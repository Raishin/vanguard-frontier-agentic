# ESM/CJS Interop and the Verification Workflow

Use this reference when the suspect module's format is ambiguous, when CJS interop is suspected as the actual blocker, and for the before/after diff and runtime-smoke-test steps that close out every review in this skill.

## What people get wrong

The naive story is:

> I imported it with `import`, so it's an ES module now.

Wrong. Writing `import x from 'pkg'` in your own source does not change what `pkg` itself is. Per Node.js's package.json documentation, a package's actual module system is determined by its own `"type"` field, its `"exports"` map conditions, and file extensions (`.mjs`/`.cjs` override `"type"`) — not by the syntax the *importer* happens to use. Bundlers perform CJS/ESM interop specifically so that `import`-syntax consumers can still consume CJS packages, and that interop is exactly what defeats static tree-shaking: the bundler cannot statically prove which of a CJS module's dynamically-assigned `module.exports` properties are unused, so it must keep the whole thing.

## Diagnosing interop as the cause

Before concluding a bundler configuration is wrong, rule out CJS interop as the actual and unfixable-by-config cause:

1. Check the dependency's `package.json` `"exports"` map for an `"import"` condition pointing at a real ESM build. If the only entry is `"require"` or a bare `"main"` pointing at a `.js` file with no `"type": "module"`, the package is CJS-only at that entry point — no bundler `sideEffects` or `treeshake` setting will make it statically analyzable.
2. Check whether the package ships *both* CJS and ESM builds but exposes only a barrel-style default export from the ESM entry (e.g. `lodash-es`'s top-level `index.js` re-exporting every function) versus a per-function subpath (`lodash-es/debounce`). Importing the barrel entry can still pull in far more than needed even when the package is technically ESM, because the bundler must still resolve and analyze the whole re-export graph; importing the specific subpath sidesteps that entirely and is usually the more reliable fix regardless of `sideEffects` configuration.
3. Check the app's *own* code for a barrel-file pattern before blaming the dependency at all: `import * as utils from './utils'` where `./utils/index.js` re-exports every internal module, or a component library's internal `index.ts` that re-exports every component. This is one of the most common tree-shaking blockers and has nothing to do with any external dependency's configuration — the fix is importing the specific submodule directly, or restructuring the barrel to preserve per-export side-effect metadata the bundler can act on.

## Example: the "why is lodash still in my bundle" case

A report of "lodash is in the bundle even though we only use `lodash-es`'s `debounce`" is almost always one of:

- The app actually imports from the CJS `lodash` package somewhere (a transitive dependency, or a stray import), not `lodash-es` — confirm via the analyzer's module-resolution path, not the package name alone.
- The app imports the `lodash-es` barrel default (`import _ from 'lodash-es'` or `import { debounce } from 'lodash-es'`) rather than the subpath (`import debounce from 'lodash-es/debounce'`). Named imports from a barrel *can* still tree-shake correctly under a fully ESM, side-effect-clean barrel, but many barrels re-export in ways that retain more than expected — subpath import is the more reliable fix and should be verified by diff, not assumed to work either way.

Recommend the narrowest import path the package actually exposes, then confirm via analyzer diff that only the intended submodule remains — do not close the finding on the recommendation alone.

## The verification workflow

Every finding in this skill closes only with evidence, not a config change alone:

1. **Confirm production mode.** Re-run (or ask for) the project's production build command. A development-mode analyzer run is not admissible evidence either for "it's broken" or "it's fixed."
2. **Capture the before state.** Record the suspect module's presence and byte size (parsed and gzipped/brotli, matching how the project already measures) from the analyzer output.
3. **Apply the narrowest fix.** Prefer, in order: correcting an app-side barrel import → correcting a `sideEffects` field with side-effecting files explicitly listed → switching to a bundler-level `moduleSideEffects`/`treeshake` override only when the first two do not apply → replacing a CJS-only dependency with an ESM-native alternative or its narrowest subpath.
4. **Capture the after state.** Re-run the identical production build and analyzer command. Diff module list and byte counts against the before state.
5. **Runtime smoke test, mandatory whenever `sideEffects: false` changed.** Load the affected page or component in the built (not dev-server) output and confirm styling, polyfilled behavior, and any global registration the module was responsible for still function. A passing build after `sideEffects: false` proves nothing about runtime correctness — that field's entire failure mode is silent at build time.

## Hard stops

- Do not apply `sideEffects: false` without having read the target module for top-level side effects.
- Do not claim tree-shaking success, or failure, from a development-mode build.
- Do not treat "no build errors" as proof of elimination in either direction.
- Do not close a finding without a before/after analyzer diff attached.

## Adversarial checklist

Before closing any review under this skill, confirm:

- Was the diff taken from a production build, not development mode?
- Is the dependency's actual module format verified from its own `package.json`, not assumed from its name or popularity?
- If `sideEffects: false` was applied or proposed, was the target module's source actually read for side effects first?
- Is a barrel-file import pattern in the app's *own* code ruled out as the real blocker, separately from the dependency's configuration?
- Was a runtime smoke test performed after any `sideEffects: false` change, not just a rebuild?

If any answer is no, the review is not done.
