# Program Graph Diagnosis

How a TypeScript program graph is structured, what project references change, and what they cost.

- Project references, `composite`, and `incremental` let the compiler skip re-checking unchanged sub-projects, but `.tsbuildinfo` must be preserved as a cache artifact between runs — including in CI — for `incremental` to provide any benefit at all; a clean-checkout CI pipeline that never restores it pays the full check cost every time regardless of configuration.
- A project sitting outside every project reference can leave the language service (editor) checking a larger, unpartitioned graph even when `tsc --build` itself is fast — editor latency and build latency are separate symptoms with separate evidence and separate fixes.
- Introducing project references can serialize a graph that a single-program build previously checked without that ordering constraint; a build that gets slower immediately after adding project references is itself diagnostic evidence, not a contradiction to be explained away.
- Generated code — codegen output, vendored declaration files, barrel re-export modules — can make up a majority of what the compiler checks; a fix aimed at hand-written source cannot help a graph whose volume is dominated by generated files.
- The official TypeScript project-references and performance documentation (the GitHub wiki) is the source for what `--build`, `composite`, and the diagnostics switches are documented to do; treat any claim about their behavior under the native TypeScript 7 compiler as unverified until confirmed against that specific binary.

## Sources

- https://github.com/microsoft/TypeScript/wiki/Performance
