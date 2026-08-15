# Declaration Emit And Rollup

Emit-strategy tradeoffs among `declaration`, `isolatedDeclarations`, and rollup output.

- Declaration emit strategy for a published surface spans three distinct decisions this skill treats separately: `declaration` (the base emitted `.d.ts` output), `isolatedDeclarations` (a stricter per-file declaration-emit mode), and rollup (flattening multiple declaration files into a single published surface via a tool such as API Extractor).
- API Extractor requires the source already be compiled with `tsc` and `declaration: true` before it can produce an API report or `.d.ts` rollup — it consumes emitted declarations rather than performing its own compilation.
- The official tsconfig documentation page is confirmed stale relative to the compiler binary for at least one option-value table (removed `moduleResolution` values); treat any declaration-emit-option semantic not directly confirmed against the installed compiler version as needing verification rather than asserted from the prose page.
- TypeScript 7.0 has no stable programmatic API until 7.1; tools such as API Extractor that consume the compiler programmatically are documented to stay on TypeScript 6.0 until that API stabilizes — confirm which compiler major actually produced the `.d.ts` under review and the tool's own supported-compiler statement before trusting either output.

## Sources

- https://api-extractor.com/
