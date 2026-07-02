# Vite Chunking Configuration

Use this reference when reviewing or writing Vite's production-build chunking configuration, or when a project reports a chunking config that "used to work" and no longer applies.

> Version note: this reference reflects Context7-grounded Vite documentation as of this skill's `updated` date. Vite's build-engine option surface has changed across major versions — re-verify against `mcp__Context7__query-docs` for `/vitejs/vite` before applying any snippet below to a specific installed version, and confirm the version with the project's own lockfile or `vite --version` output rather than assuming it from `package.json` ranges.

## What people get wrong

The naive assumption is:

> "Vite chunking config" is one API — `manualChunks` — and it works the same across every Vite version.

Wrong. Vite's production build is not one bundler; it is a build-engine seam. Depending on the Vite major version and which output-options key the config uses, the actual bundling engine is either:

- **Rollup**, addressed via `build.rollupOptions.output.manualChunks`, or
- **Rolldown** (a Rust-based bundler with a Rollup-compatible API surface, but its own option shapes for some features), addressed via `build.rolldownOptions.output.codeSplitting`.

A config snippet that is correct for one engine can silently no-op under the other. This is not a deprecation warning you can ignore — Vite's own migration guide documents the object form of `manualChunks` as **removed**, not merely discouraged.

## Officially grounded shape (what Vite/Rollup/Rolldown docs actually say)

Per Context7-grounded Vite build/migration docs and Rollup's own configuration docs:

- **Rollup's `output.manualChunks`** (used when the config's build path routes through `rollupOptions`) supports two forms:
  - **Object form**: `manualChunks: { 'vendor-react': ['react', 'react-dom'] }` — maps a chunk name to an explicit array of module specifiers. Rollup's own docs describe this as the simpler, safer form.
  - **Function form**: `manualChunks(id) { if (id.includes('node_modules')) return 'vendor'; }` — receives the module ID (and `{ getModuleInfo, getModuleIds }`) and returns a chunk name string or `null`/`undefined` to leave the module unassigned. Rollup's docs note `output.onlyExplicitManualChunks` is itself a separate, deprecated option slated to become Rollup 5's default behavior — confirm this hasn't shifted before relying on implicit fallback chunking.
  - Rollup itself has **not** removed either form; the removal described below is specific to Vite's own option surface on its migration path toward Rolldown.

- **Vite's `build.rollupOptions.output.manualChunks`** — per Vite's migration guide (grounded via Context7): the **object form is removed** and the **function form is deprecated**. A project still passing `manualChunks: { vendor: [...] }` to `rollupOptions.output` on a Vite version past this removal will see the option ignored, not an error — this is the single most common false-negative in this review.

- **Vite's `build.rolldownOptions.output.codeSplitting`** — the documented replacement. Per Vite's build guide and Rolldown's own `manualCodeSplitting` reference, this takes a declarative shape:

  ```javascript
  export default defineConfig({
    build: {
      rolldownOptions: {
        output: {
          codeSplitting: {
            groups: [
              { name: 'vendor-react', test: /node_modules\/(react|react-dom)/ },
              { name: 'chunk', test: 'chunk.css' },
            ],
          },
        },
      },
    },
  })
  ```

  Each entry in `groups` is a `{ name, test }` pair; `test` can match on module ID pattern. This is a declarative alternative to `manualChunks`'s imperative function form, intended to avoid circular-dependency footguns that hand-written `manualChunks` functions can introduce.

- **`splitVendorChunkPlugin`** — Vite ships a built-in plugin implementing a `vendor` chunk heuristic (any `node_modules` module, not CSS, statically imported by an entry point) as a `manualChunks` function. Per its own source (grounded via Context7): when composed with a user-supplied **function-form** `manualChunks`, it wraps and chains the two; when composed with an **object-form** `manualChunks`, it detects the object form and **logs a console warning that it has no effect**, then does nothing. A project reporting "the vendor-splitting plugin isn't working, no error, just a console warning" is very likely hitting exactly this interaction.

## Non-negotiable review rules

1. **Confirm the output-options key before applying either ruleset.** Grep the Vite config for `rollupOptions` vs `rolldownOptions` under `build`. These are not interchangeable, and a chunking snippet written for one silently does nothing under the other.
2. **Confirm the installed Vite major.** Do not infer it from a `^7.0.0`-style semver range in `package.json` alone — a caret range can resolve to a version past a documented removal. Prefer lockfile-resolved version or a reported `vite --version` if available as user-provided evidence.
3. **If the config uses `rollupOptions.output.manualChunks` as an object**, and the confirmed Vite major is on/after the version where Vite's migration guide documents the removal, flag this as a silent no-op, not a style nit — the chunking strategy the team believes is active is not running.
4. **If the config uses `rollupOptions.output.manualChunks` as a function**, treat it as valid but flag it as a forward-migration item toward `rolldownOptions.output.codeSplitting`, per Vite's own deprecation notice — do not present the function form as the long-term recommended pattern.
5. **When migrating a `manualChunks` function to `codeSplitting` groups**, do not assume a 1:1 mechanical translation. A `manualChunks` function can express arbitrary conditional logic (e.g., branching on `getModuleInfo(id).importers`); `codeSplitting`'s `groups` array is pattern-match-based (`test` against module ID). Confirm each branch of the original function has an equivalent `test` pattern before calling the migration complete — untranslatable branches are a real migration risk, not a formality.
6. **Do not conflate Vite's dev-server module graph with its build chunking.** Chunking config has zero effect on `vite dev`; it applies only to `vite build`. A user reporting a "chunking problem" during local dev is describing something this config cannot cause.

## Verification targets

- The confirmed Vite major (from lockfile or reported CLI version) and the output-options key actually present in `vite.config.*`.
- For a `manualChunks` object-form finding: the exact Vite version threshold from `mcp__Context7__query-docs` against `/vitejs/vite`'s migration guide, quoted, not paraphrased from memory.
- For a proposed `codeSplitting` groups migration: each `groups[].test` pattern mapped explicitly back to the branch of the prior `manualChunks` function it replaces.
- A build output diff (chunk list, sizes) confirming the new config actually changed the chunk boundaries — an unchanged chunk list after a config edit is evidence the edit didn't take effect, not evidence the config was already optimal.
