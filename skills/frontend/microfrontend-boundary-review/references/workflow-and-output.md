# Review workflow and findings contract

Use this reference for the step-by-step boundary-review procedure and the required output shape for any micro-frontend adoption review, new-remote addition, or existing-composition audit.

## What people get wrong

The common bad assumption is:

> "We're using module federation, so the frontend is already decoupled — this review is a formality."

That is backwards. Module federation (and most non-iframe composition mechanisms) decouple the *build*, not the *runtime*. Host and remotes still share one JavaScript execution context, one CSP context, and — unless explicitly pinned or negotiated — a dependency graph that can silently drift out of compatibility. The review exists precisely because "we already use the right tooling" is not the same claim as "the boundary is safe."

## Step-by-step workflow

1. **Classify the composition mechanism.** Determine whether the architecture uses module federation (Webpack Module Federation, Vite plugin federation, or similar), iframe-based composition, or build-time/server-side composition (e.g., server-side includes, build-time module stitching). Do not proceed with mechanism-specific advice until this is confirmed from config, not assumed from the user's description alone.

2. **Determine the isolation level required per remote.** For each remote in scope, ask: does it render or handle data that should not be visible to, or influenced by, a bug or compromise in a sibling remote? If yes, same-runtime composition is disqualified unless a documented, specific mitigation is in place (e.g., a dedicated CSP frame-ancestors/sandboxed iframe for that one remote, or a Trusted Types policy scoped to it).

3. **Review the shared-dependency contract.** Locate the federation config's `shared` block (or equivalent for build-time composition) and check for: pinned or ranged version requirements, `singleton`/`strictVersion`-style enforcement (tool-specific — verify against that tool's current docs, not assumed from React docs), and whether version mismatches fail the build/fail loudly at runtime or fail silently (multiple copies of a library loaded, or an incompatible version silently used).

4. **Confirm ownership is unambiguous.** Every remote must map to exactly one owning team with clear on-call responsibility. A remote with co-ownership, no documented owner, or an owner that is "whoever last touched it" is a blocking finding, not a note.

5. **Assess blast radius.** For each remote, determine what happens to the host and sibling remotes if this remote throws an uncaught error, fails to load, or ships a broken bundle. Look for an error boundary (or equivalent isolation mechanism) around each remote's mount point, and confirm each remote has an independent deployment and rollback path that does not require a host-wide release.

6. **Check same-runtime multi-root hygiene, if applicable.** If the composition mounts multiple independent applications into one page (e.g., multiple `createRoot` calls, one per remote), confirm each root passes a distinct `identifierPrefix` to prevent `useId`-generated identifier collisions across independently-owned code — this is a concrete, checkable defect per official React docs, not a stylistic nit.

7. **Issue a verdict** using the response-minimum contract below.

## Required output shape

Every response to a micro-frontend boundary review must include:

- **Composition mechanism** — module federation / iframes / build-time-server-side, and whether this was confirmed from config or asserted by the user.
- **Isolation verdict per remote** — matched explicitly against that remote's data sensitivity; state which remotes require iframe-or-equivalent isolation and whether that mitigation exists.
- **Shared-dependency policy status** — present or absent; if present, its mechanism (pinned versions, `singleton`, `strictVersion`, or equivalent) and whether mismatches fail loudly or silently.
- **Blast-radius and ownership status per remote** — error-boundary/isolation mitigation present, independent deploy/rollback path present, owning team named.
- **Ranked findings** — file:line or config-key evidence where applicable, risk class (isolation gap, versioning gap, ownership gap, blast-radius gap), and a concrete fix.
- **Verdict** — approve / approve-with-notes / block.
- **Evidence level** — `live evidence` (read from actual config/source), `user-provided evidence` (asserted by the user without config to confirm), `documentation-based` (React/CSP/tooling docs grounding a claim), or `documentation-based, unconfirmed` (a claim the official docs do not explicitly confirm, e.g. cross-major-version React compatibility across remotes) — plus open questions the proposer must answer before the boundary can be considered fully reviewed.

## Verification targets

- The federation config's `shared` block (or build-time composition manifest) — the source of truth for the version-compatibility policy, not the proposer's verbal description of it.
- The mount-point code for each remote — to confirm an error boundary or equivalent isolation wraps it, not just that one exists "somewhere."
- The deployment pipeline configuration — to confirm each remote's independent deploy/rollback path actually exists as a distinct pipeline stage or repository, not merely as an intention.
