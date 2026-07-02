# Bundler Chunking APIs

Use this reference when prescribing or reviewing chunking configuration for Vite, webpack, or Rollup. Every claim below was confirmed against Context7-indexed official documentation as of this skill's `updated` date; re-verify against the installed bundler major before use — see the Context7 Documentation Protocol in `SKILL.md`.

> Version note: bundler chunking surfaces are actively evolving (Vite's Rolldown migration in particular). Verify the installed major version before applying any snippet in this file to a real project.

## What people get wrong

The naive story is:

> "manualChunks is the Vite way to split vendor code, and that's stable."

Wrong, as of Vite 8. Confirmed via Context7 against `/vitejs/vite`'s migration guide: the **object form** of `build.rollupOptions.output.manualChunks` is removed entirely in Vite 8 (Rolldown-powered), and the **function form** is deprecated. Handing a Vite 8+ project the classic object-form snippet produces a config that silently does not apply the intended grouping.

## Vite: manualChunks → codeSplitting

**Deprecated / removed (pre-Vite-8 object form — removed in Vite 8):**

```javascript
// Vite <8, object form — REMOVED in Vite 8
build: {
  rollupOptions: {
    output: {
      manualChunks: {
        vendor: ['react', 'vue'],
      },
    },
  },
},
```

**Deprecated but still functional (function form, pre-Vite-8 and transitional):**

```javascript
// Still works pre-Vite-8; deprecated, flag as a forward-migration item
manualChunks(id) {
  if (id.includes('some-heavy-lib')) {
    return 'heavy-lib-chunk'
  }
},
```

**Current documented replacement (Vite 8+, Rolldown):**

```javascript
build: {
  rolldownOptions: {
    output: {
      codeSplitting: {
        groups: [
          { name: 'vendor', test: /[\\/]node_modules[\\/]/ },
        ],
      },
    },
  },
}
```

Decision rule: confirm the installed Vite major first.
- Vite 8+: use `rolldownOptions.output.codeSplitting.groups`. Do not offer the object-form `manualChunks` snippet — it does not apply.
- Vite <8: function-form `manualChunks(id) {...}` is valid today but is a deprecated pattern; note the future migration to `codeSplitting` as a forward-looking item rather than presenting it as the long-term answer.

## webpack: splitChunks / cacheGroups

Confirmed via Context7 against `/websites/webpack_js` guides (caching, printable, code-splitting). This surface is comparatively stable across recent majors, but `chunks` defaults and `cacheGroups` shape still depend on the installed major — confirm before prescribing.

**Vendor chunk extraction:**

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

Notes:
- `chunks: 'all'` makes both synchronously and dynamically imported modules eligible for the cache group; the webpack default for `optimization.splitChunks.chunks` (when not explicitly set) is `'async'`, which only splits dynamically imported modules. Confirm which behavior the review target actually wants — a review that recommends `cacheGroups` without checking the effective `chunks` mode can silently miss the synchronous-import case.
- Merging all of `node_modules` into a single `vendors` chunk (as shown) is explicitly called out in webpack's own guidance as not generally recommended for typical apps — it produces a single large chunk that invalidates entirely on any dependency bump. Prefer scoping `cacheGroups` to specific heavy dependencies over a single monolithic vendor bucket unless the project's caching strategy specifically wants long-term vendor-chunk stability at the cost of granularity.

**Dynamic import as a split point:**

```javascript
function onClick() {
  import("./module")
    .then((module) => module.default)
    .catch((err) => {
      console.log("Chunk loading failed")
    })
}
```

**Named dynamic-import chunk (magic comment):**

```javascript
import(
  /* webpackChunkName: "app" */
  "./app.jsx"
).then((App) => {
  // ...
})
```

Use `webpackChunkName` when the review needs a stable, human-readable chunk name for budget tracking across builds — an unnamed dynamic-import chunk gets a content-hashed name that is harder to track budget deltas against over time.

## Cross-bundler decision rule

- Do not present a Rollup, Vite, or webpack chunking snippet as bundler-agnostic; the option names and defaults are not interchangeable, and copying a webpack `cacheGroups` shape into a Vite config (or vice versa) will not work.
- Every chunking recommendation in a review must state which bundler and which major version it targets, resolved via Context7 for that specific project, not assumed from the most common current pattern in training data.
