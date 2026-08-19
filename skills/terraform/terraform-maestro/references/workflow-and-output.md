# Workflow And Output

Classification sequence and the routing output contract.

## Workflow

1. Identify the decision being made, not the technology mentioned — `terraform` appears in almost every task on this board and carries no routing signal by itself.
2. Match the decision to exactly one owning specialist from the taxonomy.
3. Test each additional specialist against its written threshold; add none that fails its threshold.
4. Check for a live-execution intent (apply, destroy, state mutation, force-unlock). If present, stop and gate before naming any agent.
5. Emit Route / Reason / Mode, dispatch, then synthesize without re-answering the question yourself.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A three-line routing decision: Route / Reason / Mode.
- The thresholds crossed and, when the mode is `single`, why the obvious second specialist was not added.
- Any cross-board handoff (cloud resource semantics, cost, live execution) named explicitly.
- For any live path: what is destroyed, whether it is reversible, the rollback path, and an explicit stop for written human confirmation.
