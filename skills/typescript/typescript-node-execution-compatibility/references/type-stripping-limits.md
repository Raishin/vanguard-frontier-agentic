# Type-Stripping Limits

The quoted documentation on no type checking and ignored `tsconfig.json`, plus the syntax that throws, `node_modules` refusal, and mandatory import extensions.

- Node's own documentation states plainly that "no type checking is performed" and that "Node.js ignores tsconfig.json files" when running TypeScript directly — a successful run proves execution, not correctness.
- `enum`, a runtime `namespace`, parameter properties, `import =`, and decorators throw `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` under Node's type stripper, because none of them are erasable — they carry runtime semantics the stripper cannot simply delete.
- A `.ts` file located under any `node_modules` directory is refused by Node's stripper unconditionally, regardless of the consuming project's own configuration.
- Import specifiers must carry an explicit extension for Node's resolver; an extension-less specifier that works under a bundler or `tsc`'s own module resolution fails at direct-execution runtime.
- Type stripping is enabled by default since Node v23.6.0/v22.18.0 and became stable since v25.2.0/v24.12.0 — a claim about Node running TypeScript must state which of these versions and stability levels the target actually meets.
- `--experimental-transform-types` was removed in Node v26.0.0; any reference to it as a currently-needed flag is stale against v26 and later.
- `erasableSyntaxOnly` restricts source to only the TypeScript syntax the stripper can erase; it is meaningful specifically for a direct-execution pipeline and is a different question from whether a full `tsc`/bundler build type-checks the same source.
