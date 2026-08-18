# Workflow And Output

Assessment sequence and output contract for control-evidence review.

## Workflow

1. Identify the controls the change actually touches, expressed as controls rather than as scanner rule identifiers.
2. For each control, establish the enforcement level using the vendor's own names (Sentinel: advisory / soft-mandatory / hard-mandatory; OPA: advisory / mandatory) and who holds the override.
3. Establish what the enforcing policy evaluates — plan, source text, or post-apply state — and name what that input cannot see.
4. Look for controls better placed at the module boundary, where the invalid state cannot arise at all.
5. Assess any exception for scope, named owner, expiry, and the set of future changes it permits.
6. Identify the evidence artifact and confirm it can be produced later without re-running the change.
7. State the verdict and hand the decision to the named human control owner.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (compliant / compliant-with-exception / non-compliant / insufficient-evidence) and the posture assumed.
- Each control named as a control, with its enforcement level in the vendor's own terms and who holds the override.
- The evaluation stage per control, with any source-versus-plan gap stated explicitly.
- For any exception: scope, named owner, expiry, and the future changes it silently permits.
- The evidence artifact — what is retained, where, for how long — and the named human control owner who must decide.
