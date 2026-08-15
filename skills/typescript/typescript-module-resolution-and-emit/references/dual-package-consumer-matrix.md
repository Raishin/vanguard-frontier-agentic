# Dual-Package Consumer Matrix

The minimum set of consumer configurations that must compile, and how to check condition ordering.

- A package claiming dual ESM/CJS support must prove resolution separately for at least: Node ESM `import`, Node CJS `require`, a bundler under `moduleResolution: bundler`, and any declared test runner — a claim not tested against all of them is unproven for the untested modes.
- The classic dual-package hazard (two separately-evaluated module instances of the same package loaded via different entry points) is under-documented in Node's current package docs, which now treat that section as a stub — verification requires actually resolving both entry points, not citing the docs.
- `publint.dev/rules` and `arethetypeswrong.github.io` are automated consumer-matrix checks: the former validates packaging conventions against `exports`/`files`, the latter simulates what a TypeScript consumer's resolver actually sees per condition — running both is stronger evidence than reading `package.json` by eye.
- A single shared `.d.ts` file serving both an ESM and a CJS build is a common source of the dual-package hazard, since `export default` interop differs between the two module systems at the type level as well as at runtime.
- `require(esm)` in current Node versions needs no flag but is synchronous-only; a CJS consumer that requires an ESM module performing a top-level `await` fails with `ERR_REQUIRE_ASYNC_MODULE` — a claim that CJS can simply require the ESM build must account for this.
