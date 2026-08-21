# Workflow And Output

Diagnostic sequence and output contract for reconciliation work.

## Workflow

1. Establish which artifacts were supplied — `-refresh-only` plan, normal plan, source, inventory — and set the evidence ceiling accordingly.
2. Separate drift from configuration change; if only a normal plan is available, say that the two are entangled and ask for a refresh-only plan.
3. Classify each drift item as unauthorized, authorized out-of-band, externally owned, or provider artifact, and assign a disposition with an owner.
4. For adoption, determine per resource type whether `id` or `identity` addressing applies, citing the provider's own documentation.
5. Define the no-op plan gate that proves the import is complete, and name the attribute differences that would indicate it is not.
6. For refactors, enumerate the exact `moved` blocks, and state explicitly for any `removed` block whether the outcome is release or destroy.
7. Sequence the work so each step is independently verifiable and reversible, and name the state preconditions before the first mutation.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the engine and version posture assumed.
- Every drift item classified and given an explicit disposition, with a named owner for anything accepted.
- For an import: the addressing form per resource, the provider documentation relied on, and the no-op plan gate.
- For a refactor: the exact `moved` blocks required, and for any release the explicit release-versus-destroy outcome.
- The adoption sequence with a verification gate between steps, and the state preconditions owed to `terraform-state-reliability-agent`.
