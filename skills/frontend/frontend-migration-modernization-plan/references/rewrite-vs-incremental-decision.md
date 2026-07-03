# Rewrite vs. Incremental Strangler Decision

Use this reference only when the user is genuinely undecided between a full rewrite and an incremental strangler-fig migration. If the user has already committed to one path, do not relitigate it here — go to the relevant execution reference instead.

## What people get wrong

The common bad assumption is:

> "The old code is so bad that a rewrite is obviously faster."

That is almost never true at the scope people imagine. A rewrite discards the accumulated bug fixes, edge-case handling, and business-rule knowledge encoded in the legacy UI — knowledge that usually is not written down anywhere else. Teams that rewrite typically re-discover that knowledge the hard way, in production, after cutover, when rollback is hardest.

The inverse bad assumption is also common:

> "We should never rewrite; incremental is always safer."

That is also false. If the legacy runtime model is fundamentally incompatible with the target (e.g. synchronous global-state jQuery plugins that assume full-page reloads, wired into a codebase with no seams at all), forcing an incremental strangler pattern onto it can cost more than a bounded rewrite of a small, well-scoped surface.

## Decision criteria (score each, do not eyeball it)

Ask these questions and get concrete answers, not vibes:

1. **Are there any existing seams?** Route boundaries, iframe boundaries, distinct page loads, or module boundaries where legacy and new code could coexist without deep coupling. No seams at all is the strongest signal toward a bounded rewrite of that specific surface — not the whole app.
2. **What is the blast radius of the surface in question?** A single internal admin page vs. the primary customer checkout flow have very different risk tolerances for "big bang" replacement.
3. **Is there a shared auth/session model that both stacks must honor simultaneously?** If yes, this is a hard security gate (see SKILL.md operating rules) and it must be solved before any coexistence phase, regardless of which path you choose.
4. **How much business logic is embedded in the view layer with no test coverage?** High untested logic in the view layer favors strangling in small verifiable slices, not a big-bang rewrite, because you can diff behavior slice-by-slice.
5. **What is the team's real capacity to run two stacks in production concurrently?** Strangler patterns are not free — they cost ongoing dual-stack operational overhead (two build pipelines, two deploy paths, two sets of dependencies to patch). If the team cannot sustain that for the migration's duration, say so explicitly; it changes the sequencing, not the recommendation to avoid rewrite.
6. **Is the target framework migration path officially supported for incremental coexistence?** For example, Next.js explicitly documents `app/` and `pages/` directories coexisting and recommends breaking migration into small steps (Context7-verified: `vercel/next.js` docs, `app-router-migration.mdx`). Absence of an official coexistence path for a given framework pair is a signal toward more conservative, smaller-scoped increments — not automatically toward a rewrite.

## Default and its exception

**Default: incremental strangler-fig migration**, scoped by seam (route, module federation boundary, or adapter layer), sequenced business-risk-first (§ see SKILL.md phase sequencing).

**Exception — recommend a bounded rewrite only when:**
- no viable seam exists for the surface in question, AND
- the surface is small/isolated enough that its blast radius is contained, AND
- the team explicitly accepts the rewrite's discovery risk (silent behavior regressions) with a stated verification plan (parity testing, feature-flagged rollout, or shadow traffic comparison).

Never recommend a rewrite for an entire application as a first move. If the user pushes for "just rewrite it all," push back and ask for the seam analysis above first.

## When to push back

Push back if the user says:

- "The old code is a mess, let's just start over" — with no seam analysis and no scoping of blast radius.
- "We'll run both stacks forever, it's fine" — dual-stack maintenance without a decommission time-box is not a migration, it's permanent debt (see `rollback-and-exit-criteria.md`).
- "Auth can stay separate for now, we'll unify it later" — unresolved shared-session risk is a security gate, not a deferred nice-to-have.

Those are not pragmatic shortcuts. They are how migrations become permanent, half-finished liabilities.
