# Workflow And Output

Assessment sequence and output contract for compatibility and engine decisions.

## Workflow

1. Fix the exact source and target versions; refuse to assess a range, since upgrade guidance is written per version pair.
2. Read the upgrade guidance for that pair and separate changes that error from changes that produce forced replacements.
3. Check the change against the compatibility promise and state what the promise does not cover.
4. Determine the ordering, and flag any combined move that would make attribution impossible.
5. Assess rollback: whether the state written by the target version is readable by the source version, and name the restore path if it is not.
6. Inventory deprecations in the estate and record remaining notice periods.
7. For an engine decision, build the divergence register and enumerate configurations coupled by `terraform_remote_state`.
8. Define the verification plan — which plan, in which workspace — that must pass before adoption.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (adopt / adopt-with-conditions / defer / block) naming the exact source and target versions assessed.
- Breaking changes for that version pair, split into those that error and those that surface as forced replacements.
- An explicit rollback assessment, including the state restore path whenever a binary downgrade will not work.
- The upgrade order and any combination that is unsupported rather than merely untested.
- For an engine decision: a divergence register naming what exists on one engine only, and what this estate would forfeit.
