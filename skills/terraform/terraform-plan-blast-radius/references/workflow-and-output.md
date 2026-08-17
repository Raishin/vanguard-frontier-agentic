# Workflow And Output

Diagnostic sequence and output contract for plan blast-radius review.

## Workflow

1. Establish the artifact actually supplied — per-resource plan, `-json` plan, summary line only, or source alone — and set the evidence ceiling for every downstream claim accordingly.
2. Extract every replace and destroy action and attribute each to a specific attribute change, a `replace_triggered_by` trigger, an address change, or an unexplained cause.
3. For each replaced or destroyed resource, decide whether it stores data, and require a named backup or reconstruction path before allowing anything but a block verdict.
4. Check replacement ordering: whether `create_before_destroy` is set where an outage would otherwise occur, and what it transitively drags with it.
5. Look for address churn — `count`/`for_each` changes, renames, module restructures — and name the `moved` blocks required.
6. Check scope and masking: `-target`, `-replace`, `ignore_changes`, and whether any of them is hiding a change from review.
7. State whether the plan was saved with `-out` and therefore binds the apply, or whether apply will re-plan.
8. List orphans, hand off cloud-specific consequences, and name the smallest artifact that would settle what remains open.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and an explicit statement of whether it binds the apply.
- Every replacement and destroy attributed to the attribute, trigger, or address change that caused it.
- A data-loss assessment for each stateful resource replaced or destroyed, naming the backup or reconstruction path.
- Address-churn findings with the required `moved` blocks, plus any orphans the plan leaves unmanaged.
- Severity- and evidence-labelled findings, and the smallest artifact that would settle any open question.
