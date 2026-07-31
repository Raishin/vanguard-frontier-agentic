---
name: ssr-hydration-streaming-diagnosis
description: Diagnoses hydration-mismatch errors and streaming/Suspense-boundary structural issues to their specific root cause — non-deterministic rendering source, missing error boundary, serial data-fetch waterfall, or premature auth-unchecked streaming — grounded in the exact React/Next.js version's diagnostic behavior.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: observability
---

# SSR Hydration & Streaming Diagnosis

## Purpose

Diagnose hydration-mismatch errors and streaming/Suspense-boundary structural issues to their specific root cause, without collapsing every hydration warning into a reflex `suppressHydrationWarning` fix or every streaming complaint into "wrap it in Suspense." This skill exists so root-cause identification — non-deterministic rendering source, missing error boundary, serial fetch waterfall, or auth-unchecked premature streaming — happens before any fix is proposed, and so that fix proposal never substitutes for diagnosis.

## When to use

Use this skill when the user asks to:

- diagnose a hydration-mismatch error or console warning (React 18 warning-style or React 19 diff-style),
- investigate a TTFB/LCP regression that followed an SSR or streaming change,
- review why a Suspense-wrapped section crashes to blank instead of degrading gracefully,
- review a new Suspense/error-boundary tree design before it ships.

Do not use this skill for:

- state/store serialization design decisions with no specific mismatch or streaming symptom — that is `state-management-decision-review`,
- route-tree or code-splitting review unrelated to SSR/streaming mechanics — that is `routing-navigation-review`,
- generic component decomposition or state-placement review — that is `react-component-architecture-review`.

## Context7 Documentation Protocol

- Resolve `/reactjs/react.dev` before diagnosing any hydration-mismatch error. The diagnostic error *format itself* is version-specific: React 19 emits a single detailed diff-style error (`Hydration failed because the server rendered HTML didn't match the client...` with a `+ Client` / `- Server` diff), while React 18 emits separate, less-specific warnings and silently "patches" mismatched nodes instead of remounting from the nearest Suspense boundary. Diagnosing a React 18 warning against the React 19 mental model (or vice versa) produces a wrong root-cause guess.
- Before recommending a fix, confirm the installed React major version (check `package.json` / lockfile) and call `query-docs` scoped to that version for "hydration mismatch" and, if `use()` or a Suspense-wrapped fetch is involved, for "use hook Suspense error boundary."
- Resolve `/vercel/next.js` and call `query-docs` scoped to the confirmed Next.js version before asserting streaming/Suspense-boundary semantics, `loading.js` behavior, or bot/crawler streaming exceptions — Next.js waits for all data fetching to finish before sending a fully rendered page to bots/crawlers instead of streaming progressively, which changes what "premature streaming" even means for that request path.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label every claim `documentation-based, unverified against current release`.

## Lean operating rules

- Root cause before fix, always. Do not propose `suppressHydrationWarning`, a Suspense-boundary restructure, or a caching change until the specific mechanism is named and evidenced. A fix proposed before root cause is identified is a guess wearing a diagnosis costume.
- The four canonical hydration-mismatch causes per React's own diagnostic message are: a server/client branch (`typeof window !== 'undefined'`), variable input (`Date.now()`, `Math.random()`), locale-dependent formatting that differs between server and client, and external/changing data rendered without a matching snapshot sent alongside the HTML — plus invalid HTML tag nesting as a fifth, structural cause. Map the reported mismatch to one of these five before naming a fix.
- `suppressHydrationWarning` is a one-level-deep escape hatch, not a fix. It is acceptable only when the root cause is a genuinely unavoidable non-determinism (for example, a legitimately locale/timezone-dependent timestamp) and only with a documented justification comment at the call site. Treat any use of it without an identified root cause and written justification as a rejected finding, not an approved one.
- Since React 18, a hydration mismatch from missing/extra text content is treated as an error, not a soft warning: React discards and re-renders client-side from the nearest `<Suspense>` boundary rather than patching individual nodes. This means the blast radius of a mismatch is bounded by Suspense placement — a missing or too-coarse Suspense boundary turns a small mismatch into a large client re-render.
- Every component that suspends via `use()` on a Promise (or any Suspense-triggering data read) must have a paired Error Boundary somewhere in its ancestor tree, because a rejected Promise propagates to the nearest Error Boundary, not the nearest Suspense fallback. A Suspense boundary with no ancestor Error Boundary is a crash-to-blank defect, not a style preference.
- Suspense boundaries that are too coarse (one boundary wrapping an entire page, mixing fast and slow content) block fast content behind the page's slowest fetch. Prefer sibling Suspense boundaries around independently-loading sections so each streams in as its own data resolves, per Next.js's own parallel-streaming pattern.
- A fetch that depends on the result of a prior fetch (for example, fetching playlists that require an artist ID from a preceding fetch) is legitimately sequential — do not flag that as a waterfall defect. Only flag fetches that are data-independent but are still awaited one after another instead of started concurrently.
- Streaming that flushes page structure or partial content to the client before an authorization check has resolved is a potential information-disclosure defect, not a performance nitpick — escalate it, do not file it as a UX note.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only) — diagnose from the reported error text, the component/boundary source, and any network/fetch-order trace the user provides.

## References

Load these only when needed:

- [Hydration mismatch diagnosis](references/hydration-mismatch-diagnosis.md) — use when diagnosing a specific hydration-mismatch error or warning to its root cause.
- [Suspense and streaming structure review](references/suspense-streaming-structure.md) — use when reviewing or designing a Suspense/error-boundary tree, or diagnosing a TTFB/LCP regression or crash-to-blank symptom.
- [Fetch waterfall and auth-timing review](references/fetch-waterfall-and-auth-timing.md) — use when diagnosing a sequential-fetch performance issue or reviewing whether streaming exposes content before authorization resolves.

## Response minimum

Return, at minimum:

- the specific root-cause mechanism identified (or explicit statement that root cause is unresolved and more evidence is needed — never a guess presented as a diagnosis),
- the React/Next.js major version the diagnosis was verified against,
- evidence level (`documentation-based`, `repo evidence`, `user-provided evidence`, or `inference`) and what Context7 query grounded the claim,
- the proposed fix scoped to the identified root cause, or explicit refusal to propose a fix until root cause is confirmed,
- security/disclosure caveat if streaming-before-auth risk is present,
- verdict: approve / approve-with-notes / block.
