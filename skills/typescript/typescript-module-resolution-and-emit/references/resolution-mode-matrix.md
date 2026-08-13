# Resolution Mode Matrix

How `module` and `moduleResolution` map onto emit and declaration behavior, with removed values flagged.

- Only `node16`, `nodenext`, and `bundler` are valid `moduleResolution` values under the current compiler; `classic` and `node10` are removed and produce error TS5108 rather than falling back to a default.
- `module` defaults to `esnext` as of TypeScript 6.0, a change from the previous CommonJS-oriented default, so a configuration relying on the old implicit default now behaves differently even with no explicit edit.
- The condition ordering inside an `exports`/`imports` map is evaluated in listed order, first match wins; the `types` condition must be listed before `import`/`require`, and `default` must be listed last, or a consumer's resolver picks the wrong branch or none at all.
- The official tsconfig reference page's value tables for `module`/`moduleResolution` are documented to lag the compiler's actual accepted and removed values — the compiler binary's own error output (TS5108 on a removed value) is the authoritative source, not the prose page.
- `moduleResolution: "bundler"` models how a bundler resolves imports and is not equivalent to how Node's own resolver behaves — code correct under `bundler` resolution is not proven correct for direct Node execution.
- `.mts` and `.cts` extensions force ESM and CJS interpretation respectively regardless of the nearest `package.json`'s `type` field, overriding the ambient default that governs plain `.ts` files.
