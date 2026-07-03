# Hydration Mismatch Diagnosis

Use this reference when a hydration-mismatch error or warning has been reported and the root cause is not yet identified.

## What people get wrong

The reflex move is:

> There's a hydration warning in the console. Add `suppressHydrationWarning` and move on.

That is not a diagnosis. It is silence applied to a symptom. `suppressHydrationWarning` only works one level deep, does not make the underlying value consistent, and React explicitly documents it as an escape hatch that should not be overused. Applying it before naming the mechanism means the next mismatch — possibly a real one, possibly deeper in the tree — gets silently absorbed into the same reflex.

## Version-specific error format — check this first

The shape of the error itself tells you which React major version you are diagnosing against, and the two shapes carry different diagnostic information:

- **React 19+**: a single detailed error with an inline diff (`+ Client` / `- Server`) pinpointing the exact mismatched node, plus an enumerated list of the five canonical causes in the error text itself, plus a link to `https://react.dev/link/hydration-mismatch`.
- **React 18**: hydration mismatches from missing/extra text content are treated as errors (not silently patched), and React reverts to client rendering up to the closest `<Suspense>` boundary — but the console output is less specific about *which* node and *why* than React 19's diff-style message.
- **Pre-18**: React would attempt to "patch up" individual mismatched nodes on the client — a materially different (and less safe) recovery behavior than 18+.

Confirm the installed React major version before interpreting the error text. Diagnosing a React 18 mismatch as if it carries a React 19 diff, or vice versa, leads to misreading what the error is actually telling you.

## The five canonical causes

React's own diagnostic message enumerates these. Map the specific reported mismatch to one of them before proposing anything:

1. **Environment branch** — `typeof window !== 'undefined'` (or equivalent) used to render different output on server vs. client.
2. **Variable input** — `Date.now()`, `Math.random()`, or any value that changes between the server render and the client's initial render.
3. **Locale-dependent formatting** — `toLocaleDateString()`, `Intl.NumberFormat`, or similar, where the server's locale/timezone differs from the client's.
4. **Unsent external state** — data that changed between server render and client hydration, rendered without the server sending a snapshot of the value it used alongside the HTML.
5. **Invalid HTML nesting** — structurally invalid tag nesting (e.g., a `<div>` inside a `<p>`) that the browser silently repairs during parsing, producing a DOM tree that no longer matches what React expects to hydrate.

A sixth, non-code cause exists and should not be misdiagnosed as a code defect: a browser extension mutating the DOM before React hydrates. If the mismatch is isolated to attributes an ad-blocker or password-manager extension is known to inject, say so explicitly rather than chasing a phantom code path.

## Diagnostic procedure

1. Confirm the React major version.
2. Read the full error/diff text (React 19) or reproduce and inspect the DOM/console output (React 18) — do not diagnose from a truncated paste.
3. Search the flagged component (and its ancestors up to the nearest previous hydration boundary) for each of the five causes in order: environment branches, non-deterministic values, locale formatting calls, externally-sourced data without a matching server-sent snapshot, and invalid nesting.
4. State the identified cause explicitly, citing the file:line and the specific expression responsible.
5. Only after the cause is named, propose a fix scoped to that cause — see fix mapping below.

## Fix mapping — do not default to suppression

| Root cause | Correct fix | `suppressHydrationWarning` acceptable? |
|---|---|---|
| Environment branch (`typeof window`) | Defer the branch to `useEffect`/client-only render after mount, or use a library-provided SSR-safe check | No |
| `Date.now()` / `Math.random()` | Compute the value once on the server and pass it down as a prop/serialized snapshot; do not recompute on the client | No |
| Locale/timezone formatting | Pass the server's resolved locale/timezone explicitly instead of relying on ambient `Intl` defaults, or accept the value as genuinely unavoidable | Only if genuinely unavoidable, with a written justification comment |
| Unsent external/changing data | Send a snapshot of the data used for the server render down to the client so the client's first render matches it | No |
| Invalid HTML nesting | Fix the markup structure | No |
| Browser extension interference | No code fix; document as an environment artifact, do not chase it as a defect | N/A |

The only row where `suppressHydrationWarning` is legitimate is the locale/timezone row, and only when the mismatch truly cannot be eliminated (for example, a "time ago" display that is inherently observer-relative) — and even then, it requires a comment at the call site stating why suppression is correct, not merely convenient.

## Adversarial checklist

Before closing a hydration-mismatch diagnosis, answer these:

- Which of the five canonical causes (or the extension exception) does the evidence actually point to — not which one is the easiest to write a fix for?
- Was the diagnosis made against the correct React-major-version error format?
- If the fix is `suppressHydrationWarning`, is there a written justification, and is the non-determinism actually unavoidable rather than just inconvenient to eliminate?
- Does the fix address the value/branch that differs, or does it just make the warning go away while the divergent values still exist?
- Is this an isolated single-node mismatch, or does the same root cause recur across multiple components (suggesting a shared utility/hook is the real source)?

If these cannot be answered, the diagnosis is not complete — say so rather than proposing a fix.
