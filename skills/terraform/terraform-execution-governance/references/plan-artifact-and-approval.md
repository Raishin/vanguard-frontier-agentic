# Plan Artifacts, Binding, And Approval Integrity

Whether the reviewed plan is the applied plan, and what makes an approval a control rather than a click.

- An apply that consumes a plan saved with `-out` applies exactly the reviewed changes; an apply that re-plans applies whatever the configuration and current remote state produce at that moment, which can differ from what the approver read.
- A saved plan file records sensitive values in cleartext, so moving it between pipeline stages moves a secret: it needs the handling given to state, not the handling given to a build artifact.
- A plan artifact retained after the apply, readable by unrelated jobs, or printed into a log becomes a durable credential exposure that outlives the change it described.
- Approval is a control only when three separate properties hold: the approver can see the actual plan, the author cannot approve their own change, and the gate cannot be bypassed without a record. Most pipelines satisfy some of these, and the missing one is the one that gets used.
- An approval that shows a diff of the configuration rather than the plan approves the intent instead of the effect, and the two differ precisely in the cases that matter — forced replacements, provider-driven changes, and drift resolution.
- Every trigger that can reach the apply path is part of the permission model: fork pull requests, comment commands, tag pushes, scheduled runs, and manual dispatch each need an explicit answer, and the dangerous path is usually one nobody enumerated.
- An emergency bypass is part of the control design rather than an exception to it; if its use is not recorded and reviewed, the bypass is the real permission model and the ordinary gate is decoration.
- Unattended apply is defensible when the blast radius is mechanically bounded, but a stated intention to auto-apply only safe changes, with no check enforcing what safe means, is an unenforced boundary rather than a policy.
