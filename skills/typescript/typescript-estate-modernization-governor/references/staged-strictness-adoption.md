# Staged Strictness Adoption

Sequencing patterns with rollback points, and the criteria for not migrating.

- Strict-family flags can be adopted per-package or per-directory in a large estate without a code freeze, provided each unit has a defined completion criterion rather than an open-ended suppression allowance.
- A staged rollout is only evidence of progress if the suppression count (`@ts-ignore`/`@ts-expect-error`) and `skipLibCheck` usage are trending down across successive checkpoints, not merely present at rollout start.
- Since TypeScript 6.0 made `strict` default-true, a package's tsconfig with no explicit strictness setting inherits strict — a staged-adoption plan must check for explicit opt-outs, not assume silence means legacy behavior.
- A migration step qualifies for a rollback point only when reverting it is both mechanically possible (the prior config/version is restorable) and has been exercised, not merely theoretically available.
- Not migrating is a legitimate outcome of this analysis when a package has no active maintainer, no consumer depending on the newer behavior, and no removed-value blocker forcing the issue.
