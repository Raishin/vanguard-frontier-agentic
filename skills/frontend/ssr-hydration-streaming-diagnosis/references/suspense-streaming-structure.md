# Suspense and Streaming Structure Review

Use this reference when reviewing or designing a Suspense/error-boundary tree, or diagnosing a TTFB/LCP regression or a crash-to-blank symptom tied to streaming.

## What people get wrong

The naive story is:

> Wrap it in `<Suspense>` and streaming just works.

Incomplete. A `<Suspense>` boundary controls *what shows while content is pending* and *how far a hydration-mismatch re-render blast radius reaches*. It does not, by itself, handle rejection (that is the Error Boundary's job), does not make sibling sections stream independently unless they are wrapped in their own boundaries, and does not change fetch ordering — a slow fetch inside a narrow Suspense boundary is still slow; the boundary just contains the wait to that section instead of blocking the whole page.

## Officially grounded shape

- `use()` reading a pending Promise causes the calling component to suspend; React shows the nearest `<Suspense fallback>` while pending.
- If the Promise **rejects**, the error propagates to the nearest **Error Boundary**, not the nearest Suspense fallback. A component that calls `use()` (or otherwise suspends on a rejectable data source) with a Suspense boundary but no ancestor Error Boundary will crash to an unhandled error when that data source fails — there is no fallback UI for the rejection case.
- Since React 18, a hydration mismatch causes React to discard and re-render client-side starting from the **nearest Suspense boundary**, not the whole tree and not just the single mismatched node. Suspense placement therefore directly bounds mismatch blast radius, independent of streaming concerns.
- Next.js's own streaming guide demonstrates wrapping **independent** async sections in **separate, sibling** `<Suspense>` boundaries so each streams in as its own data resolves, rather than one boundary gating the whole page behind its slowest child.
- Streaming can be implemented via a route's `loading.js` file or via explicit `<Suspense>` around a component. For bots/crawlers, Next.js waits for all data fetching to finish and sends the fully rendered page rather than streaming progressively — a bot-facing request does not get the same partial-render exposure window a browser request does, which matters when reasoning about what a "premature" stream can leak to which caller.

## Non-negotiable design rules

### 1. Every suspending read needs a paired Error Boundary

Do not approve a Suspense boundary wrapping a `use()` call, a suspending fetch, or any Suspense-triggering data read without confirming an Error Boundary exists somewhere in its ancestor chain. If the user has not added one, this is a defect: on rejection, the user sees an unhandled crash/blank instead of a fallback message.

### 2. Boundary granularity should match content speed, not code convenience

One `<Suspense>` around an entire page is the easiest thing to write and the worst thing to ship when the page mixes fast content (e.g., a nav shell) with slow content (e.g., a report requiring heavy aggregation). The fast content is held hostage by the slow content's fallback. Prefer sibling boundaries per independently-loading section, following the pattern Next.js documents for parallel streaming.

### 3. Boundary granularity also bounds mismatch blast radius

A too-coarse boundary is a double defect: it blocks fast content behind slow content *and* it means any hydration mismatch inside that huge boundary forces React to discard and re-render the entire boundary's subtree client-side, not just the small mismatched region. Narrower, purpose-scoped boundaries reduce both problems simultaneously.

### 4. A Suspense fallback is not a substitute for a loading-state design review

Confirm the fallback communicates something coherent (skeleton matching the eventual layout, or a clear loading indicator) rather than an empty or jarring placeholder — a correct boundary with a poor fallback is still a shippable defect, just a lower-severity one than a missing Error Boundary.

## Minimal safe review flow

1. Map every `<Suspense>` boundary in the tree under review and what it wraps.
2. For each boundary, identify what suspends inside it (a `use()` call, a Server Component awaiting a fetch, a lazy-loaded component) and confirm an Error Boundary exists in its ancestor chain — if not, that is a defect (see `fetch-waterfall-and-auth-timing.md`'s parent skill rule: crash-to-blank is not acceptable).
3. For each boundary, ask whether it wraps content of meaningfully different load-latency profiles. If yes, and content is independent, recommend splitting into sibling boundaries per the Next.js parallel-streaming pattern.
4. For each boundary wrapping content that depends on a prior async result (a genuinely sequential dependency, e.g., needing an ID from a previous fetch before the next component can render), confirm the nesting reflects that dependency rather than an accidental structure that could be flattened.
5. Confirm the fallback UI is coherent, not a placeholder afterthought.
6. State the verdict per boundary, not just for the tree as a whole — a tree can be correct in three boundaries and defective in one.

## High-risk assumptions to kill

- "It's wrapped in Suspense, so errors are handled" — Suspense handles pending state, not rejection.
- "One Suspense boundary at the top is simpler and streaming still works" — technically true, but it defeats the purpose of granular streaming and maximizes mismatch blast radius.
- "The fallback shows briefly so its content doesn't matter" — a jarring or mismatched-size fallback causes layout shift on resolve, which is a measurable UX/performance regression, not a cosmetic detail.
- "Bots see the same partial-render window a browser does" — they do not; Next.js waits for full data resolution before responding to bots/crawlers, which changes threat/exposure reasoning for that path specifically.

## When to push back

Push back if the user asks to:

- remove a Suspense boundary "to make the loading flicker go away" without addressing the underlying slow fetch,
- wrap a suspending, rejectable data read in Suspense with explicit instruction to skip the Error Boundary "for now,"
- collapse several independently-loading sections into one shared boundary purely to reduce the number of `<Suspense>` tags in the file.

Those trade a visible symptom for a worse, less visible one (either a crash-to-blank on failure, or slow content gating fast content).
