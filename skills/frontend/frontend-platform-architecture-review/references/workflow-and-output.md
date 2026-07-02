# Review Workflow and ADR Output Contract

Use this reference for the step-by-step review procedure, the decision tree governing approve/approve-with-conditions/reject, and the required output shape. Load it for every review; it is the orchestration layer the other reference does not cover.

## What people get wrong

The naive story is:

> The proposer already explained why this is a good idea, so my job is to sanity-check their reasoning.

Wrong. The proposer's framing is evidence, not a starting truth. A reviewer who only checks internal consistency of the proposal will approve a well-argued duplication, a well-argued rewrite, and a well-argued a11y-deferral, because those things can all be argued well. The job is to check the proposal against the repo's actual state and against enterprise standards the proposer did not necessarily consider — not to grade the proposal's own essay.

## Step-by-step workflow

1. **Classify the proposal.** New capability, migration of an existing capability, or boundary redraw. This determines which checks apply — a new capability has no prior art to compare against; a migration must justify the replacement of what exists; a boundary redraw must show who owns what after the change.
2. **Check for an existing in-repo equivalent.** Search the repo (Grep/Glob) for the capability the proposal claims to introduce. If one exists, the proposal must explicitly justify why the existing one cannot be extended — silence on this is a duplication defect.
3. **Verify every version-sensitive technical claim via Context7.** Resolve the library, confirm the repo's installed major version from `package.json` or lockfile, then query docs for that version. Label each claim `Context7-verified` or `documentation-based — unverified this session`.
4. **Require at least two alternatives considered with tradeoffs.** A single-option proposal is not ready for a verdict; send it back with this specific gap named rather than guessing at alternatives on the proposer's behalf.
5. **Require an incremental migration plan with a rollback path.** Reject "big bang" proposals by default. If the proposer argues incremental migration is infeasible, that argument itself needs evidence (e.g., tight coupling that genuinely cannot be strangled), not just assertion.
6. **Confirm a11y and security posture are explicitly addressed.** Not assumed, not deferred to "we'll handle it in implementation." A rendering or module-boundary change that affects focus order, hydration timing, or bundle boundaries has a11y and security surface area; the proposal must name it.
7. **Confirm Core Web Vitals budget impact is stated for any rendering-strategy change.** Lab or field data, or an explicit estimate labeled `inference, not measured` with a commitment to measure post-launch. Silence is not acceptable for a rendering change.
8. **Issue a verdict.** Approve, approve-with-conditions, or reject-with-reasoning, using the decision tree below.

## Decision tree

- Proposal duplicates an existing capability → **reject** unless the proposer justifies why the existing one cannot be extended.
- Proposal has no rollback path → **reject-with-conditions**, requiring one before re-review.
- Proposal is framed as a rewrite but an incremental strangler-fig path plausibly exists → **reject the rewrite framing**; request an incremental plan, do not simply approve the rewrite because it is well-written.
- A11y or security posture is unaddressed → **reject-with-conditions**; this is never waived even if the proposer considers it out of scope.
- Rendering-strategy change with no stated Core Web Vitals impact → **reject-with-conditions**; require lab or field data, or an explicit measurement commitment.
- None of the above triggers apply → **approve** or **approve-with-conditions** based on any remaining, narrower gaps (e.g., missing ownership documentation, unresolved naming collision).

## ADR-format output contract

Every verdict must be returned in this shape:

1. **Context** — what is being proposed, which teams/modules are affected, why now.
2. **Decision recommendation** — approve / approve-with-conditions / reject-with-reasoning, stated plainly in the first line.
3. **Duplication check** — existing in-repo equivalent found (with file/module evidence) or none found, and the search performed.
4. **Alternatives considered** — the two or more options and their tradeoffs, as supplied by the proposer or explicitly flagged as missing.
5. **Version-sensitive claims** — each claim labeled `Context7-verified` (with the library/version queried) or `documentation-based — unverified this session`.
6. **Consequences** — what this decision commits the org to, including maintenance burden and the "third way to do X" cost if a competing capability now exists.
7. **Rollback / incremental migration plan** — the specific mechanism (feature flag, route-level cutover, strangler-fig boundary), or its absence flagged as a blocking condition.
8. **A11y and security posture** — explicit statement of impact and mitigation, or flagged as a blocking condition if silent.
9. **Core Web Vitals budget impact** — for rendering-strategy changes only; lab/field data or a labeled estimate with a measurement commitment.
10. **Unresolved conditions** — the specific, named items blocking full approval, if any.

Do not compress this into prose paragraphs when the proposal is non-trivial; use the numbered shape so gaps are visible at a glance to the next reviewer.
