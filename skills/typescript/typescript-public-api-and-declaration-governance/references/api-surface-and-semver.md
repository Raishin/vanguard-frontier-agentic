# API Surface And Semver Decision

How to classify a declaration change and pick the required version bump.

- A type reachable through an exported function's parameter or return type is part of the public API surface even when the type itself carries no export statement and no documentation mentions it — structural reachability, not naming intent, determines public surface.
- Classification is independent of the runtime implementation: a `.d.ts` diff with an unchanged runtime is still assessed for breaking-ness on its own terms, because a consumer's build can fail on the type change alone.
- Adding a required parameter, a required generic type parameter, or a required property to an already-exported interface narrows what previously-valid consumer code can supply and is a breaking change, not an additive one.
- API Extractor's rollup and API-report output is the surface of record for classification — a type flattened into the rollup is public even if source-level review would call it private.
- Dual ESM/CJS declaration hazards are documented in the modules appendix of the TypeScript handbook, not on the primary declaration-publishing page — a single `.d.ts` claiming to serve both module systems is exactly the case that appendix documents as hazardous.
- API Extractor requires the source be compiled with `tsc` and `declaration: true` first before it can generate an API report or rollup — the tool consumes emitted declarations, it does not perform its own compilation.

## Sources

- https://www.typescriptlang.org/docs/handbook/modules/appendices/esm-cjs-interop.html
- https://api-extractor.com/
