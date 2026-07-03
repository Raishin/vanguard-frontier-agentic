# Fallback Verification Patterns

Use this reference when a finding requires checking whether an actual feature-detection gate or polyfill exists and correctly covers the risky code path — across JS, CSS, and HTML — rather than accepting an assertion that "there's a fallback."

## What people get wrong

The common bad assumption is:

> The PR description says there's a fallback, so the fallback question is closed.

That is not evidence. A description, a comment, or a variable name (`hasPolyfillFallback`) is not proof that a gate exists, that it is correctly scoped to the risky code path, or that it fails safe. Always read the actual gating code before closing a fallback finding.

## Three fallback mechanisms, not one

Do not treat "fallback" as a single undifferentiated concept. There are three distinct mechanisms, each with its own failure modes:

1. **CSS `@supports` (feature queries)** — gates a CSS declaration block on whether the engine supports a given property/value pair.
   - Failure mode: `@supports` itself has near-universal support, but the *query* can be written incorrectly (testing the wrong property, or a value that is always true/false), silently making the gate a no-op.
   - Verification: confirm the `@supports` condition actually names the property/value being used in the gated block, and that a real non-supporting fallback declaration exists *before* the `@supports` block (CSS cascade order — the fallback must be overridable, not overridden).

2. **JS runtime feature detection** (`if ('IntersectionObserver' in window)`, `typeof`, `in` checks, capability probing) — gates a code path on whether an API exists before calling it.
   - Failure mode: detecting the *existence* of an API is not the same as detecting *correct/complete* behavior — some browsers ship a partial or buggy implementation that passes an existence check but fails at runtime. For any feature with known partial-implementation history, verify the detection checks the specific method/behavior actually used, not just the top-level constructor.
   - Verification: confirm the detected branch and the fallback branch are both reachable in the code (not dead code), and that the fallback branch has been exercised — read it, do not assume it degrades gracefully.

3. **Polyfills** (loaded unconditionally or conditionally) — ship an implementation of the missing capability.
   - Failure mode: an unconditionally-loaded polyfill ships bytes to every user, including the ~95%+ who already have native support, with zero runtime code-splitting; a conditionally-loaded polyfill (dynamic `import()` behind a feature-detection check) avoids that cost but must be verified to not create a race condition where the gated code runs before the polyfill import resolves.
   - Verification: confirm whether the polyfill load is conditional or unconditional (check the bundler output / import site), and if conditional, confirm the code that depends on the polyfilled API awaits the import before use.

## HTML-level fallback pattern

For new HTML elements/attributes, the platform's own graceful-degradation model (unknown elements render as inline/anonymous boxes, unknown attributes are ignored) is sometimes sufficient — but only when the fallback behavior of "ignore it" is actually an acceptable UX, not a broken one. Do not accept "the browser will just ignore it" as a verified fallback without checking whether ignoring the attribute leaves the feature in a broken or misleading state (e.g. a `<dialog>` element with no polyfill on a browser that doesn't support it does not just degrade — it fails to open at all).

## Non-negotiable rules

1. **A fallback finding is not closed until the actual gating code has been read.** Comments, descriptions, and test names are not evidence.
2. **The fallback path must be reachable and non-dead.** If the "supported" branch always evaluates true in every test/CI browser, the fallback branch may be unexercised dead code — flag this explicitly.
3. **Existence checks are not behavior checks.** For features with a documented history of partial/buggy implementations, verify the detection matches the actual method/behavior used.
4. **Unconditional polyfill loading is a performance finding, not just a compatibility non-issue.** Note the bundle-size cost even when the fallback itself is technically correct.
5. **CSS cascade order matters.** A fallback declaration placed *after* the `@supports`-gated declaration will win regardless of support, silently defeating the gate — always check declaration order, not just presence of the `@supports` block.

## Verification targets

- The literal `@supports`, `if`/`in`/`typeof` check, or dynamic-`import()` gate in the diff or file, read directly — not paraphrased from a PR description.
- CSS declaration order around any `@supports` block.
- Whether the polyfill import is conditional (behind a feature check) or unconditional, and whether dependent code awaits it.
- Whether the fallback branch is exercised by any existing test, or is unexercised dead code.

## When to push back

Push back if the user says:

- "there's a fallback, trust me" without pointing at code,
- "the browser will just ignore the unsupported part" for a feature whose absence actually breaks core functionality (not genuinely inert),
- "we'll add the polyfill later" while shipping the unguarded feature now to a matrix that doesn't yet support it.
