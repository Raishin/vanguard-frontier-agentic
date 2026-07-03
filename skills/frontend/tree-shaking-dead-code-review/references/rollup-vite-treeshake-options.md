# Rollup and Vite Tree-Shaking Options

Use this reference when the project bundles with Rollup directly, or with Vite (which bundles production builds through Rollup on Vite 5/6, and through Rolldown — a Rust reimplementation with a Rollup-compatible option surface — on Vite 7+). Confirm the installed major via Context7 before applying any snippet below; the option surface has moved.

## What people get wrong

The naive story is:

> Rollup's ESM-first design means everything just gets tree-shaken automatically; there's nothing to configure.

Incomplete. Rollup's `treeshake` option is not a single on/off switch — it accepts `false` (fully disabled), a named preset, or a fine-grained options object, and the two settings most responsible for either under-elimination or unsafe over-elimination are `moduleSideEffects` and `propertyReadSideEffects`, both of which default to conservative (safe but less aggressive) behavior unless explicitly overridden.

## The treeshake option surface

```javascript
export default {
  input: 'src/index.js',
  output: { format: 'es', dir: 'dist' },
  treeshake: {
    moduleSideEffects: false,        // assume modules have no side effects
    propertyReadSideEffects: false,  // property reads are side-effect-free
    tryCatchDeoptimization: false,   // don't pessimistically include try-catch bodies
    unknownGlobalSideEffects: false, // accessing unknown globals is safe
    preset: 'smallest',              // or 'safest' / 'recommended'
  },
};
```

`treeshake` also accepts the boolean `true` (default behavior) or one of the preset strings directly (`'smallest' | 'safest' | 'recommended'`) without a full options object.

### moduleSideEffects

`moduleSideEffects` controls whether an entire imported module is retained even when none of its exports are used, mirroring in spirit what webpack's `sideEffects` package.json field does, but configured at the bundler level rather than (or in addition to) the package level. It accepts:

- `boolean` — blanket true/false for all modules.
- `'no-external'` — treat only external (`node_modules`) modules as side-effect-free; local project code keeps its analyzed side effects.
- a string array — explicit module ID patterns to treat as side-effect-free.
- a predicate function `(id, external) => boolean` for per-module logic.

Setting this to `false` globally without auditing what actually lives in `node_modules` is the Rollup-side equivalent of the webpack `sideEffects: false` correctness trap: any dependency that performs import-time registration (polyfills, global CSS, `window` patches) will be silently dropped.

### propertyReadSideEffects

This governs whether Rollup assumes a property read (including getter invocation) can safely be removed if its result is unused:

```javascript
// Removed if treeshake.propertyReadSideEffects === false
const foo = {
  get bar() {
    console.log('effect');
    return 'bar';
  },
};
const result = foo.bar;
const illegalAccess = foo.quux.tooDeep; // would also throw at runtime if kept
```

Setting this `false` can eliminate code whose only purpose was a getter side effect (logging, lazy initialization, a Proxy trap). This is a narrower and less commonly needed lever than `moduleSideEffects` — do not reach for it as a first attempt to eliminate a stubborn module; confirm the actual blocker is a property read, not module-level retention, before touching this option.

### External module elimination example

```javascript
// input
import { unused } from 'external-a';
import 'external-b';
console.log(42);
```

With `treeshake.moduleSideEffects === true` (default-safe), both imports are retained even though `unused` is never referenced, because `moduleSideEffects: true` means Rollup assumes importing `external-a` and `external-b` might do something even without using their exports. With `moduleSideEffects: false`, both are dropped — which is correct only if neither package performs import-time work.

## Vite / Rolldown specifics

Vite's production build has historically gone through Rollup, exposed via `build.rollupOptions`. Starting with Vite 7.3.1, the production bundler is Rolldown, and `build.rollupOptions` is now documented as a deprecated alias for `build.rolldownOptions` — if both are set, `rollupOptions` is ignored and Vite emits a warning. Verify the installed Vite major via Context7 before choosing which option key to write into a config; a `rollupOptions.output.manualChunks` object-form snippet, or a Rollup-specific `treeshake` shape, is not guaranteed to carry over unchanged into `rolldownOptions`.

`import.meta.env.DEV` and `import.meta.hot` guards are statically replaceable constants in Vite's build pipeline — code inside an `if (import.meta.env.DEV)` block is tree-shaken out of production builds by design. If a user reports dev-only code appearing in a production bundle, check whether the guard condition is actually one of these statically-analyzable forms rather than a runtime environment-variable read that the bundler cannot constant-fold.

## Verification target

A Rollup/Rolldown build with `--sourcemap` or the project's configured `rollup-plugin-visualizer`/equivalent output, diffed before and after any `treeshake` option change, on a production-mode build. Confirm via Context7 that the exact option key exists on the installed bundler version before writing it into config — `rollupOptions` vs `rolldownOptions` is the most likely place this silently drifts.
