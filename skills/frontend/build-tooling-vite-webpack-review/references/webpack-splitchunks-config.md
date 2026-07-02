# Webpack SplitChunksPlugin Configuration

Use this reference when reviewing or writing Webpack's `optimization.splitChunks` configuration, `cacheGroups`, or `runtimeChunk` settings.

> Version note: confirm the installed Webpack major via `mcp__Context7__query-docs` against Webpack's own docs before asserting a specific default value below applies — `SplitChunksPlugin` defaults have shifted across major versions historically, and this reference documents the currently Context7-grounded defaults, not a version-pinned guarantee.

## What people get wrong

The naive assumption is:

> `optimization.splitChunks: { chunks: 'all' }` is "the vendor-splitting setting" and turning it on is the whole job.

Incomplete. `chunks: 'all'` only changes *which* chunks (`async`, `initial`, or both) are eligible for splitting — it does not by itself define a vendor group, a size threshold, or a request-count ceiling. Webpack's actual chunk-splitting behavior is the product of several settings interacting: `chunks`, `minSize`, `minChunks`, `maxAsyncRequests`, `maxInitialRequests`, `enforceSizeThreshold`, and the `cacheGroups` map. Reviewing only `chunks` and ignoring the rest of the object produces a config that "does something" without the reviewer knowing what.

## Officially grounded shape (what Webpack docs actually say)

Per Context7-grounded Webpack docs (`SplitChunksPlugin` reference and code-splitting/caching guides):

- **Default `optimization.splitChunks` shape**, documented explicitly:

  ```javascript
  optimization: {
    splitChunks: {
      chunks: "async",
      minSize: 20000,
      minRemainingSize: 0,
      minChunks: 1,
      maxAsyncRequests: 30,
      maxInitialRequests: 30,
      enforceSizeThreshold: 50000,
      cacheGroups: {
        defaultVendors: {
          test: /[\\/]node_modules[\\/]/,
          priority: -10,
          reuseExistingChunk: true,
        },
        default: {
          minChunks: 2,
          priority: -20,
          reuseExistingChunk: true,
        },
      },
    },
  }
  ```

  The default `chunks: "async"` means only dynamically-imported (`import()`) chunks are eligible for splitting out of the box — a project relying on defaults and expecting *initial* (entry-point) chunks to also get vendor-split needs to explicitly set `chunks: "all"`, either globally or per cache group.

- **Explicit vendor cache group** — the documented pattern for pulling all `node_modules` code into a named `vendors` chunk:

  ```javascript
  optimization: {
    runtimeChunk: 'single',
    splitChunks: {
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },
  ```

  Webpack's own caching guide pairs this with `runtimeChunk: 'single'` specifically for long-term-caching correctness — extracting the webpack runtime/manifest into its own chunk so that changes to application code don't invalidate the vendor chunk's content hash. A `vendor` cache group added without `runtimeChunk: 'single'` is a common half-fix: the vendor chunk hash still churns on unrelated app changes because the runtime is bundled with it.

- **Custom cache-group merging** — Webpack's docs note that merging all of `node_modules` into a single `vendors` chunk via a broad `test: /node_modules/` cache group "is not generally recommended" as a default strategy, but can be a deliberate trade-off for long-term caching in specific deployment setups. Treat a broad single-vendor-chunk config as an explicit trade-off to confirm with the team, not a default best practice to apply unprompted.

## Non-negotiable review rules

1. **Read the whole `splitChunks` object, not just `chunks`.** `minSize`, `maxInitialRequests`, and `enforceSizeThreshold` jointly determine whether a module that "should" be split actually gets split. A module below `minSize` (default 20000 bytes) will not become its own chunk regardless of `cacheGroups` targeting, unless a cache group's own `enforce: true` overrides that.
2. **Confirm `chunks` scope per cache group, not just globally.** A global `chunks: 'async'` with a `vendor` cache group that doesn't override `chunks: 'all'` will not split vendor code out of the initial/entry bundle — only out of dynamically-imported chunks. This is the most common "I added a vendor cache group and nothing changed in the initial bundle" defect.
3. **Pair any named vendor cache group with `runtimeChunk: 'single'` when the goal is long-term-caching stability**, and say so explicitly if the config is missing it — otherwise the vendor chunk's cache-busting behavior won't match what the team expects.
4. **Do not propose a broad `test: /node_modules/` single-vendor-chunk config as a default recommendation.** Per Webpack's own docs, this is an explicit trade-off (fewer, larger, more cache-stable chunks vs. finer-grained, more parallel-loadable chunks), not a universal best practice — surface it as a choice with a named trade-off, not a fix.
5. **Confirm the installed Webpack major before asserting any specific default value** (e.g., `minSize: 20000`) — quote the value from `mcp__Context7__query-docs` against the confirmed major rather than this reference's cached snapshot, since defaults have changed across majors historically.
6. **Distinguish `optimization.splitChunks` (automatic, heuristic-driven) from explicit multi-entry configuration** (`entry: { index: ..., another: ... }`) — a duplicate-dependency finding across two *entry points* (not two dynamically-split chunks) is a `splitChunks` cache-group problem, not an entry-config problem; do not propose merging entries as the fix when the actual defect is a missing/misconfigured cache group.

## Verification targets

- The confirmed Webpack major and the full resolved `optimization.splitChunks` object (defaults plus overrides), not just the cache-group diff.
- The `chunks` scope value effective for the specific cache group in question (group-level override, if present, wins over the global setting).
- A build stats output (`webpack --json` or `webpack-bundle-analyzer` treemap) confirming the proposed cache-group change actually produced the intended chunk split, with a before/after chunk-name and chunk-size comparison.
- Whether `runtimeChunk` is configured, when reviewing a named vendor cache group intended for long-term-caching stability.
