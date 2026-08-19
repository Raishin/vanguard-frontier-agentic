# Exceptions, Expiry, And Evidence Artifacts

What makes an exception governable and what makes evidence retrievable a year later.

- An exception without an expiry is a permanent policy change made without a policy review, and it is indistinguishable from a suppression once the person who granted it has moved on.
- An exception's real blast radius is the set of future changes it permits, not the change it was granted for; an exception scoped to a repository or a workspace covers everything that will ever be added there, including resources nobody has designed yet.
- An exception needs a named accountable owner rather than a team, because a team cannot be asked whether the justification still holds, and renewal is the only mechanism that ever removes an exception.
- Evidence must be reproducible without re-running the change: if proving a control was satisfied requires re-planning against infrastructure that has since moved on, the evidence does not exist regardless of how carefully the control was implemented.
- The retained artifact should record what the policy evaluated, not merely that it passed — a pass record without the evaluated input cannot distinguish a control that examined the change from one that examined nothing.
- Plan output can contain sensitive values in cleartext, so an evidence artifact derived from a plan needs the same handling as state; an evidence pipeline that leaks a credential has created an incident rather than a control.
- Audit readiness is measured by time to produce evidence for a named change. Counting defined policies measures activity, and the two numbers routinely move in opposite directions.
- Evidence produced as a side effect of the change is retrievable; evidence assembled during an audit is reconstructed, and reconstruction is where the engineer weeks go.
