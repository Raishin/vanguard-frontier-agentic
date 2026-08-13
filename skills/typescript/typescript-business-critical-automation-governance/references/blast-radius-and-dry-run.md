# Blast Radius And Dry-Run Controls

How to bound scope and verify a dry-run covers the write path.

- A dry-run is only a control if it is demonstrated to execute every code path up to, and not including, the actual write — a dry-run that returns early before reaching the write-path branch verifies nothing about that branch.
- Blast radius is bounded by selection criteria, batch size, and scope; an operation with no explicit bound (an unfiltered query, an unbatched loop over an entire table) has an unbounded blast radius regardless of how careful its individual write logic is.
- A credential broader than the operation's actual need is itself a blast-radius finding, independent of the script's own logic, because it sets the ceiling on what a defect in that logic can reach.
- Approval separation requires the approving party be unable to also trigger the run through the same credential or account — documentation of a required approval step is not evidence the mechanism enforces it.
