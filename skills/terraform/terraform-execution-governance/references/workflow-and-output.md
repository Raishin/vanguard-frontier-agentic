# Workflow And Output

Assessment sequence and output contract for execution-path review.

## Workflow

1. Establish which artifacts were supplied — workflow definitions, runner image, CLI configuration, trust policies — and mark the assessment incomplete if the runner environment is unavailable.
2. Identify the principal that runs plan and the principal that runs apply, and whether they are the same.
3. Determine credential lifetime and permission scope for each, and compare the scope against what the configuration actually manages.
4. Determine whether apply consumes a saved plan or re-plans, and state which.
5. Trace the plan artifact: where it is written, who can read it, how long it is kept, and whether it reaches a log.
6. Assess approval as three separate properties: can the approver see the plan, can the author approve it, can it be bypassed.
7. Enumerate every trigger that can reach apply, including forks, comments, tags, schedules, and manual dispatch.
8. Establish where execution actually happens and which trust boundary therefore applies.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and whether the artifacts supplied were sufficient to assess the path at all.
- The identity running plan and the identity running apply, with credential lifetime and permission scope for each.
- An explicit statement of whether apply consumes the reviewed saved plan or re-plans.
- Approval integrity answered as three separate questions: visibility, author-separation, and bypass.
- Every trigger that can reach the apply path, and the handling of plan artifacts that contain cleartext secrets.
