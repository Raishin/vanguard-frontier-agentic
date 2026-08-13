# Tarball And Types Surface

How to determine what a published package actually ships, including declarations and source maps.

- The packed tarball is determined by `files` in `package.json`, `.npmignore`, and `exports` — not by the contents of the working tree — so a source-tree review alone cannot confirm what ships.
- `publint` checks a published package's structural correctness, including its `exports`/`types` configuration, against documented packaging rules.
- `arethetypeswrong` checks whether a package's declared TypeScript types actually match what each supported resolution mode would load, catching a declaration that resolves to the wrong module shape for a given consumer.
- A shipped `.d.ts` or source map can reveal an internal module path, an unpublished dependency's shape, or a build-machine filesystem path that the compiled JavaScript alone would not expose.
