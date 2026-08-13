# Workflow And Output

Diagnostic sequence and output contract for estate-modernization review.

## Workflow

1. Inventory current compiler/runtime versions per package and cross-check against the removed-value blocker list.
2. Confirm the TS 6.0/7.0 tooling split is named for any package touching the compiler's programmatic API.
3. Check staged-strictness adoption for a stated unit and a decreasing suppression/skipLibCheck trend.
4. Check the ownership map for any unowned package on the upgrade's critical path.
5. Sequence the migration into steps, each with a named rollback point, before recommending an order.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the compiler/runtime version split assumed.
- Sequencing/reversibility, removed-value blocker, suppression-debt, staged-strictness, and portfolio-prioritization findings, each with an evidence-basis label.
- A severity-labelled finding list plus safe next actions and open questions, including anything `typescript-engineering-economics-agent` or `frontend-migration-modernization-agent` must confirm.
